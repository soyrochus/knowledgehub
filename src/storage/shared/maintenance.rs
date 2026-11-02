//! Exclusive maintenance retains the complete accepted history, never just current selection.
use super::*;
use super::{
    generation,
    objects::{self, BlockHeader, Location, Objects},
    trees::Tree,
};
use crate::{Action, Context, ontology::ModelScope};
use serde_json::{Value, json as value};
use std::io::{Seek, SeekFrom};

impl FileBackend {
    /// Cumulative physical work for this backend and its clones. No filesystem scan.
    pub fn file_work(&self) -> FileWork {
        self.store.work.snapshot()
    }

    /// Verify all accepted generations, semantic/source digests, receipts and physical dependencies.
    pub async fn scrub(
        &self,
        context: Context,
        scope: ModelScope,
    ) -> Result<FileStorageReport, Error> {
        let b = self.clone();
        self.jobs
            .run(move || maintain(&b, &context, &scope, false))
            .await
    }
    /// Pack retained objects and remove preparation debris/redundant encodings under exclusive coordination.
    /// This creates no model generation and preserves all historical operation outcomes.
    pub async fn compact(
        &self,
        context: Context,
        scope: ModelScope,
    ) -> Result<FileStorageReport, Error> {
        let b = self.clone();
        self.jobs
            .run(move || maintain(&b, &context, &scope, true))
            .await
    }
}

fn disk(path: &Path) -> Result<(u64, u64), Error> {
    disk_seen(path, &mut BTreeSet::new())
}
fn disk_seen(path: &Path, seen: &mut BTreeSet<(u64, u64)>) -> Result<(u64, u64), Error> {
    let meta = fs::symlink_metadata(path).map_err(|_| integrity("storage measurement"))?;
    if meta.is_file() {
        if !seen.insert((meta.dev(), meta.ino())) {
            return Ok((0, 0));
        }
        return Ok((meta.len(), meta.blocks() * 512));
    }
    if !meta.is_dir() {
        return Err(integrity("nonordinary storage entry"));
    }
    let mut total = (0, 0);
    for entry in fs::read_dir(path).map_err(|_| integrity("storage entries"))? {
        let (bytes, allocated) =
            disk_seen(&entry.map_err(|_| integrity("storage entry"))?.path(), seen)?;
        total.0 += bytes;
        total.1 += allocated;
    }
    Ok(total)
}

