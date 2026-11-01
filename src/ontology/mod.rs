//! Complete, schema-governed System Definition Ontology and model API (opt-in files/2).
mod api;
mod catalogue;
pub(crate) mod impact;
pub(crate) mod navigation;
pub(crate) mod neighbours;
mod types;
mod validation;
pub use api::OntologyHub;
pub(crate) use catalogue::meta_profile;
pub use catalogue::{
    Catalogue, builtin_catalogue, ontology_ref, profile_ref, relationship_ref, self_profile_ref,
    type_ref,
};
pub use impact::*;
pub use neighbours::*;
pub use types::*;
pub(crate) use validation::history_valid;
pub use validation::{assess, catalogue_coverage};
