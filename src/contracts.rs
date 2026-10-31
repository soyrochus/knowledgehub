use crate::codec;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt, sync::Arc};

pub const PROFILE_VERSION: &str = "knowledgehub-seed/1";
pub const FILE_VERSION: &str = "knowledgehub-files/1";
pub const FINGERPRINT_VERSION: &str = "knowledgehub-request/1";

/// An opaque semantic ID and exact immutable revision. IDs are qualified by the enclosing scope.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RevisionRef {
    pub id: String,
    pub revision: String,
}

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

/// The host supplies this exact bootstrap vocabulary. Arbitrary profile registration is deferred.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub version: String,
    pub ontology: RevisionRef,
    pub definitions: Vec<Definition>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Definition {
    pub reference: RevisionRef,
    pub kind: String,
    pub rules: Vec<String>,
}

impl Profile {
    pub(crate) fn meta() -> Self {
        crate::ontology::meta_profile()
    }
    pub fn seed(ontology: RevisionRef) -> Self {
        let definitions = [
            (
                "Requirement",
                "element",
                vec![
                    "title:document:one:nonempty",
                    "statement:document:one:nonempty",
                    "interpretation:one:authored",
                ],
            ),
            (
                "UseCase",
                "element",
                vec![
                    "title:document:one:nonempty",
                    "actor:document:one:nonempty",
                    "trigger:document:one:nonempty",
                    "outcome:document:one:nonempty",
                    "interpretation:one:authored",
                ],
            ),
            (
                "satisfies",
                "relationship",
                vec![
                    "from:UseCase:one:exact",
                    "to:Requirement:one:exact",
                    "endpoints:same-model-snapshot",
                    "interpretation:one:authored",
                    "elements:zero-or-more-assertions",
                ],
            ),
            (
                "seed-integrity",
                "constraint",
                vec![
                    "identity:unique-per-scope",
                    "revisions:immutable:sha256-tagged-canonical-record/1",
                    "narrative:one-document-fragment-owner",
                    "semantic-fields:graph-owned",
                    "source:exact-utf8-byte-range-and-quote",
                    "unknown-normative-fields:reject",
                    "complete-seed:Requirement-UseCase-satisfies",
                    "source:ontology-section-21.4",
                ],
            ),
        ]
        .into_iter()
        .map(|(name, kind, rules)| {
            let rules: Vec<String> = rules.into_iter().map(str::to_owned).collect();
            let revision = codec::hash_value("knowledgehub-definition/1", &(name, kind, &rules))
                .expect("fixed definition encoding");
            Definition {
                reference: RevisionRef {
                    id: format!("knowledgehub-seed:{name}"),
                    revision,
                },
                kind: kind.into(),
                rules,
            }
        })
        .collect();
        Self {
            version: PROFILE_VERSION.into(),
            ontology,
            definitions,
        }
    }

