//! Source-independent standalone host over the public library API.
use knowledgehub::{backends::files::*, ontology::*, *};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value as Json, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
    sync::Arc,
};

const FORMAT: &str = "knowledgehub-authoring/1";
const JSON_LIMIT: u64 = 256 * 1024 * 1024;
type Result<T> = std::result::Result<T, Error>;
fn invalid(s: impl Into<String>) -> Error {
    Error::InvalidInput(s.into())
}
fn io(e: std::io::Error) -> Error {
    Error::Unavailable(e.to_string())
}
fn read(path: &Path, limit: u64) -> Result<Vec<u8>> {
    if fs::symlink_metadata(path)
        .map_err(io)?
        .file_type()
        .is_symlink()
    {
        return Err(invalid(format!("symlink: {}", path.display())));
    }
    let mut bytes = vec![];
    File::open(path)
        .map_err(io)?
        .take(limit + 1)
        .read_to_end(&mut bytes)
        .map_err(io)?;
    if bytes.len() as u64 > limit {
        return Err(invalid(format!("byte limit at {}", path.display())));
    }
    Ok(bytes)
}
fn load<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let limit = if path.file_name().is_some_and(|n| n == "journal.json") {
        JSON_LIMIT * 4
    } else {
        JSON_LIMIT
    };
    let bytes = read(path, limit)?;
    strict_json(&bytes).and_then(|v| {
        serde_json::from_value(v).map_err(|e| invalid(format!("{}: {e}", path.display())))
    })
}
fn strict_json(bytes: &[u8]) -> Result<Json> {
    struct Strict(Json);
    impl<'de> Deserialize<'de> for Strict {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
            struct Visitor;
            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = Strict;
                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str("JSON without duplicate keys")
                }
                fn visit_map<A: serde::de::MapAccess<'de>>(
                    self,
                    mut a: A,
                ) -> std::result::Result<Strict, A::Error> {
                    let mut m = serde_json::Map::new();
                    while let Some((k, v)) = a.next_entry::<String, Strict>()? {
                        if m.insert(k, v.0).is_some() {
                            return Err(serde::de::Error::custom("duplicate JSON key"));
                        }
                    }
                    Ok(Strict(Json::Object(m)))
                }
                fn visit_seq<A: serde::de::SeqAccess<'de>>(
                    self,
                    mut a: A,
                ) -> std::result::Result<Strict, A::Error> {
                    let mut v = vec![];
                    while let Some(x) = a.next_element::<Strict>()? {
                        v.push(x.0);
                    }
                    Ok(Strict(Json::Array(v)))
                }
                fn visit_str<E: serde::de::Error>(self, s: &str) -> std::result::Result<Strict, E> {
                    Ok(Strict(Json::String(s.into())))
                }
                fn visit_bool<E: serde::de::Error>(
                    self,
                    v: bool,
                ) -> std::result::Result<Strict, E> {
                    Ok(Strict(Json::Bool(v)))
                }
                fn visit_unit<E: serde::de::Error>(self) -> std::result::Result<Strict, E> {
                    Ok(Strict(Json::Null))
                }
                fn visit_i64<E: serde::de::Error>(self, v: i64) -> std::result::Result<Strict, E> {
                    Ok(Strict(Json::from(v)))
                }
                fn visit_u64<E: serde::de::Error>(self, v: u64) -> std::result::Result<Strict, E> {
                    Ok(Strict(Json::from(v)))
                }
                fn visit_f64<E: serde::de::Error>(self, v: f64) -> std::result::Result<Strict, E> {
                    serde_json::Number::from_f64(v)
                        .map(|n| Strict(Json::Number(n)))
                        .ok_or_else(|| E::custom("nonfinite JSON"))
                }
            }
            d.deserialize_any(Visitor)
        }
    }
    let mut d = serde_json::Deserializer::from_slice(bytes);
    let value = Strict::deserialize(&mut d).map_err(|e| invalid(e.to_string()))?;
    d.end().map_err(|e| invalid(e.to_string()))?;
    Ok(value.0)
}
fn digest(bytes: &[u8]) -> ContentRef {
    ContentRef::of(bytes)
}
fn encoded(v: &impl Serialize) -> Result<Vec<u8>> {
    serde_json::to_vec_pretty(v).map_err(|e| invalid(e.to_string()))
}
fn full_sync(file: &File) -> Result<()> {
    file.sync_all().map_err(io)?;
    #[cfg(target_os = "macos")]
    {
        use std::os::fd::AsRawFd;
        if unsafe { libc::fcntl(file.as_raw_fd(), libc::F_FULLFSYNC) } == -1 {
            return Err(io(std::io::Error::last_os_error()));
        }
    }
    Ok(())
}
fn save(path: &Path, value: &impl Serialize) -> Result<()> {
    let bytes = encoded(value)?;
    if bytes.len() as u64 > JSON_LIMIT * 4 {
        return Err(invalid("journal byte limit"));
    }
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let temp = path.with_extension(format!("{}-pending", std::process::id()));
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)
        .map_err(io)?;
    f.write_all(&bytes).map_err(io)?;
    full_sync(&f)?;
    fs::rename(&temp, path).map_err(io)?;
    File::open(parent).map_err(io)?.sync_all().map_err(io)?;
    full_sync(&f)
}
fn relative(path: &str) -> Result<&Path> {
    let p = Path::new(path);
    if path.is_empty() || p.components().any(|c| !matches!(c, Component::Normal(_))) {
        return Err(invalid(format!("non-normal relative path: {path}")));
    }
    Ok(p)
}
fn selected(root: &Path, path: &str) -> Result<PathBuf> {
    let joined = root.join(relative(path)?);
    let actual = joined.canonicalize().map_err(io)?;
    if actual != joined || !actual.starts_with(root) {
        return Err(invalid(format!("source path escape/symlink: {path}")));
    }
    Ok(actual)
}
fn new_absolute(path: &Path) -> Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().map_err(io)?.join(path)
    };
    if absolute
        .components()
        .any(|c| matches!(c, Component::ParentDir))
    {
        return Err(invalid("destination/work path traversal"));
    }
    let supplied_parent = absolute
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .ok_or_else(|| invalid("destination/work parent"))?;
    let parent = supplied_parent.canonicalize().map_err(io)?;
    if parent != supplied_parent {
        return Err(invalid("destination/work parent escape/symlink"));
    }
    let name = absolute.file_name().ok_or_else(|| invalid("path name"))?;
    Ok(parent.join(name))
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Limits {
    files: usize,
    file_bytes: u64,
    total_bytes: u64,
    records: usize,
    pages: usize,
    page_size: usize,
    subjects: usize,
    depth: usize,
    visited: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            files: 10000,
            file_bytes: 32 << 20,
            total_bytes: 256 << 20,
            records: 100000,
            pages: 1000,
            page_size: 1000,
            subjects: 10000,
            depth: 32,
            visited: 100000,
        }
    }
}
impl Limits {
    fn check(&self) -> Result<()> {
        if [
            self.files,
            self.records,
            self.pages,
            self.page_size,
            self.subjects,
            self.depth,
            self.visited,
        ]
        .contains(&0)
            || self.page_size > 1000
            || self.subjects > 10000
            || self.depth > 32
            || self.file_bytes == 0
            || self.total_bytes == 0
            || self.total_bytes > JSON_LIMIT
            || self.file_bytes > self.total_bytes
            || self.visited > 100000
        {
            return Err(invalid(
                "unsupported finite limits; native page<=1000 subjects<=10000 depth<=32, capture<=256MiB",
            ));
        }
        Ok(())
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    key: String,
    path: String,
    media_type: String,
    authority: String,
    status: String,
    rendition_group: Option<String>,
    dependencies: Vec<String>,
    exclude_reason: Option<String>,
    expected: Option<ContentRef>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    format: String,
    source: PathBuf,
    destination: PathBuf,
    work: PathBuf,
    actor: String,
    grants: BTreeSet<String>,
    scope: ModelScope,
    inputs: Vec<Input>,
    authority: Authority,
    #[serde(default)]
    limits: Limits,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Authority {
    field_owners: BTreeMap<String, String>,
    precedence: Vec<String>,
    unresolved_conflicts: Vec<String>,
    review_status: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Source {
    input: Input,
    original: ContentRef,
    missing_dependencies: Vec<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PlanRecord {
    key: String,
    type_ref: Json,
    data: Json,
    provenance: Vec<String>,
    capture: Option<Vec<u8>>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    format: String,
    catalogue: ExactRef,
    profile: ExactRef,
    records: Vec<PlanRecord>,
    gaps: Json,
    baseline: String,
    selection: Option<(String, String)>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Obligation {
    id: String,
    source: String,
    disposition: String,
    reason: Option<String>,
    targets: Vec<Mapping>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Mapping {
    key: String,
    path: Option<String>,
    expected: Option<Json>,
    anchor: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Check {
    name: String,
    kind: String,
    query: Json,
    expected_ids: BTreeSet<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Coverage {
    format: String,
    items: Vec<Obligation>,
    queries: Vec<Check>,
    interpretation_review: String,
    source_gaps: Vec<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Dispatch {
    actor: String,
    request: Change,
    fingerprint: String,
    receipt: Option<Receipt>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Journal {
    format: String,
    config: Config,
    sources: Vec<Source>,
    plan_digest: Option<ContentRef>,
    coverage_digest: Option<ContentRef>,
    key_map: BTreeMap<String, ExactRef>,
    operations: Vec<Operation>,
    dispatches: Vec<Dispatch>,
    initialization: bool,
    ready: bool,
    baseline: Option<ExactRef>,
    head: Option<SnapshotId>,
}
#[derive(Clone)]
struct Host {
    actor: String,
    scope: ModelScope,
    grants: BTreeSet<String>,
}
impl Policy for Host {
    fn allows(
        &self,
        actor: &str,
        scope: &ModelScope,
        action: Action,
    ) -> std::result::Result<bool, PolicyFailure> {
        Ok(actor == self.actor
            && scope == &self.scope
            && self.grants.contains(&format!("{action:?}")))
    }
}
fn context(c: &Config) -> Context {
    Context::new(
        c.actor.clone(),
        Arc::new(Host {
            actor: c.actor.clone(),
            scope: c.scope.clone(),
            grants: c.grants.clone(),
        }),
    )
}
fn backend(c: &Config) -> Result<FileBackend> {
    FileBackend::new(&c.destination, FileOptions::native_local()?)
}
fn ref_key(c: &Config, key: &str) -> ExactRef {
    ExactRef::new(c.scope.model.clone(), format!("model:{key}"), "1")
}
fn scan(root: &Path, dir: &Path, out: &mut BTreeSet<String>, limit: usize) -> Result<()> {
    for entry in fs::read_dir(dir).map_err(io)? {
        let entry = entry.map_err(io)?;
        let ft = entry.file_type().map_err(io)?;
        if ft.is_symlink() {
            return Err(invalid(format!(
                "source symlink: {}",
                entry.path().display()
            )));
        }
        if ft.is_dir() {
            scan(root, &entry.path(), out, limit)?;
        } else if ft.is_file() {
            out.insert(
                entry
                    .path()
                    .strip_prefix(root)
                    .unwrap()
                    .to_str()
                    .ok_or_else(|| invalid("non-UTF8 source path"))?
                    .into(),
            );
            if out.len() > limit {
                return Err(invalid("file inventory limit"));
            }
        } else {
            return Err(invalid("nonregular source"));
        }
    }
    Ok(())
}
fn checkpoint(j: &Journal) -> Result<()> {
    save(&j.config.work.join("journal.json"), j)
}
fn lock_run(work: &Path) -> Result<File> {
    use std::os::{fd::AsRawFd, unix::fs::OpenOptionsExt};
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(work.join("run.lock"))
        .map_err(io)?;
    if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        return Err(Error::Busy);
    }
    Ok(f)
}
fn check_sources(j: &Journal) -> Result<()> {
    for s in &j.sources {
        let bytes = read(
            &selected(&j.config.source, &s.input.path)?,
            j.config.limits.file_bytes,
        )?;
        if digest(&bytes) != s.original {
            return Err(invalid(format!("changed source: {}", s.input.path)));
        }
    }
    Ok(())
}
fn check_inputs(j: &Journal) -> Result<()> {
    if j.format != FORMAT {
        return Err(Error::Incompatible("authoring journal".into()));
    }
    if j.plan_digest.as_ref()
        != Some(&digest(&read(
            &j.config.work.join("plan.json"),
            JSON_LIMIT,
        )?))
        || j.coverage_digest.as_ref()
            != Some(&digest(&read(
                &j.config.work.join("coverage.json"),
                JSON_LIMIT,
            )?))
    {
        return Err(invalid(
            "changed plan/coverage; use a new run and destination",
        ));
    }
    Ok(())
}
fn replace(v: &mut Json, keys: &BTreeMap<String, ExactRef>) -> Result<()> {
    match v {
        Json::Object(m) if m.contains_key("$ref") => {
            if m.len() != 1 {
                return Err(invalid("reference token has extra fields"));
            }
            let k = m["$ref"]
                .as_str()
                .ok_or_else(|| invalid("reference token key"))?;
            *v = serde_json::to_value(
                keys.get(k)
                    .ok_or_else(|| invalid(format!("unknown logical reference: {k}")))?,
            )
            .unwrap();
        }
        Json::Object(m) => {
            for v in m.values_mut() {
                replace(v, keys)?;
            }
        }
        Json::Array(a) => {
            for v in a {
                replace(v, keys)?;
            }
        }
        _ => {}
    }
    Ok(())
}
async fn capture(config_path: &Path) -> Result<Json> {
    let mut c: Config = load(config_path)?;
    if c.format != FORMAT {
        return Err(Error::Incompatible("source manifest contract".into()));
    }
    c.limits.check()?;
    if !c.actor.contains(':') {
        return Err(invalid("explicit qualified host actor required"));
    }
    c.source = c.source.canonicalize().map_err(io)?;
    c.destination = new_absolute(&c.destination)?;
    c.work = new_absolute(&c.work)?;
    if c.source.starts_with(&c.destination)
        || c.destination.starts_with(&c.source)
        || c.work.starts_with(&c.source)
        || c.source.starts_with(&c.work)
        || c.destination.starts_with(&c.work)
        || c.work.starts_with(&c.destination)
    {
        return Err(invalid("source/destination/work roots overlap"));
    }
    if c.destination.symlink_metadata().is_ok() || c.work.symlink_metadata().is_ok() {
        return Err(Error::AlreadyExists);
    }
    let mut inventory = BTreeSet::new();
    scan(&c.source, &c.source, &mut inventory, c.limits.files)?;
    if c.inputs.len() > c.limits.files {
        return Err(invalid("input inventory limit"));
    }
    let declared = c
        .inputs
        .iter()
        .map(|i| i.path.clone())
        .collect::<BTreeSet<_>>();
    if inventory != declared || declared.len() != c.inputs.len() {
        return Err(invalid(
            "every source file requires exactly one disposition",
        ));
    }
    let mut keys = BTreeSet::new();
    let mut sources = vec![];
    let mut total = 0;
    for input in &c.inputs {
        if input.key.trim().is_empty()
            || !keys.insert(input.key.clone())
            || input.authority.trim().is_empty()
            || input.media_type.trim().is_empty()
        {
            return Err(invalid("source key/media/authority missing or duplicate"));
        }
        if input
            .exclude_reason
            .as_ref()
            .is_some_and(|s| s.trim().is_empty())
        {
            return Err(invalid("empty exclusion reason"));
        }
        let bytes = read(&selected(&c.source, &input.path)?, c.limits.file_bytes)?;
        let original = digest(&bytes);
        total += bytes.len() as u64;
        if total > c.limits.total_bytes {
            return Err(invalid("total captured byte limit"));
        }
        if input.expected.as_ref().is_some_and(|e| e != &original) {
            return Err(invalid(format!("changed pinned source: {}", input.path)));
        }
        let mut missing = vec![];
        for d in &input.dependencies {
            relative(d)?;
            if !declared.contains(d) {
                missing.push(d.clone());
            }
        }
        sources.push(Source {
            input: input.clone(),
            original,
            missing_dependencies: missing,
        });
    }
    if c.authority.review_status.trim().is_empty()
        || c.authority.precedence.iter().any(|k| !keys.contains(k))
        || c.authority
            .field_owners
            .iter()
            .any(|(path, key)| path.trim().is_empty() || !keys.contains(key))
    {
        return Err(invalid(
            "authority ownership/precedence must select declared source keys and retain review status",
        ));
    }
    fs::create_dir(&c.work).map_err(io)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&c.work, fs::Permissions::from_mode(0o700)).map_err(io)?;
    }
    fs::create_dir(c.work.join("originals")).map_err(io)?;
    for (i, s) in sources.iter().enumerate() {
        let bytes = read(&selected(&c.source, &s.input.path)?, c.limits.file_bytes)?;
        if digest(&bytes) != s.original {
            return Err(invalid("source changed during capture"));
        }
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(c.work.join("originals").join(i.to_string()))
            .map_err(io)?;
        f.write_all(&bytes).map_err(io)?;
        full_sync(&f)?;
    }
    File::open(c.work.join("originals"))
        .map_err(io)?
        .sync_all()
        .map_err(io)?;
    let j = Journal {
        format: FORMAT.into(),
        config: c,
        sources,
        plan_digest: None,
        coverage_digest: None,
        key_map: BTreeMap::new(),
        operations: vec![],
        dispatches: vec![],
        initialization: false,
        ready: false,
        baseline: None,
        head: None,
    };
    checkpoint(&j)?;
    save(&j.config.work.join("source-manifest.json"), &j.sources)?;
    Ok(
        json!({"status":"captured","files":j.sources.len(),"bytes":total,"work":j.config.work,"limits":j.config.limits}),
    )
}
fn prepare(work: &Path, plan_path: &Path, coverage_path: &Path) -> Result<Json> {
    let mut j: Journal = load(&work.join("journal.json"))?;
    if j.initialization || !j.dispatches.is_empty() {
        return Err(invalid("cannot reprepare a dispatched run"));
    }
    check_sources(&j)?;
    let bytes = read(plan_path, JSON_LIMIT)?;
    let coverage_bytes = read(coverage_path, JSON_LIMIT)?;
    let plan: Plan =
        serde_json::from_value(strict_json(&bytes)?).map_err(|e| invalid(format!("plan: {e}")))?;
    let coverage: Coverage = serde_json::from_value(strict_json(&coverage_bytes)?)
        .map_err(|e| invalid(format!("coverage: {e}")))?;
    if plan.format != FORMAT
        || coverage.format != FORMAT
        || plan.catalogue != ontology_ref()
        || plan.profile != profile_ref()
    {
        return Err(Error::Incompatible(
            "plan/coverage/catalogue/profile contract".into(),
        ));
    }
    if plan.records.len() > j.config.limits.records
        || coverage.items.len() > j.config.limits.records
        || coverage.queries.len() > j.config.limits.records
    {
        return Err(invalid("plan/coverage entry limit"));
    }
    let mut ops = vec![];
    let mut keys = BTreeMap::new();
    for (i, s) in j
        .sources
        .iter()
        .enumerate()
        .filter(|(_, s)| s.input.exclude_reason.is_none())
    {
        let source = ref_key(&j.config, &format!("source:{}", s.input.key));
        let capture = ref_key(&j.config, &format!("capture:{}", s.input.key));
        let anchor = ref_key(&j.config, &format!("anchor:{}", s.input.key));
        keys.insert(format!("source:{}", s.input.key), source.clone());
        keys.insert(format!("capture:{}", s.input.key), capture.clone());
        let bytes = read(
            &j.config.work.join("originals").join(i.to_string()),
            j.config.limits.file_bytes,
        )?;
        if digest(&bytes) != s.original {
            return Err(invalid("changed retained original"));
        }
        ops.push(Operation::Put(Record::new(
            source.clone(),
            type_ref("SourceArtifact"),
            RecordData::SourceArtifact(SourceArtifactData {
                origin: s.input.path.clone(),
                media_type: s.input.media_type.clone(),
                owner: s.input.authority.clone(),
                status: s.input.status.clone(),
            }),
            vec![],
        )?));
        ops.push(Operation::Capture {
            record: Record::new(
                capture.clone(),
                type_ref("SourceRevision"),
                RecordData::SourceRevision(SourceRevisionData {
                    source,
                    original: s.original.clone(),
                    media_type: s.input.media_type.clone(),
                    derived_from: None,
                    derivation: None,
                }),
                vec![],
            )?,
            content: Content::capture(bytes.clone()),
        });
        if let Ok(text) = std::str::from_utf8(&bytes) {
            if !text.trim().is_empty() {
                keys.insert(format!("anchor:{}", s.input.key), anchor.clone());
                ops.push(Operation::Put(Record::new(
                    anchor,
                    type_ref("SourceAnchor"),
                    RecordData::Anchor(AnchorData {
                        source: capture,
                        label: s.input.path.clone(),
                        locator: Locator::ByteRange {
                            start: 0,
                            end: bytes.len() as u64,
                            quote: text.into(),
                        },
                    }),
                    vec![],
                )?));
            }
        }
    }
    for r in &plan.records {
        if r.key.trim().is_empty()
            || keys
                .insert(r.key.clone(), ref_key(&j.config, &r.key))
                .is_some()
        {
            return Err(invalid(format!("duplicate/empty logical key: {}", r.key)));
        }
    }
    if keys.len() > j.config.limits.visited {
        return Err(invalid("planned subject/visited limit"));
    }
    for r in &plan.records {
        let mut data = r.data.clone();
        replace(&mut data, &keys)?;
        let data: RecordData = serde_json::from_value(data)
            .map_err(|e| invalid(format!("record {} /data: {e}", r.key)))?;
        let provenance = r
            .provenance
            .iter()
            .map(|k| {
                keys.get(k)
                    .cloned()
                    .ok_or_else(|| invalid(format!("record {} /provenance: {k}", r.key)))
            })
            .collect::<Result<Vec<_>>>()?;
        let mut selected_type = r.type_ref.clone();
        replace(&mut selected_type, &keys)?;
        let selected_type: ExactRef = serde_json::from_value(selected_type)
            .map_err(|e| invalid(format!("record {} /type_ref: {e}", r.key)))?;
        let record = Record::new(keys[&r.key].clone(), selected_type, data, provenance)?;
        if let Some(bytes) = &r.capture {
            ops.push(Operation::Capture {
                record,
                content: Content::capture(bytes.clone()),
            });
        } else {
            ops.push(Operation::Put(record));
        }
    }
    if let Some((o, p)) = &plan.selection {
        ops.push(Operation::SelectProfile {
            ontology: keys
                .get(o)
                .cloned()
                .ok_or_else(|| invalid("unknown ontology selection"))?,
            profile: keys
                .get(p)
                .cloned()
                .ok_or_else(|| invalid("unknown profile selection"))?,
        });
    }
    let mut gaps = plan.gaps.clone();
    replace(&mut gaps, &keys)?;
    let gaps: Vec<Gap> =
        serde_json::from_value(gaps).map_err(|e| invalid(format!("/gaps: {e}")))?;
    ops.push(Operation::Gaps(gaps));
    let baseline = ref_key(&j.config, &format!("baseline:{}", plan.baseline));
    if keys.values().any(|r| r == &baseline) {
        return Err(invalid("baseline collides with logical key"));
    }
    for (name, bytes) in [
        ("plan", bytes.clone()),
        ("coverage", coverage_bytes.clone()),
        ("sources", encoded(&j.sources)?),
        ("authority", encoded(&j.config.authority)?),
    ] {
        let source = ref_key(&j.config, &format!("handover:{name}:source"));
        let capture = ref_key(&j.config, &format!("handover:{name}:capture"));
        keys.insert(format!("handover:{name}:capture"), capture.clone());
        let content = Content::capture(bytes);
        ops.push(Operation::Put(Record::new(
            source.clone(),
            type_ref("SourceArtifact"),
            RecordData::SourceArtifact(SourceArtifactData {
                origin: format!("generation/{name}.json"),
                media_type: "application/json".into(),
                owner: "operator:authoring".into(),
                status: "design-decision".into(),
            }),
            vec![],
        )?));
        ops.push(Operation::Capture {
            record: Record::new(
                capture,
                type_ref("SourceRevision"),
                RecordData::SourceRevision(SourceRevisionData {
                    source,
                    original: content.reference.clone(),
                    media_type: "application/json".into(),
                    derived_from: None,
                    derivation: None,
                }),
                vec![],
            )?,
            content,
        });
    }
    j.operations = ops;
    j.key_map = keys;
    j.baseline = Some(baseline);
    // Preserve exact input bytes, not a normalized re-serialization, for change detection.
    fs::write(j.config.work.join("plan.json"), &bytes).map_err(io)?;
    fs::write(j.config.work.join("coverage.json"), &coverage_bytes).map_err(io)?;
    full_sync(&File::open(j.config.work.join("plan.json")).map_err(io)?)?;
    full_sync(&File::open(j.config.work.join("coverage.json")).map_err(io)?)?;
    j.plan_digest = Some(digest(&bytes));
    j.coverage_digest = Some(digest(&coverage_bytes));
    checkpoint(&j)?;
    save(&j.config.work.join("key-map.json"), &j.key_map)?;
    Ok(
        json!({"status":"prepared-not-assessed","records":j.operations.len(),"keys":j.key_map.len(),"plan":j.plan_digest,"catalogue":ontology_ref(),"profile":profile_ref()}),
    )
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct HostInput {
    actor: String,
    grants: BTreeSet<String>,
}
fn host(j: &mut Journal, path: &Path) -> Result<()> {
    let h: HostInput = load(path)?;
    if h.actor != j.config.actor {
        return Err(invalid(
            "current host actor differs from recorded attribution",
        ));
    }
    j.config.grants = h.grants;
    Ok(())
}
fn fault(point: &str, operation: &str) {
    let selected = std::env::var("KNOWLEDGEHUB_MODEL_FAULT").ok();
    if selected.as_deref() == Some(point)
        || selected.as_deref() == Some(&format!("{point}:{operation}"))
    {
        std::process::exit(86);
    }
}
async fn dispatch(
    h: &KnowledgeHub,
    j: &mut Journal,
    index: usize,
    request: Change,
) -> Result<Receipt> {
    let fingerprint = request.fingerprint(&j.config.actor)?;
    if j.dispatches.len() == index {
        j.dispatches.push(Dispatch {
            actor: j.config.actor.clone(),
            request: request.clone(),
            fingerprint: fingerprint.clone(),
            receipt: None,
        });
        checkpoint(j)?;
    }
    let original = j
        .dispatches
        .get(index)
        .ok_or_else(|| invalid("dispatch journal sequence"))?;
    if original.request != request
        || original.actor != j.config.actor
        || original.fingerprint != fingerprint
    {
        return Err(Error::OperationReuse);
    }
    fault("before-dispatch", &request.operation_id);
    let operation_id = request.operation_id.clone();
    let receipt = match h
        .operation_result(context(&j.config), request.operation_id.clone())
        .await?
    {
        OperationOutcome::Committed(receipt) => {
            // The public same-intent retry additionally checks the stored fingerprint.
            let retried = h.commit(context(&j.config), request.clone()).await?;
            if retried != *receipt {
                return Err(invalid("original receipt mismatch"));
            }
            *receipt
        }
        OperationOutcome::Absent { .. } => h.commit(context(&j.config), request).await?,
    };
    fault("after-commit", &operation_id);
    if j.dispatches[index]
        .receipt
        .as_ref()
        .is_some_and(|r| r != &receipt)
    {
        return Err(invalid("changed original receipt"));
    }
    j.dispatches[index].receipt = Some(receipt.clone());
    checkpoint(j)?;
    fault("after-receipt", &operation_id);
    Ok(receipt)
}
fn preview(mut state: State, ops: &[Operation]) -> Result<State> {
    for op in ops {
        match op {
            Operation::Put(r) | Operation::Capture { record: r, .. } => {
                if state.get(&r.reference).is_some_and(|old| old != r) {
                    return Err(invalid("plan record overwrites accepted revision"));
                }
                state
                    .selected
                    .insert(r.reference.identity(), r.reference.clone());
                state.records.insert(r.reference.key(), r.clone());
                if let Operation::Capture { record, content } = op {
                    state
                        .artifacts
                        .insert(record.reference.key(), content.clone());
                }
            }
            Operation::Gaps(g) => state.gaps = g.clone(),
            Operation::SelectProfile { ontology, profile } => {
                state.ontology = Some(ontology.clone());
                state.profile = Some(profile.clone());
            }
            _ => return Err(invalid("unsupported fresh authoring operation")),
        }
    }
    Ok(state)
}
async fn populate(j: &mut Journal, resume: bool) -> Result<()> {
    check_inputs(j)?;
    check_sources(j)?;
    if !resume && (j.initialization || j.config.destination.symlink_metadata().is_ok()) {
        return Err(Error::AlreadyExists);
    }
    if !j.initialization {
        j.initialization = true;
        checkpoint(j)?;
    }
    // Native create performs the exclusive directory reservation and persistence.
    let h = if !j.config.destination.exists() {
        KnowledgeHub::create(
            backend(&j.config)?,
            context(&j.config),
            j.config.scope.clone(),
        )
        .await?
    } else {
        KnowledgeHub::open(
            backend(&j.config)?,
            context(&j.config),
            j.config.scope.clone(),
        )
        .await?
    };
    let registration = if let Some(d) = j.dispatches.first() {
        d.request.clone()
    } else {
        let (head, state) = h.snapshot(context(&j.config)).await?;
        let mut c = Change::at(&state, head, "authoring:register");
        c.operations = builtin_catalogue()?.operations();
        c
    };
    dispatch(&h, j, 0, registration).await?;
    let model = if let Some(d) = j.dispatches.get(1) {
        d.request.clone()
    } else {
        let (head, state) = h.snapshot(context(&j.config)).await?;
        let candidate = preview(state.clone(), &j.operations)?;
        let mut c = Change::at(&state, head, "authoring:model");
        c.operations = j.operations.clone();
        let provenance = j
            .operations
            .iter()
            .filter_map(Operation::record)
            .find(|r| matches!(r.data, RecordData::Anchor(_)))
            .map(|r| vec![r.reference.clone()])
            .ok_or_else(|| invalid("source-bound baseline requires an anchor"))?;
        c.operations.push(Operation::Put(Record::new(
            j.baseline
                .clone()
                .ok_or_else(|| invalid("unprepared baseline"))?,
            type_ref("ModelBaseline"),
            RecordData::Baseline(candidate.baseline_data()?),
            provenance,
        )?));
        let report = h.validate(context(&j.config), c.clone()).await?;
        save(
            &j.config.work.join("assessment.json"),
            &json!({"valid":report.is_valid(),"complete":report.is_complete(),"violations":report.violations,"gaps":report.gaps}),
        )?;
        if !report.is_valid() {
            return Err(Error::Invalid(report.violations));
        }
        c
    };
    let receipt = dispatch(&h, j, 1, model).await?;
    j.head = Some(receipt.head);
    checkpoint(j)?;
    h.drain().await;
    Ok(())
}
async fn resolve(j: &mut Journal) -> Result<Json> {
    check_inputs(j)?;
    let h = KnowledgeHub::open(
        backend(&j.config)?,
        context(&j.config),
        j.config.scope.clone(),
    )
    .await?;
    let mut results = vec![];
    for i in 0..j.dispatches.len() {
        let d = j.dispatches[i].clone();
        if d.actor != j.config.actor || d.fingerprint != d.request.fingerprint(&j.config.actor)? {
            return Err(Error::OperationReuse);
        }
        match h.operation_result(context(&j.config),d.request.operation_id.clone()).await? {
            OperationOutcome::Committed(r)=>{if d.receipt.as_ref().is_some_and(|known|known!=r.as_ref()){return Err(invalid("receipt mismatch"));}
                j.dispatches[i].receipt=Some(*r.clone());if d.request.operation_id=="authoring:model" {j.head=Some(r.head.clone());}results.push(json!({"operation":d.request.operation_id,"outcome":"committed","receipt":r}));}
            OperationOutcome::Absent{recovered_head}=>results.push(json!({"operation":d.request.operation_id,"outcome":"positively-absent","head":recovered_head})),
        }
    }
    checkpoint(j)?;
    h.drain().await;
    Ok(json!({"status":"reconciled","known_operations":results}))
}
async fn verify(j: &mut Journal) -> Result<Json> {
    check_inputs(j)?;
    j.config.limits.check()?;
    let coverage: Coverage = load(&j.config.work.join("coverage.json"))?;
    let h = KnowledgeHub::open(
        backend(&j.config)?,
        context(&j.config),
        j.config.scope.clone(),
    )
    .await?;
    let selection = Selection::Baseline(
        j.baseline
            .clone()
            .ok_or_else(|| invalid("unprepared baseline"))?,
    );
    let mut failures = vec![];
    if j.head.is_none()
        || !j
            .dispatches
            .iter()
            .any(|d| d.request.operation_id == "authoring:model" && d.receipt.is_some())
    {
        failures.push("model publication has no reconciled original receipt".into());
    }
    failures.extend(
        j.config
            .authority
            .unresolved_conflicts
            .iter()
            .map(|s| format!("unresolved authority conflict: {s}")),
    );
    let mut original_bytes = 0u64;
    let mut record_bytes = 0u64;
    let mut read_count = 0usize;
    let mut records = BTreeMap::new();
    for (key, reference) in &j.key_map {
        if read_count >= j.config.limits.visited {
            return Err(invalid("exact verification visited limit"));
        }
        let record = h
            .read(context(&j.config), selection.clone(), reference.clone())
            .await?;
        record_bytes += encoded(&record)?.len() as u64;
        read_count += 1;
        records.insert(key.clone(), record);
    }
    for s in j
        .sources
        .iter()
        .filter(|s| s.input.exclude_reason.is_none())
    {
        let capture = &j.key_map[&format!("capture:{}", s.input.key)];
        let bytes = h
            .read_artifact(context(&j.config), selection.clone(), capture.clone())
            .await?;
        if digest(&bytes.bytes) != s.original {
            failures.push(format!("original digest {}", s.input.key));
        }
        original_bytes += bytes.bytes.len() as u64;
        if !s.missing_dependencies.is_empty() {
            failures.push(format!(
                "missing dependencies {}: {:?}",
                s.input.path, s.missing_dependencies
            ));
        }
    }
    let mut seen = BTreeSet::new();
    let mut obligations = vec![];
    for item in &coverage.items {
        if item.id.trim().is_empty()
            || !seen.insert(&item.id)
            || !j.sources.iter().any(|s| s.input.key == item.source)
        {
            failures.push(format!("invalid obligation identity/source {}", item.id));
        }
        let mut passed = item.disposition == "mapped" && !item.targets.is_empty();
        for target in &item.targets {
            let Some(record) = records.get(&target.key) else {
                passed = false;
                continue;
            };
            let Some(anchor) = records.get(&target.anchor) else {
                passed = false;
                continue;
            };
            if !matches!(anchor.data, RecordData::Anchor(_)) {
                passed = false;
            }
            if let RecordData::Anchor(a) = &anchor.data {
                if j.key_map.get(&format!("capture:{}", item.source)) != Some(&a.source) {
                    passed = false;
                }
            }
            if !record.provenance.contains(&anchor.reference) {
                passed = false;
            }
            let value = serde_json::to_value(record).unwrap();
            let mut expected = target.expected.clone();
            if let Some(v) = &mut expected {
                if let Err(error) = replace(v, &j.key_map) {
                    passed = false;
                    failures.push(format!("unbound expected value {}: {error}", item.id));
                }
            }
            if let Some(path) = &target.path {
                if value.pointer(path).is_none()
                    || expected
                        .as_ref()
                        .is_some_and(|v| value.pointer(path) != Some(v))
                {
                    passed = false;
                }
            }
        }
        if item.disposition != "mapped" {
            passed = false;
            if item.reason.as_ref().is_none_or(|s| s.trim().is_empty()) {
                failures.push(format!("unreasoned disposition {}", item.id));
            }
        }
        if !passed {
            failures.push(format!(
                "unmapped/unsupported source obligation {}",
                item.id
            ));
        }
        obligations.push(json!({"id":item.id,"disposition":item.disposition,"passed":passed,"reason":item.reason}));
    }
    // Every captured file must have an explicit semantic or unsupported disposition.
    for s in j
        .sources
        .iter()
        .filter(|s| s.input.exclude_reason.is_none())
    {
        if !coverage.items.iter().any(|i| i.source == s.input.key) {
            failures.push(format!("no file obligation {}", s.input.key));
        }
    }
    let mut query_results = vec![];
    for check in &coverage.queries {
        let mut v = check.query.clone();
        if let Err(error) = replace(&mut v, &j.key_map) {
            failures.push(format!("unbound semantic query {}: {error}", check.name));
            query_results.push(json!({"name":check.name,"kind":check.kind,"status":"failed","diagnostic":error.to_string(),"exhausted":false}));
            continue;
        }
        if check.kind == "neighbours" {
            let mut q: Neighbours = serde_json::from_value(v)
                .map_err(|e| invalid(format!("neighbours {}: {e}", check.name)))?;
            q.selection = selection.clone();
            if q.page_size > j.config.limits.page_size
                || q.examined_limit > j.config.limits.subjects
            {
                return Err(invalid("neighbour query exceeds configured bounds"));
            }
            let mut page = h.neighbours(context(&j.config), q.clone()).await?;
            let mut ids = BTreeSet::new();
            let mut pages = 0;
            loop {
                pages += 1;
                if read_count.saturating_add(page.work.record_reads as usize)
                    > j.config.limits.visited
                {
                    failures.push(format!("visited limit {}", check.name));
                    break;
                }
                for n in &page.neighbours {
                    ids.insert(n.opposite.id.clone());
                }
                read_count += page.work.record_reads as usize;
                record_bytes += page.work.record_bytes;
                if let Some(cursor) = page.continuation.clone() {
                    if pages >= j.config.limits.pages {
                        failures.push(format!("page limit {}", check.name));
                        break;
                    }
                    page = h
                        .continue_neighbours(context(&j.config), q.clone(), cursor)
                        .await?;
                } else {
                    if !page.exhausted {
                        failures.push(format!("unexhausted neighbours {}", check.name));
                    }
                    break;
                }
            }
            if ids != check.expected_ids {
                failures.push(format!("semantic neighbours {} IDs differ", check.name));
            }
            query_results.push(json!({"name":check.name,"kind":"neighbours","intent":q,"pages":pages,"ids":ids,"exhausted":page.exhausted,"reached":page.reached}));
            continue;
        }
        if check.kind != "query" {
            return Err(invalid(format!("unsupported read kind {}", check.kind)));
        }
        let mut q: Query =
            serde_json::from_value(v).map_err(|e| invalid(format!("query {}: {e}", check.name)))?;
        q.selection = selection.clone();
        if q.page_size > j.config.limits.page_size
            || q.subject_limit > j.config.limits.subjects
            || q.depth > j.config.limits.depth
        {
            return Err(invalid(format!(
                "query {} exceeds configured limits",
                check.name
            )));
        }
        let mut page = h.query(context(&j.config), q.clone()).await?;
        let mut ids = BTreeSet::new();
        let mut pages = 0;
        loop {
            pages += 1;
            if read_count.saturating_add(page.records.len() + page.assertions.len())
                > j.config.limits.visited
            {
                failures.push(format!("visited limit {}", check.name));
                break;
            }
            for r in page.records.iter().chain(&page.assertions) {
                ids.insert(r.reference.id.clone());
                record_bytes += encoded(r)?.len() as u64;
                read_count += 1;
            }
            if let Some(cursor) = page.continuation.clone() {
                if pages >= j.config.limits.pages {
                    failures.push(format!("page limit {}", check.name));
                    break;
                }
                page = h
                    .continue_query(context(&j.config), q.clone(), cursor)
                    .await?;
            } else {
                if !page.exhausted {
                    failures.push(format!(
                        "unexhausted query {}: {:?}",
                        check.name, page.truncated
                    ));
                }
                break;
            }
        }
        if ids != check.expected_ids {
            failures.push(format!("semantic query {} IDs differ", check.name));
        }
        query_results.push(json!({"name":check.name,"kind":"query","intent":q,"pages":pages,"ids":ids,"exhausted":page.exhausted,"truncated":page.truncated}));
    }
    let (current_head, state) = h.snapshot(context(&j.config)).await?;
    let baseline = state
        .baseline(j.baseline.as_ref().unwrap())
        .ok_or(Error::NotFound)?;
    let assessment = assess(&state.view(baseline));
    if !assessment.is_valid() {
        failures.push("native baseline integrity".into());
    }
    for d in &j.dispatches {
        match h
            .operation_result(context(&j.config), d.request.operation_id.clone())
            .await?
        {
            OperationOutcome::Committed(receipt)
                if d.receipt.as_ref() == Some(receipt.as_ref()) => {}
            _ => failures.push(format!(
                "unreconciled original operation {}",
                d.request.operation_id
            )),
        }
    }
    h.drain().await;
    let report = json!({"format":FORMAT,"status":if failures.is_empty(){"verified"}else{"incomplete"},"source_coverage":failures.is_empty(),"failures":failures,"native_validity":assessment.is_valid(),"native_completeness":assessment.is_complete(),"native_gaps":assessment.gaps,"interpretation_review":coverage.interpretation_review,"source_gaps":coverage.source_gaps,"implementation_evidence":"not established by import","human_acceptance":"pending","application_execution":"not performed","destination":j.config.destination,"scope":j.config.scope,"head":j.head,"current_head":current_head,"baseline":j.baseline,"catalogue":ontology_ref(),"ontology":baseline.ontology,"profile":baseline.profile,"sources":j.sources,"plan_digest":j.plan_digest,"coverage_digest":j.coverage_digest,"known_receipts":j.dispatches.iter().map(|d|&d.receipt).collect::<Vec<_>>(),"obligations":obligations,"queries":query_results,"read_records":read_count,"record_bytes":record_bytes,"original_bytes":original_bytes,"limits":j.config.limits});
    save(&j.config.work.join("verification.json"), &report)?;
    let text = format!(
        "# Native model verification\n\nStatus: {}\n\nNative validity: {}; completeness: {}. Interpretation: {}. Human acceptance: pending.\n\nRead {} records ({} serialized bytes), verified {} original bytes.\n\nFailures:\n{}\n\nSee verification.json for every file, obligation, gap, query, limit and known receipt.\n",
        report["status"],
        assessment.is_valid(),
        assessment.is_complete(),
        coverage.interpretation_review,
        read_count,
        record_bytes,
        original_bytes,
        failures
            .iter()
            .map(|f| format!("- {f}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
    fs::write(j.config.work.join("verification.md"), text).map_err(io)?;
    j.ready = failures.is_empty();
    checkpoint(j)?;
    save(
        &j.config.work.join("generation-manifest.json"),
        &json!({"format":FORMAT,"ready":j.ready,"destination":j.config.destination,"scope":j.config.scope,"head":j.head,"baseline":j.baseline,"catalogue":ontology_ref(),"ontology":report["ontology"],"profile":report["profile"],"sources":j.sources,"plan_digest":j.plan_digest,"coverage_digest":j.coverage_digest,"key_map":j.key_map,"known_operations":j.dispatches}),
    )?;
    Ok(report)
}
pub async fn run(args: Vec<String>) -> Result<Json> {
    let command = args.first().map(String::as_str).unwrap_or("");
    if command == "catalogue" && args.len() == 2 {
        let c = builtin_catalogue()?;
        let out = json!({"format":FORMAT,"catalogue":ontology_ref(),"profile":profile_ref(),"record_codec":RECORD_FORMAT,"request_codec":REQUEST_FORMAT,"records":c.records});
        save(Path::new(&args[1]), &out)?;
        return Ok(json!({"status":"catalogue","output":args[1]}));
    }
    if command == "capture" && args.len() == 2 {
        return capture(Path::new(&args[1])).await;
    }
    if command == "prepare" && args.len() == 4 {
        let _lock = lock_run(Path::new(&args[1]))?;
        return prepare(
            Path::new(&args[1]),
            Path::new(&args[2]),
            Path::new(&args[3]),
        );
    }
    if ["assess", "create", "resume", "resolve", "verify"].contains(&command) && args.len() == 3 {
        let work = Path::new(&args[1]).canonicalize().map_err(io)?;
        let _lock = lock_run(&work)?;
        let mut j: Journal = load(&work.join("journal.json"))?;
        if j.config.work != work {
            return Err(invalid("journal work identity differs"));
        }
        host(&mut j, Path::new(&args[2]))?;
        if command == "resolve" {
            return resolve(&mut j).await;
        }
        if command == "verify" {
            return verify(&mut j).await;
        }
        if command == "assess" {
            check_inputs(&j)?;
            check_sources(&j)?;
            let dir = j
                .config
                .work
                .join(format!("assessment-{}", std::process::id()));
            fs::create_dir(&dir).map_err(io)?;
            j.config.work = dir.clone();
            j.config.destination = dir.join("store");
            j.dispatches.clear();
            j.initialization = false;
            j.ready = false;
            for name in ["plan.json", "coverage.json"] {
                fs::copy(work.join(name), dir.join(name)).map_err(io)?;
            }
            checkpoint(&j)?;
            populate(&mut j, false).await?;
            let report = verify(&mut j).await?;
            return Ok(
                json!({"status":"assessed-disposable","destination_published":false,"report":report}),
            );
        }
        populate(&mut j, command == "resume").await?;
        resolve(&mut j).await?;
        return verify(&mut j).await;
    }
    Err(invalid(
        "usage: knowledgehub-model catalogue OUTPUT | capture CONFIG | prepare WORK PLAN COVERAGE | assess|create|resume|resolve|verify WORK HOST.json",
    ))
}
