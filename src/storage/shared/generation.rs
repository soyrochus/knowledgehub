use super::*;
use super::{
    content::{self, Binding},
    objects::{Objects, bounded_file},
    projection,
    trees::{Tree, exact_key, identity_key, ref_key},
};
use crate::ontology::{
    self, ArtifactRange, ExactRef, ModelScope, Record, RecordData, Selection, State,
};
use serde_json::{Value, json as value};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Metadata {
    format: String,
    hub_identity: String,
    replay_domain: String,
    scope: ModelScope,
    record_codec: String,
    request_codec: String,
    object_grammar: String,
    tree_grammars: Vec<String>,
    required_features: Vec<String>,
    filesystem_profile: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Generation {
    pub schema: String,
    pub scope: ModelScope,
    pub snapshot_id: SnapshotId,
    pub sequence: String,
    pub parent: Option<String>,
    pub ontology: Option<ExactRef>,
    pub profile: Option<ExactRef>,
    pub selected_root: String,
    pub revisions_root: String,
    pub retained_refs_root: String,
    pub artifacts_root: String,
    pub gaps_root: String,
    pub operations_root: String,
    pub ancestors_root: String,
    pub receipt: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Reference {
    format: String,
    scope: ModelScope,
    snapshot_id: SnapshotId,
    sequence: String,
    generation: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Ancestor {
    generation: String,
    sequence: String,
}

fn scope(s: &ModelScope) -> Result<ModelScope, Error> {
    model::check_scope(s)?;
    Ok(s.clone())
}
pub(super) fn head_path(store: &FileStore, s: &ModelScope) -> Result<PathBuf, Error> {
    Ok(store.root.join("refs").join(format!(
        "{}.json",
        codec::digest(&json(&value!([s.hub, s.model, s.branch]))?)
    )))
}
fn paths(store: &FileStore) -> Result<(), Error> {
    if store.root.components().any(|c| {
        matches!(
            c,
            std::path::Component::ParentDir | std::path::Component::CurDir
        )
    }) {
        return Err(Error::InvalidInput(
            "ordinary absolute path required".into(),
        ));
    }
    for p in store.root.ancestors().skip(1) {
        if fs::symlink_metadata(p)
            .map_err(|_| integrity("store ancestor"))?
            .file_type()
            .is_symlink()
        {
            return Err(integrity("symlink store ancestor"));
        }
    }
    Ok(())
}
fn features() -> Vec<String> {
    ["raw", "zstd", "fixed-chunks/1", "block-packs/1"]
        .into_iter()
        .map(String::from)
        .collect()
}
pub(super) fn metadata(store: &FileStore, s: &ModelScope) -> Result<(), Error> {
    paths(store)?;
    store.check_profile(&store.root)?;
    let expected = scope(s)?;
    let bytes = bounded_file(&store.root.join("hub.json"), 1_048_576)?;
    let declared: Value = parse(&bytes)?;
    if declared.get("format").and_then(Value::as_str) != Some(ontology::FILE_FORMAT) {
        return Err(Error::Incompatible(
            "explicit shared backend format differs".into(),
        ));
    }
    let meta: Metadata = parse(&bytes)?;
    if meta.format != ontology::FILE_FORMAT
        || meta.hub_identity != expected.hub
        || meta.scope != expected
        || !model::hash(&meta.replay_domain)
        || meta.record_codec != ontology::RECORD_FORMAT
        || meta.request_codec != ontology::REQUEST_FORMAT
        || meta.object_grammar != "knowledgehub-object/1"
        || meta.tree_grammars != ["knowledgehub-map-node/1", "knowledgehub-sequence-node/1"]
        || meta.required_features != features()
        || !matches!(
            meta.filesystem_profile.as_str(),
            native::EXT4 | native::APFS
        )
    {
        return Err(Error::Incompatible(
            "files/4 metadata, exact scope or required codec/features differ".into(),
        ));
    }
    let dev = fs::metadata(&store.root)
        .map_err(|_| integrity("store device"))?
        .dev();
    for name in [
        "refs",
        "locks",
        "objects",
        "objects/loose",
        "objects/packs",
        "staging",
        "cache",
        "views",
    ] {
        let p = store.root.join(name);
        is_dir(&p)?;
        native::same_device(
            dev,
            fs::metadata(p)
                .map_err(|_| integrity("layout device"))?
                .dev(),
        )?;
    }
    Ok(())
}
fn lock(store: &FileStore, shared: bool) -> Result<File, Error> {
    let f = open_read(&store.root.join("locks/store.lock"))?;
    let mode = if shared { libc::LOCK_SH } else { libc::LOCK_EX };
    // SAFETY: f owns a regular descriptor; no pointers, no waiting for another writer.
    if unsafe { libc::flock(f.as_raw_fd(), mode | libc::LOCK_NB) } != 0 {
        return Err(
            if std::io::Error::last_os_error().kind() == std::io::ErrorKind::WouldBlock {
                Error::Busy
            } else {
                storage("shared store coordination")
            },
        );
    }
    Ok(f)
}
pub(super) struct SharedLease<'a> {
    pub store: &'a FileStore,
    pub scope: ModelScope,
    _lock: File,
    shared: bool,
}
pub(in crate::storage) fn acquire<'a>(
    store: &'a FileStore,
    s: &ModelScope,
    read: bool,
) -> Result<Box<dyn BranchStorage + 'a>, Error> {
    metadata(store, s)?;
    let f = lock(store, read)?;
    metadata(store, s)?;
    Ok(Box::new(SharedLease {
        store,
        scope: scope(s)?,
        _lock: f,
        shared: read,
    }))
}

