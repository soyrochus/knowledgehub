//! Sealed variable-size packs for disposable indexes. Accepted objects use their existing protocol.
use super::objects::{self, BlockHeader, Location, LocationCache, Object, Objects};
use super::*;
pub(super) type Stamp = (u64, u64, u128);
const BLOCK: usize = 1_048_576;
const ENTRIES: usize = 512;
const MAX_LOCATIONS: usize = 1_000_000;

#[derive(Default)]
pub(super) struct Pending {
    pub values: BTreeMap<String, Object>,
    pub locations: BTreeMap<String, Location>,
    bytes: usize,
    packs: BTreeSet<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn packed(b: &mut FileBackend, quota: u64) {
        let s = Arc::get_mut(&mut b.store).unwrap();
        s.packed_cache = true;
        s.loose_budget = Some(AtomicU64::new(quota));
        s.work.allocation_limit.store(quota, Ordering::Relaxed);
    }
    fn snapshot(s: &FileStore) -> BTreeMap<PathBuf, Vec<u8>> {
        fs::read_dir(s.root.join("objects/packs"))
            .unwrap()
            .map(|e| {
                let p = e.unwrap().path();
                let bytes = fs::read(&p).unwrap();
                (p, bytes)
            })
            .collect()
    }

    #[test]
    fn large_packed_index_is_bounded_selective_and_immutable() {
        let (_s, mut b) = super::super::object_backend("packed-large");
        let loose = Objects::new(&b.store).put("blob", b"old loose").unwrap();
        packed(&mut b, 256 * 1024 * 1024);
        let o = Objects::new(&b.store);
        let mut ids = Vec::new();
        for n in 0..66_000 {
            let bytes = json(&serde_json::json!({"schema":"fixture-node/1", "n":n})).unwrap();
            let id = o.put("map-node", &bytes).unwrap();
            assert_eq!(id, objects::oid("map-node", &bytes).unwrap());
            if n % 512 == 0 || n == 65_999 {
                assert_eq!(o.get(&id).unwrap().bytes, bytes);
                ids.push((id, bytes));
            }
        }
        o.finish().unwrap();
        assert_eq!(o.get(&loose).unwrap().bytes, b"old loose");
        assert!(b.store.work.peak_allocated.load(Ordering::Relaxed) < 256 * 1024 * 1024);
        let before = snapshot(&b.store);
        assert!(before.len() < 300); // packs and sidecars, not 66,000 inode allocations
        for (p, bytes) in &before {
            if p.extension().is_some_and(|e| e == "khpack") {
                assert_eq!(
                    codec::digest(bytes),
                    p.file_stem().unwrap().to_str().unwrap()
                );
                assert_eq!(u64::from_be_bytes(bytes[8..16].try_into().unwrap()), 1);
                let n = u32::from_be_bytes(bytes[16..20].try_into().unwrap()) as usize;
                let h: BlockHeader = parse(&bytes[20..20 + n]).unwrap();
                assert!(number(&h.logical_length).unwrap() <= BLOCK as u64);
                assert!(number(&h.object_count).unwrap() <= ENTRIES as u64);
            }
        }
        drop(o);
        *b.store.work.locations.lock().unwrap() = None;
        let cold = b.file_work().index_bytes_read;
        for (id, bytes) in &ids {
            assert_eq!(Objects::new(&b.store).get(id).unwrap().bytes, *bytes);
        }
        let warm = b.file_work().index_bytes_read;
        assert!(warm > cold);
        for (id, _) in &ids {
            Objects::new(&b.store).get(id).unwrap();
        }
        assert_eq!(b.file_work().index_bytes_read, warm);
        let next = Objects::new(&b.store);
        let big = vec![b'x'; BLOCK + 17];
        let id = next.put("blob", &big).unwrap();
        assert_eq!(next.get(&id).unwrap().bytes, big);
        next.finish().unwrap();
        assert_eq!(Objects::new(&b.store).get(&id).unwrap().bytes, big);
        for (path, bytes) in before {
            assert_eq!(fs::read(path).unwrap(), bytes);
        }
        // Independent readers notice atomic catalog replacement; eviction invalidates the epoch.
        let mut other =
            FileBackend::new(&b.store.root, FileOptions::native_local().unwrap()).unwrap();
        packed(&mut other, 256 * 1024 * 1024);
        Objects::new(&other.store).get(&id).unwrap();
        let update = Objects::new(&b.store);
        let added = update.put("blob", b"external addition").unwrap();
        update.finish().unwrap();
        assert_eq!(
            Objects::new(&other.store).get(&added).unwrap().bytes,
            b"external addition"
        );
        let missing = ids[0].0.clone();
        for e in fs::read_dir(b.store.root.join("objects/packs")).unwrap() {
            fs::remove_file(e.unwrap().path()).unwrap();
        }
        fs::remove_file(b.store.root.join("objects/catalog.json")).unwrap();
        assert!(Objects::new(&other.store).get(&missing).is_err());
    }

