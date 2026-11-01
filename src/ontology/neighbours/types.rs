use crate::ontology::{ExactRef, Record, Selection};
use crate::{Error, codec};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Fact owners are selected by default. Closure additionally admits the exact view's supporting owners.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactScope {
    Selected,
    Closure,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum NeighbourDirection {
    Outgoing,
    Incoming,
    Both,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum NeighbourCategory {
    Association,
    Property,
    Representation,
    Provenance,
    Administrative,
}
/// Property definitions pin an element type or the relationship definition declaring a qualifier.
/// Paths are escaped JSON-pointer schema paths; `*` denotes a collection item.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum NeighbourSelector {
    Assertion { definition: ExactRef },
    Property { definition: ExactRef, path: String },
    Native { definition: ExactRef, path: String },
}
impl NeighbourSelector {
    pub(crate) fn definition(&self) -> &ExactRef {
        match self {
            Self::Assertion { definition }
            | Self::Property { definition, .. }
            | Self::Native { definition, .. } => definition,
        }
    }
    pub(crate) fn valid(&self) -> bool {
        self.definition().is_valid()
            && match self {
                Self::Assertion { .. } => true,
                Self::Property { path, .. } | Self::Native { path, .. } => {
                    path.starts_with('/')
                        && path.len() <= 4096
                        && path.as_bytes().iter().enumerate().all(|(i, b)| {
                            *b != b'~' || matches!(path.as_bytes().get(i + 1), Some(b'0' | b'1'))
                        })
                }
            }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NeighbourOrigin {
    pub owner: ExactRef,
    pub owner_type: ExactRef,
    pub occurrence: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeighbourProjection {
    References,
    Records,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NeighbourPreparation {
    pub allow_rebuild: bool,
    pub owner_limit: usize,
    pub fact_limit: usize,
    pub byte_limit: usize,
}
impl Default for NeighbourPreparation {
    fn default() -> Self {
        Self {
            allow_rebuild: true,
            owner_limit: 10000,
            fact_limit: 100000,
            byte_limit: 64 * 1024 * 1024,
        }
    }
}

/// Additive one-hop request. Defaults: 100 results, 10,000 examined candidates, 256 KiB projected data.
/// Results sort by exact starting reference, selector, opposite reference, origin and occurrence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Neighbours {
    pub selection: Selection,
    pub starts: Vec<ExactRef>,
    pub direction: NeighbourDirection,
    pub selectors: BTreeSet<NeighbourSelector>,
    pub categories: BTreeSet<NeighbourCategory>,
    pub opposite_type: Option<ExactRef>,
    pub include_subtypes: bool,
    pub fact_scope: FactScope,
    pub projection: NeighbourProjection,
    pub page_size: usize,
    pub examined_limit: usize,
    pub byte_limit: usize,
    pub preparation: NeighbourPreparation,
}
impl Neighbours {
    pub fn new(selection: Selection, starts: Vec<ExactRef>) -> Self {
        Self {
            selection,
            starts,
            direction: NeighbourDirection::Both,
            selectors: BTreeSet::new(),
            categories: BTreeSet::from([
                NeighbourCategory::Association,
                NeighbourCategory::Property,
            ]),
            opposite_type: None,
            include_subtypes: false,
            fact_scope: FactScope::Selected,
            projection: NeighbourProjection::References,
            page_size: 100,
            examined_limit: 10000,
            byte_limit: 256 * 1024,
            preparation: NeighbourPreparation::default(),
        }
    }
    pub fn from_json(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() > 1_048_576 {
            return Err(Error::Capacity);
        }
        let q = codec::decode(bytes)?;
        super::validate(&q)?;
        Ok(q)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Neighbour {
    pub subject: ExactRef,
    pub selector: NeighbourSelector,
    pub opposite: ExactRef,
    pub origin: NeighbourOrigin,
    pub category: NeighbourCategory,
    pub directions: BTreeSet<NeighbourDirection>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NeighbourVertex {
    pub reference: ExactRef,
    pub available: bool,
    pub selected: bool,
    pub type_ref: Option<ExactRef>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeighbourLimit {
    Page,
    ExaminedPostings,
    ProjectedBytes,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NeighbourWork {
    pub indexed_owners: u64,
    pub indexed_facts: u64,
    pub indexed_postings: u64,
    pub prepared_owners: u64,
    pub prepared_facts: u64,
    pub index_objects_read: u64,
    pub index_bytes_read: u64,
    /// Catalog/sidecar bytes included in index_bytes_read; zero for a healthy warm location map.
    #[serde(default)]
    pub index_metadata_bytes_read: u64,
    pub record_reads: u64,
    pub record_bytes: u64,
    pub examined_postings: u64,
    pub postings_read: u64,
    pub index_rebuilt: bool,
    pub rebuild_reason: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NeighbourPage {
    pub selection: Selection,
    pub ontology: Option<ExactRef>,
    pub profile: Option<ExactRef>,
    pub fact_scope: FactScope,
    pub neighbours: Vec<Neighbour>,
    pub vertices: Vec<NeighbourVertex>,
    pub records: Vec<Record>,
    pub continuation: Option<String>,
    pub reached: Option<NeighbourLimit>,
    pub exhausted: bool,
    pub work: NeighbourWork,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Fact {
    pub from: ExactRef,
    pub selector: NeighbourSelector,
    pub to: ExactRef,
    pub origin: NeighbourOrigin,
    pub category: NeighbourCategory,
}
/// Selector and starting endpoint are supplied by their enclosing maps.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Posting {
    pub opposite: ExactRef,
    pub origin: NeighbourOrigin,
    pub category: NeighbourCategory,
}
impl Posting {
    pub(crate) fn valid(&self) -> bool {
        self.opposite.is_valid()
            && self.origin.owner.is_valid()
            && self.origin.owner_type.is_valid()
            && (self.origin.occurrence.is_empty() || self.origin.occurrence.starts_with('/'))
    }
}
