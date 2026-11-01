use crate::{ModelScope, codec};
use serde::{Deserialize, Serialize};
use std::{fmt, sync::Arc};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SnapshotId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContentRef {
    pub sha256: String,
    pub length: u64,
}

impl ContentRef {
    pub fn of(bytes: &[u8]) -> Self {
        Self {
            sha256: codec::digest(bytes),
            length: bytes.len() as u64,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    pub reference: ContentRef,
    pub bytes: Vec<u8>,
}

impl Content {
    pub fn capture(bytes: Vec<u8>) -> Self {
        Self {
            reference: ContentRef::of(&bytes),
            bytes,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    RegisterOntology,
    Create,
    Open,
    Read,
    Validate,
    Commit,
    OperationResult,
}

#[derive(Clone, Copy, Debug)]
pub struct PolicyFailure;

/// Host authentication is trusted. Modelled roles never confer runtime permission.
/// Returning an error fails closed; an implementation that overrides nothing denies everything.
pub trait Policy: Send + Sync + 'static {
    fn allows(
        &self,
        _actor: &str,
        _scope: &ModelScope,
        _action: Action,
    ) -> Result<bool, PolicyFailure> {
        Ok(false)
    }
}

#[derive(Default)]
pub struct DenyAll;
impl Policy for DenyAll {}

#[derive(Clone)]
pub struct Context {
    pub actor: String,
    pub policy: Arc<dyn Policy>,
}

impl Context {
    pub fn new(actor: impl Into<String>, policy: Arc<dyn Policy>) -> Self {
        Self {
            actor: actor.into(),
            policy,
        }
    }
    pub(crate) fn check(&self, scope: &ModelScope, action: Action) -> Result<(), Error> {
        if self.actor.trim().is_empty() || !self.actor.contains(':') {
            return Err(Error::InvalidInput("qualified actor ID required".into()));
        }
        match self.policy.allows(&self.actor, scope, action) {
            Ok(true) => Ok(()),
            Ok(false) => Err(Error::Denied { action }),
            Err(_) => Err(Error::Unavailable("host policy evaluation failed".into())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub scope: ModelScope,
    pub operation_id: String,
    pub actor: String,
    pub fingerprint_version: String,
    pub fingerprint: String,
    pub previous_head: SnapshotId,
    pub head: SnapshotId,
    pub sequence: u64,
    pub affected: Vec<crate::ontology::ExactRef>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OperationOutcome {
    Committed(Box<Receipt>),
    Absent { recovered_head: SnapshotId },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Violation {
    pub rule: String,
    pub subject: String,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    IndexNotReady {
        reason: String,
    },
    Denied {
        action: Action,
    },
    Unavailable(String),
    Capacity,
    Busy,
    Incompatible(String),
    InvalidInput(String),
    Invalid(Vec<Violation>),
    Conflict {
        expected: SnapshotId,
        actual: SnapshotId,
    },
    GuardConflict {
        id: String,
    },
    OperationReuse,
    NotFound,
    InvalidCursor,
    AlreadyExists,
    Integrity(String),
    Storage {
        stage: String,
    },
    Uncertain {
        operation_id: Option<String>,
        stage: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