pub(in crate::storage) fn create(store: &FileStore, initial: Snapshot) -> Result<(), Error> {
    let s = scope(&initial.scope)?;
    if initial.sequence != 0 || initial.state != State::empty(s.clone()) {
        return Err(Error::InvalidInput(
            "files/4 genesis must be empty typed state".into(),
        ));
    }
    paths(store)?;
    let parent = store
        .root
        .parent()
        .ok_or_else(|| integrity("store parent"))?;
    store.check_profile(parent)?;
    let _creation = lock_file(
        &parent.join(format!(
            ".knowledgehub-create-{}.lock",
            codec::digest(store.root.file_name().unwrap().as_encoded_bytes())
        )),
        true,
    )?;
    store.sync_dir(parent)?;
    match fs::create_dir(&store.root) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            acquire(store, &initial.scope, true)?.head()?;
            return Err(Error::AlreadyExists);
        }
        Err(_) => return Err(storage("exclusive shared creation")),
    }
    for name in [
        "refs",
        "locks",
        "objects",
        "objects/loose",
        "objects/packs",
        "staging",
        "cache",
        "views",
    ] {
        fs::create_dir(store.root.join(name)).map_err(|_| storage("create shared layout"))?;
    }
    store.persist(&store.root.join("locks/store.lock"), b"")?;
    let meta = Metadata {
        format: ontology::FILE_FORMAT.into(),
        hub_identity: s.hub.clone(),
        replay_domain: store.allocate_id()?.0,
        scope: s.clone(),
        record_codec: ontology::RECORD_FORMAT.into(),
        request_codec: ontology::REQUEST_FORMAT.into(),
        object_grammar: "knowledgehub-object/1".into(),
        tree_grammars: vec![
            "knowledgehub-map-node/1".into(),
            "knowledgehub-sequence-node/1".into(),
        ],
        required_features: features(),
        filesystem_profile: native::declared(&store.support)?.into(),
    };
    store.persist(&store.root.join("hub.json"), &json(&meta)?)?;
    for name in [
        "objects/loose",
        "objects/packs",
        "objects",
        "locks",
        "refs",
        "staging",
        "cache",
        "views",
    ] {
        store.sync_dir(&store.root.join(name))?;
    }
    store.sync_dir(&store.root)?;
    store.sync_dir(parent)?;
    let _coordination = lock(store, false)?;
    store.point("staging")?;
    let objects = Objects::new(store);
    let id = write_generation(&objects, &initial, None, None)?;
    publish_reference(store, &load_generation(&objects, &id, &s)?, &id, None)?;
    store
        .sync_dir(&store.root)
        .and_then(|()| store.sync_dir(parent))
        .and_then(|()| store.fence(&store.root.join("hub.json")))
        .map_err(|_| Error::Uncertain {
            operation_id: None,
            stage: "shared creation persistence".into(),
        })
}