fn ids_in(value: &Value, field: &str, out: &mut Vec<String>) -> Result<(), Error> {
    let id = value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| integrity("physical dependency field"))?;
    if !model::hash(id) {
        return Err(integrity("physical dependency address"));
    }
    out.push(id.into());
    Ok(())
}
fn dependencies(object: &objects::Object) -> Result<Vec<String>, Error> {
    if object.kind == "blob" {
        return Ok(vec![]);
    }
    let v: Value = parse(&object.bytes)?;
    let mut out = Vec::new();
    let fields: &[&str] = match object.kind.as_str() {
        "generation" => &[
            "selected_root",
            "revisions_root",
            "retained_refs_root",
            "artifacts_root",
            "gaps_root",
            "operations_root",
            "ancestors_root",
        ],
        "record" => &["data_oid"],
        "baseline-data" => &[
            "selected_root",
            "supporting_root",
            "artifacts_root",
            "gaps_root",
        ],
        "receipt" => &["affected_root"],
        "blob-manifest" => &["chunks_root"],
        "anchor-data" | "record-data" | "gap" | "map-node" | "sequence-node" => &[],
        _ => return Err(integrity("unknown dependency grammar")),
    };
    for f in fields {
        ids_in(&v, f, &mut out)?;
    }
    if object.kind == "generation" {
        for f in ["parent", "receipt"] {
            if !v[f].is_null() {
                ids_in(&v, f, &mut out)?;
            }
        }
    }
    if object.kind == "baseline-data" {
        for f in ["selected_order", "supporting_order"] {
            if !v[f].is_null() {
                ids_in(&v, f, &mut out)?;
            }
        }
    }
    if object.kind == "anchor-data" && !v["quotation"].is_null() {
        match v["quotation"]["kind"].as_str() {
            Some("span" | "blob") => ids_in(&v["quotation"], "content_oid", &mut out)?,
            Some("literal") => {}
            _ => return Err(integrity("quote dependency kind")),
        }
    }
    if object.kind == "map-node" {
        match v["variant"].as_str() {
            Some("branch") => {
                for child in v["children"]
                    .as_object()
                    .ok_or_else(|| integrity("map children"))?
                    .values()
                {
                    let id = child.as_str().ok_or_else(|| integrity("map child OID"))?;
                    if !model::hash(id) {
                        return Err(integrity("map child address"));
                    }
                    out.push(id.into());
                }
            }
            Some("leaf") => {
                for e in v["entries"]
                    .as_array()
                    .ok_or_else(|| integrity("map entries"))?
                {
                    match v["keyspace"].as_str() {
                        Some("revisions/1" | "operations/1") => {
                            let id = e["value"]
                                .as_str()
                                .ok_or_else(|| integrity("map value OID"))?;
                            if !model::hash(id) {
                                return Err(integrity("map value address"));
                            }
                            out.push(id.into());
                        }
                        Some("artifacts/1") => ids_in(&e["value"], "content_oid", &mut out)?,
                        Some("ancestors/1") => ids_in(&e["value"], "generation", &mut out)?,
                        Some("selection/1" | "exact-reference-set/1") => {}
                        _ => return Err(integrity("map dependency keyspace")),
                    }
                }
            }
            _ => return Err(integrity("map dependency variant")),
        }
    }
    if object.kind == "sequence-node" {
        match v["variant"].as_str() {
            Some("branch") => {
                for child in v["children"]
                    .as_array()
                    .ok_or_else(|| integrity("sequence children"))?
                {
                    ids_in(child, "oid", &mut out)?;
                }
            }
            Some("leaf") => {
                for item in v["items"]
                    .as_array()
                    .ok_or_else(|| integrity("sequence items"))?
                {
                    match v["item_type"].as_str() {
                        Some("chunks/1") => ids_in(item, "oid", &mut out)?,
                        Some("gap-oids/1") => {
                            let id = item.as_str().ok_or_else(|| integrity("gap dependency"))?;
                            if !model::hash(id) {
                                return Err(integrity("gap address"));
                            }
                            out.push(id.into());
                        }
                        Some("exact-refs/1") => {}
                        _ => return Err(integrity("sequence dependency item type")),
                    }
                }
            }
            _ => return Err(integrity("sequence dependency variant")),
        }
    }
    Ok(out)
}

type RetainedClosure = (BTreeSet<String>, BTreeMap<String, u64>, u64);
fn closure(objects: &Objects<'_>, head: &str) -> Result<RetainedClosure, Error> {
    let mut seen = BTreeSet::new();
    let mut todo = vec![head.to_owned()];
    let mut bytes: BTreeMap<String, u64> = BTreeMap::new();
    let mut baselines = 0;
    while let Some(id) = todo.pop() {
        if !seen.insert(id.clone()) {
            continue;
        }
        if seen.len() > 1_000_000 || todo.len() > 1_000_000 {
            return Err(Error::Capacity);
        }
        let o = objects.get(&id)?;
        *bytes.entry(o.kind.clone()).or_default() += o.bytes.len() as u64;
        if o.kind == "record" {
            let r: Value = parse(&o.bytes)?;
            if objects
                .get(
                    r["data_oid"]
                        .as_str()
                        .ok_or_else(|| integrity("record data dependency"))?,
                )?
                .kind
                == "baseline-data"
            {
                baselines += 1;
            }
        }
        todo.extend(dependencies(&o)?);
    }
    Ok((seen, bytes, baselines))
}

