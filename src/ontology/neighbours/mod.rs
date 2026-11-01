//! Exact recorded one-hop facts. Extraction is a model responsibility; indexes are derived adapters.
mod api;
pub(crate) mod facts;
mod types;
use super::*;
use crate::Error;
pub(crate) use api::NeighbourCursor;
pub use types::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Position {
    pub group: usize,
    pub outgoing: u64,
    pub incoming: u64,
}
pub(crate) struct Slice {
    pub page: NeighbourPage,
    pub next: Option<Position>,
}

pub(crate) fn validate(q: &Neighbours) -> Result<(), Error> {
    let refs = q
        .starts
        .iter()
        .chain(q.selectors.iter().map(NeighbourSelector::definition))
        .chain(q.opposite_type.iter());
    let mut bytes = 0usize;
    for r in refs {
        for component in [&r.model, &r.id, &r.revision] {
            if component.len() > 4096 {
                return Err(Error::Capacity);
            }
            bytes = bytes.checked_add(component.len()).ok_or(Error::Capacity)?;
        }
    }
    bytes +=
        q.selectors
            .iter()
            .map(|s| match s {
                NeighbourSelector::Assertion { .. } => 0,
                NeighbourSelector::Property { path, .. }
                | NeighbourSelector::Native { path, .. } => path.len(),
            })
            .sum::<usize>();
    if bytes > 1_048_576 {
        return Err(Error::Capacity);
    }
    if q.starts.is_empty()
        || q.starts.len() > 1000
        || q.starts.iter().any(|r| !r.is_valid())
        || q.selectors.len() > 1000
        || q.page_size == 0
        || q.page_size > 1000
        || q.examined_limit == 0
        || q.examined_limit > 10000
        || q.byte_limit == 0
        || q.byte_limit > 1_048_576
        || q.preparation.owner_limit > 100_000
        || q.preparation.fact_limit > 100_000
        || q.preparation.byte_limit > 64 * 1024 * 1024
        || q.selectors.iter().any(|s| !s.valid())
        || q.opposite_type.as_ref().is_some_and(|r| !r.is_valid())
        || (q.include_subtypes && q.opposite_type.is_none())
    {
        return Err(Error::InvalidInput(
            "neighbour request exceeds finite bounds or contains invalid exact filters".into(),
        ));
    }
    Ok(())
}
