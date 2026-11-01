//! Disposable trigram candidates; exact retained fields remain authoritative.
use super::*;
use super::{
    generation::{self, Generation, View},
    objects::Objects,
    trees::Tree,
};
use crate::ontology::{
    impact::text::{self, FieldRef},
    *,
};
use serde_json::json as value;
const GRAMS: &str = "text-grams/1";
const FIELDS: &str = "text-fields/1";
const POSTINGS: &str = "text-postings/1";
const QUOTA: u64 = 128 * 1024 * 1024;
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Binding {
    scope: ModelScope,
    selected: String,
    supporting: Option<String>,
    ontology: Option<ExactRef>,
    profile: Option<ExactRef>,
    fields: BTreeSet<TextField>,
    mode: LiteralMode,
    version: String,
}
impl Binding {
    fn new(v: &View, q: &SearchText) -> Self {
        Self {
            scope: v.g.scope.clone(),
            selected: v.selected.clone(),
            supporting: v.supporting.clone(),
            ontology: v.g.ontology.clone(),
            profile: v.g.profile.clone(),
            fields: q.text.fields.clone(),
            mode: q.text.mode,
            version: "knowledgehub-text-extractor/1".into(),
        }
    }
    fn key(&self) -> Result<String, Error> {
        codec::hash_value("knowledgehub-text-view/1", self)
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    sha256: String,
    source: Option<SourceSpan>,
    grams: BTreeSet<String>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    binding: Binding,
    grams: String,
    fields: String,
    checksum: String,
}
impl Manifest {
    fn digest(&self) -> Result<String, Error> {
        codec::hash_value(
            "knowledgehub-text-index/1",
            &(&self.binding, &self.grams, &self.fields),
        )
    }
}
fn missing(e: impl std::fmt::Display) -> Error {
    Error::IndexNotReady {
        reason: format!("text index: {e}"),
    }
}
fn cache(s: &FileStore) -> FileStore {
    FileStore {
        root: s.root.join("cache/text"),

        support: s.support.clone(),
        work: Counters {
            locations: s.work.text_locations.clone(),
            peak_allocated: s.work.text_peak.clone(),
            ..Counters::default()
        },
        loose_budget: None,
        packed_cache: true,
        #[cfg(test)]
        hook: std::sync::Mutex::new(s.hook.lock().unwrap().clone()),
    }
}
fn usage(p: &Path) -> Result<u64, Error> {
    let m = fs::symlink_metadata(p).map_err(|_| missing("cache metadata"))?;
    if m.is_file() {
        return Ok(m.blocks() * 512);
    }
    if !m.is_dir() {
        return Err(missing("nonordinary cache entry"));
    }
    let mut n = 0;
    for e in fs::read_dir(p).map_err(|_| missing("cache inventory"))? {
        n += usage(&e.map_err(|_| missing("cache entry"))?.path())?;
        if n > QUOTA {
            return Err(Error::Capacity);
        }
    }
    Ok(n)
}
fn writer(s: &FileStore, clear: bool) -> Result<FileStore, Error> {
    let mut c = cache(s);
    if clear
        || usage(&c.root).is_ok_and(|n| n > QUOTA * 3 / 4)
        || fs::read_dir(c.root.join("views")).is_ok_and(|es| es.count() >= 32)
    {
        *c.work.locations.lock().unwrap() = None;
        match fs::symlink_metadata(&c.root) {
            Ok(m) if m.is_dir() => fs::remove_dir_all(&c.root).map_err(|_| missing("evict"))?,
            Ok(_) => return Err(missing("cache is not a directory")),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => return Err(missing("inspect")),
        }
    }
    for p in [
        "",
        "objects",
        "objects/loose",
        "objects/packs",
        "staging",
        "views",
    ] {
        let p = c.root.join(p);
        match fs::create_dir(&p) {
            Ok(()) => sync_dir(p.parent().unwrap())?,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => is_dir(&p)?,
            Err(_) => return Err(missing("create cache")),
        }
    }
    c.loose_budget = Some(AtomicU64::new(QUOTA.saturating_sub(usage(&c.root)?)));
    c.work.allocation_limit.store(QUOTA, Ordering::Relaxed);
    Ok(c)
}
fn path(c: &FileStore, b: &Binding) -> Result<PathBuf, Error> {
    Ok(c.root.join("views").join(format!("{}.json", b.key()?)))
}
fn load(c: &FileStore, b: &Binding) -> Result<Manifest, Error> {
    let bytes = objects::bounded_file(&path(c, b)?, 65536).map_err(missing)?;
    let m: Manifest = parse(&bytes).map_err(missing)?;
    if m.binding != *b || m.checksum != m.digest()? || json(&m)? != bytes {
        return Err(missing("binding/checksum"));
    }
    Ok(m)
}
fn install(c: &FileStore, mut m: Manifest) -> Result<(), Error> {
    m.checksum = m.digest()?;
    let bytes = json(&m)?;
    reserve_file(c, bytes.len())?;
    let tmp = c.root.join("staging/manifest.json");
    if tmp.exists() {
        fs::remove_file(&tmp).map_err(|_| missing("discard staging"))?;
    }
    persist(&tmp, &bytes)?;
    sync_dir(&c.root.join("staging"))?;
    fs::rename(&tmp, path(c, &m.binding)?).map_err(|_| missing("publish manifest"))?;
    sync_dir(&c.root.join("views"))?;
    sync_dir(&c.root.join("staging"))
}
fn span_bytes(
    o: &Objects<'_>,
    v: &View,
    r: &ExactRef,
    start: u64,
    len: u64,
) -> Result<Vec<u8>, Error> {
    let b = generation::binding(o, v, r)?;
    Ok(content::range(o, &b, start, len)?.0)
}
fn documents(
    o: &Objects<'_>,
    v: &View,
    fs: &BTreeSet<TextField>,
    r: &Record,
) -> Result<Vec<(FieldRef, String, Option<SourceSpan>)>, Error> {
    text::values(r, fs)?
        .into_iter()
        .map(|(field, value)| {
            let (t, s) = text::fragment(
                &value,
                &mut |r| generation::record(o, v, r),
                &mut |r, a, n| span_bytes(o, v, r, a, n),
            )?;
            Ok((
                FieldRef {
                    record: r.reference.clone(),
                    field,
                },
                t,
                s,
            ))
        })
        .collect()
}
fn build(
    s: &FileStore,
    o: &Objects<'_>,
    v: &View,
    q: &SearchText,
    b: Binding,
    force: bool,
) -> Result<(FileStore, Manifest, ReadWork), Error> {
    if !q.budgets.preparation.allow_rebuild {
        return Err(missing("preparation disabled"));
    }
    let t = Tree { objects: o };
    let count = t.map_count(&v.selected, "selection/1")?;
    if count > q.budgets.preparation.owner_limit as u64 {
        return Err(missing("preparation owner budget"));
    }
    let c = writer(s, force)?;
    let index = Objects::new(&c);
    let tree = Tree { objects: &index };
    let mut docs = Vec::new();
    let mut grams: BTreeMap<String, BTreeSet<FieldRef>> = BTreeMap::new();
    let mut work = ReadWork::default();
    let mut posting_count = 0;
    for (_, r) in t.entries(&v.selected, "selection/1")? {
        let r: ExactRef = serde_json::from_value(r).map_err(|_| integrity("text selection"))?;
        let r = generation::record(o, v, &r)?;
        work.prepared_owners += 1;
        work.prepared_bytes += json(&r)?.len() as u64;
        for (key, text, source) in documents(o, v, &q.text.fields, &r)? {
            work.prepared_fields += 1;
            work.prepared_bytes += text.len() as u64;
            if work.prepared_fields > q.budgets.preparation.fact_limit as u64
                || work.prepared_bytes > q.budgets.preparation.byte_limit as u64
            {
                return Err(missing("preparation field/byte budget"));
            }
            let gs = text::grams(q.text.mode, &text)?;
            posting_count += gs.len();
            if posting_count > 1_000_000 {
                return Err(missing("text total posting preparation budget"));
            }
            for g in &gs {
                grams.entry(g.clone()).or_default().insert(key.clone());
            }
            docs.push((
                value!(key),
                value!(Document {
                    sha256: codec::digest(text.as_bytes()),
                    source,
                    grams: gs
                }),
            ));
        }
        if work.prepared_bytes > q.budgets.preparation.byte_limit as u64 {
            return Err(missing("preparation byte budget"));
        }
    }
    let fields = tree.map(FIELDS, docs)?;
    let mut entries = Vec::new();
    for (g, refs) in grams {
        let count = refs.len();
        let root = tree.sequence(
            POSTINGS,
            &refs.iter().map(|r| value!(r)).collect::<Vec<_>>(),
        )?;
        entries.push((value!(g), value!({"root":root,"count":count})));
    }
    let m = Manifest {
        binding: b,
        grams: tree.map(GRAMS, entries)?,
        fields,
        checksum: String::new(),
    };
    index.finish()?;
    install(&c, m.clone())?;
    drop(index);
    Ok((c, m, work))
}
pub(super) fn search(
    s: &FileStore,
    scope: &ModelScope,
    q: &SearchText,
    prepare: bool,
) -> Result<SearchResult, Error> {
    q.text.validate()?;
    q.budgets.validate()?;
    if prepare {
        match search(s, scope, q, false) {
            Ok(result) => return Ok(result),
            Err(Error::IndexNotReady { .. }) => {}
            Err(e) => return Err(e),
        }
    }
    let o = Objects::new(s);
    let (id, g) = generation::root(s, scope, &o)?;
    let v = generation::view(&o, &id, &g, &q.selection)?;
    let ontology = generation::record(
        &o,
        &v,
        v.g.ontology
            .as_ref()
            .ok_or_else(|| Error::InvalidInput("ontology required".into()))?,
    )?;
    text::validate_fields(&q.text.fields, &ontology, &mut |r| {
        generation::record(&o, &v, r)
    })?;
    let b = Binding::new(&v, q);
    let c = cache(s);
    let (c, m, mut work) = match if prepare {
        Err(missing("explicit repair"))
    } else {
        load(&c, &b)
    } {
        Ok(m) => (c, m, ReadWork::default()),
        Err(_e) if prepare => {
            let force = path(&c, &b)?.exists();
            build(s, &o, &v, q, b, force)?
        }
        Err(e) => return Err(e),
    };
    let index = Objects::new(&c);
    let tree = Tree { objects: &index };
    let mut streams = Vec::new();
    for gram in text::grams(q.text.mode, &q.text.phrase)? {
        let Some(v) = tree.get(&m.grams, GRAMS, &value!(gram)).map_err(missing)? else {
            let w = c.work.snapshot();
            work.index_objects_read = w.object_reads;
            work.index_bytes_read = w.encoded_bytes_read + w.index_bytes_read;
            work.index_metadata_bytes_read = w.index_bytes_read;
            return Ok(SearchResult {
                selection: q.selection.clone(),
                mode: q.text.mode,
                hits: vec![],
                reached: BTreeSet::new(),
                exhausted: true,
                work,
            });
        };
        let root = v["root"]
            .as_str()
            .ok_or_else(|| missing("posting root"))?
            .to_owned();
        let n = v["count"]
            .as_u64()
            .ok_or_else(|| missing("posting count"))?;
        if n > 100000 || tree.seq_count(&root, POSTINGS).map_err(missing)? != n {
            return Err(missing("posting count differs"));
        }
        streams.push((n, root));
    }
    streams.sort();
    let mut hits = Vec::new();
    let mut reached = BTreeSet::new();
    let mut starts = BTreeSet::new();
    let phrase = q.text.mode.bytes(&q.text.phrase);
    if let Some((n, root)) = streams.first() {
        'candidates: for i in 0..*n {
            if work.text_postings_examined >= q.budgets.examined as u64 {
                reached.insert(ReadLimit::Examined);
                break;
            }
            let k = tree
                .seq_range(root, POSTINGS, i, 1)
                .map_err(missing)?
                .pop()
                .ok_or_else(|| missing("posting item"))?;
            work.text_postings_examined += 1;
            let key: FieldRef = serde_json::from_value(k.clone()).map_err(missing)?;
            for (n, root) in streams.iter().skip(1) {
                let (mut lo, mut hi) = (0, *n);
                let mut found = false;
                while lo < hi {
                    if work.text_postings_examined >= q.budgets.examined as u64 {
                        reached.insert(ReadLimit::Examined);
                        break 'candidates;
                    }
                    let mid = (lo + hi) / 2;
                    let r = tree
                        .seq_range(root, POSTINGS, mid, 1)
                        .map_err(missing)?
                        .pop()
                        .ok_or_else(|| missing("intersection posting"))?;
                    work.text_postings_examined += 1;
                    let r: FieldRef = serde_json::from_value(r).map_err(missing)?;
                    match r.cmp(&key) {
                        std::cmp::Ordering::Less => lo = mid + 1,
                        std::cmp::Ordering::Greater => hi = mid,
                        std::cmp::Ordering::Equal => {
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    continue 'candidates;
                }
            }
            if work.candidates_verified >= q.budgets.candidates as u64 {
                reached.insert(ReadLimit::Candidates);
                break;
            }
            work.candidates_verified += 1;
            let d: Document = serde_json::from_value(
                tree.get(&m.fields, FIELDS, &k)
                    .map_err(missing)?
                    .ok_or_else(|| missing("field descriptor"))?,
            )
            .map_err(missing)?;
            let r = generation::record(&o, &v, &key.record)?;
            work.record_reads += 1;
            work.hydrated_bytes += json(&r)?.len() as u64;
            let (_, text, source) = documents(&o, &v, &q.text.fields, &r)?
                .into_iter()
                .find(|(k, _, _)| k == &key)
                .ok_or_else(|| missing("field no longer in projection"))?;
            work.hydrated_bytes += text.len() as u64;
            if d.sha256 != codec::digest(text.as_bytes()) || d.source != source {
                return Err(missing("exact field/source verification differs"));
            }
            let folded = q.text.mode.bytes(&text);
            if let Some(start) = folded.windows(phrase.len()).position(|b| b == phrase) {
                if !starts.contains(&key.record) && starts.len() == q.budgets.starts {
                    reached.insert(ReadLimit::Starts);
                    break;
                }
                starts.insert(key.record.clone());
                let end = start + phrase.len();
                let mut span = source;
                if let Some(s) = &mut span {
                    s.end = s.start + end as u64;
                    s.start += start as u64;
                }
                hits.push(TextHit {
                    record: key.record,
                    field: key.field,
                    start: start as u64,
                    end: end as u64,
                    source: span,
                    snippet: Some(text[start..end].into()),
                    omitted: false,
                });
                if hits.len() >= q.budgets.targets {
                    reached.insert(ReadLimit::Targets);
                    break;
                }
            }
        }
    }
    hits.sort_by(|a, b| (&a.record, &a.field, a.start).cmp(&(&b.record, &b.field, b.start)));
    let mut total = 0;
    hits.retain_mut(|h| {
        let n = json(h).map(|b| b.len()).unwrap_or(usize::MAX);
        if total + n > q.budgets.bytes / 2 {
            h.snippet = None;
            h.omitted = true;
            reached.insert(ReadLimit::Bytes);
        }
        let n = json(h).map(|b| b.len()).unwrap_or(usize::MAX);
        if total + n > q.budgets.bytes / 2 {
            return false;
        }
        total += n;
        true
    });
    let w = c.work.snapshot();
    work.index_objects_read = w.object_reads;
    work.index_bytes_read = w.encoded_bytes_read + w.index_bytes_read;
    work.index_metadata_bytes_read = w.index_bytes_read;
    Ok(SearchResult {
        selection: q.selection.clone(),
        mode: q.text.mode,
        hits,
        exhausted: reached.is_empty(),
        reached,
        work,
    })
}

/// Only manifests for a prepared current view are advanced. Old views remain reproducible.
pub(super) fn after_commit(
    s: &FileStore,
    old: &Generation,
    new: &Generation,
    before: &State,
    after: &State,
) -> Result<(), Error> {
    let c = cache(s);
    let Ok(entries) = fs::read_dir(c.root.join("views")) else {
        return Ok(());
    };
    let paths = entries
        .take(32)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| missing("incremental inventory"))?;
    for p in paths {
        let bytes = objects::bounded_file(&p.path(), 65536).map_err(missing)?;
        let m: Manifest = parse(&bytes).map_err(missing)?;
        if m.binding.selected != old.selected_root
            || m.binding.supporting.is_some()
            || m.binding.scope != old.scope
            || m.binding.ontology != new.ontology
            || m.binding.profile != new.profile
        {
            continue;
        }
        if m.checksum != m.digest()? {
            continue;
        }
        let changed = before
            .selected_records()
            .map(|r| r.reference.clone())
            .collect::<BTreeSet<_>>()
            .symmetric_difference(
                &after
                    .selected_records()
                    .map(|r| r.reference.clone())
                    .collect(),
            )
            .cloned()
            .collect::<Vec<_>>();
        let c = writer(s, false)?;
        let o = Objects::new(&c);
        let t = Tree { objects: &o };
        let mut field_changes = Vec::new();
        let mut gram_changes: BTreeMap<String, (BTreeSet<FieldRef>, BTreeSet<FieldRef>)> =
            BTreeMap::new();
        let mut budget = 0;
        for r in changed {
            for (state, add) in [(before, false), (after, true)] {
                let Some(record) = state.selected_records().find(|v| v.reference == r) else {
                    continue;
                };
                for (field, v) in text::values(record, &m.binding.fields)? {
                    let (text, source) = text::fragment(
                        &v,
                        &mut |r| state.resolve(r).ok_or(Error::NotFound),
                        &mut |r, a, n| {
                            let b = state.artifact(r).ok_or(Error::NotFound)?;
                            let end = a.checked_add(n).ok_or(Error::Capacity)?;
                            Ok(b.bytes
                                .get(a as usize..end as usize)
                                .ok_or(Error::Capacity)?
                                .to_vec())
                        },
                    )?;
                    budget += text.len();
                    if budget > 64 * 1024 * 1024 {
                        return Err(missing("incremental byte budget"));
                    }
                    let key = FieldRef {
                        record: r.clone(),
                        field,
                    };
                    let gs = text::grams(m.binding.mode, &text)?;
                    for g in &gs {
                        let (a, b) = gram_changes.entry(g.clone()).or_default();
                        if add {
                            b.insert(key.clone());
                        } else {
                            a.insert(key.clone());
                        }
                    }
                    field_changes.push((
                        value!(key),
                        if add {
                            Some(value!(Document {
                                sha256: codec::digest(text.as_bytes()),
                                source,
                                grams: gs
                            }))
                        } else {
                            None
                        },
                    ));
                }
            }
        }
        let fields = t.update_many(&m.fields, FIELDS, field_changes)?;
        let mut changes = Vec::new();
        for (g, (remove, add)) in gram_changes {
            let old = t.get(&m.grams, GRAMS, &value!(g))?;
            let root = old.as_ref().and_then(|v| v["root"].as_str());
            let mut refs = BTreeSet::new();
            if let Some(root) = root {
                for v in t.items(root, POSTINGS)? {
                    refs.insert(serde_json::from_value::<FieldRef>(v).map_err(missing)?);
                }
            }
            for r in remove {
                refs.remove(&r);
            }
            refs.extend(add);
            let next = if refs.is_empty() {
                None
            } else {
                let count = refs.len();
                let values = refs.iter().map(|r| value!(r)).collect::<Vec<_>>();
                Some(value!({"root":t.reconcile(root,POSTINGS,&values)?,"count":count}))
            };
            changes.push((value!(g), next));
        }
        let mut b = m.binding;
        b.selected = new.selected_root.clone();
        let grams = t.update_many(&m.grams, GRAMS, changes)?;
        o.finish()?;
        install(
            &c,
            Manifest {
                binding: b,
                grams,
                fields,
                checksum: String::new(),
            },
        )?;
    }
    Ok(())
}
