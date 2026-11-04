//! Request-local accounting on the bounded blocking worker. It includes lease metadata.
use crate::{Error, HistoryLimits, HistoryResource, HistoryWork};
use std::cell::RefCell;

thread_local! { static ACTIVE: RefCell<Option<(HistoryLimits, HistoryWork)>> = const { RefCell::new(None) }; }
pub(super) struct Budget;
impl Budget {
    pub(super) fn start(limits: &HistoryLimits) -> Self {
        ACTIVE.with(|b| {
            assert!(b.borrow().is_none());
            *b.borrow_mut() = Some((limits.clone(), HistoryWork::default()));
        });
        Self
    }
    pub(super) fn work(&self) -> HistoryWork {
        ACTIVE.with(|b| b.borrow().as_ref().unwrap().1.clone())
    }
}
impl Drop for Budget {
    fn drop(&mut self) {
        ACTIVE.with(|b| *b.borrow_mut() = None);
    }
}
pub(in crate::storage::shared) fn active() -> bool {
    ACTIVE.with(|b| b.borrow().is_some())
}
fn charge(resource: HistoryResource, n: u64, decoded: bool, object: bool) -> Result<(), Error> {
    ACTIVE.with(|b| {
        let mut b = b.borrow_mut();
        let Some((limits, work)) = b.as_mut() else {
            return Ok(());
        };
        let (current, limit) = match resource {
            HistoryResource::Nodes => (work.visited_nodes, limits.nodes),
            _ => (work.storage_bytes, limits.storage_bytes),
        };
        let next = current
            .checked_add(n)
            .filter(|v| *v <= limit)
            .ok_or_else(|| Error::HistoryLimit {
                resource,
                limit,
                work: work.clone(),
            })?;
        if resource == HistoryResource::Nodes {
            work.visited_nodes = next;
            if object {
                work.object_reads = work.object_reads.checked_add(1).ok_or(Error::Capacity)?;
            }
        } else {
            work.storage_bytes = next;
            let c = if decoded {
                &mut work.decoded_bytes
            } else {
                &mut work.encoded_bytes_read
            };
            *c = c.checked_add(n).ok_or(Error::Capacity)?;
        }
        Ok(())
    })
}
pub(in crate::storage::shared) fn node() -> Result<(), Error> {
    charge(HistoryResource::Nodes, 1, false, false)
}
pub(in crate::storage::shared) fn object() -> Result<(), Error> {
    charge(HistoryResource::Nodes, 1, false, true)
}
pub(in crate::storage::shared) fn read(n: u64) -> Result<(), Error> {
    charge(HistoryResource::StorageBytes, n, false, false)
}
pub(in crate::storage::shared) fn decoded(n: u64) -> Result<(), Error> {
    charge(HistoryResource::StorageBytes, n, true, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checked_counter_overflow_is_explicit_and_guard_cleans_up() {
        {
            let _guard = Budget::start(&HistoryLimits::MAX);
            ACTIVE.with(|b| b.borrow_mut().as_mut().unwrap().1.storage_bytes = u64::MAX);
            assert!(matches!(
                read(1),
                Err(Error::HistoryLimit {
                    resource: HistoryResource::StorageBytes,
                    ..
                })
            ));
        }
        assert!(!active());
    }
}
