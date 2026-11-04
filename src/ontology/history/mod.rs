//! Current-authority publication reads. Historical attribution never grants authority.
//! `operation_result` remains actor-bound mutation recovery, independently of these reads.
//! Every call requires scoped `Read` plus `ReadHistory`. Hosts with exhaustive `Action`
//! matches must add the new action. Default policies deny it; all-allow policies grant it.
//!
//! ```no_run
//! # async fn example(hub: knowledgehub::KnowledgeHub, context: knowledgehub::Context) -> Result<(), knowledgehub::Error> {
//! use knowledgehub::{Publication, PublicationLookup};
//! let anchor = hub.head(context.clone()).await?;
//! let result = hub.publication(context, Publication::new(anchor, "handover:create")).await?;
//! if let PublicationLookup::Published(header) = result.outcome {
//!     println!("original publisher: {}", header.actor);
//! }
//! # Ok(()) }
//! ```
mod api;
mod types;
pub use types::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Position {
    pub next: Option<crate::SnapshotId>,
    pub offset: u64,
}
pub(crate) struct Slice {
    pub page: HistoryPage,
    pub next: Option<Position>,
}
#[derive(Clone)]
pub(crate) struct HistoryCursor {
    pub position: Position,
}
