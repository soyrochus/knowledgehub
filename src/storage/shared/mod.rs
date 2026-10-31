//! Shared immutable files/3 adapter. The model remains independent of these codecs.
use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

mod content;
mod generation;
mod maintenance;
mod objects;
mod projection;
#[cfg(test)]
use crate::test_specimens as specimens;
#[cfg(test)]
mod tests;
mod trees;
pub(super) use generation::{acquire, create};

/// Cumulative work since this backend instance was constructed (including its clones).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileWork {
    pub object_reads: u64,
    pub encoded_bytes_read: u64,
    pub objects_written: u64,
    pub encoded_bytes_written: u64,
    pub index_bytes_read: u64,
    pub index_rebuilds: u64,
    pub payload_bytes_written: u64,
    pub structural_bytes_written: u64,
}

/// File-adapter diagnostics; these counts do not establish model completeness or acceptance.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileStorageReport {
    pub logical_object_bytes: u64,
    pub encoded_file_bytes: u64,
    pub allocated_file_bytes: u64,
    pub objects: u64,
    pub packs: u64,
    pub generations: u64,
    pub removed_files: u64,
    pub peak_compaction_bytes: u64,
    pub peak_compaction_allocated_bytes: u64,
    pub derived_file_bytes: u64,
    pub artifact_logical_bytes: u64,
    pub record_data_bytes: u64,
    pub baseline_data_bytes: u64,
    pub anchor_data_bytes: u64,
    pub history_tree_bytes: u64,
    pub files: u64,
    pub pack_blocks: u64,
    pub baselines: u64,
    pub work: FileWork,
}

#[derive(Default)]
pub(super) struct Counters {
    reads: AtomicU64,
    read_bytes: AtomicU64,
    writes: AtomicU64,
    write_bytes: AtomicU64,
    index_bytes: AtomicU64,
    rebuilds: AtomicU64,
    locations: std::sync::Mutex<Option<objects::LocationCache>>,
    payload_writes: AtomicU64,
    structural_writes: AtomicU64,
}
impl Counters {
    fn snapshot(&self) -> FileWork {
        FileWork {
            object_reads: self.reads.load(Ordering::Relaxed),
            encoded_bytes_read: self.read_bytes.load(Ordering::Relaxed),
            objects_written: self.writes.load(Ordering::Relaxed),
            encoded_bytes_written: self.write_bytes.load(Ordering::Relaxed),
            index_bytes_read: self.index_bytes.load(Ordering::Relaxed),
            index_rebuilds: self.rebuilds.load(Ordering::Relaxed),
            payload_bytes_written: self.payload_writes.load(Ordering::Relaxed),
            structural_bytes_written: self.structural_writes.load(Ordering::Relaxed),
        }
    }
}

fn number(s: &str) -> Result<u64, Error> {
    if s.is_empty()
        || s.len() > 20
        || (s.len() > 1 && s.starts_with('0'))
        || !s.bytes().all(|c| c.is_ascii_digit())
    {
        return Err(integrity("invalid canonical decimal"));
    }
    s.parse().map_err(|_| integrity("decimal out of range"))
}

#[cfg(test)]
fn object_backend(_name: &str) -> (crate::test_support::Sandbox, FileBackend) {
    let sandbox = crate::test_support::Sandbox::new();
    let backend =
        FileBackend::new_shared_ontology(&sandbox.store, FileOptions::local_ext4()).unwrap();
    for dir in ["objects/loose", "objects/packs", "staging"] {
        fs::create_dir_all(sandbox.store.join(dir)).unwrap();
    }
    (sandbox, backend)
}