pub(super) fn load_generation(
    objects: &Objects<'_>,
    id: &str,
    s: &ModelScope,
) -> Result<Generation, Error> {
    let g: Generation = objects.get_json(id, "generation")?;
    let seq = number(&g.sequence)?;
    if g.schema != "knowledgehub-generation/1"
        || g.scope != *s
        || !model::hash(&g.snapshot_id.0)
        || seq > 9_007_199_254_740_991
        || (seq == 0) != (g.parent.is_none() && g.receipt.is_none())
        || (seq > 0 && (g.parent.is_none() || g.receipt.is_none()))
    {
        return Err(integrity("generation identity/scope/sequence"));
    }
    for r in [
        &g.selected_root,
        &g.revisions_root,
        &g.retained_refs_root,
        &g.artifacts_root,
        &g.gaps_root,
        &g.operations_root,
        &g.ancestors_root,
    ]
    .into_iter()
    .chain(g.parent.iter())
    .chain(g.receipt.iter())
    {
        if !model::hash(r) {
            return Err(integrity("generation object address"));
        }
    }
    Ok(g)
}

pub(super) fn root(
    store: &FileStore,
    s: &ModelScope,
    objects: &Objects<'_>,
) -> Result<(String, Generation), Error> {
    let path = head_path(store, s)?;
    let r: Reference = parse(&bounded_file(&path, 1_048_576)?)?;
    if r.format != ontology::FILE_FORMAT || r.scope != *s {
        return Err(Error::Incompatible(
            "shared branch reference format/scope".into(),
        ));
    }
    let g = load_generation(objects, &r.generation, s)?;
    if r.snapshot_id != g.snapshot_id || r.sequence != g.sequence {
        return Err(integrity("head/generation binding differs"));
    }
    store
        .sync_file(&open_read(&path)?, "shared recovery reference persistence")
        .and_then(|()| store.sync_dir(&store.root.join("refs")))
        .and_then(|()| store.fence(&path))
        .map_err(|_| Error::Uncertain {
            operation_id: None,
            stage: "shared recovery reference persistence".into(),
        })?;
    Ok((r.generation, g))
}

fn publish_reference(
    store: &FileStore,
    g: &Generation,
    id: &str,
    operation: Option<String>,
) -> Result<(), Error> {
    store.point("content-synced")?;
    store.point("generation-installed")?;
    let path = head_path(store, &g.scope)?;
    let temp = store
        .root
        .join("refs")
        .join(format!(".pending-{}", store.allocate_id()?.0));
    store.persist(
        &temp,
        &json(&Reference {
            format: ontology::FILE_FORMAT.into(),
            scope: g.scope.clone(),
            snapshot_id: g.snapshot_id.clone(),
            sequence: g.sequence.clone(),
            generation: id.into(),
        })?,
    )?;
    store.point("reference-synced")?;
    fs::rename(&temp, &path).map_err(|_| storage("replace accepted shared head"))?;
    let finish = || -> Result<(), Error> {
        store.point("head-replaced")?;
        store.sync_dir(&store.root.join("refs"))?;
        store.fence(&path)?;
        store.point("head-synced")?;
        store.point("acknowledgement")?;
        Ok(())
    };
    finish().map_err(|_| Error::Uncertain {
        operation_id: operation,
        stage: "shared head publication persistence".into(),
    })
}

