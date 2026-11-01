//! Disposable, bounded indexes. Accepted roots never depend on this namespace.
use super::*;
use super::{
    generation::{self, Generation, View},
    objects::Objects,
    trees::{Tree, exact_key, identity_key, ref_key},
};
use crate::ontology::{
    self,
    neighbours::{Fact, Position, Posting, Slice, facts},
    *,
};
use serde_json::{Value as Json, json as value};
mod read;
#[cfg(test)]
mod tests;
pub(super) use read::read;
const QUOTA: u64 = 256 * 1024 * 1024;
const VIEWS: usize = 32;
const ENDPOINTS: &str = "neighbour-endpoints/1";
const SELECTORS: &str = "neighbour-selectors/1";
const POSTINGS: &str = "neighbour-postings/1";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Binding {
    scope: ModelScope,
    selected: String,
    supporting: String,
    ontology: Option<ExactRef>,
    profile: Option<ExactRef>,
    fact_scope: FactScope,
    extractor: String,
    categories: String,
}
impl Binding {
    fn view(v: &View, scope: FactScope) -> Self {
        Self {
            scope: v.g.scope.clone(),
            selected: v.selected.clone(),
            supporting: v
                .supporting
                .clone()
                .unwrap_or_else(|| v.g.retained_refs_root.clone()),
            ontology: v.g.ontology.clone(),
            profile: v.g.profile.clone(),
            fact_scope: scope,
            extractor: "knowledgehub-neighbour-extractor/1".into(),
            categories: "knowledgehub-neighbour-categories/1".into(),
        }
    }
    fn generation(g: &Generation, scope: FactScope) -> Self {
        Self {
            scope: g.scope.clone(),
            selected: g.selected_root.clone(),
            supporting: g.retained_refs_root.clone(),
            ontology: g.ontology.clone(),
            profile: g.profile.clone(),
            fact_scope: scope,
            extractor: "knowledgehub-neighbour-extractor/1".into(),
            categories: "knowledgehub-neighbour-categories/1".into(),
        }
    }
    fn key(&self) -> Result<String, Error> {
        codec::hash_value("knowledgehub-neighbour-view/1", self)
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema: String,
    binding: Binding,
    outgoing: String,
    incoming: String,
    owners: u64,
    facts: u64,
    checksum: String,
}
impl Manifest {
    fn digest(&self) -> Result<String, Error> {
        codec::hash_value(
            "knowledgehub-neighbour-completion/1",
            &(
                &self.schema,
                &self.binding,
                &self.outgoing,
                &self.incoming,
                self.owners,
                self.facts,
            ),
        )
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Group {
    root: String,
    count: String,
    categories: BTreeSet<NeighbourCategory>,
}
fn missing(reason: impl Into<String>) -> Error {
    Error::IndexNotReady {
        reason: reason.into(),
    }
}
fn derived(error: Error) -> Error {
    missing(format!("derived neighbour cache: {error}"))
}
fn cache(store: &FileStore) -> FileStore {
    FileStore {
        root: store.root.join("cache/neighbours"),
        format: ontology::SHARED_FILE_FORMAT,
        support: store.support.clone(),
        work: Counters {
            locations: store.work.neighbour_locations.clone(),
            peak_allocated: store.work.neighbour_peak.clone(),
            ..Counters::default()
        },
        loose_budget: None,
        packed_cache: true,
        #[cfg(test)]
        hook: std::sync::Mutex::new(store.hook.lock().unwrap().clone()),
    }
}
fn layout(c: &FileStore) -> Result<(), Error> {
    for name in [
        "",
        "objects",
        "objects/loose",
        "objects/packs",
        "staging",
        "views",
    ] {
        let p = c.root.join(name);
        match fs::create_dir(&p) {
            Ok(()) => sync_dir(p.parent().unwrap())?,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => is_dir(&p)?,
            Err(_) => return Err(storage("create neighbour cache layout")),
        }
    }
    Ok(())
}
fn usage(path: &Path) -> Result<u64, Error> {
    let m = fs::symlink_metadata(path).map_err(|_| integrity("cache quota metadata"))?;
    if m.is_file() {
        return Ok(m.blocks() * 512);
    }
    if !m.is_dir() {
        return Err(integrity("nonordinary derived cache entry"));
    }
    let mut n = 0;
    for e in fs::read_dir(path).map_err(|_| integrity("cache quota inventory"))? {
        n += usage(&e.map_err(|_| integrity("cache quota entry"))?.path())?;
        if n > QUOTA {
            return Err(Error::Capacity);
        }
    }
    Ok(n)
}
fn clear(store: &FileStore) -> Result<(), Error> {
    *store.work.neighbour_locations.lock().unwrap() = None;
    let c = cache(store);
    match fs::symlink_metadata(&c.root) {
        Ok(m) => {
            if m.is_dir() {
                fs::remove_dir_all(&c.root)
            } else {
                fs::remove_file(&c.root)
            }
            .map_err(|_| storage("evict neighbour cache"))?;
            sync_dir(&store.root.join("cache"))?;
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(_) => return Err(storage("inspect neighbour cache")),
    }
    layout(&c)
}
fn writer_cache(store: &FileStore, force_clear: bool) -> Result<FileStore, Error> {
    let mut c = cache(store);
    if force_clear || layout(&c).is_err() {
        clear(store)?;
    }
    let count = fs::read_dir(c.root.join("views"))
        .map_err(|_| integrity("cache view inventory"))?
        .count();
    let occupied = usage(&c.root);
    if count >= VIEWS
        || occupied.as_ref().is_err()
        || occupied.as_ref().is_ok_and(|n| *n > QUOTA * 3 / 4)
    {
        clear(store)?;
    }
    c.loose_budget = Some(AtomicU64::new(QUOTA.saturating_sub(usage(&c.root)?)));
    c.work.allocation_limit.store(QUOTA, Ordering::Relaxed);
    Ok(c)
}
fn path(c: &FileStore, b: &Binding) -> Result<PathBuf, Error> {
    Ok(c.root.join("views").join(format!("{}.json", b.key()?)))
}
fn manifest(c: &FileStore, b: &Binding) -> Result<Manifest, Error> {
    is_dir(&c.root).map_err(derived)?;
    is_dir(&c.root.join("views")).map_err(derived)?;
    let bytes = super::objects::bounded_file(&path(c, b)?, 65536).map_err(derived)?;
    let m: Manifest = parse(&bytes).map_err(derived)?;
    if m.schema != "knowledgehub-neighbour-index/1"
        || m.binding != *b
        || m.checksum != m.digest()?
        || json(&m)? != bytes
        || !model::hash(&m.outgoing)
        || !model::hash(&m.incoming)
        || m.owners > 100_000
        || m.facts > 100_000
    {
        return Err(missing(
            "neighbour manifest version, binding or completed-build checksum differs",
        ));
    }
    Ok(m)
}
fn install(store: &FileStore, c: &FileStore, mut m: Manifest) -> Result<(), Error> {
    m.checksum = m.digest()?;
    let bytes = json(&m)?;
    if bytes.len() > 65536 {
        return Err(Error::Capacity);
    }
    super::reserve_file(c, bytes.len())?;
    store.point("neighbour-nodes-synced")?;
    let temp = c
        .root
        .join("staging")
        .join(format!("manifest-{}", store.allocate_id()?.0));
    persist(&temp, &bytes)?;
    sync_dir(&c.root.join("staging"))?;
    store.point("neighbour-manifest-synced")?;
    fs::rename(&temp, path(c, &m.binding)?)
        .map_err(|_| storage("install complete neighbour view"))?;
    sync_dir(&c.root.join("views"))?;
    sync_dir(&c.root.join("staging"))?;
    store.point("neighbour-cache-installed")
}

struct Records<'a, 'b> {
    objects: &'a Objects<'b>,
    view: &'a View,
    values: BTreeMap<ExactRef, Record>,
    reads: u64,
    bytes: usize,
    limit: usize,
}
impl Records<'_, '_> {
    fn get(&mut self, r: &ExactRef) -> Result<Record, Error> {
        if let Some(v) = self.values.get(r) {
            return Ok(v.clone());
        }
        let record = generation::record(self.objects, self.view, r)?;
        let n = json(&record)?.len();
        self.bytes = self.bytes.checked_add(n).ok_or(Error::Capacity)?;
        if self.bytes > self.limit {
            return Err(Error::Capacity);
        }
        self.reads += 1;
        self.values.insert(r.clone(), record.clone());
        Ok(record)
    }
}
fn owners(
    objects: &Objects<'_>,
    view: &View,
    scope: FactScope,
    limit: usize,
) -> Result<Vec<ExactRef>, Error> {
    let t = Tree { objects };
    if t.map_count(&view.selected, "selection/1")? > limit as u64 {
        return Err(missing("index preparation owner budget"));
    }
    let mut result = BTreeSet::new();
    for (_, v) in t.entries(&view.selected, "selection/1")? {
        result.insert(serde_json::from_value(v).map_err(|_| integrity("index owner selection"))?);
    }
    if scope == FactScope::Closure {
        let root = view
            .supporting
            .as_ref()
            .unwrap_or(&view.g.retained_refs_root);
        if t.map_count(root, "exact-reference-set/1")? > limit as u64 {
            return Err(missing("index preparation closure owner budget"));
        }
        for (key, v) in t.entries(root, "exact-reference-set/1")? {
            if v != Json::Null {
                return Err(integrity("index supporting set value"));
            }
            result.insert(exact_key(&key)?);
        }
    }
    if result.len() > limit {
        return Err(missing("index preparation owner budget"));
    }
    Ok(result.into_iter().collect())
}
fn group(
    tree: &Tree<'_, '_>,
    previous: Option<&Group>,
    items: &BTreeSet<Posting>,
) -> Result<Group, Error> {
    let values = items.iter().map(|p| value!(p)).collect::<Vec<_>>();
    Ok(Group {
        root: tree.reconcile(previous.map(|g| g.root.as_str()), POSTINGS, &values)?,
        count: items.len().to_string(),
        categories: items.iter().map(|p| p.category).collect(),
    })
}
type Adjacency = BTreeMap<ExactRef, BTreeMap<NeighbourSelector, BTreeSet<Posting>>>;
fn build_root(tree: &Tree<'_, '_>, facts: &BTreeSet<Fact>, reverse: bool) -> Result<String, Error> {
    let mut endpoints: Adjacency = BTreeMap::new();
    for f in facts {
        let (from, to) = if reverse {
            (&f.to, &f.from)
        } else {
            (&f.from, &f.to)
        };
        endpoints
            .entry(from.clone())
            .or_default()
            .entry(f.selector.clone())
            .or_default()
            .insert(Posting {
                opposite: to.clone(),
                origin: f.origin.clone(),
                category: f.category,
            });
    }
    let mut outer = Vec::new();
    for (endpoint, selectors) in endpoints {
        let mut inner = Vec::new();
        for (selector, items) in selectors {
            inner.push((value!(selector), value!(group(tree, None, &items)?)));
        }
        outer.push((ref_key(&endpoint), value!(tree.map(SELECTORS, inner)?)));
    }
    tree.map(ENDPOINTS, outer)
}

pub(super) fn prepare(
    store: &FileStore,
    scope: &ModelScope,
    q: &Neighbours,
    force: bool,
) -> Result<NeighbourWork, Error> {
    let objects = Objects::new(store);
    let (id, g) = generation::root(store, scope, &objects)?;
    let v = generation::view(&objects, &id, &g, &q.selection)?;
    let binding = Binding::view(&v, q.fact_scope);
    let prior = cache(store);
    if !force {
        if let Ok(m) = manifest(&prior, &binding) {
            return Ok(NeighbourWork {
                indexed_owners: m.owners,
                indexed_facts: m.facts,
                indexed_postings: 2 * m.facts,
                ..NeighbourWork::default()
            });
        }
    }
    if !q.preparation.allow_rebuild {
        return Err(missing("index not ready; preparation disabled"));
    }
    let exists = path(&prior, &binding)?
        .try_exists()
        .map_err(|_| storage("inspect cache readiness"))?;
    let c = writer_cache(store, force && exists)?;
    let index = Objects::new(&c);
    let tree = Tree { objects: &index };
    let refs = owners(&objects, &v, q.fact_scope, q.preparation.owner_limit)?;
    let mut records = Records {
        objects: &objects,
        view: &v,
        values: BTreeMap::new(),
        reads: 0,
        bytes: 0,
        limit: q.preparation.byte_limit,
    };
    let ontology = v.g.ontology.as_ref().map(|r| records.get(r)).transpose()?;
    let mut all = BTreeSet::new();
    let mut bytes = 0;
    for r in &refs {
        let owner = records.get(r)?;
        let extracted = facts::extract(&owner, ontology.as_ref(), &mut |r| records.get(r))?;
        for fact in extracted {
            bytes += json(&fact)?.len();
            if bytes > q.preparation.byte_limit || all.len() >= q.preparation.fact_limit {
                return Err(missing("index preparation fact/byte budget"));
            }
            all.insert(fact);
        }
    }
    let outgoing = build_root(&tree, &all, false)?;
    let incoming = build_root(&tree, &all, true)?;
    index.finish()?;
    install(
        store,
        &c,
        Manifest {
            schema: "knowledgehub-neighbour-index/1".into(),
            binding,
            outgoing,
            incoming,
            owners: refs.len() as u64,
            facts: all.len() as u64,
            checksum: String::new(),
        },
    )?;
    Ok(NeighbourWork {
        indexed_owners: refs.len() as u64,
        indexed_facts: all.len() as u64,
        indexed_postings: 2 * all.len() as u64,
        prepared_owners: refs.len() as u64,
        prepared_facts: all.len() as u64,
        index_objects_read: c.work.snapshot().object_reads,
        index_bytes_read: c.work.snapshot().encoded_bytes_read + c.work.snapshot().index_bytes_read,
        index_metadata_bytes_read: c.work.snapshot().index_bytes_read,
        record_reads: records.reads,
        record_bytes: records.bytes as u64,
        index_rebuilt: true,
        rebuild_reason: Some(
            if exists {
                "discarded incompatible/corrupt cache"
            } else {
                "view not prepared"
            }
            .into(),
        ),
        ..NeighbourWork::default()
    })
}

type Deltas = BTreeMap<(ExactRef, NeighbourSelector), (BTreeSet<Posting>, BTreeSet<Posting>)>;
fn update_root(
    tree: &Tree<'_, '_>,
    root: &str,
    removed: &BTreeSet<Fact>,
    added: &BTreeSet<Fact>,
    reverse: bool,
) -> Result<String, Error> {
    let mut deltas: Deltas = BTreeMap::new();
    for (is_add, facts) in [(false, removed), (true, added)] {
        for f in facts {
            let (from, to) = if reverse {
                (&f.to, &f.from)
            } else {
                (&f.from, &f.to)
            };
            let entry = deltas
                .entry((from.clone(), f.selector.clone()))
                .or_default();
            let p = Posting {
                opposite: to.clone(),
                origin: f.origin.clone(),
                category: f.category,
            };
            if is_add {
                entry.1.insert(p);
            } else {
                entry.0.insert(p);
            }
        }
    }
    let mut by_endpoint: BTreeMap<ExactRef, Vec<_>> = BTreeMap::new();
    for ((endpoint, selector), delta) in deltas {
        by_endpoint
            .entry(endpoint)
            .or_default()
            .push((selector, delta));
    }
    let mut outer = Vec::new();
    for (endpoint, deltas) in by_endpoint {
        let inner = tree.get(root, ENDPOINTS, &ref_key(&endpoint))?;
        let inner = match inner {
            Some(v) => v
                .as_str()
                .ok_or_else(|| integrity("endpoint selector root"))?
                .to_string(),
            None => tree.map(SELECTORS, [])?,
        };
        let mut changes = Vec::new();
        for (selector, (remove, add)) in deltas {
            let previous = tree
                .get(&inner, SELECTORS, &value!(selector))?
                .map(|v| {
                    serde_json::from_value::<Group>(v)
                        .map_err(|_| integrity("index selector group"))
                })
                .transpose()?;
            let mut items = match &previous {
                Some(g) => tree
                    .items(&g.root, POSTINGS)?
                    .into_iter()
                    .map(|v| {
                        serde_json::from_value::<Posting>(v).map_err(|_| integrity("posting value"))
                    })
                    .collect::<Result<BTreeSet<_>, Error>>()?,
                None => BTreeSet::new(),
            };
            for p in remove {
                if !items.remove(&p) {
                    return Err(integrity("incremental removal absent"));
                }
            }
            items.extend(add);
            let v = if items.is_empty() {
                None
            } else {
                Some(value!(group(tree, previous.as_ref(), &items)?))
            };
            changes.push((value!(selector), v));
        }
        let inner = tree.update_many(&inner, SELECTORS, changes)?;
        outer.push((
            ref_key(&endpoint),
            if tree.map_count(&inner, SELECTORS)? == 0 {
                None
            } else {
                Some(value!(inner))
            },
        ));
    }
    tree.update_many(root, ENDPOINTS, outer)
}

fn state_owners(s: &State, scope: FactScope) -> BTreeSet<ExactRef> {
    match scope {
        FactScope::Selected => s.selected.values().cloned().collect(),
        FactScope::Closure => s.records.values().map(|r| r.reference.clone()).collect(),
    }
}
pub(super) fn after_commit(
    store: &FileStore,
    old: &Generation,
    next: &Generation,
    before: &State,
    after: &State,
) -> Result<(), Error> {
    if old.ontology != next.ontology || old.profile != next.profile {
        return Ok(());
    }
    for scope in [FactScope::Selected, FactScope::Closure] {
        let binding = Binding::generation(old, scope);
        let original = cache(store);
        let Ok(m) = manifest(&original, &binding) else {
            continue;
        };
        let c = writer_cache(store, false)?;
        // Quota eviction may discard the old view; leave the new view explicitly unprepared.
        if manifest(&c, &binding).is_err() {
            continue;
        }
        let index = Objects::new(&c);
        let tree = Tree { objects: &index };
        let old_owners = state_owners(before, scope);
        let new_owners = state_owners(after, scope);
        let extract = |refs: Vec<ExactRef>, s: &State| -> Result<BTreeSet<Fact>, Error> {
            let ontology = s.ontology.as_ref().and_then(|r| s.get(r));
            let mut all = BTreeSet::new();
            for r in refs {
                all.extend(facts::extract(
                    s.get(&r).ok_or_else(|| integrity("changed index owner"))?,
                    ontology,
                    &mut |r| s.resolve(r).ok_or(Error::NotFound),
                )?);
                if all.len() > 100_000 {
                    return Err(Error::Capacity);
                }
            }
            Ok(all)
        };
        let removed = extract(
            old_owners.difference(&new_owners).cloned().collect(),
            before,
        )?;
        let added = extract(new_owners.difference(&old_owners).cloned().collect(), after)?;
        let facts = m
            .facts
            .checked_sub(removed.len() as u64)
            .and_then(|n| n.checked_add(added.len() as u64))
            .filter(|n| *n <= 100_000)
            .ok_or(Error::Capacity)?;
        let outgoing = update_root(&tree, &m.outgoing, &removed, &added, false)?;
        let incoming = update_root(&tree, &m.incoming, &removed, &added, true)?;
        index.finish()?;
        install(
            store,
            &c,
            Manifest {
                schema: m.schema,
                binding: Binding::generation(next, scope),
                outgoing,
                incoming,
                owners: new_owners.len() as u64,
                facts,
                checksum: String::new(),
            },
        )?;
    }
    Ok(())
}

impl FileBackend {
    /// Evict only disposable neighbour indexes, coordinated with all readers and writers.
    pub async fn clear_neighbour_cache(
        &self,
        context: crate::Context,
        scope: ModelScope,
    ) -> Result<(), Error> {
        let b = self.clone();
        self.jobs
            .run(move || {
                context.check(&scope.transport(), crate::Action::Commit)?;
                if b.store.version() != ontology::SHARED_FILE_FORMAT {
                    return Err(Error::Incompatible(
                        "neighbour cache requires files/3".into(),
                    ));
                }
                let _lease = b.store.acquire(&scope.transport())?;
                context.check(&scope.transport(), crate::Action::Commit)?;
                clear(&b.store)
            })
            .await
    }
}
