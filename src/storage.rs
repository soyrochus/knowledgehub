//! Private guarded storage port and the initial Linux file adapter.
use crate::{
    codec,
    jobs::Jobs,
    model::{self, ValidatedPlan},
    *,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::{
        fs::{MetadataExt, OpenOptionsExt},
        io::AsRawFd,
    },
    path::{Path, PathBuf},
    sync::Arc,
};

mod shared;
pub use shared::{FileStorageReport, FileWork};

#[derive(Clone, Copy, Debug)]
pub struct JobLimits {
    pub active: usize,
    pub waiting: usize,
}
impl Default for JobLimits {
    fn default() -> Self {
        Self {
            active: 4,
            waiting: 32,
        }
    }
}

/// A declaration by the trusted host, checked against observable filesystem facts.
#[derive(Clone, Debug)]
pub struct FileSupport {
    pub host: String,
    pub filesystem: String,
    pub coordinated_single_host: bool,
}
impl FileSupport {
    pub fn local_ext4() -> Self {
        Self {
            host: "linux-x86_64".into(),
            filesystem: "ext4".into(),
            coordinated_single_host: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FileOptions {
    pub support: FileSupport,
    pub jobs: JobLimits,
}
impl FileOptions {
    pub fn local_ext4() -> Self {
        Self {
            support: FileSupport::local_ext4(),
            jobs: JobLimits::default(),
        }
    }
}

/// Construct at the host composition boundary; ordinary operations use semantic scope values.
#[derive(Clone)]
pub struct FileBackend {
    pub(crate) store: Arc<FileStore>,
    pub(crate) jobs: Arc<Jobs>,
    pub(crate) cursors: Arc<crate::ontology::navigation::Cursors>,
}
impl FileBackend {
    pub fn new(path: impl Into<PathBuf>, options: FileOptions) -> Result<Self, Error> {
        let path = path.into();
        if !path.is_absolute() || path.file_name().is_none() {
            return Err(Error::InvalidInput(
                "absolute store path with a parent required".into(),
            ));
        }
        let jobs = Jobs::new(options.jobs.active, options.jobs.waiting)?;
        Ok(Self {
            store: Arc::new(FileStore {
                root: path,
                format: FILE_VERSION,
                support: options.support,
                work: shared::Counters::default(),
                #[cfg(test)]
                hook: std::sync::Mutex::new(None),
            }),
            jobs,
            cursors: Arc::new(crate::ontology::navigation::Cursors::new()),
        })
    }
    /// Explicitly opt in to the typed ontology format. Existing stores are never migrated.
    pub fn new_ontology(path: impl Into<PathBuf>, options: FileOptions) -> Result<Self, Error> {
        let mut backend = Self::new(path, options)?;
        Arc::get_mut(&mut backend.store)
            .expect("new unshared backend")
            .format = crate::ontology::FILE_FORMAT;
        Ok(backend)
    }
    /// Explicit shared immutable files/3 storage; never converts an existing store.
    pub fn new_shared_ontology(
        path: impl Into<PathBuf>,
        options: FileOptions,
    ) -> Result<Self, Error> {
        let mut backend = Self::new(path, options)?;
        Arc::get_mut(&mut backend.store)
            .expect("new unshared backend")
            .format = crate::ontology::SHARED_FILE_FORMAT;
        Ok(backend)
    }
    /// Close admission for all clones and wait for every admitted job. Keep the host runtime alive.
    pub async fn drain(&self) {
        self.cursors.clear();
        self.jobs.drain().await;
    }
    #[cfg(test)]
    pub(crate) fn set_hook(
        &self,
        hook: impl Fn(&str) -> Result<(), Error> + Send + Sync + 'static,
    ) {
        *self.store.hook.lock().unwrap() = Some(Arc::new(hook));
    }
}

pub(crate) trait Storage: Send + Sync {
    fn version(&self) -> &str;
    fn allocate_id(&self) -> Result<SnapshotId, Error>;
    fn create(&self, initial: Snapshot) -> Result<(), Error>;
    fn acquire(&self, scope: &Scope) -> Result<Box<dyn BranchStorage + '_>, Error>;
    fn acquire_read(&self, scope: &Scope) -> Result<Box<dyn BranchStorage + '_>, Error> {
        self.acquire(scope)
    }
}
pub(crate) trait BranchStorage {
    fn recover(&self) -> Result<History, Error>;
    fn publish(&self, plan: ValidatedPlan, parent: HeadRef) -> Result<Receipt, Error>;
    fn current(&self) -> Result<Stored, Error> {
        self.recover()?
            .entries
            .pop()
            .ok_or_else(|| integrity("missing genesis"))
    }
    fn head(&self) -> Result<HeadRef, Error> {
        Ok(self.current()?.reference)
    }
    fn snapshot(&self, id: &SnapshotId) -> Result<Stored, Error> {
        self.recover()?
            .entries
            .into_iter()
            .find(|s| s.snapshot.id == *id)
            .ok_or(Error::NotFound)
    }
    fn operation(&self, id: &str) -> Result<Option<Receipt>, Error> {
        Ok(self.recover()?.operation(id).cloned())
    }
    fn state(
        &self,
        selection: &crate::ontology::Selection,
    ) -> Result<crate::ontology::State, Error> {
        use crate::ontology::Selection;
        let stored = match selection {
            Selection::Snapshot(id) => self.snapshot(id)?,
            Selection::Baseline(_) => self.current()?,
        };
        let state = stored
            .snapshot
            .ontology
            .ok_or_else(|| Error::Incompatible("not an ontology state".into()))?;
        match selection {
            Selection::Snapshot(_) => Ok(*state),
            Selection::Baseline(r) => Ok(state.view(state.baseline(r).ok_or(Error::NotFound)?)),
        }
    }
    fn record(
        &self,
        selection: &crate::ontology::Selection,
        r: &crate::ontology::ExactRef,
    ) -> Result<crate::ontology::Record, Error> {
        self.state(selection)?.resolve(r).ok_or(Error::NotFound)
    }
    fn artifact(
        &self,
        selection: &crate::ontology::Selection,
        r: &crate::ontology::ExactRef,
    ) -> Result<Content, Error> {
        self.state(selection)?
            .artifact(r)
            .cloned()
            .ok_or(Error::NotFound)
    }
    fn artifact_range(
        &self,
        selection: &crate::ontology::Selection,
        r: &crate::ontology::ExactRef,
        start: u64,
        len: u64,
    ) -> Result<crate::ontology::ArtifactRange, Error> {
        let content = self.artifact(selection, r)?;
        if start > content.reference.length || len > content.reference.length - start {
            return Err(Error::InvalidInput("artifact range out of bounds".into()));
        }
        Ok(crate::ontology::ArtifactRange {
            selection: selection.clone(),
            reference: r.clone(),
            original: content.reference,
            offset: start,
            bytes: content.bytes[start as usize..(start + len) as usize].to_vec(),
            full_content_verified: true,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct HeadRef {
    pub(crate) snapshot: SnapshotId,
    pub(crate) manifest_sha256: String,
}
#[derive(Clone)]
pub(crate) struct Stored {
    pub snapshot: Snapshot,
    pub receipt: Option<Receipt>,
    pub reference: HeadRef,
}
pub(crate) struct History {
    pub entries: Vec<Stored>,
}
impl History {
    pub fn current(&self) -> &Stored {
        self.entries.last().expect("recovered genesis")
    }
    pub fn operation(&self, id: &str) -> Option<&Receipt> {
        self.entries
            .iter()
            .filter_map(|s| s.receipt.as_ref())
            .find(|r| r.operation_id == id)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Metadata {
    format: String,
    fingerprint_format: String,
    scope: Scope,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    format: String,
    snapshot: SnapshotId,
    sequence: u64,
    parent: Option<HeadRef>,
    files: BTreeMap<String, ContentRef>,
}

#[cfg(test)]
type Hook = Arc<dyn Fn(&str) -> Result<(), Error> + Send + Sync>;
pub(crate) struct FileStore {
    root: PathBuf,
    format: &'static str,
    support: FileSupport,
    work: shared::Counters,
    #[cfg(test)]
    hook: std::sync::Mutex<Option<Hook>>,
}
struct Lease<'a> {
    store: &'a FileStore,
    scope: Scope,
    _lock: File,
}

fn storage(stage: &str) -> Error {
    Error::Storage {
        stage: stage.into(),
    }
}
fn integrity(what: &str) -> Error {
    Error::Integrity(what.into())
}
fn sync_dir(path: &Path) -> Result<(), Error> {
    File::open(path)
        .and_then(|f| f.sync_all())
        .map_err(|_| storage("sync directory"))
}
fn is_dir(path: &Path) -> Result<(), Error> {
    if !fs::symlink_metadata(path)
        .map_err(|_| integrity("required directory missing"))?
        .is_dir()
    {
        return Err(integrity("managed directory is not an ordinary directory"));
    }
    Ok(())
}
fn open_read(path: &Path) -> Result<File, Error> {
    let f = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .map_err(|_| integrity("required file missing or inaccessible"))?;
    if !f
        .metadata()
        .map_err(|_| integrity("required file metadata"))?
        .is_file()
    {
        return Err(integrity("required object is not an ordinary file"));
    }
    Ok(f)
}
fn read(path: &Path) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    open_read(path)?
        .read_to_end(&mut bytes)
        .map_err(|_| integrity("required object read"))?;
    Ok(bytes)
}
fn persist(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .map_err(|_| storage("create immutable file"))?;
    f.write_all(bytes)
        .and_then(|_| f.sync_all())
        .map_err(|_| storage("persist immutable file"))
}
fn json<T: Serialize>(v: &T) -> Result<Vec<u8>, Error> {
    codec::canonical(&serde_json::to_value(v).map_err(codec::encoding)?)
}
fn parse<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, Error> {
    codec::decode(bytes).map_err(|_| integrity("malformed stored JSON"))
}
fn lock_file(path: &Path, create: bool) -> Result<File, Error> {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(create)
        .truncate(false)
        .custom_flags(libc::O_NOFOLLOW)
        .open(path)
        .map_err(|_| integrity("stable lock file missing or inaccessible"))?;
    if !f
        .metadata()
        .map_err(|_| integrity("lock metadata"))?
        .is_file()
    {
        return Err(integrity("lock is not an ordinary file"));
    }
    // SAFETY: the descriptor remains owned by f; flock takes no pointers and never blocks.
    if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        let error = std::io::Error::last_os_error();
        return Err(if error.kind() == std::io::ErrorKind::WouldBlock {
            Error::Busy
        } else {
            storage("writer lock")
        });
    }
    // Closing this independent file description releases the lock, including on process death.
    Ok(f)
}

impl FileStore {
    fn request_format(&self) -> &str {
        if self.format == FILE_VERSION {
            FINGERPRINT_VERSION
        } else {
            crate::ontology::REQUEST_FORMAT
        }
    }
    fn check_profile(&self, path: &Path) -> Result<(), Error> {
        if !cfg!(all(target_os = "linux", target_arch = "x86_64"))
            || self.support.host != "linux-x86_64"
            || self.support.filesystem != "ext4"
            || !self.support.coordinated_single_host
        {
            return Err(Error::Incompatible(
                "only coordinated single-host Linux x86_64/local ext4 is supported".into(),
            ));
        }
        is_dir(path)?;
        let f = File::open(path).map_err(|_| storage("open filesystem profile"))?;
        let mut stat = std::mem::MaybeUninit::<libc::statfs>::uninit();
        // SAFETY: stat points to sufficient writable storage; f is a live descriptor.
        if unsafe { libc::fstatfs(f.as_raw_fd(), stat.as_mut_ptr()) } != 0 {
            return Err(storage("inspect filesystem profile"));
        }
        // SAFETY: successful fstatfs initialized stat.
        let stat = unsafe { stat.assume_init() };
        if stat.f_type != 0xef53 {
            return Err(Error::Incompatible(
                "detected filesystem is outside local ext4 profile".into(),
            ));
        }
        // ext2/ext3 share statfs's magic. Confirm the actual mounted filesystem type.
        let dev = f
            .metadata()
            .map_err(|_| storage("inspect mount device"))?
            .dev();
        let device = format!("{}:{}", libc::major(dev), libc::minor(dev));
        let mounts = fs::read_to_string("/proc/self/mountinfo")
            .map_err(|_| Error::Unavailable("cannot establish mounted filesystem type".into()))?;
        if !mounts
            .lines()
            .filter_map(|line| line.split_once(" - "))
            .any(|(left, right)| {
                left.split_whitespace().nth(2) == Some(device.as_str())
                    && right.split_whitespace().next() == Some("ext4")
            })
        {
            return Err(Error::Incompatible(
                "mounted device is outside ext4 profile".into(),
            ));
        }
        Ok(())
    }

    fn metadata(&self, scope: &Scope) -> Result<(), Error> {
        self.check_profile(&self.root)?;
        let raw = read(&self.root.join("hub.json"))?;
        let declared: serde_json::Value = parse(&raw)?;
        if declared.get("format").and_then(serde_json::Value::as_str) != Some(self.format) {
            return Err(Error::Incompatible(
                "explicit file backend format differs".into(),
            ));
        }
        let device = fs::metadata(&self.root)
            .map_err(|_| integrity("store metadata"))?
            .dev();
        for name in ["staging", "generations", "refs"] {
            let path = self.root.join(name);
            self.check_profile(&path)?;
            if fs::metadata(path)
                .map_err(|_| integrity("layout metadata"))?
                .dev()
                != device
            {
                return Err(Error::Incompatible(
                    "publication directories must share one filesystem".into(),
                ));
            }
        }
        let meta: Metadata = parse(&raw)?;
        if meta.format != self.format
            || meta.fingerprint_format != self.request_format()
            || &meta.scope != scope
        {
            return Err(Error::Incompatible(
                "file, fingerprint or exact scope/profile version differs".into(),
            ));
        }
        model::check_scope(&meta.scope)?;
        Ok(())
    }

    fn point(&self, _stage: &str) -> Result<(), Error> {
        #[cfg(test)]
        {
            let hook = self.hook.lock().unwrap().clone();
            if let Some(hook) = hook {
                hook(_stage)?;
            }
        }
        Ok(())
    }

    fn prepare_generation(
        &self,
        snapshot: &Snapshot,
        receipt: Option<&Receipt>,
        parent: Option<HeadRef>,
    ) -> Result<HeadRef, Error> {
        if !model::hash(&snapshot.id.0) {
            return Err(Error::InvalidInput("invalid snapshot identity".into()));
        }
        let stage = self.root.join("staging").join(&snapshot.id.0);
        fs::create_dir(&stage).map_err(|_| storage("create staging generation"))?;
        fs::create_dir(stage.join("documents")).map_err(|_| storage("create document staging"))?;
        fs::create_dir(stage.join("sources")).map_err(|_| storage("create source staging"))?;
        fs::create_dir(stage.join("artifacts")).map_err(|_| storage("create artifact staging"))?;
        self.point("staging")?;
        let mut graph = snapshot.clone();
        for d in &mut graph.documents {
            d.content.bytes.clear();
        }
        for s in &mut graph.sources {
            s.content.bytes.clear();
        }
        if let Some(state) = &mut graph.ontology {
            for c in state.artifacts.values_mut() {
                c.bytes.clear();
            }
        }
        let mut objects = BTreeMap::new();
        objects.insert("graph.json".to_string(), json(&graph)?);
        if let Some(receipt) = receipt {
            objects.insert("receipt.json".into(), json(receipt)?);
        }
        for d in &snapshot.documents {
            objects.insert(
                format!("documents/{}.md", codec::digest(d.reference.id.as_bytes())),
                d.content.bytes.clone(),
            );
        }
        for s in &snapshot.sources {
            objects.insert(
                format!("sources/{}.bin", codec::digest(s.reference.id.as_bytes())),
                s.content.bytes.clone(),
            );
        }
        if let Some(state) = &snapshot.ontology {
            for (key, content) in &state.artifacts {
                let ext=if state.records.get(key).is_some_and(|r|matches!(&r.data,crate::ontology::RecordData::SourceRevision(s) if s.media_type=="text/markdown")){"md"}else{"bin"};
                objects.insert(
                    format!("artifacts/{}.{ext}", codec::digest(key.as_bytes())),
                    content.bytes.clone(),
                );
            }
        }
        let files = objects
            .iter()
            .map(|(name, bytes)| (name.clone(), ContentRef::of(bytes)))
            .collect();
        for (name, bytes) in objects {
            persist(&stage.join(name), &bytes)?;
            self.point("content-file")?;
        }
        let manifest = Manifest {
            format: self.format.into(),
            snapshot: snapshot.id.clone(),
            sequence: snapshot.sequence,
            parent,
            files,
        };
        let bytes = json(&manifest)?;
        persist(&stage.join("manifest.json"), &bytes)?;
        for path in [
            stage.join("documents"),
            stage.join("sources"),
            stage.join("artifacts"),
            stage.clone(),
            self.root.join("staging"),
        ] {
            sync_dir(&path)?;
        }
        self.point("content-synced")?;
        let generation = self.root.join("generations").join(&snapshot.id.0);
        // Names are random, never reused; do not replace any existing generation.
        if generation
            .try_exists()
            .map_err(|_| storage("check immutable destination"))?
        {
            return Err(integrity("generation identity collision"));
        }
        fs::rename(&stage, &generation).map_err(|_| storage("install immutable generation"))?;
        sync_dir(&self.root.join("staging"))?;
        sync_dir(&self.root.join("generations"))?;
        self.point("generation-installed")?;
        Ok(HeadRef {
            snapshot: snapshot.id.clone(),
            manifest_sha256: codec::digest(&bytes),
        })
    }

    fn publish_reference(&self, head: &HeadRef, operation_id: Option<String>) -> Result<(), Error> {
        let refs = self.root.join("refs");
        let temp = refs.join(format!("{}.tmp", head.snapshot.0));
        persist(&temp, &json(head)?)?;
        self.point("reference-synced")?;
        fs::rename(temp, refs.join("head.json"))
            .map_err(|_| storage("replace branch reference"))?;
        // From here on, failure is never evidence that the old head still owns the branch.
        (|| {
            self.point("head-replaced")?;
            sync_dir(&refs)?;
            self.point("head-synced")?;
            self.point("acknowledgement")
        })()
        .map_err(|_| Error::Uncertain {
            operation_id,
            stage: "after head replacement; recover original operation".into(),
        })
    }

    fn load(&self, head: &HeadRef, scope: &Scope) -> Result<(Stored, Option<HeadRef>), Error> {
        if !model::hash(&head.snapshot.0) || !model::hash(&head.manifest_sha256) {
            return Err(integrity("invalid accepted head"));
        }
        let path = self.root.join("generations").join(&head.snapshot.0);
        is_dir(&path)?;
        let bytes = read(&path.join("manifest.json"))?;
        if codec::digest(&bytes) != head.manifest_sha256 {
            return Err(integrity("accepted manifest digest mismatch"));
        }
        let manifest: Manifest = parse(&bytes)?;
        if manifest.format != self.format {
            return Err(Error::Incompatible("generation file format".into()));
        }
        if manifest.snapshot != head.snapshot {
            return Err(integrity("manifest snapshot mismatch"));
        }
        let mut objects = BTreeMap::new();
        for (name, reference) in &manifest.files {
            let safe = name == "graph.json"
                || name == "receipt.json"
                || name
                    .strip_prefix("documents/")
                    .and_then(|s| s.strip_suffix(".md"))
                    .is_some_and(model::hash)
                || name
                    .strip_prefix("sources/")
                    .and_then(|s| s.strip_suffix(".bin"))
                    .is_some_and(model::hash);
            let safe = safe
                || name
                    .strip_prefix("artifacts/")
                    .and_then(|s| s.strip_suffix(".md").or_else(|| s.strip_suffix(".bin")))
                    .is_some_and(model::hash);
            if !safe {
                return Err(integrity("unsupported manifest object path"));
            }
            if name.contains('/') {
                is_dir(&path.join(name.split('/').next().unwrap()))?;
            }
            let bytes = read(&path.join(name))?;
            if &ContentRef::of(&bytes) != reference {
                return Err(integrity("accepted object digest or length mismatch"));
            }
            objects.insert(name.clone(), bytes);
        }
        let mut snapshot: Snapshot = parse(
            objects
                .remove("graph.json")
                .as_deref()
                .ok_or_else(|| integrity("missing graph binding"))?,
        )?;
        if snapshot.id != manifest.snapshot
            || snapshot.sequence != manifest.sequence
            || &snapshot.scope != scope
        {
            return Err(integrity("graph/manifest/scope mismatch"));
        }
        for d in &mut snapshot.documents {
            if !d.content.bytes.is_empty() {
                return Err(integrity("graph must bind separate Markdown bytes"));
            }
            d.content.bytes = objects
                .remove(&format!(
                    "documents/{}.md",
                    codec::digest(d.reference.id.as_bytes())
                ))
                .ok_or_else(|| integrity("missing Markdown binding"))?;
        }
        for s in &mut snapshot.sources {
            if !s.content.bytes.is_empty() {
                return Err(integrity("graph must bind separate original source bytes"));
            }
            s.content.bytes = objects
                .remove(&format!(
                    "sources/{}.bin",
                    codec::digest(s.reference.id.as_bytes())
                ))
                .ok_or_else(|| integrity("missing source binding"))?;
        }
        if (self.format == crate::ontology::FILE_FORMAT) != snapshot.ontology.is_some() {
            return Err(integrity(
                "snapshot codec does not match explicit store format",
            ));
        }
        if let Some(state) = &mut snapshot.ontology {
            for (key, content) in &mut state.artifacts {
                if !content.bytes.is_empty() {
                    return Err(integrity(
                        "graph must bind separate original artifact bytes",
                    ));
                }
                let ext=if state.records.get(key).is_some_and(|r|matches!(&r.data,crate::ontology::RecordData::SourceRevision(s) if s.media_type=="text/markdown")){"md"}else{"bin"};
                content.bytes = objects
                    .remove(&format!(
                        "artifacts/{}.{ext}",
                        codec::digest(key.as_bytes())
                    ))
                    .ok_or_else(|| integrity("required original artifact binding missing"))?;
            }
        }
        let receipt = objects
            .remove("receipt.json")
            .map(|b| parse::<Receipt>(&b))
            .transpose()?;
        if !objects.is_empty() || !model::validate_snapshot(&snapshot).is_empty() {
            return Err(integrity(
                "accepted snapshot does not meet seed integrity rules",
            ));
        }
        match &receipt {
            Some(r)
                if r.scope == *scope
                    && r.head == snapshot.id
                    && r.sequence == snapshot.sequence
                    && r.fingerprint_version == self.request_format()
                    && model::hash(&r.fingerprint)
                    && !r.operation_id.trim().is_empty()
                    && r.actor.contains(':')
                    && manifest
                        .parent
                        .as_ref()
                        .is_some_and(|p| p.snapshot == r.previous_head) => {}
            None if snapshot.sequence == 0 && manifest.parent.is_none() => {}
            _ => return Err(integrity("receipt/history binding mismatch")),
        }
        Ok((
            Stored {
                snapshot,
                receipt,
                reference: head.clone(),
            },
            manifest.parent,
        ))
    }
}

impl Storage for FileStore {
    fn version(&self) -> &str {
        self.format
    }
    fn allocate_id(&self) -> Result<SnapshotId, Error> {
        let mut bytes = [0u8; 32];
        File::open("/dev/urandom")
            .and_then(|mut f| f.read_exact(&mut bytes))
            .map_err(|_| storage("allocate commit identity"))?;
        Ok(SnapshotId(codec::digest(&bytes)))
    }
    fn create(&self, initial: Snapshot) -> Result<(), Error> {
        if self.format == crate::ontology::SHARED_FILE_FORMAT {
            return shared::create(self, initial);
        }
        if (self.format == crate::ontology::FILE_FORMAT) != initial.ontology.is_some() {
            return Err(Error::Incompatible(
                "explicit snapshot/store format mismatch".into(),
            ));
        }
        let parent = self
            .root
            .parent()
            .ok_or_else(|| Error::InvalidInput("store parent required".into()))?;
        self.check_profile(parent)?;
        let lock_name = format!(
            ".knowledgehub-create-{}.lock",
            codec::digest(self.root.file_name().unwrap().as_encoded_bytes())
        );
        let _lock = lock_file(&parent.join(lock_name), true)?;
        sync_dir(parent)?;
        match fs::create_dir(&self.root) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                self.acquire(&initial.scope)?.recover()?;
                return Err(Error::AlreadyExists);
            }
            Err(_) => return Err(storage("exclusive store creation")),
        }
        for name in ["staging", "generations", "refs"] {
            fs::create_dir(self.root.join(name)).map_err(|_| storage("create store layout"))?;
        }
        persist(&self.root.join("branch.lock"), b"")?;
        persist(
            &self.root.join("hub.json"),
            &json(&Metadata {
                format: self.format.into(),
                fingerprint_format: self.request_format().into(),
                scope: initial.scope.clone(),
            })?,
        )?;
        sync_dir(&self.root)?;
        sync_dir(parent)?;
        // Branch lock is never replaced, including after crash recovery.
        let _branch = lock_file(&self.root.join("branch.lock"), false)?;
        let head = self.prepare_generation(&initial, None, None)?;
        self.publish_reference(&head, None)?;
        sync_dir(&self.root)
            .and_then(|()| sync_dir(parent))
            .map_err(|_| Error::Uncertain {
                operation_id: None,
                stage: "creation persistence after head publication; reopen original scope".into(),
            })?;
        Ok(())
    }
    fn acquire(&self, scope: &Scope) -> Result<Box<dyn BranchStorage + '_>, Error> {
        if self.format == crate::ontology::SHARED_FILE_FORMAT {
            return shared::acquire(self, scope, false);
        }
        self.metadata(scope)?;
        let lock = lock_file(&self.root.join("branch.lock"), false)?;
        Ok(Box::new(Lease {
            store: self,
            scope: scope.clone(),
            _lock: lock,
        }))
    }
    fn acquire_read(&self, scope: &Scope) -> Result<Box<dyn BranchStorage + '_>, Error> {
        if self.format == crate::ontology::SHARED_FILE_FORMAT {
            shared::acquire(self, scope, true)
        } else {
            self.acquire(scope)
        }
    }
}

impl BranchStorage for Lease<'_> {
    fn recover(&self) -> Result<History, Error> {
        self.store.metadata(&self.scope)?;
        let path = self.store.root.join("refs/head.json");
        let mut head: HeadRef = parse(&read(&path)?)?;
        let mut entries = Vec::new();
        let mut seen = BTreeSet::new();
        let mut operations = BTreeSet::new();
        loop {
            if !seen.insert(head.snapshot.clone()) {
                return Err(integrity("cycle in accepted history"));
            }
            let (entry, parent) = self.store.load(&head, &self.scope)?;
            if let Some(r) = &entry.receipt {
                if !operations.insert(r.operation_id.clone()) {
                    return Err(integrity("duplicate committed operation key"));
                }
            }
            entries.push(entry);
            match parent {
                Some(p) => head = p,
                None => break,
            }
        }
        entries.reverse();
        for pair in entries.windows(2) {
            let (a, b) = (&pair[0], &pair[1]);
            let r = b
                .receipt
                .as_ref()
                .ok_or_else(|| integrity("missing history receipt"))?;
            if a.snapshot.sequence.checked_add(1) != Some(b.snapshot.sequence)
                || !model::verify_affected(&a.snapshot, &b.snapshot, r)
                || !b.snapshot.sources.starts_with(&a.snapshot.sources)
                || !b.snapshot.documents.starts_with(&a.snapshot.documents)
                || !b
                    .snapshot
                    .interpretations
                    .starts_with(&a.snapshot.interpretations)
                || !b.snapshot.elements.starts_with(&a.snapshot.elements)
                || !b.snapshot.assertions.starts_with(&a.snapshot.assertions)
            {
                return Err(integrity("incomplete immutable revision history"));
            }
        }
        // Required for an authoritative outcome after a response was lost at reference publication.
        open_read(&path)?.sync_all().map_err(|_| Error::Uncertain {
            operation_id: None,
            stage: "recovery reference persistence".into(),
        })?;
        sync_dir(&self.store.root.join("refs")).map_err(|_| Error::Uncertain {
            operation_id: None,
            stage: "recovery directory persistence".into(),
        })?;
        Ok(History { entries })
    }
    fn publish(&self, plan: ValidatedPlan, parent: HeadRef) -> Result<Receipt, Error> {
        let head =
            self.store
                .prepare_generation(&plan.snapshot, Some(&plan.receipt), Some(parent))?;
        self.store
            .publish_reference(&head, Some(plan.receipt.operation_id.clone()))?;
        Ok(plan.receipt)
    }
}