fn verify_history(
    objects: &Objects<'_>,
    head_id: &str,
    head: &generation::Generation,
) -> Result<u64, Error> {
    let tree = Tree { objects };
    let mut id = head_id.to_owned();
    let mut g = head.clone();
    let mut child: Option<(generation::Generation, Stored)> = None;
    let mut seen = BTreeSet::new();
    let mut count = 0;
    loop {
        if !seen.insert(id.clone()) || seen.len() > 1_000_000 {
            return Err(integrity("retained history cycle"));
        }
        let current = generation::materialise(objects, &g, &id)?;
        if tree.entries(&g.ancestors_root, "ancestors/1")?.len() as u64 != number(&g.sequence)?
            || tree.entries(&g.operations_root, "operations/1")?.len() as u64
                != number(&g.sequence)?
        {
            return Err(integrity("history root cardinality"));
        }
        if let Some((cg, cs)) = &child {
            let receipt = cs
                .receipt
                .as_ref()
                .ok_or_else(|| integrity("history receipt missing"))?;
            if current.snapshot.sequence + 1 != cs.snapshot.sequence
                || receipt.previous_head != current.snapshot.id
                || !model::verify_affected(&current.snapshot, &cs.snapshot, receipt)
            {
                return Err(integrity("retained history transition"));
            }
            let mut ancestors = tree
                .entries(&g.ancestors_root, "ancestors/1")?
                .into_iter()
                .map(|(k, v)| Ok((json(&k)?, v)))
                .collect::<Result<BTreeMap<_, _>, Error>>()?;
            if ancestors
                .insert(
                    json(&value!(g.snapshot_id.0))?,
                    value!({"generation":id,"sequence":g.sequence}),
                )
                .is_some()
            {
                return Err(integrity("repeated accepted snapshot"));
            }
            let actual = tree
                .entries(&cg.ancestors_root, "ancestors/1")?
                .into_iter()
                .map(|(k, v)| Ok((json(&k)?, v)))
                .collect::<Result<BTreeMap<_, _>, Error>>()?;
            if actual != ancestors {
                return Err(integrity("ancestor lifetime map transition"));
            }
            let mut operations = tree
                .entries(&g.operations_root, "operations/1")?
                .into_iter()
                .map(|(k, v)| Ok((json(&k)?, v)))
                .collect::<Result<BTreeMap<_, _>, Error>>()?;
            if operations
                .insert(json(&value!(receipt.operation_id))?, value!(cg.receipt))
                .is_some()
            {
                return Err(integrity("repeated operation"));
            }
            let actual = tree
                .entries(&cg.operations_root, "operations/1")?
                .into_iter()
                .map(|(k, v)| Ok((json(&k)?, v)))
                .collect::<Result<BTreeMap<_, _>, Error>>()?;
            if actual != operations {
                return Err(integrity("operation lifetime map transition"));
            }
        }
        count += 1;
        let Some(parent) = g.parent.clone() else {
            break;
        };
        let previous = generation::load_generation(objects, &parent, &head.scope)?;
        child = Some((g, current));
        g = previous;
        id = parent;
    }
    Ok(count)
}