    #[test]
    fn publication_faults_quota_and_sidecar_reconstruction_are_safe() {
        for stage in [
            "cache-pack-staged",
            "cache-pack-installed",
            "cache-pack-indexed",
            "cache-pack-catalog-installed",
        ] {
            let (_s, mut b) = super::super::object_backend("packed-fault");
            packed(&mut b, 1_048_576);
            b.set_hook(move |at| {
                if at == stage {
                    Err(storage("injected"))
                } else {
                    Ok(())
                }
            });
            let o = Objects::new(&b.store);
            let id = o.put("blob", b"not a completed view").unwrap();
            assert!(o.finish().is_err());
            assert!(b.store.work.peak_allocated.load(Ordering::Relaxed) <= 1_048_576);
            b.set_hook(|_| Ok(()));
            *b.store.work.locations.lock().unwrap() = None;
            let retry = Objects::new(&b.store);
            retry.put("blob", b"not a completed view").unwrap();
            retry.finish().unwrap();
            assert_eq!(
                Objects::new(&b.store).get(&id).unwrap().bytes,
                b"not a completed view"
            );
            for e in fs::read_dir(b.store.root.join("objects/packs")).unwrap() {
                let p = e.unwrap().path();
                if p.extension().is_some_and(|e| e == "idx") {
                    fs::write(p, b"broken").unwrap();
                }
            }
            *b.store.work.locations.lock().unwrap() = None;
            assert_eq!(
                Objects::new(&b.store).get(&id).unwrap().bytes,
                b"not a completed view"
            );
        }
        // Two files fit, but the replacement catalog also needs allocated space.
        let (_s, mut b) = super::super::object_backend("packed-quota");
        packed(&mut b, 8192);
        let o = Objects::new(&b.store);
        o.put("blob", b"quota").unwrap();
        assert!(matches!(o.finish(), Err(Error::Capacity)));
        assert!(!b.store.root.join("objects/catalog.json").exists());
        assert_eq!(b.store.work.peak_allocated.load(Ordering::Relaxed), 8192);
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Catalog {
    schema: String,
    packs: Vec<String>,
}

fn stamp(path: &Path) -> Result<Option<Stamp>, Error> {
    let m = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err(integrity("cache catalog metadata")),
    };
    if !m.is_file() {
        return Err(integrity("cache catalog is not ordinary"));
    }
    let modified = m
        .modified()
        .map_err(|_| integrity("catalog time"))?
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| integrity("catalog epoch"))?
        .as_nanos();
    Ok(Some((m.ino(), m.len(), modified)))
}
fn pack_names(store: &FileStore, epoch: Option<Stamp>) -> Result<Vec<String>, Error> {
    if epoch.is_some() {
        let bytes = objects::bounded_file(
            &store.root.join("objects/catalog.json"),
            objects::MAX_OBJECT as u64,
        )?;
        store
            .work
            .index_bytes
            .fetch_add(bytes.len() as u64, Ordering::Relaxed);
        let c: Catalog = parse(&bytes)?;
        if c.schema != "knowledgehub-pack-catalog/1"
            || c.packs.len() > MAX_LOCATIONS
            || c.packs.iter().any(|s| !model::hash(s))
            || c.packs.windows(2).any(|p| p[0] >= p[1])
        {
            return Err(integrity("cache catalog schema/order"));
        }
        return Ok(c.packs);
    }
    let mut packs = Vec::new();
    for e in fs::read_dir(store.root.join("objects/packs"))
        .map_err(|_| integrity("cache pack inventory"))?
    {
        let name = e
            .map_err(|_| integrity("cache pack entry"))?
            .file_name()
            .to_string_lossy()
            .into_owned();
        if let Some(id) = name.strip_suffix(".khpack") {
            if !model::hash(id) {
                return Err(integrity("cache pack identity"));
            }
            packs.push(id.into());
            if packs.len() > MAX_LOCATIONS {
                return Err(Error::Capacity);
            }
        }
    }
    packs.sort();
    Ok(packs)
}
fn replace(store: &FileStore, path: &Path, bytes: &[u8]) -> Result<(), Error> {
    let old = match fs::symlink_metadata(path) {
        Ok(m) if m.is_file() => m.blocks() * 512,
        Ok(_) => return Err(integrity("nonordinary derived metadata")),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => 0,
        Err(_) => return Err(storage("inspect derived metadata")),
    };
    reserve_file(store, bytes.len())?;
    let temp = store
        .root
        .join("staging")
        .join(format!("metadata-{}", store.allocate_id()?.0));
    store.persist(&temp, bytes)?;
    fs::rename(&temp, path).map_err(|_| storage("install derived pack metadata"))?;
    store.sync_dir(path.parent().unwrap())?;
    store.sync_dir(&store.root.join("staging"))?;
    store.fence(path)?;
    if let Some(b) = &store.loose_budget {
        b.fetch_add(old, Ordering::Relaxed);
    }
    Ok(())
}

