//! Private guarded storage port and the native local file adapter.
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

mod native;
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
    /// Coordinated single-host macOS arm64 on writable local APFS.
    pub fn local_apfs() -> Self {
        Self {
            host: "macos-aarch64".into(),
            filesystem: "apfs".into(),
            coordinated_single_host: true,
        }
    }
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
    pub fn local_apfs() -> Self {
        Self {
            support: FileSupport::local_apfs(),
            jobs: JobLimits::default(),
        }
    }

    /// Compose a native declaration. Create/open still check the actual path.
    pub fn native_local() -> Result<Self, Error> {
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        return Ok(Self::local_ext4());
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        return Ok(Self::local_apfs());
        #[cfg(not(any(
            all(target_os = "linux", target_arch = "x86_64"),
            all(target_os = "macos", target_arch = "aarch64")
        )))]
        Err(Error::Incompatible(
            "unsupported native file-backend host".into(),
        ))
    }
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
        Ok(Self {
            store: Arc::new(FileStore {
                root: path,
                support: options.support,
                work: shared::Counters::default(),
                loose_budget: None,
                packed_cache: false,
                #[cfg(test)]
                hook: std::sync::Mutex::new(None),
            }),
            jobs: Jobs::new(options.jobs.active, options.jobs.waiting)?,
            cursors: Arc::new(crate::ontology::navigation::Cursors::new()),
        })
    }
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
    fn publication(
        &self,
        scope: &ModelScope,
        request: &Publication,
    ) -> Result<PublicationResult, Error>;
    fn history_page(
        &self,
        scope: &ModelScope,
        request: &HistoryRequest,
        position: &crate::ontology::history::Position,
    ) -> Result<crate::ontology::history::Slice, Error>;
    fn allocate_id(&self) -> Result<SnapshotId, Error>;
    fn create(&self, initial: Snapshot) -> Result<(), Error>;
    fn acquire(&self, scope: &ModelScope) -> Result<Box<dyn BranchStorage + '_>, Error>;
    fn acquire_read(&self, scope: &ModelScope) -> Result<Box<dyn BranchStorage + '_>, Error>;
}
pub(crate) trait BranchStorage {
    fn model_view(&self, selection: &Selection)
    -> Result<crate::ontology::impact::ViewInfo, Error>;
    fn search_text(&self, request: &SearchText, prepare: bool) -> Result<SearchResult, Error>;
    fn prepare_neighbours(&self, request: &Neighbours, force: bool)
    -> Result<NeighbourWork, Error>;
    fn neighbours(
        &self,
        request: &Neighbours,
        position: &crate::ontology::neighbours::Position,
    ) -> Result<crate::ontology::neighbours::Slice, Error>;
    fn publish(&self, plan: ValidatedPlan, parent: HeadRef) -> Result<Receipt, Error>;
    fn current(&self) -> Result<Stored, Error>;
    fn head(&self) -> Result<HeadRef, Error>;
    fn operation(&self, id: &str) -> Result<Option<Receipt>, Error>;
    fn state(&self, selection: &Selection) -> Result<State, Error>;
    fn record(&self, selection: &Selection, reference: &ExactRef) -> Result<Record, Error>;
    fn artifact(&self, selection: &Selection, reference: &ExactRef) -> Result<Content, Error>;
    fn artifact_range(
        &self,
        selection: &Selection,
        reference: &ExactRef,
        start: u64,
        len: u64,
    ) -> Result<ArtifactRange, Error>;
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
#[cfg(test)]
type Hook = Arc<dyn Fn(&str) -> Result<(), Error> + Send + Sync>;
pub(crate) struct FileStore {
    root: PathBuf,
    support: FileSupport,
    work: shared::Counters,
    loose_budget: Option<std::sync::atomic::AtomicU64>,
    packed_cache: bool,
    #[cfg(test)]
    hook: std::sync::Mutex<Option<Hook>>,
}
fn storage(stage: &str) -> Error {
    Error::Storage {
        stage: stage.into(),
    }
}
fn integrity(what: &str) -> Error {
    Error::Integrity(what.into())
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
    fn check_profile(&self, path: &Path) -> Result<(), Error> {
        native::check(&self.support, path)
    }

    fn sync_dir(&self, path: &Path) -> Result<(), Error> {
        self.point("directory-sync")?;
        native::sync_directory(path).map_err(|_| storage("sync directory"))
    }

    fn sync_file(&self, file: &File, stage: &str) -> Result<(), Error> {
        self.point("regular-file-sync")?;
        native::sync_file(file).map_err(|_| storage(stage))
    }

    fn persist(&self, path: &Path, bytes: &[u8]) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(path)
            .map_err(|_| storage("create immutable file"))?;
        file.write_all(bytes)
            .map_err(|_| storage("persist immutable file"))?;
        self.sync_file(&file, "persist immutable file")
    }

    fn fence(&self, path: &Path) -> Result<(), Error> {
        #[cfg(target_os = "macos")]
        {
            self.point("publication-fence")?;
            native::sync_file(&open_read(path)?).map_err(|_| storage("full publication fence"))?;
        }
        #[cfg(not(target_os = "macos"))]
        let _ = path;
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
}
impl Storage for FileStore {
    fn publication(
        &self,
        scope: &ModelScope,
        request: &Publication,
    ) -> Result<PublicationResult, Error> {
        shared::history::publication(self, scope, request)
    }
    fn history_page(
        &self,
        scope: &ModelScope,
        request: &HistoryRequest,
        position: &crate::ontology::history::Position,
    ) -> Result<crate::ontology::history::Slice, Error> {
        shared::history::page(self, scope, request, position)
    }
    fn allocate_id(&self) -> Result<SnapshotId, Error> {
        let mut bytes = [0u8; 32];
        File::open("/dev/urandom")
            .and_then(|mut f| f.read_exact(&mut bytes))
            .map_err(|_| storage("allocate commit identity"))?;
        Ok(SnapshotId(codec::digest(&bytes)))
    }
    fn create(&self, initial: Snapshot) -> Result<(), Error> {
        shared::create(self, initial)
    }
    fn acquire(&self, scope: &ModelScope) -> Result<Box<dyn BranchStorage + '_>, Error> {
        shared::acquire(self, scope, false)
    }
    fn acquire_read(&self, scope: &ModelScope) -> Result<Box<dyn BranchStorage + '_>, Error> {
        shared::acquire(self, scope, true)
    }
}
