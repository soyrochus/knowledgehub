use crate::{Error, ExactRef, ModelScope, Receipt, SnapshotId};
use serde::{Deserialize, Serialize};

pub const HISTORY_FORMAT: &str = "knowledgehub-history/1";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryResource {
    Items,
    Nodes,
    StorageBytes,
    OutputBytes,
}

/// Per response/page limits. Positive values are validated, never clamped.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryLimits {
    pub items: u64,
    pub nodes: u64,
    pub storage_bytes: u64,
    pub output_bytes: u64,
}
impl Default for HistoryLimits {
    fn default() -> Self {
        Self {
            items: 100,
            nodes: 256,
            storage_bytes: 16 * 1024 * 1024,
            output_bytes: 256 * 1024,
        }
    }
}
impl HistoryLimits {
    pub const MAX: Self = Self {
        items: 1000,
        nodes: 8192,
        storage_bytes: 128 * 1024 * 1024,
        output_bytes: 4 * 1024 * 1024,
    };
    pub(crate) fn validate(&self) -> Result<(), Error> {
        for (value, max) in [
            (self.items, Self::MAX.items),
            (self.nodes, Self::MAX.nodes),
            (self.storage_bytes, Self::MAX.storage_bytes),
            (self.output_bytes, Self::MAX.output_bytes),
        ] {
            if value == 0 || value > max {
                return Err(Error::InvalidInput(
                    "history limits must be positive and within documented maxima".into(),
                ));
            }
        }
        Ok(())
    }
}

/// Logical verification visits include cache hits. Storage bytes are encoded reads PLUS
/// decoded bytes (raw objects included). Metadata/index/pack preparation is included.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryWork {
    pub visited_nodes: u64,
    pub encoded_bytes_read: u64,
    pub decoded_bytes: u64,
    pub storage_bytes: u64,
    pub object_reads: u64,
    pub output_bytes: u64,
}

/// All original receipt scalars, unchanged. This projection is not a truncated receipt.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationHeader {
    pub scope: ModelScope,
    pub operation_id: String,
    pub actor: String,
    pub fingerprint_version: String,
    pub fingerprint: String,
    pub previous_head: SnapshotId,
    pub head: SnapshotId,
    pub sequence: u64,
    pub affected_count: u64,
}
impl PublicationHeader {
    /// Reconstruct only after collecting every affected page in retained order.
    pub fn with_affected(self, affected: Vec<ExactRef>) -> Result<Receipt, Error> {
        if affected.len() as u64 != self.affected_count || affected.iter().any(|r| !r.is_valid()) {
            return Err(Error::InvalidInput(
                "complete exact affected sequence required".into(),
            ));
        }
        Ok(Receipt {
            scope: self.scope,
            operation_id: self.operation_id,
            actor: self.actor,
            fingerprint_version: self.fingerprint_version,
            fingerprint: self.fingerprint,
            previous_head: self.previous_head,
            head: self.head,
            sequence: self.sequence,
            affected,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    pub anchor: SnapshotId,
    pub operation_id: String,
    pub limits: HistoryLimits,
}
impl Publication {
    pub fn new(anchor: SnapshotId, operation_id: impl Into<String>) -> Self {
        Self {
            anchor,
            operation_id: operation_id.into(),
            limits: HistoryLimits::default(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publications {
    pub anchor: SnapshotId,
    pub limits: HistoryLimits,
}
impl Publications {
    pub fn new(anchor: SnapshotId) -> Self {
        Self {
            anchor,
            limits: HistoryLimits::default(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationAffected {
    pub anchor: SnapshotId,
    pub operation_id: String,
    pub limits: HistoryLimits,
}
impl PublicationAffected {
    pub fn new(anchor: SnapshotId, operation_id: impl Into<String>) -> Self {
        Self {
            anchor,
            operation_id: operation_id.into(),
            limits: HistoryLimits::default(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicationLookup {
    Published(Box<PublicationHeader>),
    /// Only non-membership in the verified anchor ancestry. Never current operation absence.
    NotPublishedAt,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationResult {
    pub format: String,
    pub scope: ModelScope,
    pub anchor: SnapshotId,
    pub operation_id: String,
    pub outcome: PublicationLookup,
    pub work: HistoryWork,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryRequest {
    Publications(Publications),
    Affected(PublicationAffected),
}
impl HistoryRequest {
    pub fn anchor(&self) -> &SnapshotId {
        match self {
            Self::Publications(q) => &q.anchor,
            Self::Affected(q) => &q.anchor,
        }
    }
    pub fn limits(&self) -> &HistoryLimits {
        match self {
            Self::Publications(q) => &q.limits,
            Self::Affected(q) => &q.limits,
        }
    }
    pub(crate) fn validate(&self) -> Result<(), Error> {
        validate_anchor(self.anchor())?;
        self.limits().validate()?;
        if let Self::Affected(q) = self {
            validate_id(&q.operation_id)?;
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryItems {
    /// Descending native sequence; no timestamps or review/execution claims.
    Publications(Vec<PublicationHeader>),
    /// Original vector order, including repetitions. Missing operation is explicit.
    Affected {
        operation_id: String,
        published: bool,
        total: u64,
        offset: u64,
        references: Vec<ExactRef>,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryPage {
    pub format: String,
    pub scope: ModelScope,
    pub anchor: SnapshotId,
    pub items: HistoryItems,
    pub complete: bool,
    /// Initial no-receipt snapshot, present only on completed publication discovery.
    pub initial_boundary: Option<SnapshotId>,
    pub stopped_by: Option<HistoryResource>,
    pub continuation: Option<String>,
    pub work: HistoryWork,
}
pub(crate) fn validate_anchor(anchor: &SnapshotId) -> Result<(), Error> {
    if !crate::model::hash(&anchor.0) {
        return Err(Error::InvalidInput(
            "exact history snapshot required".into(),
        ));
    }
    Ok(())
}
pub(crate) fn validate_id(id: &str) -> Result<(), Error> {
    if id.trim().is_empty() {
        return Err(Error::InvalidInput("history operation ID required".into()));
    }
    Ok(())
}
/// Count the actual serde JSON response, including its own byte counter and token.
pub(crate) fn output<T: Serialize>(value: &T) -> Result<u64, Error> {
    struct Counter(u64);
    impl std::io::Write for Counter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0 = self
                .0
                .checked_add(bytes.len() as u64)
                .ok_or_else(|| std::io::Error::other("history output overflow"))?;
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut c = Counter(0);
    serde_json::to_writer(&mut c, value).map_err(crate::codec::encoding)?;
    Ok(c.0)
}