impl Objects<'_> {
    pub(super) fn discover_derived(&self) -> Result<(), Error> {
        let epoch = stamp(&self.store.root.join("objects/catalog.json"))?;
        if let Some(c) = self
            .store
            .work
            .locations
            .lock()
            .unwrap()
            .as_ref()
            .filter(|c| c.catalog_stamp == epoch)
        {
            *self.locations.borrow_mut() = Some(c.entries.clone());
            return Ok(());
        }
        let packs = pack_names(self.store, epoch)?;
        let mut entries = BTreeMap::new();
        for id in &packs {
            entries.extend(self.scan_pack(id)?);
            if entries.len() > MAX_LOCATIONS {
                return Err(Error::Capacity);
            }
        }
        let entries = Arc::new(entries);
        *self.store.work.locations.lock().unwrap() = Some(LocationCache {
            inventory: packs
                .into_iter()
                .map(|p| (format!("{p}.khpack"), 0, 0))
                .collect(),
            entries: entries.clone(),
            catalog_stamp: epoch,
        });
        *self.locations.borrow_mut() = Some(entries);
        Ok(())
    }
    pub(super) fn buffer_object(&self, id: String, object: Object) -> Result<(), Error> {
        if let Some(old) = self.pending.borrow().values.get(&id) {
            if old.kind != object.kind || old.bytes != object.bytes {
                return Err(integrity("pending object collision"));
            }
            return Ok(());
        }
        let n = 2 + object.kind.len() + 32 + 8 + object.bytes.len();
        let flush = {
            let p = self.pending.borrow();
            !p.values.is_empty() && (p.bytes + n > BLOCK || p.values.len() == ENTRIES)
        };
        if flush {
            self.seal()?;
        }
        {
            let mut p = self.pending.borrow_mut();
            p.bytes += n;
            p.values.insert(id, object);
        }
        if n > BLOCK {
            self.seal()?;
        }
        Ok(())
    }
    fn seal(&self) -> Result<(), Error> {
        let values = {
            let mut p = self.pending.borrow_mut();
            p.bytes = 0;
            std::mem::take(&mut p.values)
        };
        if values.is_empty() {
            return Ok(());
        }
        let mut block = Vec::new();
        let mut locations = BTreeMap::new();
        for (id, o) in &values {
            let at = block.len();
            block.extend_from_slice(&(o.kind.len() as u16).to_be_bytes());
            block.extend(o.kind.as_bytes());
            for pair in id.as_bytes().chunks_exact(2) {
                block.push(u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap());
            }
            block.extend_from_slice(&(o.bytes.len() as u64).to_be_bytes());
            block.extend(&o.bytes);
            locations.insert(
                id.clone(),
                Location {
                    pack: String::new(),
                    block_offset: "16".into(),
                    block_length: String::new(),
                    entry_offset: at.to_string(),
                    kind: o.kind.clone(),
                    length: o.bytes.len().to_string(),
                },
            );
        }
        let (codec, encoded) = objects::encode(&block)?;
        let header = json(&BlockHeader {
            schema: "knowledgehub-pack-block/1".into(),
            codec,
            logical_length: block.len().to_string(),
            stored_length: encoded.len().to_string(),
            object_count: values.len().to_string(),
            logical_sha256: codec::digest(&block),
        })?;
        let mut frame = Vec::from(objects::PACK_MAGIC.as_slice());
        frame.extend_from_slice(&1u64.to_be_bytes());
        frame.extend_from_slice(&(header.len() as u32).to_be_bytes());
        frame.extend(header);
        frame.extend(encoded);
        let id = codec::digest(&frame);
        let block_length = (frame.len() - 16).to_string();
        for l in locations.values_mut() {
            l.pack = id.clone();
            l.block_length = block_length.clone();
        }
        let index = json(&locations)?;
        // Reserve pack and sidecar before either allocation; the final link shares the staging inode.
        reserve_file(self.store, frame.len())?;
        reserve_file(self.store, index.len())?;
        let temporary = self
            .store
            .root
            .join("staging")
            .join(format!("pack-{}", self.store.allocate_id()?.0));
        self.store.persist(&temporary, &frame)?;
        self.store.point("cache-pack-staged")?;
        let path = self
            .store
            .root
            .join("objects/packs")
            .join(format!("{id}.khpack"));
        let duplicate = match fs::hard_link(&temporary, &path) {
            Ok(()) => false,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => true,
            Err(_) => return Err(storage("install sealed cache pack")),
        };
        self.store.sync_dir(path.parent().unwrap())?;
        self.store.point("cache-pack-installed")?;
        let actual = objects::bounded_file(&path, (objects::MAX_OBJECT + 4096 + 256) as u64)?;
        if codec::digest(&actual) != id {
            return Err(integrity("installed cache pack hash"));
        }
        let independent = Objects::new(self.store);
        let verified = independent.scan_pack_entries(&id)?;
        if verified.keys().ne(values.keys()) {
            return Err(integrity("sealed cache membership"));
        }
        let index_path = path.with_extension("idx");
        // The sidecar reservation above covers its temporary publication.
        let old = fs::symlink_metadata(&index_path)
            .ok()
            .map_or(0, |m| m.blocks() * 512);
        if index_path.is_symlink() {
            return Err(integrity("cache sidecar symlink"));
        }
        let temp_index = self
            .store
            .root
            .join("staging")
            .join(format!("index-{}", self.store.allocate_id()?.0));
        self.store.persist(&temp_index, &index)?;
        fs::rename(&temp_index, &index_path).map_err(|_| storage("install cache sidecar"))?;
        self.store.sync_dir(path.parent().unwrap())?;
        self.store.point("cache-pack-indexed")?;
        fs::remove_file(&temporary).map_err(|_| storage("remove sealed cache staging"))?;
        self.store.sync_dir(&self.store.root.join("staging"))?;
        self.store.fence(&path)?;
        if let Some(b) = &self.store.loose_budget {
            b.fetch_add(
                old + if duplicate {
                    (frame.len() as u64).div_ceil(4096) * 4096
                } else {
                    0
                },
                Ordering::Relaxed,
            );
        }
        let mut p = self.pending.borrow_mut();
        p.packs.insert(id);
        p.locations.extend(locations);
        if p.locations.len() > MAX_LOCATIONS {
            return Err(Error::Capacity);
        }
        self.store
            .work
            .writes
            .fetch_add(values.len() as u64, Ordering::Relaxed);
        self.store
            .work
            .write_bytes
            .fetch_add((frame.len() + index.len()) as u64, Ordering::Relaxed);
        self.store.work.structural_writes.fetch_add(
            values.values().map(|v| v.bytes.len() as u64).sum(),
            Ordering::Relaxed,
        );
        Ok(())
    }
    /// Close every pending block before a completed index manifest can become visible.
    pub(super) fn finish(&self) -> Result<(), Error> {
        if !self.store.packed_cache {
            return Ok(());
        }
        self.seal()?;
        if self.pending.borrow().packs.is_empty() {
            return Ok(());
        }
        self.discover_derived()?;
        let mut packs = BTreeSet::new();
        if let Some(c) = self.store.work.locations.lock().unwrap().as_ref() {
            packs.extend(
                c.inventory
                    .iter()
                    .filter_map(|(n, _, _)| n.strip_suffix(".khpack").map(str::to_owned)),
            );
        }
        packs.extend(self.pending.borrow().packs.iter().cloned());
        let catalog = json(&Catalog {
            schema: "knowledgehub-pack-catalog/1".into(),
            packs: packs.iter().cloned().collect(),
        })?;
        replace(
            self.store,
            &self.store.root.join("objects/catalog.json"),
            &catalog,
        )?;
        self.store.point("cache-pack-catalog-installed")?;
        let epoch = stamp(&self.store.root.join("objects/catalog.json"))?;
        // Release this writer's old Arc before merging once; healthy reads share the new immutable map.
        self.locations.borrow_mut().take();
        let mut shared = self.store.work.locations.lock().unwrap();
        let old = shared.take();
        let mut entries = old.map(|c| c.entries).unwrap_or_default();
        Arc::make_mut(&mut entries)
            .extend(std::mem::take(&mut self.pending.borrow_mut().locations));
        if entries.len() > MAX_LOCATIONS {
            return Err(Error::Capacity);
        }
        *shared = Some(LocationCache {
            inventory: packs
                .into_iter()
                .map(|p| (format!("{p}.khpack"), 0, 0))
                .collect(),
            entries: entries.clone(),
            catalog_stamp: epoch,
        });
        *self.locations.borrow_mut() = Some(entries);
        self.pending.borrow_mut().packs.clear();
        Ok(())
    }
}