fn write_generation(
    objects: &Objects<'_>,
    snapshot: &Snapshot,
    receipt: Option<&Receipt>,
    parent: Option<(&str, &Generation)>,
) -> Result<String, Error> {
    let state = &snapshot.state;
    let tree = Tree { objects };
    if state.scope.clone() != snapshot.scope {
        return Err(integrity("snapshot scope differs"));
    }
    let empty = || -> std::iter::Empty<(Value, Value)> { std::iter::empty() };
    let mut artifacts = Vec::new();
    for (key, c) in &state.artifacts {
        let r = state
            .records
            .get(key)
            .ok_or_else(|| integrity("artifact without source record"))?;
        let binding = content::put(objects, c)?;
        artifacts.push((ref_key(&r.reference), value!(binding)));
    }
    let artifacts_root = tree.map("artifacts/1", artifacts)?;
    let mut records = Vec::new();
    let mut refs = Vec::new();
    for (key, r) in &state.records {
        if *key != r.reference.key() {
            return Err(integrity("record map identity differs"));
        }
        let existing = parent
            .map(|(_, g)| tree.get(&g.revisions_root, "revisions/1", &ref_key(&r.reference)))
            .transpose()?
            .flatten();
        let id = match existing {
            Some(v) => v
                .as_str()
                .ok_or_else(|| integrity("record OID"))?
                .to_string(),
            None => projection::encode_record(objects, r, state, &artifacts_root)?,
        };
        records.push((ref_key(&r.reference), Value::String(id)));
        refs.push((ref_key(&r.reference), Value::Null));
    }
    let revisions_root = tree.map("revisions/1", records)?;
    let retained_refs_root = tree.map("exact-reference-set/1", refs)?;
    let mut selected = Vec::new();
    for (key, r) in &state.selected {
        if *key != r.identity() {
            return Err(integrity("selection key differs"));
        }
        selected.push((identity_key(r), value!(r)));
    }
    let selected_root = tree.map("selection/1", selected)?;
    let gaps_root = projection::put_gaps(
        objects,
        parent.map(|(_, g)| g.gaps_root.as_str()),
        &state.gaps,
    )?;
    let mut operations_root = match parent {
        Some((_, g)) => g.operations_root.clone(),
        None => tree.map("operations/1", empty())?,
    };
    let mut ancestors_root = match parent {
        Some((_, g)) => g.ancestors_root.clone(),
        None => tree.map("ancestors/1", empty())?,
    };
    if let Some((id, g)) = parent {
        ancestors_root = tree.update(
            &ancestors_root,
            "ancestors/1",
            value!(g.snapshot_id.0),
            Some(value!(Ancestor {
                generation: id.into(),
                sequence: g.sequence.clone()
            })),
        )?;
    }
    let receipt_id = receipt
        .map(|r| projection::encode_receipt(objects, r))
        .transpose()?;
    if let (Some(r), Some(id)) = (receipt, &receipt_id) {
        if tree
            .get(&operations_root, "operations/1", &value!(r.operation_id))?
            .is_some()
        {
            return Err(Error::OperationReuse);
        }
        operations_root = tree.update(
            &operations_root,
            "operations/1",
            value!(r.operation_id),
            Some(value!(id)),
        )?;
    }
    objects.put_json(
        "generation",
        &Generation {
            schema: "knowledgehub-generation/1".into(),
            scope: state.scope.clone(),
            snapshot_id: snapshot.id.clone(),
            sequence: snapshot.sequence.to_string(),
            parent: parent.map(|(id, _)| id.into()),
            ontology: state.ontology.clone(),
            profile: state.profile.clone(),
            selected_root,
            revisions_root,
            retained_refs_root,
            artifacts_root,
            gaps_root,
            operations_root,
            ancestors_root,
            receipt: receipt_id,
        },
    )
}