    pub fn definition(&self, name: &str) -> Option<RevisionRef> {
        self.definitions
            .iter()
            .find(|d| d.reference.id == format!("knowledgehub-seed:{name}"))
            .map(|d| d.reference.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scope {
    pub hub: String,
    pub model: String,
    pub branch: String,
    pub owner: String,
    pub profile: Profile,
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
    fn allows(&self, _actor: &str, _scope: &Scope, _action: Action) -> Result<bool, PolicyFailure> {
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
    pub(crate) fn check(&self, scope: &Scope, action: Action) -> Result<(), Error> {
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

/// Byte offsets are UTF-8 boundaries in one exact source capture.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Anchor {
    pub id: String,
    pub start: u64,
    pub end: u64,
    pub quote: String,
    pub section: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub reference: RevisionRef,
    pub content: Content,
    pub anchors: Vec<Anchor>,
}

impl Source {
    pub fn capture(id: impl Into<String>, bytes: Vec<u8>, anchors: Vec<Anchor>) -> Self {
        let content = Content::capture(bytes);
        Self {
            reference: RevisionRef {
                id: id.into(),
                revision: content.reference.sha256.clone(),
            },
            content,
            anchors,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fragment {
    pub id: String,
    pub start: u64,
    pub end: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Document {
    pub reference: RevisionRef,
    pub content: Content,
    pub fragments: Vec<Fragment>,
}

impl Document {
    pub fn new(
        id: impl Into<String>,
        bytes: Vec<u8>,
        fragments: Vec<Fragment>,
    ) -> Result<Self, Error> {
        let mut document = Self {
            reference: RevisionRef {
                id: id.into(),
                revision: String::new(),
            },
            content: Content::capture(bytes),
            fragments,
        };
        document.reference.revision = codec::record_revision(&document)?;
        Ok(document)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Interpretation {
    pub reference: RevisionRef,
    pub source: RevisionRef,
    pub anchor: String,
    pub text: String,
    pub interpreter: String,
    pub method: String,
    pub status: InterpretationStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterpretationStatus {
    Authored,
}

impl Interpretation {
    pub fn seal(mut self) -> Result<Self, Error> {
        self.reference.revision = codec::record_revision(&self)?;
        Ok(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FragmentRef {
    pub document: RevisionRef,
    pub fragment: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "representation", content = "binding", deny_unknown_fields)]
pub enum Owner {
    Graph,
    Document(FragmentRef),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityBinding {
    pub field: String,
    pub owner: Owner,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Element {
    pub reference: RevisionRef,
    pub element_type: RevisionRef,
    pub interpretation: RevisionRef,
    pub authority: Vec<AuthorityBinding>,
}

impl Element {
    pub fn seal(mut self) -> Result<Self, Error> {
        self.reference.revision = codec::record_revision(&self)?;
        Ok(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assertion {
    pub reference: RevisionRef,
    pub relationship_type: RevisionRef,
    pub from: RevisionRef,
    pub to: RevisionRef,
    pub interpretation: RevisionRef,
    pub authority: Vec<AuthorityBinding>,
}

impl Assertion {
    pub fn seal(mut self) -> Result<Self, Error> {
        self.reference.revision = codec::record_revision(&self)?;
        Ok(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", content = "record", deny_unknown_fields)]
pub enum Operation {
    CaptureSource(Source),
    AddDocument(Document),
    Interpret(Interpretation),
    Define(Element),
    Assert(Assertion),
}

impl Operation {
    pub fn reference(&self) -> &RevisionRef {
        match self {
            Self::CaptureSource(r) => &r.reference,
            Self::AddDocument(r) => &r.reference,
            Self::Interpret(r) => &r.reference,
            Self::Define(r) => &r.reference,
            Self::Assert(r) => &r.reference,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RevisionGuard {
    pub id: String,
    /// None means this identity must be absent.
    pub expected: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChangeSet {
    pub scope: Scope,
    pub operation_id: String,
    pub expected_head: SnapshotId,
    pub guards: Vec<RevisionGuard>,
    /// Order is part of intent. References are resolved against the complete candidate.
    pub operations: Vec<Operation>,
}

impl ChangeSet {
    /// Strict typed parsing: duplicate keys, unknown fields, invalid Unicode and unsupported numbers fail.
    pub fn from_json(bytes: &[u8]) -> Result<Self, Error> {
        codec::decode(bytes)
    }
    pub fn fingerprint(&self, actor: &str) -> Result<String, Error> {
        for op in &self.operations {
            let content = match op {
                Operation::CaptureSource(s) => Some(&s.content),
                Operation::AddDocument(d) => Some(&d.content),
                _ => None,
            };
            if let Some(c) = content {
                if ContentRef::of(&c.bytes) != c.reference {
                    return Err(Error::InvalidInput(
                        "content digest or length mismatch".into(),
                    ));
                }
            }
        }
        let mut value = serde_json::to_value(self).map_err(codec::encoding)?;
        codec::omit_content_bytes(&mut value);
        codec::hash_value(FINGERPRINT_VERSION, &(actor, value))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub id: SnapshotId,
    pub scope: Scope,
    pub sequence: u64,
    pub sources: Vec<Source>,
    pub documents: Vec<Document>,
    pub interpretations: Vec<Interpretation>,
    pub elements: Vec<Element>,
    pub assertions: Vec<Assertion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ontology: Option<Box<crate::ontology::State>>,
}

impl Snapshot {
    pub fn seed_complete(&self) -> bool {
        !self.assertions.is_empty()
    }
    pub fn revisions(&self) -> BTreeMap<String, String> {
        self.sources
            .iter()
            .map(|r| &r.reference)
            .chain(self.documents.iter().map(|r| &r.reference))
            .chain(self.interpretations.iter().map(|r| &r.reference))
            .chain(self.elements.iter().map(|r| &r.reference))
            .chain(self.assertions.iter().map(|r| &r.reference))
            .map(|r| (r.id.clone(), r.revision.clone()))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub scope: Scope,
    pub operation_id: String,
    pub actor: String,
    pub fingerprint_version: String,
    pub fingerprint: String,
    pub previous_head: SnapshotId,
    pub head: SnapshotId,
    pub sequence: u64,
    pub affected: Vec<RevisionRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub qualified_affected: Vec<crate::ontology::ExactRef>,
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
pub struct ValidationReport {
    pub base: SnapshotId,
    pub violations: Vec<Violation>,
}

impl ValidationReport {
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }
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
