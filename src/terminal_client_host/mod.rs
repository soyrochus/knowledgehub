mod wire;
mod workspace;
use knowledgehub::{backends::files::*, *};
use serde::de::DeserializeOwned;
use serde_json::{Value as Json, json};
use std::{
    collections::BTreeMap,
    io,
    path::{Path, PathBuf},
};
use workspace::{Authority, invalid};
const METHODS: &[&str] = &[
    "models",
    "open",
    "info",
    "head",
    "authorize",
    "read",
    "query",
    "continue_query",
    "neighbours",
    "prepare_neighbours",
    "continue_neighbours",
    "search_text",
    "traverse",
    "continue_traverse",
    "publication",
    "publications",
    "publication_affected",
    "continue_history",
    "release_history",
    "compare",
    "artifact",
    "artifact_range",
    "release_cursor",
    "drain",
];
struct Session {
    hub: KnowledgeHub,
    auth: Authority,
}
fn args(r: &wire::Request, allowed: &[&str]) -> Result<Json, Error> {
    let v = if r.args.is_null() {
        json!({})
    } else {
        r.args.clone()
    };
    let m = v
        .as_object()
        .ok_or_else(|| invalid("args object required"))?;
    if m.keys().any(|k| !allowed.contains(&k.as_str())) {
        return Err(invalid(
            "unknown request argument or authority/path override",
        ));
    }
    Ok(v)
}
fn field<T: DeserializeOwned>(j: &Json, key: &str) -> Result<T, Error> {
    serde_json::from_value(
        j.get(key)
            .ok_or_else(|| invalid("missing required argument"))?
            .clone(),
    )
    .map_err(|_| invalid("invalid typed argument"))
}
fn request<T: DeserializeOwned>(j: &Json) -> Result<T, Error> {
    let mut q = j
        .get("request")
        .ok_or_else(|| invalid("request required"))?
        .clone();
    wire::request_numbers(&mut q)?;
    serde_json::from_value(q).map_err(|_| invalid("invalid native request fields"))
}
fn summary(r: &Record) -> Json {
    let name = match &r.data {
        RecordData::Element(e) => e
            .properties
            .get("name")
            .and_then(knowledgehub::Value::as_text)
            .map(str::to_owned),
        RecordData::Type(t) => Some(t.name.clone()),
        RecordData::Relationship(t) => Some(t.name.clone()),
        RecordData::SourceArtifact(s) => Some(s.origin.clone()),
        RecordData::Anchor(a) => Some(a.label.clone()),
        _ => None,
    };
    json!({"reference":r.reference,"type_ref":r.type_ref,"kind":r.data.native_kind().unwrap_or("Element"),"name":name,"predecessor":r.predecessor,"provenance":r.provenance,"sha256":r.sha256})
}
fn page(p: Page, projection: &str) -> Result<Json, Error> {
    let all = p
        .records
        .iter()
        .chain(&p.assertions)
        .map(|r| {
            if projection == "summary" {
                Ok(summary(r))
            } else {
                wire::owned(r)
            }
        })
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(
        json!({"selection":p.selection,"include_subtypes":p.include_subtypes,"projection":projection,"items":all,"continuation":p.continuation,"truncated":p.truncated,"exhausted":p.exhausted}),
    )
}
fn build() -> Json {
    use sha2::{Digest, Sha256};
    let mut d = Sha256::new();
    d.update(include_bytes!("mod.rs"));
    d.update(include_bytes!("workspace.rs"));
    d.update(include_bytes!("wire.rs"));
    json!({"library":env!("CARGO_PKG_VERSION"),"source_sha256":format!("{:x}",d.finalize()),"os":std::env::consts::OS,"arch":std::env::consts::ARCH,"record_codec":RECORD_FORMAT,"request_codec":REQUEST_FORMAT,"file_format":FILE_FORMAT,"catalogue":ontology_ref(),"history_projection":HISTORY_FORMAT})
}
async fn dispatch(
    file: &Path,
    sessions: &mut BTreeMap<String, Session>,
    r: &wire::Request,
) -> Result<Json, Error> {
    if r.method == "handshake" {
        let a = args(r, &["protocol"])?;
        if a["protocol"] != wire::PROTOCOL {
            return Err(Error::Incompatible("client protocol".into()));
        }
        return Ok(
            json!({"protocol":wire::PROTOCOL,"build":build(),"methods":METHODS,"limits":{"frame_bytes":wire::FRAME.to_string(),"result_bytes":wire::RESULT.to_string(),"pending":"16","source_chunk":"65536","history":wire::owned(&HistoryLimits::MAX)?,"cursor_capacity":"64","cursor_idle_seconds":"1800"},"permissions":{"ordinary":["Open","Read"],"history":["Open","Read","ReadHistory"]}}),
        );
    }
    if !METHODS.contains(&r.method.as_str()) {
        return Err(invalid("unsupported read-only method"));
    }
    if r.method == "drain" {
        args(r, &[])?;
        for (_, s) in std::mem::take(sessions) {
            s.hub.drain().await;
        }
        return Ok(json!({"drained":true}));
    }
    let w = workspace::load(file)?;
    if r.method == "models" {
        args(r, &[])?;
        return Ok(
            json!({"default":w.default,"models":w.models.iter().map(|e|json!({"alias":e.alias,"name":e.name,"scope":e.scope})).collect::<Vec<_>>()}),
        );
    }
    let alias = r.model.as_ref().unwrap_or(&w.default).clone();
    if !sessions.contains_key(&alias) {
        let entry = w
            .models
            .into_iter()
            .find(|e| e.alias == alias)
            .ok_or_else(|| invalid("unknown configured model alias"))?;
        let directory = workspace::directory(file, &entry)?;
        let auth = Authority {
            file: file.to_path_buf(),
            identity: workspace::identity(&directory)?,
            directory: directory.clone(),
            entry,
        };
        auth.check(Action::Open)?;
        auth.check(Action::Read)?;
        let hub = KnowledgeHub::open(
            FileBackend::new(directory, FileOptions::native_local()?)?,
            auth.context(),
            auth.entry.scope.clone(),
        )
        .await?;
        sessions.insert(alias.clone(), Session { hub, auth });
    }
    let s = &sessions[&alias];
    s.auth.current()?;
    s.auth.check(Action::Open)?;
    s.auth.check(Action::Read)?;
    let h = &s.hub;
    let c = s.auth.context();
    let out = match r.method.as_str() {
        "open" | "info" => {
            args(r, &[])?;
            let current = s.auth.current()?;
            let head = h.head(c).await?;
            for b in &current.known_baselines {
                let record = h
                    .read(
                        s.auth.context(),
                        Selection::Snapshot(head.clone()),
                        b.clone(),
                    )
                    .await?;
                if !matches!(record.data, RecordData::Baseline(_)) {
                    return Err(invalid("handover reference is not a native baseline"));
                }
            }
            json!({"alias":alias,"name":current.name,"scope":h.scope(),"head":head,"reader":current.actor,"grants":current.grants,"known_baselines":current.known_baselines,"build":build(),"read_only":true})
        }
        "head" => {
            args(r, &[])?;
            json!({"head":h.head(c).await?})
        }
        "authorize" => {
            let a = args(r, &["history"])?;
            if a["history"].as_bool().unwrap_or(false) {
                s.auth.check(Action::ReadHistory)?;
            }
            json!({"authorized":true})
        }
        "read" => {
            let a = args(r, &["selection", "reference"])?;
            wire::owned(
                &h.read(c, field(&a, "selection")?, field(&a, "reference")?)
                    .await?,
            )?
        }
        "query" | "continue_query" => {
            let a = args(r, &["request", "continuation", "projection"])?;
            let q: Query = request(&a)?;
            let projection = a["projection"].as_str().unwrap_or("records");
            if !matches!(projection, "records" | "summary") {
                return Err(invalid("unknown projection"));
            }
            let p = if r.method == "query" {
                h.query(c, q).await?
            } else {
                h.continue_query(c, q, field(&a, "continuation")?).await?
            };
            page(p, projection)?
        }
        "neighbours" | "continue_neighbours" => {
            let a = args(r, &["request", "continuation"])?;
            let q: Neighbours = request(&a)?;
            wire::owned(&if r.method == "neighbours" {
                h.neighbours(c, q).await?
            } else {
                h.continue_neighbours(c, q, field(&a, "continuation")?)
                    .await?
            })?
        }
        "prepare_neighbours" => {
            let a = args(r, &["request"])?;
            wire::owned(&h.prepare_neighbours(c, request(&a)?).await?)?
        }
        "search_text" => {
            let a = args(r, &["request"])?;
            wire::owned(&h.search_text(c, request(&a)?).await?)?
        }
        "traverse" | "continue_traverse" => {
            let a = args(r, &["request", "continuation"])?;
            let q: Traverse = request(&a)?;
            wire::owned(&if r.method == "traverse" {
                h.traverse(c, q).await?
            } else {
                h.continue_traverse(c, q, field(&a, "continuation")?)
                    .await?
            })?
        }
        "publication" => {
            let a = args(r, &["request"])?;
            wire::owned(&h.publication(c, request(&a)?).await?)?
        }
        "publications" => {
            let a = args(r, &["request"])?;
            wire::owned(&h.publications(c, request(&a)?).await?)?
        }
        "publication_affected" => {
            let a = args(r, &["request"])?;
            wire::owned(&h.publication_affected(c, request(&a)?).await?)?
        }
        "continue_history" => {
            let a = args(r, &["request", "continuation"])?;
            let mut q = a["request"].clone();
            if let Some(m) = q.as_object_mut() {
                for v in m.values_mut() {
                    wire::request_numbers(v)?;
                }
            }
            let q: HistoryRequest =
                serde_json::from_value(q).map_err(|_| invalid("invalid history intent"))?;
            wire::owned(&h.continue_history(c, q, field(&a, "continuation")?).await?)?
        }
        "release_history" | "release_cursor" => {
            let a = args(r, &["continuation"])?;
            let t = field(&a, "continuation")?;
            if r.method == "release_history" {
                h.release_history(c, t).await?;
            } else {
                h.release_cursor(c, t).await?;
            }
            json!({"released":true})
        }
        "compare" => {
            let a = args(r, &["before", "after"])?;
            Json::Array(h.compare(c,field(&a,"before")?,field(&a,"after")?).await?.iter().map(|d|json!({"identity":d.identity,"before":d.before,"after":d.after,"properties_changed":d.properties_changed,"provenance_changed":d.provenance_changed,"representation_changed":d.representation_changed})).collect())
        }
        "artifact" => {
            let a = args(r, &["selection", "reference"])?;
            let data = h
                .read_artifact(c, field(&a, "selection")?, field(&a, "reference")?)
                .await?;
            json!({"original":wire::owned(&data.reference)?,"bytes_base64":wire::base64(&data.bytes),"full_content_verified":true})
        }
        "artifact_range" => {
            let a = args(r, &["selection", "reference", "offset", "length"])?;
            let off: String = field(&a, "offset")?;
            let len: String = field(&a, "length")?;
            let length = wire::decimal(&len)?;
            if length > 65536 {
                return Err(invalid("source chunk exceeds 65536 bytes"));
            }
            let data = h
                .read_artifact_range(
                    c,
                    field(&a, "selection")?,
                    field(&a, "reference")?,
                    wire::decimal(&off)?,
                    length,
                )
                .await?;
            json!({"selection":data.selection,"reference":data.reference,"original":wire::owned(&data.original)?,"offset":data.offset.to_string(),"bytes_base64":wire::base64(&data.bytes),"full_content_verified":data.full_content_verified})
        }
        _ => return Err(invalid("unsupported method")),
    };
    s.auth.check(Action::Read)?;
    if matches!(
        r.method.as_str(),
        "publication" | "publications" | "publication_affected" | "continue_history"
    ) {
        s.auth.check(Action::ReadHistory)?;
    }
    // A token in an indivisible oversized response was never disclosed. Release
    // it here rather than silently occupying the shared pool until host exit.
    if !wire::fits(r, &out)?
        && let Some(token) = out.get("continuation").and_then(Json::as_str)
    {
        if matches!(
            r.method.as_str(),
            "publications" | "publication_affected" | "continue_history"
        ) {
            let _ = h.release_history(s.auth.context(), token.to_owned()).await;
        } else {
            let _ = h.release_cursor(s.auth.context(), token.to_owned()).await;
        }
    }
    Ok(out)
}
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
pub fn run() -> Result<(), Error> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 2 || args[0] != "--workspace" {
        return Err(invalid("usage: knowledgehub-client-host --workspace FILE"));
    }
    let file = workspace::ordinary(&PathBuf::from(&args[1]))?;
    workspace::load(&file)?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .map_err(|_| Error::Unavailable("host runtime".into()))?;
    let mut sessions = BTreeMap::new();
    let mut input = io::stdin().lock();
    let mut output = io::stdout().lock();
    let mut handshaken = false;
    let outcome = loop {
        let r = match wire::read(&mut input) {
            Ok(Some(r)) => r,
            Ok(None) => break Ok(()),
            Err(e) => break Err(e),
        };
        let result = if let Err(e) = wire::limit(&r) {
            Err(e)
        } else if !handshaken && r.method != "handshake" {
            Err(Error::Incompatible("handshake required".into()))
        } else {
            runtime.block_on(dispatch(&file, &mut sessions, &r))
        };
        if r.method == "handshake" && result.is_ok() {
            handshaken = true;
        }
        let quit = r.method == "drain" && result.is_ok();
        if let Err(e) = wire::write(&mut output, &r, result) {
            break Err(e);
        }
        if quit {
            break Ok(());
        }
    };
    runtime.block_on(async {
        for (_, s) in sessions {
            s.hub.drain().await;
        }
    });
    outcome
}