fn ancestor(
    objects: &Objects<'_>,
    head_id: &str,
    head: &Generation,
    id: &SnapshotId,
) -> Result<(String, Generation), Error> {
    if *id == head.snapshot_id {
        return Ok((head_id.into(), head.clone()));
    }
    let a = Tree { objects }
        .get(&head.ancestors_root, "ancestors/1", &value!(id.0))?
        .ok_or(Error::NotFound)?;
    let a: Ancestor = serde_json::from_value(a).map_err(|_| integrity("ancestor mapping"))?;
    let g = load_generation(objects, &a.generation, &head.scope)?;
    if g.snapshot_id != *id
        || g.sequence != a.sequence
        || number(&g.sequence)? >= number(&head.sequence)?
    {
        return Err(integrity("accepted ancestor differs"));
    }
    Ok((a.generation, g))
}
fn original(
    objects: &Objects<'_>,
    head_id: &str,
    head: &Generation,
    id: &str,
) -> Result<Option<Receipt>, Error> {
    let Some(v) = Tree { objects }.get(&head.operations_root, "operations/1", &value!(id))? else {
        return Ok(None);
    };
    let oid = v
        .as_str()
        .ok_or_else(|| integrity("operation receipt address"))?;
    let r = projection::decode_receipt(objects, oid)?;
    let (_, g) = ancestor(objects, head_id, head, &r.head)
        .map_err(|_| integrity("receipt references unaccepted history"))?;
    if r.operation_id != id
        || r.scope != head.scope.clone()
        || r.fingerprint_version != ontology::REQUEST_FORMAT
        || !model::hash(&r.fingerprint)
        || r.actor.trim().is_empty()
        || r.sequence != number(&g.sequence)?
        || g.receipt.as_deref() != Some(oid)
    {
        return Err(integrity("original receipt binding"));
    }
    let parent = load_generation(
        objects,
        g.parent
            .as_deref()
            .ok_or_else(|| integrity("receipt generation parent"))?,
        &head.scope,
    )?;
    if r.previous_head != parent.snapshot_id
        || number(&parent.sequence)?.checked_add(1) != Some(r.sequence)
    {
        return Err(integrity("original receipt parent binding"));
    }
    Ok(Some(r))
}

pub(super) fn materialise(
    objects: &Objects<'_>,
    g: &Generation,
    id: &str,
) -> Result<Stored, Error> {
    objects
        .store
        .work
        .materialisations
        .fetch_add(1, Ordering::Relaxed);
    let tree = Tree { objects };
    let mut state = State::empty(g.scope.clone());
    state.ontology = g.ontology.clone();
    state.profile = g.profile.clone();
    let registry = tree.entries(&g.revisions_root, "revisions/1")?;
    let refs = tree.entries(&g.retained_refs_root, "exact-reference-set/1")?;
    if registry.len() != refs.len()
        || registry
            .iter()
            .map(|(k, _)| k)
            .ne(refs.iter().map(|(k, _)| k))
        || refs.iter().any(|(_, v)| *v != Value::Null)
    {
        return Err(integrity("companion retained key set differs"));
    }
    for (key, v) in registry {
        let r = projection::decode_record(
            objects,
            v.as_str().ok_or_else(|| integrity("record address"))?,
        )?;
        if ref_key(&r.reference) != key {
            return Err(integrity("registry/record exact reference"));
        }
        state.records.insert(r.reference.key(), r);
    }
    for (key, v) in tree.entries(&g.selected_root, "selection/1")? {
        let r: ExactRef = serde_json::from_value(v).map_err(|_| integrity("selected reference"))?;
        if identity_key(&r) != key || state.get(&r).is_none() {
            return Err(integrity("selection differs from registry"));
        }
        state.selected.insert(r.identity(), r);
    }
    for (key, v) in tree.entries(&g.artifacts_root, "artifacts/1")? {
        let r = exact_key(&key)?;
        let binding: Binding =
            serde_json::from_value(v).map_err(|_| integrity("artifact binding"))?;
        state
            .artifacts
            .insert(r.key(), content::read(objects, &binding)?);
    }
    state.gaps = projection::read_gaps(objects, &g.gaps_root)?;
    let mut snapshot = model::empty(g.scope.clone(), g.snapshot_id.clone());
    snapshot.sequence = number(&g.sequence)?;
    snapshot.state = state;
    if snapshot.sequence == 0 && snapshot.state != State::empty(g.scope.clone()) {
        return Err(integrity("nonempty shared genesis"));
    }
    if !model::validate_snapshot(&snapshot).is_empty() {
        return Err(integrity("recovered state violates model integrity"));
    }
    let receipt = g
        .receipt
        .as_ref()
        .map(|oid| projection::decode_receipt(objects, oid))
        .transpose()?;
    if let Some(r) = &receipt {
        if r.scope != snapshot.scope
            || r.head != snapshot.id
            || r.sequence != snapshot.sequence
            || r.fingerprint_version != ontology::REQUEST_FORMAT
            || !model::hash(&r.fingerprint)
        {
            return Err(integrity("generation receipt differs"));
        }
    }
    Ok(Stored {
        snapshot,
        receipt,
        reference: HeadRef {
            snapshot: g.snapshot_id.clone(),
            manifest_sha256: id.into(),
        },
    })
}

