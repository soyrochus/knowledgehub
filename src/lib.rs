#![doc = include_str!("../README.md")]
mod application;
mod codec;
mod contracts;
mod jobs;
mod model;
pub mod ontology;
mod storage;

pub use application::KnowledgeHub;
pub use contracts::*;

#[cfg(test)]
extern crate self as knowledgehub;
#[cfg(test)]
#[path = "../tests/internal/faults.rs"]
mod fault_tests;
#[cfg(test)]
#[path = "../examples/support/mod.rs"]
mod seed;

/// Host composition for the default local file adapter.
pub mod backends {
    pub mod files {
        pub use crate::storage::{FileBackend, FileOptions, FileSupport, JobLimits};
    }
}

#[cfg(test)]
#[path = "../tests/internal/ontology_faults.rs"]
mod ontology_fault_tests;

#[cfg(test)]
#[path = "../tests/support/mod.rs"]
mod test_support;
