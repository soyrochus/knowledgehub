//! Closed deterministic search, traversal and candidate dependency reads.
mod api;
mod engine;
#[cfg(test)]
mod tests;
pub(crate) mod text;
mod types;
pub(crate) use api::ImpactCursor;
pub(crate) use engine::ViewInfo;
pub use types::*;