pub(super) struct View {
    pub g: Generation,
    pub selected: String,
    pub supporting: Option<String>,
    pub artifacts: String,
}
pub(super) fn view(
    objects: &Objects<'_>,
    head_id: &str,
    head: &Generation,
    selection: &Selection,
) -> Result<View, Error> {
    match selection {
        Selection::Snapshot(id) => {
            let (_, g) = ancestor(objects, head_id, head, id)?;
            Ok(View {
                selected: g.selected_root.clone(),
                supporting: None,
                artifacts: g.artifacts_root.clone(),
                g,
            })
        }
        Selection::Baseline(r) => {
            let oid = Tree { objects }
                .get(&head.revisions_root, "revisions/1", &ref_key(r))?
                .ok_or(Error::NotFound)?;
            let b = projection::baseline(
                objects,
                oid.as_str().ok_or_else(|| integrity("baseline address"))?,
                r,
            )?;
            let mut g = head.clone();
            g.ontology = Some(b.ontology);
            g.profile = Some(b.profile);
            g.gaps_root = b.gaps_root;
            Ok(View {
                g,
                selected: b.selected_root,
                supporting: Some(b.supporting_root),
                artifacts: b.artifacts_root,
            })
        }
    }
}
pub(super) fn eligible(tree: &Tree<'_, '_>, v: &View, r: &ExactRef) -> Result<bool, Error> {
    let Some(supporting) = &v.supporting else {
        return Ok(true);
    };
    Ok(
        tree.get(&v.selected, "selection/1", &identity_key(r))? == Some(value!(r))
            || tree.get(supporting, "exact-reference-set/1", &ref_key(r))? == Some(Value::Null),
    )
}
pub(super) fn record(objects: &Objects<'_>, v: &View, r: &ExactRef) -> Result<Record, Error> {
    let tree = Tree { objects };
    if r.revision == "@identity" {
        let selected = tree.get(&v.selected, "selection/1", &identity_key(r))?;
        let mut current = if let Some(value) = selected {
            serde_json::from_value(value).map_err(|_| integrity("identity selection"))?
        } else {
            let refs = match &v.supporting {
                Some(root) => tree.entries(root, "exact-reference-set/1")?,
                None => tree.entries(&v.g.retained_refs_root, "exact-reference-set/1")?,
            };
            refs.into_iter()
                .filter_map(|(key, _)| exact_key(&key).ok())
                .find(|candidate| candidate.identity() == r.identity())
                .ok_or(Error::NotFound)?
        };
        let mut seen = BTreeSet::new();
        loop {
            if !seen.insert(current.clone()) || seen.len() > 1_000_000 {
                return Err(integrity("identity lineage cycle"));
            }
            let original = record(objects, v, &current)?;
            if let Some(parent) = original.predecessor {
                current = parent;
                continue;
            }
            return Record::new(
                r.clone(),
                ontology::type_ref("ElementIdentity"),
                RecordData::Identity {
                    subject: original.reference,
                    declared_type: original.type_ref,
                },
                original.provenance,
            );
        }
    }
    if !eligible(&tree, v, r)? {
        return Err(Error::NotFound);
    }
    let oid = tree
        .get(&v.g.revisions_root, "revisions/1", &ref_key(r))?
        .ok_or(Error::NotFound)?;
    let result = projection::decode_record(
        objects,
        oid.as_str()
            .ok_or_else(|| integrity("record lookup address"))?,
    )?;
    if result.reference != *r {
        return Err(integrity("point record identity differs"));
    }
    Ok(result)
}
pub(super) fn binding(objects: &Objects<'_>, v: &View, r: &ExactRef) -> Result<Binding, Error> {
    let tree = Tree { objects };
    if !eligible(&tree, v, r)? {
        return Err(Error::NotFound);
    }
    serde_json::from_value(
        tree.get(&v.artifacts, "artifacts/1", &ref_key(r))?
            .ok_or(Error::NotFound)?,
    )
    .map_err(|_| integrity("content binding"))
}

