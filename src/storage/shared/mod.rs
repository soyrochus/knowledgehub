//! Shared immutable files/4 adapter. The model remains independent of these codecs.
use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

mod content;
mod generation;
pub(super) mod history;
mod maintenance;
mod neighbours;
mod objects;
mod packed_cache;
mod projection;
#[cfg(test)]
use crate::test_specimens as specimens;
#[cfg(test)]
mod tests;
mod text;
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
    pub state_materialisations: u64,
    pub neighbour_cache_failures: u64,
    #[serde(default)]
    pub neighbour_cache_peak_allocated_bytes: u64,
    #[serde(default)]
    pub text_cache_peak_allocated_bytes: u64,
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
    locations: Arc<std::sync::Mutex<Option<objects::LocationCache>>>,
    neighbour_locations: Arc<std::sync::Mutex<Option<objects::LocationCache>>>,
    text_locations: Arc<std::sync::Mutex<Option<objects::LocationCache>>>,
    payload_writes: AtomicU64,
    structural_writes: AtomicU64,
    materialisations: AtomicU64,
    neighbour_failures: AtomicU64,
    allocation_limit: AtomicU64,
    peak_allocated: Arc<AtomicU64>,
    neighbour_peak: Arc<AtomicU64>,
    text_peak: Arc<AtomicU64>,
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
            state_materialisations: self.materialisations.load(Ordering::Relaxed),
            neighbour_cache_failures: self.neighbour_failures.load(Ordering::Relaxed),
            neighbour_cache_peak_allocated_bytes: self.neighbour_peak.load(Ordering::Relaxed),
            text_cache_peak_allocated_bytes: self.text_peak.load(Ordering::Relaxed),
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

fn reserve_file(store: &FileStore, bytes: usize) -> Result<(), Error> {
    if let Some(budget) = &store.loose_budget {
        let allocated = (bytes as u64).div_ceil(4096) * 4096;
        let mut remaining = budget.load(Ordering::Relaxed);
        loop {
            let next = remaining.checked_sub(allocated).ok_or(Error::Capacity)?;
            match budget.compare_exchange_weak(
                remaining,
                next,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => remaining = actual,
            }
        }
        let limit = store.work.allocation_limit.load(Ordering::Relaxed);
        store.work.peak_allocated.fetch_max(
            limit.saturating_sub(budget.load(Ordering::Relaxed)),
            Ordering::Relaxed,
        );
    }
    Ok(())
}

#[cfg(test)]
fn object_backend(_name: &str) -> (crate::test_support::Sandbox, FileBackend) {
    let sandbox = crate::test_support::Sandbox::new();
    let backend = FileBackend::new(&sandbox.store, FileOptions::native_local().unwrap()).unwrap();
    for dir in ["objects/loose", "objects/packs", "staging"] {
        fs::create_dir_all(sandbox.store.join(dir)).unwrap();
    }
    (sandbox, backend)
}
