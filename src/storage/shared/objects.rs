use super::*;
use serde_json::Value;
use std::{
    cell::RefCell,
    io::{Seek, SeekFrom},
};

pub(super) const MAX_OBJECT: usize = 64 * 1024 * 1024;
const MAX_HEADER: usize = 4096;
const MAGIC: &[u8; 5] = b"KHO3\n";
pub(super) const PACK_MAGIC: &[u8; 8] = b"KHPACK3\n";
const KINDS: &[&str] = &[
    "blob",
    "blob-manifest",
    "record",
    "record-data",
    "baseline-data",
    "anchor-data",
    "gap",
    "map-node",
    "sequence-node",
    "receipt",
    "generation",
];

#[derive(Clone)]
pub(super) struct Object {
    pub kind: String,
    pub bytes: Vec<u8>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Header {
    schema: String,
    kind: String,
    oid: String,
    codec: String,
    logical_length: String,
    stored_length: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Location {
    pub pack: String,
    pub block_offset: String,
    pub block_length: String,
    pub entry_offset: String,
    pub kind: String,
    pub length: String,
}

type CachedBlock = (String, u64, Arc<Vec<u8>>, u64);
pub(super) struct Objects<'a> {
    pub store: &'a FileStore,
    cache: RefCell<BTreeMap<String, Object>>,
    pub(super) locations: RefCell<Option<Arc<BTreeMap<String, Location>>>>,
    pub(super) pending: RefCell<super::packed_cache::Pending>,
    block_cache: RefCell<Option<CachedBlock>>,
}
pub(super) struct LocationCache {
    pub(super) inventory: Vec<(String, u64, u128)>,
    pub(super) entries: Arc<BTreeMap<String, Location>>,
    pub(super) catalog_stamp: Option<super::packed_cache::Stamp>,
}

pub(super) fn oid(kind: &str, bytes: &[u8]) -> Result<String, Error> {
    if !KINDS.contains(&kind) {
        return Err(integrity("unknown object kind"));
    }
    if bytes.len() > MAX_OBJECT {
        return Err(Error::Capacity);
    }
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(format!("knowledgehub-object/1\n{kind}\n{}\n", bytes.len()).as_bytes());
    h.update(bytes);
    Ok(format!("{:x}", h.finalize()))
}

fn payload(kind: &str, bytes: &[u8]) -> Result<(), Error> {
    if kind != "blob" {
        let v: Value = parse(bytes)?;
        if !v.is_object() || !v.get("schema").is_some_and(Value::is_string) || json(&v)? != bytes {
            return Err(integrity("object must contain canonical versioned JSON"));
        }
    }
    Ok(())
}

pub(super) fn encode(bytes: &[u8]) -> Result<(String, Vec<u8>), Error> {
    if bytes.len() > MAX_OBJECT + 256 {
        return Err(Error::Capacity);
    }
    let compressed = zstd::bulk::compress(bytes, 3).map_err(|_| storage("compress object"))?;
    Ok(if compressed.len().saturating_add(32) <= bytes.len() {
        ("zstd".into(), compressed)
    } else {
        ("raw".into(), bytes.to_vec())
    })
}

pub(super) fn decode(codec: &str, bytes: &[u8], length: u64) -> Result<Vec<u8>, Error> {
    let length = usize::try_from(length).map_err(|_| Error::Capacity)?;
    if length > MAX_OBJECT + 256 || bytes.len() > MAX_OBJECT + 256 {
        return Err(Error::Capacity);
    }
    let decoded = match codec {
        "raw" => bytes.to_vec(),
        "zstd" => {
            if !bytes.starts_with(&[0x28, 0xb5, 0x2f, 0xfd])
                || zstd::zstd_safe::find_frame_compressed_size(bytes).ok() != Some(bytes.len())
                || zstd::zstd_safe::get_dict_id_from_frame(bytes).is_some()
            {
                return Err(integrity(
                    "unsupported, concatenated or dictionary zstd frame",
                ));
            }
            let mut decoder =
                zstd::bulk::Decompressor::new().map_err(|_| storage("zstd decoder"))?;
            decoder
                .set_parameter(zstd::zstd_safe::DParameter::WindowLogMax(26))
                .map_err(|_| integrity("zstd window bound"))?;
            decoder
                .decompress(bytes, length)
                .map_err(|_| integrity("invalid compressed payload"))?
        }
        _ => return Err(Error::Incompatible("unsupported object encoding".into())),
    };
    if decoded.len() != length {
        return Err(integrity("decoded length differs"));
    }
    Ok(decoded)
}

pub(super) fn bounded_file(path: &Path, max: u64) -> Result<Vec<u8>, Error> {
    let f = open_read(path)?;
    if f.metadata().map_err(|_| integrity("file length"))?.len() > max {
        return Err(Error::Capacity);
    }
    let mut out = Vec::new();
    f.take(max + 1)
        .read_to_end(&mut out)
        .map_err(|_| integrity("object read"))?;
    if out.len() as u64 > max {
        return Err(Error::Capacity);
    }
    Ok(out)
}

impl<'a> Objects<'a> {
    fn cache(&self, id: String, object: Object) {
        const LIMIT: usize = 32 * 1024 * 1024;
        if object.bytes.len() > LIMIT {
            return;
        }
        let mut cache = self.cache.borrow_mut();
        if cache.values().map(|o| o.bytes.len() + 128).sum::<usize>() + object.bytes.len() > LIMIT {
            cache.clear();
        }
        cache.insert(id, object);
    }
    pub fn new(store: &'a FileStore) -> Self {
        Self {
            store,
            cache: RefCell::new(BTreeMap::new()),
            locations: RefCell::new(None),
            pending: RefCell::new(super::packed_cache::Pending::default()),
            block_cache: RefCell::new(None),
        }
    }
    pub fn path(&self, id: &str) -> Result<PathBuf, Error> {
        if !model::hash(id) {
            return Err(integrity("invalid object address"));
        }
        Ok(self
            .store
            .root
            .join("objects/loose")
            .join(&id[..2])
            .join(format!("{}.obj", &id[2..])))
    }
    pub fn put_json<T: Serialize>(&self, kind: &str, value: &T) -> Result<String, Error> {
        self.put(kind, &json(value)?)
    }
    pub fn put(&self, kind: &str, bytes: &[u8]) -> Result<String, Error> {
        let id = oid(kind, bytes)?;
        payload(kind, bytes)?;
        if let Some(previous) = self.cache.borrow().get(&id) {
            if previous.kind != kind || previous.bytes != bytes {
                return Err(integrity("object address collision"));
            }
            return Ok(id);
        }
        if self
            .path(&id)?
            .try_exists()
            .map_err(|_| storage("inspect object"))?
            || self.has_packed(&id)?
        {
            let previous = self.get(&id)?;
            if previous.kind != kind || previous.bytes != bytes {
                return Err(integrity("immutable object collision"));
            }
            return Ok(id);
        }
        if self.store.packed_cache {
            self.buffer_object(
                id.clone(),
                Object {
                    kind: kind.into(),
                    bytes: bytes.to_vec(),
                },
            )?;
            return Ok(id);
        }
        let path = self.path(&id)?;
        let parent = path.parent().unwrap();
        match fs::create_dir(parent) {
            Ok(()) => {
                sync_dir(&self.store.root.join("objects/loose"))?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => is_dir(parent)?,
            Err(_) => return Err(storage("create object shard")),
        }
        let (codec, encoded) = encode(bytes)?;
        let header = json(&Header {
            schema: "knowledgehub-loose/1".into(),
            kind: kind.into(),
            oid: id.clone(),
            codec,
            logical_length: bytes.len().to_string(),
            stored_length: encoded.len().to_string(),
        })?;
        let mut frame = Vec::from(MAGIC.as_slice());
        frame.extend_from_slice(&(header.len() as u32).to_be_bytes());
        frame.extend(header);
        frame.extend(encoded);
        super::reserve_file(self.store, frame.len())?;
        let temporary = self
            .store
            .root
            .join("staging")
            .join(format!("object-{}", self.store.allocate_id()?.0));
        persist(&temporary, &frame)?;
        self.store.point("content-file")?;
        match fs::hard_link(&temporary, &path) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                let existing = self.read_loose(&id)?;
                if existing.kind != kind || existing.bytes != bytes {
                    return Err(integrity("existing object differs"));
                }
            }
            Err(_) => return Err(storage("install immutable object")),
        }
        sync_dir(parent)?;
        fs::remove_file(&temporary).map_err(|_| storage("remove installed staging object"))?;
        sync_dir(&self.store.root.join("staging"))?;
        self.store.work.writes.fetch_add(1, Ordering::Relaxed);
        self.store
            .work
            .write_bytes
            .fetch_add(frame.len() as u64, Ordering::Relaxed);
        let counter = if matches!(kind, "blob" | "record-data") {
            &self.store.work.payload_writes
        } else {
            &self.store.work.structural_writes
        };
        counter.fetch_add(bytes.len() as u64, Ordering::Relaxed);
        self.cache(
            id.clone(),
            Object {
                kind: kind.into(),
                bytes: bytes.to_vec(),
            },
        );
        Ok(id)
    }
    fn read_loose(&self, id: &str) -> Result<Object, Error> {
        let path = self.path(id)?;
        is_dir(path.parent().unwrap())?;
        let bytes = bounded_file(&path, (MAX_OBJECT + MAX_HEADER + 256) as u64)?;
        self.store
            .work
            .read_bytes
            .fetch_add(bytes.len() as u64, Ordering::Relaxed);
        if bytes.len() < 9 || &bytes[..5] != MAGIC {
            return Err(integrity("invalid loose framing"));
        }
        let n = u32::from_be_bytes(bytes[5..9].try_into().unwrap()) as usize;
        if n > MAX_HEADER || n > bytes.len() - 9 {
            return Err(integrity("invalid loose header length"));
        }
        let h: Header = parse(&bytes[9..9 + n])?;
        if json(&h)? != bytes[9..9 + n]
            || h.schema != "knowledgehub-loose/1"
            || h.oid != id
            || number(&h.stored_length)? != (bytes.len() - 9 - n) as u64
        {
            return Err(integrity("invalid object header"));
        }
        let logical = decode(&h.codec, &bytes[9 + n..], number(&h.logical_length)?)?;
        if oid(&h.kind, &logical)? != id {
            return Err(integrity("logical object hash differs"));
        }
        payload(&h.kind, &logical)?;
        Ok(Object {
            kind: h.kind,
            bytes: logical,
        })
    }
    pub fn get(&self, id: &str) -> Result<Object, Error> {
        if let Some(v) = self.cache.borrow().get(id) {
            return Ok(v.clone());
        }
        if let Some(v) = self.pending.borrow().values.get(id) {
            return Ok(v.clone());
        }
        if !model::hash(id) {
            return Err(integrity("invalid object address"));
        }
        self.store.work.reads.fetch_add(1, Ordering::Relaxed);
        self.store.point("read-object")?;
        let added = self.pending.borrow().locations.get(id).cloned();
        let v = if let Some(location) = added {
            self.locate(id, &location)
        } else if self
            .path(id)?
            .try_exists()
            .map_err(|_| integrity("object metadata"))?
        {
            self.read_loose(id)
                .or_else(|original| self.read_packed(id).map_err(|_| original))
        } else {
            self.read_packed(id)
        }?;
        self.cache(id.into(), v.clone());
        Ok(v)
    }
    pub fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        id: &str,
        kind: &str,
    ) -> Result<T, Error> {
        let v = self.get(id)?;
        if v.kind != kind {
            return Err(integrity("wrong object kind"));
        }
        parse(&v.bytes)
    }
    fn has_packed(&self, id: &str) -> Result<bool, Error> {
        if self.pending.borrow().locations.contains_key(id) {
            return Ok(true);
        }
        self.discover()?;
        Ok(self.locations.borrow().as_ref().unwrap().contains_key(id))
    }
    fn discover(&self) -> Result<(), Error> {
        if self.locations.borrow().is_some() {
            return Ok(());
        }
        if self.store.packed_cache {
            return self.discover_derived();
        }
        let mut inventory = Vec::new();
        for entry in fs::read_dir(self.store.root.join("objects/packs"))
            .map_err(|_| integrity("pack inventory"))?
        {
            let e = entry.map_err(|_| integrity("pack inventory entry"))?;
            let name = e.file_name().to_string_lossy().into_owned();
            if name.ends_with(".idx") || name.ends_with(".khpack") {
                let m = fs::symlink_metadata(e.path())
                    .map_err(|_| integrity("pack inventory metadata"))?;
                if !m.is_file() {
                    return Err(integrity("pack representation not regular file"));
                }
                let time = m
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map_or(0, |d| d.as_nanos());
                inventory.push((name, m.len(), time));
            }
        }
        inventory.sort();
        if let Some(cache) = self
            .store
            .work
            .locations
            .lock()
            .unwrap()
            .as_ref()
            .filter(|c| c.inventory == inventory)
        {
            *self.locations.borrow_mut() = Some(cache.entries.clone());
            return Ok(());
        }
        let mut entries = BTreeMap::new();
        let dir = self.store.root.join("objects/packs");
        is_dir(&dir)?;
        for file in fs::read_dir(dir).map_err(|_| integrity("pack directory"))? {
            let file = file.map_err(|_| integrity("pack entry"))?;
            let name = file.file_name().to_string_lossy().into_owned();
            if let Some(id) = name.strip_suffix(".khpack") {
                if !model::hash(id) {
                    return Err(integrity("invalid closed pack name"));
                }
                entries.extend(self.scan_pack(id)?);
                if entries.len() > 1_000_000 {
                    return Err(Error::Capacity);
                }
            }
        }
        let entries = Arc::new(entries);
        *self.store.work.locations.lock().unwrap() = Some(LocationCache {
            inventory,
            entries: entries.clone(),
            catalog_stamp: None,
        });
        *self.locations.borrow_mut() = Some(entries);
        Ok(())
    }
    // Scanning block headers rebuilds offsets without decompressing unrelated blocks.
    pub(super) fn scan_pack(&self, id: &str) -> Result<BTreeMap<String, Location>, Error> {
        let path = self
            .store
            .root
            .join("objects/packs")
            .join(format!("{id}.khpack"));
        let index = path.with_extension("idx");
        if let Ok(bytes) = bounded_file(&index, MAX_OBJECT as u64) {
            self.store
                .work
                .index_bytes
                .fetch_add(bytes.len() as u64, Ordering::Relaxed);
            if let Ok(entries) = parse::<BTreeMap<String, Location>>(&bytes) {
                if entries.iter().all(|(k, v)| model::hash(k) && v.pack == id) {
                    return Ok(entries);
                }
            }
        }
        self.scan_pack_entries(id)
    }
    pub(super) fn scan_pack_entries(&self, id: &str) -> Result<BTreeMap<String, Location>, Error> {
        if !model::hash(id) {
            return Err(integrity("pack address"));
        }
        self.store.work.rebuilds.fetch_add(1, Ordering::Relaxed);
        let path = self
            .store
            .root
            .join("objects/packs")
            .join(format!("{id}.khpack"));
        let mut file = open_read(&path)?;
        let mut magic = [0; 8];
        file.read_exact(&mut magic)
            .map_err(|_| integrity("pack header"))?;
        if &magic != PACK_MAGIC {
            return Err(integrity("pack magic"));
        }
        let mut count = [0; 8];
        file.read_exact(&mut count)
            .map_err(|_| integrity("pack count"))?;
        let count = u64::from_be_bytes(count);
        if count > 1_000_000 {
            return Err(Error::Capacity);
        }
        let mut result = BTreeMap::new();
        let mut previous = String::new();
        for _ in 0..count {
            let offset = file
                .stream_position()
                .map_err(|_| integrity("pack position"))?;
            let (header, logical, block_length) = self.block(&mut file)?;
            let count = number(&header.object_count)?;
            if count == 0 || count > 512 || (logical.len() > 1_048_576 && count != 1) {
                return Err(integrity("pack block count/size"));
            }
            let mut at = 0;
            for _ in 0..number(&header.object_count)? {
                let begin = at;
                let (oid, object) = entry(&logical, &mut at)?;
                if oid <= previous {
                    return Err(integrity("duplicate or unsorted pack entry"));
                }
                previous = oid.clone();
                result.insert(
                    oid,
                    Location {
                        pack: id.into(),
                        block_offset: offset.to_string(),
                        block_length: block_length.to_string(),
                        entry_offset: begin.to_string(),
                        kind: object.kind,
                        length: object.bytes.len().to_string(),
                    },
                );
            }
            if result.len() > 1_000_000 {
                return Err(Error::Capacity);
            }
            if at != logical.len() {
                return Err(integrity("unexplained block bytes"));
            }
        }
        if file
            .stream_position()
            .map_err(|_| integrity("pack length"))?
            != file
                .metadata()
                .map_err(|_| integrity("pack metadata"))?
                .len()
        {
            return Err(integrity("pack trailing data"));
        }
        Ok(result)
    }
    fn block(&self, file: &mut File) -> Result<(BlockHeader, Vec<u8>, u64), Error> {
        let mut n = [0; 4];
        file.read_exact(&mut n)
            .map_err(|_| integrity("pack block length"))?;
        let n = u32::from_be_bytes(n) as usize;
        if n > MAX_HEADER {
            return Err(Error::Capacity);
        }
        let mut bytes = vec![0; n];
        file.read_exact(&mut bytes)
            .map_err(|_| integrity("pack block header"))?;
        let h: BlockHeader = parse(&bytes)?;
        if h.schema != "knowledgehub-pack-block/1" || json(&h)? != bytes {
            return Err(integrity("block schema"));
        }
        let len = number(&h.stored_length)?;
        if len > (MAX_OBJECT + 256) as u64 {
            return Err(Error::Capacity);
        }
        let mut encoded = vec![0; len as usize];
        file.read_exact(&mut encoded)
            .map_err(|_| integrity("pack block payload"))?;
        self.store
            .work
            .read_bytes
            .fetch_add(4 + n as u64 + len, Ordering::Relaxed);
        let logical = decode(&h.codec, &encoded, number(&h.logical_length)?)?;
        if codec::digest(&logical) != h.logical_sha256 {
            return Err(integrity("pack block hash"));
        }
        Ok((h, logical, 4 + n as u64 + len))
    }
    fn locate(&self, id: &str, location: &Location) -> Result<Object, Error> {
        let offset = number(&location.block_offset)?;
        let cached = self
            .block_cache
            .borrow()
            .as_ref()
            .filter(|(pack, at, _, _)| *pack == location.pack && *at == offset)
            .map(|(_, _, bytes, len)| (bytes.clone(), *len));
        let (bytes, len) = match cached {
            Some(v) => v,
            None => {
                let mut f = open_read(
                    &self
                        .store
                        .root
                        .join("objects/packs")
                        .join(format!("{}.khpack", location.pack)),
                )?;
                f.seek(SeekFrom::Start(offset))
                    .map_err(|_| integrity("pack seek"))?;
                let (_, bytes, len) = self.block(&mut f)?;
                let bytes = Arc::new(bytes);
                if bytes.len() <= 1_048_576 {
                    *self.block_cache.borrow_mut() =
                        Some((location.pack.clone(), offset, bytes.clone(), len));
                }
                (bytes, len)
            }
        };
        if len != number(&location.block_length)? {
            return Err(integrity("index block length"));
        }
        let mut at =
            usize::try_from(number(&location.entry_offset)?).map_err(|_| Error::Capacity)?;
        let (actual, object) = entry(&bytes, &mut at)?;
        if actual != id
            || object.kind != location.kind
            || object.bytes.len() as u64 != number(&location.length)?
        {
            return Err(integrity("invalid location index"));
        }
        Ok(object)
    }
    fn read_packed(&self, id: &str) -> Result<Object, Error> {
        self.discover()?;
        let location = self.locations.borrow().as_ref().unwrap().get(id).cloned();
        let try_location = || -> Result<Object, Error> {
            let location = location
                .as_ref()
                .ok_or_else(|| integrity("required object absent from derived index"))?;
            self.locate(id, location)
        };
        match try_location() {
            Ok(v) => Ok(v),
            Err(e) if self.store.packed_cache => Err(e),
            Err(_) => {
                // The index is derived. Rebuild from verified closed blocks before deciding absence.
                let mut rebuilt = BTreeMap::new();
                for e in fs::read_dir(self.store.root.join("objects/packs"))
                    .map_err(|_| integrity("packs"))?
                {
                    let name = e
                        .map_err(|_| integrity("pack entry"))?
                        .file_name()
                        .to_string_lossy()
                        .into_owned();
                    if let Some(pack) = name.strip_suffix(".khpack") {
                        if let Ok(entries) = self.scan_pack_entries(pack) {
                            rebuilt.extend(entries);
                        }
                    }
                }
                *self.store.work.locations.lock().unwrap() = None;
                *self.locations.borrow_mut() = Some(Arc::new(rebuilt));
                let loc = self
                    .locations
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .get(id)
                    .cloned()
                    .ok_or_else(|| integrity("required packed object missing"))?;
                self.locate(id, &loc)
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct BlockHeader {
    pub schema: String,
    pub codec: String,
    pub logical_length: String,
    pub stored_length: String,
    pub object_count: String,
    pub logical_sha256: String,
}

pub(super) fn entry(bytes: &[u8], at: &mut usize) -> Result<(String, Object), Error> {
    fn take<'a>(bytes: &'a [u8], at: &mut usize, n: usize) -> Result<&'a [u8], Error> {
        let end = at
            .checked_add(n)
            .ok_or_else(|| integrity("entry offset overflow"))?;
        let v = bytes
            .get(*at..end)
            .ok_or_else(|| integrity("truncated pack entry"))?;
        *at = end;
        Ok(v)
    }
    let n = u16::from_be_bytes(take(bytes, at, 2)?.try_into().unwrap()) as usize;
    let kind = std::str::from_utf8(take(bytes, at, n)?)
        .map_err(|_| integrity("entry kind"))?
        .to_string();
    let id = take(bytes, at, 32)?
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    let n = u64::from_be_bytes(take(bytes, at, 8)?.try_into().unwrap());
    if n > MAX_OBJECT as u64 {
        return Err(Error::Capacity);
    }
    let logical = take(bytes, at, n as usize)?.to_vec();
    if oid(&kind, &logical)? != id {
        return Err(integrity("entry OID differs"));
    }
    payload(&kind, &logical)?;
    Ok((
        id,
        Object {
            kind,
            bytes: logical,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frames_are_strict_bounded_lossless_and_kind_separated() {
        let (_s, b) = super::super::object_backend("shared-frames");
        let o = Objects::new(&b.store);
        let p = br#"{"schema":"test/1","value":"Hello"}"#;
        assert_ne!(oid("blob", p).unwrap(), oid("record-data", p).unwrap());
        assert_eq!(
            oid("blob", b"abc").unwrap(),
            codec::digest(b"knowledgehub-object/1\nblob\n3\nabc")
        );
        let id = o.put("record-data", p).unwrap();
        assert_eq!(Objects::new(&b.store).get(&id).unwrap().bytes, p);
        for bad in [
            br#"{"schema":"x","x":1,"x":2}"#.as_slice(),
            br#"{"schema":"x","x":"\ud800"}"#,
            br#"{"schema":"x","x":1.0}"#,
        ] {
            assert!(o.put("record-data", bad).is_err());
        }
        let v = vec![b'x'; 1_000_000];
        let (codec, encoded) = encode(&v).unwrap();
        assert_eq!(codec, "zstd");
        assert_eq!(decode(&codec, &encoded, v.len() as u64).unwrap(), v);
        let mut trailing = encoded.clone();
        trailing.push(0);
        assert!(decode("zstd", &trailing, v.len() as u64).is_err());
        assert!(decode("zstd", &encoded, MAX_OBJECT as u64 + 1000).is_err());
        assert!(decode("unknown", b"", 0).is_err());
        let dictionary_frame = [0x28, 0xb5, 0x2f, 0xfd, 0x21, 0x01, 0x00, 0x01, 0x00, 0x00];
        assert!(zstd::zstd_safe::get_dict_id_from_frame(&dictionary_frame).is_some());
        assert!(decode("zstd", &dictionary_frame, 0).is_err());
        let original = fs::read(o.path(&id).unwrap()).unwrap();
        let header_len = u32::from_be_bytes(original[5..9].try_into().unwrap()) as usize;
        let header: Value = serde_json::from_slice(&original[9..9 + header_len]).unwrap();
        let mut compressed_header = header.clone();
        let compressed = zstd::bulk::compress(p, 9).unwrap();
        compressed_header["codec"] = "zstd".into();
        compressed_header["stored_length"] = compressed.len().to_string().into();
        let h = json(&compressed_header).unwrap();
        let mut frame = Vec::from(MAGIC.as_slice());
        frame.extend_from_slice(&(h.len() as u32).to_be_bytes());
        frame.extend(h);
        frame.extend(compressed);
        fs::write(o.path(&id).unwrap(), frame).unwrap();
        assert_eq!(Objects::new(&b.store).get(&id).unwrap().bytes, p);
        for (field, value) in [
            ("schema", Value::String("unknown/9".into())),
            ("surprise", Value::Bool(true)),
            ("logical_length", Value::String("0001".into())),
        ] {
            let mut h = header.clone();
            h[field] = value;
            let bytes = json(&h).unwrap();
            let mut bad = Vec::from(MAGIC.as_slice());
            bad.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
            bad.extend(bytes);
            bad.extend_from_slice(&original[9 + header_len..]);
            fs::write(o.path(&id).unwrap(), bad).unwrap();
            assert!(Objects::new(&b.store).get(&id).is_err());
        }
        let mut frame = original;
        frame.push(0);
        fs::write(o.path(&id).unwrap(), frame).unwrap();
        assert!(Objects::new(&b.store).get(&id).is_err());
    }
}