fn pack_ids(store: &FileStore) -> Result<Vec<String>, Error> {
    let mut result = Vec::new();
    for e in
        fs::read_dir(store.root.join("objects/packs")).map_err(|_| integrity("pack inventory"))?
    {
        let name = e
            .map_err(|_| integrity("pack name"))?
            .file_name()
            .to_string_lossy()
            .into_owned();
        if let Some(id) = name.strip_suffix(".khpack") {
            if !model::hash(id) {
                return Err(integrity("pack filename"));
            }
            result.push(id.into());
        }
    }
    result.sort();
    Ok(result)
}
fn file_hash(path: &Path) -> Result<String, Error> {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    let mut f = open_read(path)?;
    let mut buf = [0; 65536];
    loop {
        let n = f.read(&mut buf).map_err(|_| integrity("pack hash read"))?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(format!("{:x}", h.finalize()))
}
fn verify_packs(objects: &Objects<'_>) -> Result<Vec<String>, Error> {
    let ids = pack_ids(objects.store)?;
    for id in &ids {
        let p = objects
            .store
            .root
            .join("objects/packs")
            .join(format!("{id}.khpack"));
        if file_hash(&p)? != *id {
            return Err(integrity("complete pack hash differs"));
        }
        objects.scan_pack_entries(id)?;
    }
    Ok(ids)
}

fn derived(store: &FileStore, path: &Path, bytes: &[u8]) -> Result<(), Error> {
    let temp = store
        .root
        .join("staging")
        .join(format!("index-{}", store.allocate_id()?.0));
    store.persist(&temp, bytes)?;
    fs::rename(&temp, path).map_err(|_| storage("replace derived location data"))?;
    store.sync_dir(path.parent().unwrap())?;
    store.sync_dir(&store.root.join("staging"))?;
    store.fence(path)
}
fn write_pack(
    objects: &Objects<'_>,
    ids: &BTreeSet<String>,
) -> Result<(String, (u64, u64)), Error> {
    let store = objects.store;
    let temp = store
        .root
        .join("staging")
        .join(format!("pack-{}", store.allocate_id()?.0));
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .read(true)
        .open(&temp)
        .map_err(|_| storage("create pack staging"))?;
    f.write_all(objects::PACK_MAGIC)
        .and_then(|()| f.write_all(&0u64.to_be_bytes()))
        .map_err(|_| storage("pack prefix"))?;
    let mut block = Vec::new();
    let mut entries: Vec<(String, String, u64, u64)> = Vec::new();
    let mut locations = BTreeMap::new();
    let mut blocks = 0u64;
    fn flush(
        f: &mut File,
        block: &mut Vec<u8>,
        entries: &mut Vec<(String, String, u64, u64)>,
        locations: &mut BTreeMap<String, Location>,
        blocks: &mut u64,
    ) -> Result<(), Error> {
        if entries.is_empty() {
            return Ok(());
        }
        let offset = f.stream_position().map_err(|_| storage("pack offset"))?;
        let (codec, encoded) = objects::encode(block)?;
        let header = json(&BlockHeader {
            schema: "knowledgehub-pack-block/1".into(),
            codec,
            logical_length: block.len().to_string(),
            stored_length: encoded.len().to_string(),
            object_count: entries.len().to_string(),
            logical_sha256: codec::digest(block),
        })?;
        let len = 4 + header.len() as u64 + encoded.len() as u64;
        f.write_all(&(header.len() as u32).to_be_bytes())
            .and_then(|()| f.write_all(&header))
            .and_then(|()| f.write_all(&encoded))
            .map_err(|_| storage("pack block write"))?;
        for (id, kind, n, at) in entries.drain(..) {
            locations.insert(
                id,
                Location {
                    pack: String::new(),
                    block_offset: offset.to_string(),
                    block_length: len.to_string(),
                    entry_offset: at.to_string(),
                    kind,
                    length: n.to_string(),
                },
            );
        }
        block.clear();
        *blocks += 1;
        Ok(())
    }
    for id in ids {
        let o = objects.get(id)?;
        let n = 2 + o.kind.len() + 32 + 8 + o.bytes.len();
        if !entries.is_empty() && (block.len() + n > 1_048_576 || entries.len() == 512) {
            flush(
                &mut f,
                &mut block,
                &mut entries,
                &mut locations,
                &mut blocks,
            )?;
        }
        entries.push((
            id.clone(),
            o.kind.clone(),
            o.bytes.len() as u64,
            block.len() as u64,
        ));
        block.extend_from_slice(&(o.kind.len() as u16).to_be_bytes());
        block.extend(o.kind.as_bytes());
        for pair in id.as_bytes().chunks_exact(2) {
            let hex = std::str::from_utf8(pair).unwrap();
            block.push(u8::from_str_radix(hex, 16).unwrap());
        }
        block.extend_from_slice(&(o.bytes.len() as u64).to_be_bytes());
        block.extend(o.bytes);
    }
    flush(
        &mut f,
        &mut block,
        &mut entries,
        &mut locations,
        &mut blocks,
    )?;
    f.seek(SeekFrom::Start(8))
        .and_then(|_| f.write_all(&blocks.to_be_bytes()))
        .map_err(|_| storage("finish immutable pack"))?;
    store.sync_file(&f, "finish immutable pack")?;
    store.point("pack-file")?;
    let id = file_hash(&temp)?;
    let path = store
        .root
        .join("objects/packs")
        .join(format!("{id}.khpack"));
    match fs::hard_link(&temp, &path) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            if file_hash(&path)? != id {
                return Err(integrity("existing pack differs"));
            }
        }
        Err(_) => return Err(storage("install immutable pack")),
    }
    store.sync_dir(&store.root.join("objects/packs"))?;
    store.point("pack-installed")?;
    let verified = objects.scan_pack_entries(&id)?;
    if verified.keys().ne(ids.iter()) {
        return Err(integrity("pack retained membership differs"));
    }
    for loc in locations.values_mut() {
        loc.pack = id.clone();
    }
    derived(store, &path.with_extension("idx"), &json(&locations)?)?;
    store.point("pack-index")?;
    derived(
        store,
        &store.root.join("objects/catalog.json"),
        &json(&value!({"schema":"knowledgehub-pack-catalog/1","packs":[id]}))?,
    )?;
    store.point("pack-catalog")?;
    let peak = disk(&store.root)?;
    fs::remove_file(&temp).map_err(|_| storage("remove pack staging"))?;
    store.sync_dir(&store.root.join("staging"))?;
    // Complete and persist pack/index/catalogue closure before admitting deletion.
    store.fence(&path)?;
    *store.work.locations.lock().unwrap() = None;
    Ok((id, peak))
}