impl BranchStorage for SharedLease<'_> {
    fn model_view(&self, selection: &Selection) -> Result<ontology::impact::ViewInfo, Error> {
        let o = Objects::new(self.store);
        let (id, g) = root(self.store, &self.scope, &o)?;
        let v = view(&o, &id, &g, selection)?;
        let (gaps, gaps_omitted) = projection::bounded_gaps(&o, &v.g.gaps_root)?;
        Ok(ontology::impact::ViewInfo {
            ontology: v.g.ontology,
            profile: v.g.profile,
            gaps,
            gaps_omitted,
        })
    }
    fn search_text(
        &self,
        q: &ontology::SearchText,
        prepare: bool,
    ) -> Result<ontology::SearchResult, Error> {
        if prepare && self.shared {
            return Err(storage("text preparation through read lease"));
        }
        super::text::search(self.store, &self.scope, q, prepare)
    }
    fn prepare_neighbours(
        &self,
        request: &ontology::Neighbours,
        force: bool,
    ) -> Result<ontology::NeighbourWork, Error> {
        if self.shared {
            return Err(storage("index preparation through read lease"));
        }
        super::neighbours::prepare(self.store, &self.scope, request, force)
    }
    fn neighbours(
        &self,
        request: &ontology::Neighbours,
        position: &ontology::neighbours::Position,
    ) -> Result<ontology::neighbours::Slice, Error> {
        super::neighbours::read(self.store, &self.scope, request, position)
    }
    fn head(&self) -> Result<HeadRef, Error> {
        let objects = Objects::new(self.store);
        let (id, g) = root(self.store, &self.scope, &objects)?;
        Ok(HeadRef {
            snapshot: g.snapshot_id,
            manifest_sha256: id,
        })
    }
    fn current(&self) -> Result<Stored, Error> {
        let objects = Objects::new(self.store);
        let (id, g) = root(self.store, &self.scope, &objects)?;
        materialise(&objects, &g, &id)
    }
    fn operation(&self, id: &str) -> Result<Option<Receipt>, Error> {
        let objects = Objects::new(self.store);
        let (head, g) = root(self.store, &self.scope, &objects)?;
        original(&objects, &head, &g, id)
    }
    fn state(&self, selection: &Selection) -> Result<State, Error> {
        let objects = Objects::new(self.store);
        let (id, g) = root(self.store, &self.scope, &objects)?;
        if let Selection::Snapshot(snapshot) = selection {
            let (id, g) = ancestor(&objects, &id, &g, snapshot)?;
            return Ok(materialise(&objects, &g, &id)?.snapshot.state);
        }
        let v = view(&objects, &id, &g, selection)?;
        let tree = Tree { objects: &objects };
        let mut state = State::empty(v.g.scope.clone());
        state.ontology = v.g.ontology.clone();
        state.profile = v.g.profile.clone();
        let mut refs = BTreeSet::new();
        for (key, value) in tree.entries(&v.selected, "selection/1")? {
            let r: ExactRef =
                serde_json::from_value(value).map_err(|_| integrity("baseline selected value"))?;
            if identity_key(&r) != key {
                return Err(integrity("baseline selection key"));
            }
            state.selected.insert(r.identity(), r.clone());
            refs.insert(r);
        }
        for (key, value) in tree.entries(v.supporting.as_ref().unwrap(), "exact-reference-set/1")? {
            if value != Value::Null {
                return Err(integrity("supporting set value"));
            }
            refs.insert(exact_key(&key)?);
        }
        for r in refs {
            state.records.insert(r.key(), record(&objects, &v, &r)?);
            if tree
                .get(&v.artifacts, "artifacts/1", &ref_key(&r))?
                .is_some()
            {
                state.artifacts.insert(
                    r.key(),
                    content::read(&objects, &binding(&objects, &v, &r)?)?,
                );
            }
        }
        state.gaps = projection::read_gaps(&objects, &v.g.gaps_root)?;
        Ok(state)
    }
    fn record(&self, selection: &Selection, r: &ExactRef) -> Result<Record, Error> {
        let objects = Objects::new(self.store);
        let (id, g) = root(self.store, &self.scope, &objects)?;
        record(&objects, &view(&objects, &id, &g, selection)?, r)
    }
    fn artifact(&self, selection: &Selection, r: &ExactRef) -> Result<Content, Error> {
        let objects = Objects::new(self.store);
        let (id, g) = root(self.store, &self.scope, &objects)?;
        content::read(
            &objects,
            &binding(&objects, &view(&objects, &id, &g, selection)?, r)?,
        )
    }
    fn artifact_range(
        &self,
        selection: &Selection,
        r: &ExactRef,
        start: u64,
        len: u64,
    ) -> Result<ArtifactRange, Error> {
        let objects = Objects::new(self.store);
        let (id, g) = root(self.store, &self.scope, &objects)?;
        let v = view(&objects, &id, &g, selection)?;
        let b = binding(&objects, &v, r)?;
        let (bytes, full_content_verified) = content::range(&objects, &b, start, len)?;
        Ok(ArtifactRange {
            selection: selection.clone(),
            reference: r.clone(),
            original: b.reference,
            offset: start,
            bytes,
            full_content_verified,
        })
    }
    fn publish(&self, plan: ValidatedPlan, parent: HeadRef) -> Result<Receipt, Error> {
        if self.shared {
            return Err(storage("publication through read lease"));
        }
        let objects = Objects::new(self.store);
        let (id, g) = root(self.store, &self.scope, &objects)?;
        if id != parent.manifest_sha256 || g.snapshot_id != parent.snapshot {
            return Err(Error::Conflict {
                expected: parent.snapshot,
                actual: g.snapshot_id,
            });
        }
        let old = materialise(&objects, &g, &id)?;
        if plan.snapshot.sequence != old.snapshot.sequence + 1
            || !model::verify_affected(&old.snapshot, &plan.snapshot, &plan.receipt)
            || !model::validate_snapshot(&plan.snapshot).is_empty()
            || plan.receipt.previous_head != old.snapshot.id
            || plan.receipt.head != plan.snapshot.id
        {
            return Err(integrity("invalid shared publication plan"));
        }
        if original(&objects, &id, &g, &plan.receipt.operation_id)?.is_some() {
            return Err(Error::OperationReuse);
        }
        self.store.point("staging")?;
        let oid = write_generation(
            &objects,
            &plan.snapshot,
            Some(&plan.receipt),
            Some((&id, &g)),
        )?;
        let next = load_generation(&objects, &oid, &self.scope)?;
        publish_reference(
            self.store,
            &next,
            &oid,
            Some(plan.receipt.operation_id.clone()),
        )?;
        if super::neighbours::after_commit(
            self.store,
            &g,
            &next,
            &old.snapshot.state,
            &plan.snapshot.state,
        )
        .is_err()
        {
            self.store
                .work
                .neighbour_failures
                .fetch_add(1, Ordering::Relaxed);
        }
        // Derived maintenance cannot invalidate an already durable accepted receipt.
        let _ = super::text::after_commit(
            self.store,
            &g,
            &next,
            &old.snapshot.state,
            &plan.snapshot.state,
        );
        Ok(plan.receipt)
    }
}