fn remove(store: &FileStore, path: &Path, removed: &mut u64) -> Result<(), Error> {
    store.point("cleanup-before-remove")?;
    fs::remove_file(path).map_err(|_| storage("cleanup file"))?;
    store.sync_dir(path.parent().unwrap())?;
    *removed += 1;
    store.point("cleanup-after-remove")
}
fn maintain(
    backend: &FileBackend,
    context: &Context,
    scope: &ModelScope,
    compact: bool,
) -> Result<FileStorageReport, Error> {
    context.check(
        scope,
        if compact {
            Action::Commit
        } else {
            Action::Read
        },
    )?;
    // Even scrub is exclusive: a stable finite inventory also covers debris and complete pack hashes.
    let _lease = backend.store.acquire(scope)?;
    context.check(
        scope,
        if compact {
            Action::Commit
        } else {
            Action::Read
        },
    )?;
    let store = &backend.store;
    let objects = Objects::new(store);
    let (head, g) = generation::root(store, scope, &objects)?;
    let packs = verify_packs(&objects)?;
    let (ids, categories, baselines) = closure(&objects, &head)?;
    let logical = categories.values().sum();
    let generations = verify_history(&objects, &head, &g)?;
    let before = disk(&store.root)?;
    let mut removed = 0;
    let mut peak = before.0;
    let mut peak_allocated = before.1;
    if compact {
        let (installed, highwater) = write_pack(&objects, &ids)?;
        peak = peak.max(highwater.0);
        peak_allocated = peak_allocated.max(highwater.1);
        // Start from a fresh reader so a populated logical-object cache cannot hide bad replacement bytes.
        let replacement = Objects::new(store);
        replacement.scan_pack_entries(&installed)?;
        store.point("cleanup-ready")?;
        for shard in
            fs::read_dir(store.root.join("objects/loose")).map_err(|_| integrity("loose shards"))?
        {
            let shard = shard.map_err(|_| integrity("loose shard"))?;
            is_dir(&shard.path())?;
            for entry in fs::read_dir(shard.path()).map_err(|_| integrity("loose objects"))? {
                let p = entry.map_err(|_| integrity("loose entry"))?.path();
                open_read(&p)?;
                remove(store, &p, &mut removed)?;
            }
        }
        for old in &packs {
            if *old != installed {
                let p = store
                    .root
                    .join("objects/packs")
                    .join(format!("{old}.khpack"));
                remove(store, &p, &mut removed)?;
                if p.with_extension("idx")
                    .try_exists()
                    .map_err(|_| integrity("old index"))?
                {
                    remove(store, &p.with_extension("idx"), &mut removed)?;
                }
            }
        }
        for dir in ["staging", "refs"] {
            for e in
                fs::read_dir(store.root.join(dir)).map_err(|_| integrity("preparation debris"))?
            {
                let e = e.map_err(|_| integrity("debris entry"))?;
                if dir == "staging" || e.file_name().to_string_lossy().starts_with(".pending-") {
                    open_read(&e.path())?;
                    remove(store, &e.path(), &mut removed)?;
                }
            }
        }
        store.fence(
            &store
                .root
                .join("objects/packs")
                .join(format!("{installed}.khpack")),
        )?;
        *store.work.locations.lock().unwrap() = None;
    }
    let (encoded, allocated) = disk(&store.root)?;
    let mut files = 0;
    let mut derived = 0;
    fn inventory(
        path: &Path,
        root: &Path,
        files: &mut u64,
        derived: &mut u64,
    ) -> Result<(), Error> {
        for e in fs::read_dir(path).map_err(|_| integrity("diagnostic inventory"))? {
            let p = e.map_err(|_| integrity("diagnostic entry"))?.path();
            let m = fs::symlink_metadata(&p).map_err(|_| integrity("diagnostic metadata"))?;
            if m.is_dir() {
                inventory(&p, root, files, derived)?;
            } else {
                *files += 1;
                let relative = p
                    .strip_prefix(root)
                    .map_err(|_| integrity("diagnostic root"))?;
                if p.extension().is_some_and(|e| e == "idx")
                    || relative == Path::new("objects/catalog.json")
                    || relative.starts_with("cache")
                    || relative.starts_with("views")
                {
                    *derived += m.len();
                }
            }
        }
        Ok(())
    }
    inventory(&store.root, &store.root, &mut files, &mut derived)?;
    let mut blocks = 0;
    for id in pack_ids(store)? {
        let mut f = open_read(
            &store
                .root
                .join("objects/packs")
                .join(format!("{id}.khpack")),
        )?;
        let mut prefix = [0; 16];
        f.read_exact(&mut prefix)
            .map_err(|_| integrity("diagnostic pack header"))?;
        blocks += u64::from_be_bytes(prefix[8..].try_into().unwrap());
    }
    let bytes = |kind: &str| categories.get(kind).copied().unwrap_or(0);
    Ok(FileStorageReport {
        logical_object_bytes: logical,
        encoded_file_bytes: encoded,
        allocated_file_bytes: allocated,
        objects: ids.len() as u64,
        packs: pack_ids(store)?.len() as u64,
        generations,
        removed_files: removed,
        peak_compaction_bytes: peak,
        peak_compaction_allocated_bytes: peak_allocated,
        derived_file_bytes: derived,
        artifact_logical_bytes: bytes("blob"),
        record_data_bytes: bytes("record-data"),
        baseline_data_bytes: bytes("baseline-data"),
        anchor_data_bytes: bytes("anchor-data"),
        history_tree_bytes: logical
            - bytes("blob")
            - bytes("record-data")
            - bytes("baseline-data")
            - bytes("anchor-data"),
        files,
        pack_blocks: blocks,
        baselines,
        work: store.work.snapshot(),
    })
}
