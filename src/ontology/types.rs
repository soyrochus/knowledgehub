use crate::{Content, ContentRef, Error, SnapshotId, Violation, codec};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const KERNEL_KINDS: &[&str] = &[
    "KnowledgeHub",
    "SystemModel",
    "OntologyDefinition",
    "ElementTypeDefinition",
    "RelationshipTypeDefinition",
    "ConstraintDefinition",
    "ProfileDefinition",
    "ElementIdentity",
    "ElementRevision",
    "RelationshipAssertion",
    "SourceArtifact",
    "SourceRevision",
    "SourceAnchor",
    "Interpretation",
    "ModelBaseline",
    "RepresentationBinding",
    "ReviewDecision",
];

pub const FILE_FORMAT: &str = "knowledgehub-files/2";
pub const SHARED_FILE_FORMAT: &str = "knowledgehub-files/3";
pub const REQUEST_FORMAT: &str = "knowledgehub-request/2";
pub const RECORD_FORMAT: &str = "knowledgehub-record/2";

/// Qualified semantic identity and opaque immutable revision token. Content hashes are separate.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExactRef {
    pub model: String,
    pub id: String,
    pub revision: String,
}
impl ExactRef {
    pub fn new(
        model: impl Into<String>,
        id: impl Into<String>,
        revision: impl Into<String>,
    ) -> Self {
        Self {
            model: model.into(),
            id: id.into(),
            revision: revision.into(),
        }
    }
    pub fn identity(&self) -> String {
        format!("{}\0{}", self.model, self.id)
    }
    pub fn key(&self) -> String {
        format!("{}\0{}\0{}", self.model, self.id, self.revision)
    }
    pub fn is_valid(&self) -> bool {
        [&self.model, &self.id, &self.revision]
            .iter()
            .all(|s| !s.trim().is_empty() && !s.contains('\0'))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelScope {
    pub hub: String,
    pub model: String,
    pub branch: String,
    pub owner: String,
}
impl ModelScope {
    pub(crate) fn transport(&self) -> crate::Scope {
        crate::Scope {
            hub: self.hub.clone(),
            model: self.model.clone(),
            branch: self.branch.clone(),
            owner: self.owner.clone(),
            profile: crate::Profile::meta(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", deny_unknown_fields)]
pub enum Value {
    Null,
    Text(String),
    Bool(bool),
    Integer(String),
    Decimal {
        coefficient: String,
        exponent: String,
    },
    Enum(String),
    Ref(ExactRef),
    Artifact(ExactRef),
    Narrative {
        document: ExactRef,
        fragment: ExactRef,
    },
    List(Vec<Value>),
    Set(Vec<Value>),
    Record(BTreeMap<String, Value>),
}
impl Value {
    pub fn text(s: impl Into<String>) -> Self {
        Self::Text(s.into())
    }
    pub fn enumeration(s: impl Into<String>) -> Self {
        Self::Enum(s.into())
    }
    pub fn reference(r: ExactRef) -> Self {
        Self::Ref(r)
    }
    pub fn list(v: impl IntoIterator<Item = Value>) -> Self {
        Self::List(v.into_iter().collect())
    }
    pub fn as_ref(&self) -> Option<&ExactRef> {
        if let Self::Ref(r) | Self::Artifact(r) = self {
            Some(r)
        } else {
            None
        }
    }
    pub fn as_text(&self) -> Option<&str> {
        if let Self::Text(s) | Self::Enum(s) | Self::Integer(s) = self {
            Some(s)
        } else {
            None
        }
    }
    pub fn refs(&self) -> Vec<&ExactRef> {
        match self {
            Self::Ref(r) | Self::Artifact(r) => vec![r],
            Self::Narrative { document, fragment } => vec![document, fragment],
            Self::List(v) | Self::Set(v) => v.iter().flat_map(Self::refs).collect(),
            Self::Record(v) => v.values().flat_map(Self::refs).collect(),
            _ => vec![],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "schema", deny_unknown_fields)]
pub enum ValueKind {
    Text,
    Bool,
    Integer,
    Decimal,
    Timestamp,
    Enum(Vec<String>),
    Reference(Vec<String>),
    Artifact,
    Narrative,
    List { item: Box<ValueKind>, min: usize },
    Set { item: Box<ValueKind>, min: usize },
    Record(BTreeMap<String, Property>),
    Structured(String),
    Union(Vec<ValueKind>),
    Any,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyOwner {
    Graph,
    Document,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Property {
    pub value: ValueKind,
    pub required: bool,
    pub nullable: bool,
    pub owner: PropertyOwner,
}
impl Property {
    pub fn required(value: ValueKind) -> Self {
        Self {
            owner: if value == ValueKind::Narrative {
                PropertyOwner::Document
            } else {
                PropertyOwner::Graph
            },
            value,
            required: true,
            nullable: false,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Citation {
    pub original: ContentRef,
    pub section: String,
    pub source_status: String,
    pub refinement: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypeDefinition {
    pub name: String,
    pub family: String,
    pub coverage: String,
    pub native: bool,
    pub parents: Vec<ExactRef>,
    pub properties: BTreeMap<String, Property>,
    pub constraints: Vec<ExactRef>,
    pub citation: Citation,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelationshipDefinition {
    pub name: String,
    pub coverage: String,
    pub from: Vec<String>,
    pub to: Vec<String>,
    pub min_complete: usize,
    pub max: Option<usize>,
    pub acyclic: bool,
    pub qualifiers: BTreeMap<String, Property>,
    pub citation: Citation,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "rule", content = "parameters", deny_unknown_fields)]
pub enum Constraint {
    Required(Vec<String>),
    Unique {
        property: String,
    },
    Acyclic {
        property: String,
    },
    SameOwner {
        property: String,
        owner: String,
    },
    Cardinality {
        property: String,
        min: usize,
        max: Option<usize>,
    },
    Range {
        property: String,
        min: String,
        max: String,
    },
    All(Vec<Constraint>),
    Any(Vec<Constraint>),
    Builtin(String),
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConstraintDefinition {
    pub name: String,
    pub coverage: String,
    pub rule: Constraint,
    pub citation: Citation,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileDefinition {
    pub name: String,
    pub ontology: ExactRef,
    pub vocabulary: Vec<ExactRef>,
    pub allow_gaps: bool,
    pub required_types: Vec<String>,
    pub inapplicable: BTreeMap<String, String>,
    pub citation: Citation,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OntologyDefinition {
    pub name: String,
    pub version: String,
    pub types: Vec<ExactRef>,
    pub relationships: Vec<ExactRef>,
    pub constraints: Vec<ExactRef>,
    pub profiles: Vec<ExactRef>,
    pub structures: BTreeMap<String, ValueKind>,
    pub citation: Citation,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gap {
    pub subject: ExactRef,
    pub path: String,
    pub reason: String,
    pub provenance: Vec<ExactRef>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ownership {
    pub path: String,
    pub owner: PropertyOwner,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ElementData {
    pub properties: BTreeMap<String, Value>,
    pub owners: Vec<Ownership>,
}
pub(crate) fn pointer_segment(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}
impl ElementData {
    pub fn new(properties: BTreeMap<String, Value>) -> Self {
        fn walk(path: String, v: &Value, out: &mut Vec<Ownership>) {
            out.push(Ownership {
                path: path.clone(),
                owner: if matches!(v, Value::Narrative { .. }) {
                    PropertyOwner::Document
                } else {
                    PropertyOwner::Graph
                },
            });
            match v {
                Value::Record(m) => {
                    for (k, v) in m {
                        walk(format!("{path}/{}", pointer_segment(k)), v, out)
                    }
                }
                Value::List(a) | Value::Set(a) => {
                    for (i, v) in a.iter().enumerate() {
                        walk(format!("{path}/{i}"), v, out)
                    }
                }
                _ => {}
            }
        }
        let mut owners = vec![];
        for (k, v) in &properties {
            walk(format!("/{}", pointer_segment(k)), v, &mut owners)
        }
        Self { properties, owners }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssertionData {
    pub signature: ExactRef,
    pub from: ExactRef,
    pub to: ExactRef,
    pub qualifiers: BTreeMap<String, Value>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceArtifactData {
    pub origin: String,
    pub media_type: String,
    pub owner: String,
    pub status: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRevisionData {
    pub source: ExactRef,
    pub original: ContentRef,
    pub media_type: String,
    pub derived_from: Option<ExactRef>,
    pub derivation: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocatorKind {
    Section,
    RequirementId,
    TableRow,
    JsonPointer,
    CodeSymbol,
    UiElement,
    ImageRegion,
    TimeRange,
    PdfPage,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Locator {
    ByteRange {
        start: u64,
        end: u64,
        quote: String,
    },
    Manifest {
        locator: LocatorKind,
        manifest: ExactRef,
        entry: String,
        quote: Option<String>,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnchorData {
    pub source: ExactRef,
    pub label: String,
    pub locator: Locator,
}
/// A host's source-bound extraction observation, retained verbatim as an artifact.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocatorManifest {
    pub source: ExactRef,
    pub original: ContentRef,
    pub tool: String,
    pub version: String,
    pub entries: BTreeMap<String, LocatorEntry>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocatorEntry {
    pub kind: LocatorKind,
    pub label: String,
    pub location: SourceLocation,
    pub derived: ExactRef,
    pub start: u64,
    pub end: u64,
}
/// Original-source coordinates are typed independently from the derived UTF-8 quotation span.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum SourceLocation {
    Section {
        path: Vec<String>,
    },
    RequirementId {
        id: String,
    },
    TableRow {
        table: String,
        row: String,
    },
    JsonPointer {
        pointer: String,
    },
    CodeSymbol {
        language: String,
        symbol: String,
    },
    UiElement {
        screen: String,
        element: String,
    },
    ImageRegion {
        x: u64,
        y: u64,
        width: u64,
        height: u64,
        image_width: u64,
        image_height: u64,
    },
    TimeRange {
        start_ms: u64,
        end_ms: u64,
        duration_ms: u64,
    },
    PdfPage {
        page: u64,
        pages: u64,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InterpretationData {
    pub subject: ExactRef,
    pub property: Option<String>,
    pub anchor: ExactRef,
    pub text: String,
    pub interpreter: String,
    pub method: String,
    pub review: Option<ExactRef>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepresentationData {
    pub subject: ExactRef,
    pub anchor: ExactRef,
    pub role: RepresentationRole,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepresentationRole {
    Authoritative,
    OriginalEvidence,
    Implementation,
    Derived,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BaselineData {
    pub ontology: ExactRef,
    pub profile: ExactRef,
    pub selected: Vec<ExactRef>,
    pub supporting: Vec<ExactRef>,
    pub parents: Vec<ExactRef>,
    pub gaps: Vec<Gap>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelData {
    pub purpose: String,
    pub owner: String,
    pub ontology: ExactRef,
    pub profile: ExactRef,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewData {
    pub assessor: String,
    pub targets: Vec<ExactRef>,
    pub decision: String,
    pub rationale: String,
    pub assessed_at: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data", deny_unknown_fields)]
pub enum RecordData {
    Element(ElementData),
    Assertion(AssertionData),
    Ontology(OntologyDefinition),
    Type(TypeDefinition),
    Relationship(RelationshipDefinition),
    Constraint(ConstraintDefinition),
    Profile(ProfileDefinition),
    Hub {
        owner: String,
        participants: Vec<String>,
        policy_references: Vec<String>,
    },
    Model(ModelData),
    Identity {
        subject: ExactRef,
        declared_type: ExactRef,
    },
    SourceArtifact(SourceArtifactData),
    SourceRevision(SourceRevisionData),
    Anchor(AnchorData),
    Interpretation(InterpretationData),
    Representation(RepresentationData),
    Baseline(BaselineData),
    Review(ReviewData),
}
impl RecordData {
    pub fn native_kind(&self) -> Option<&'static str> {
        Some(match self {
            Self::Element(_) => return None,
            Self::Assertion(_) => "RelationshipAssertion",
            Self::Ontology(_) => "OntologyDefinition",
            Self::Type(_) => "ElementTypeDefinition",
            Self::Relationship(_) => "RelationshipTypeDefinition",
            Self::Constraint(_) => "ConstraintDefinition",
            Self::Profile(_) => "ProfileDefinition",
            Self::Hub { .. } => "KnowledgeHub",
            Self::Model(_) => "SystemModel",
            Self::Identity { .. } => "ElementIdentity",
            Self::SourceArtifact(_) => "SourceArtifact",
            Self::SourceRevision(_) => "SourceRevision",
            Self::Anchor(_) => "SourceAnchor",
            Self::Interpretation(_) => "Interpretation",
            Self::Representation(_) => "RepresentationBinding",
            Self::Baseline(_) => "ModelBaseline",
            Self::Review(_) => "ReviewDecision",
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub reference: ExactRef,
    pub type_ref: ExactRef,
    pub predecessor: Option<ExactRef>,
    pub provenance: Vec<ExactRef>,
    pub data: RecordData,
    pub sha256: String,
}
impl Record {
    pub fn new(
        reference: ExactRef,
        type_ref: ExactRef,
        data: RecordData,
        provenance: Vec<ExactRef>,
    ) -> Result<Self, Error> {
        let mut r = Self {
            reference,
            type_ref,
            predecessor: None,
            provenance,
            data,
            sha256: String::new(),
        };
        r.seal()?;
        Ok(r)
    }
    pub fn seal(&mut self) -> Result<(), Error> {
        self.sha256 = self.digest()?;
        Ok(())
    }
    pub fn digest(&self) -> Result<String, Error> {
        let mut v = serde_json::to_value(self).map_err(codec::encoding)?;
        v.as_object_mut().unwrap().remove("sha256");
        codec::hash_value(RECORD_FORMAT, &v)
    }
    pub fn element(&self) -> Option<&BTreeMap<String, Value>> {
        if let RecordData::Element(e) = &self.data {
            Some(&e.properties)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub scope: ModelScope,
    pub ontology: Option<ExactRef>,
    pub profile: Option<ExactRef>,
    pub records: BTreeMap<String, Record>,
    pub selected: BTreeMap<String, ExactRef>,
    pub artifacts: BTreeMap<String, Content>,
    pub gaps: Vec<Gap>,
}
impl State {
    pub fn empty(scope: ModelScope) -> Self {
        Self {
            scope,
            ontology: None,
            profile: None,
            records: BTreeMap::new(),
            selected: BTreeMap::new(),
            artifacts: BTreeMap::new(),
            gaps: vec![],
        }
    }
    pub fn get(&self, r: &ExactRef) -> Option<&Record> {
        self.records.get(&r.key()).filter(|v| v.reference == *r)
    }
    pub fn current(&self, model: &str, id: &str) -> Option<&Record> {
        self.selected
            .get(&format!("{model}\0{id}"))
            .and_then(|r| self.get(r))
    }
    pub fn selected_records(&self) -> impl Iterator<Item = &Record> {
        self.selected.values().filter_map(|r| self.get(r))
    }
    pub fn kind<'a>(&'a self, r: &ExactRef) -> Option<&'a str> {
        if r.revision == "@identity" && self.resolve(r).is_some() {
            return Some("ElementIdentity");
        }
        let v = self.get(r)?;
        if let Some(k) = v.data.native_kind() {
            Some(k)
        } else {
            self.get(&v.type_ref).and_then(|t| {
                if let RecordData::Type(t) = &t.data {
                    Some(t.name.as_str())
                } else {
                    None
                }
            })
        }
    }
    pub fn artifact(&self, r: &ExactRef) -> Option<&Content> {
        self.artifacts.get(&r.key())
    }
    pub fn type_definition(&self, r: &ExactRef) -> Option<&TypeDefinition> {
        self.get(r).and_then(|r| {
            if let RecordData::Type(t) = &r.data {
                Some(t)
            } else {
                None
            }
        })
    }
    pub fn profile_definition(&self) -> Option<&ProfileDefinition> {
        self.profile
            .as_ref()
            .and_then(|r| self.get(r))
            .and_then(|r| {
                if let RecordData::Profile(p) = &r.data {
                    Some(p)
                } else {
                    None
                }
            })
    }
    pub fn structures(&self) -> Option<&BTreeMap<String, ValueKind>> {
        self.ontology
            .as_ref()
            .and_then(|r| self.get(r))
            .and_then(|r| {
                if let RecordData::Ontology(o) = &r.data {
                    Some(&o.structures)
                } else {
                    None
                }
            })
    }
    pub fn baseline(&self, r: &ExactRef) -> Option<&BaselineData> {
        self.get(r).and_then(|r| {
            if let RecordData::Baseline(b) = &r.data {
                Some(b)
            } else {
                None
            }
        })
    }
    pub fn view(&self, b: &BaselineData) -> Self {
        let mut s = self.clone();
        s.ontology = Some(b.ontology.clone());
        s.profile = Some(b.profile.clone());
        s.selected = b
            .selected
            .iter()
            .map(|r| (r.identity(), r.clone()))
            .collect();
        s.gaps = b.gaps.clone();
        let closure: BTreeSet<_> = b
            .selected
            .iter()
            .chain(&b.supporting)
            .map(ExactRef::key)
            .collect();
        s.records.retain(|k, _| closure.contains(k));
        s.artifacts.retain(|k, _| closure.contains(k));
        s
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Guard {
    pub model: String,
    pub id: String,
    pub expected: Option<ExactRef>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operation", content = "data", deny_unknown_fields)]
pub enum Operation {
    Put(Record),
    Capture {
        record: Record,
        content: Content,
    },
    SelectProfile {
        ontology: ExactRef,
        profile: ExactRef,
    },
    Gaps(Vec<Gap>),
    Unselect(ExactRef),
    Select(ExactRef),
}
impl Operation {
    pub fn record(&self) -> Option<&Record> {
        match self {
            Self::Put(r) | Self::Capture { record: r, .. } => Some(r),
            _ => None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Change {
    pub scope: ModelScope,
    pub operation_id: String,
    pub expected_head: SnapshotId,
    pub expected_ontology: Option<ExactRef>,
    pub expected_profile: Option<ExactRef>,
    pub guards: Vec<Guard>,
    pub operations: Vec<Operation>,
}
impl Change {
    pub fn new(scope: ModelScope, head: SnapshotId, id: impl Into<String>) -> Self {
        Self {
            scope,
            operation_id: id.into(),
            expected_head: head,
            expected_ontology: None,
            expected_profile: None,
            guards: vec![],
            operations: vec![],
        }
    }
    pub fn at(state: &State, head: SnapshotId, id: impl Into<String>) -> Self {
        let mut c = Self::new(state.scope.clone(), head, id);
        c.expected_ontology = state.ontology.clone();
        c.expected_profile = state.profile.clone();
        c
    }
    pub fn from_json(bytes: &[u8]) -> Result<Self, Error> {
        codec::decode(bytes)
    }
    pub fn fingerprint(&self, actor: &str) -> Result<String, Error> {
        for op in &self.operations {
            if let Operation::Capture { content, .. } = op {
                if ContentRef::of(&content.bytes) != content.reference {
                    return Err(Error::InvalidInput(
                        "artifact digest/length mismatch".into(),
                    ));
                }
            }
        }
        let mut v = serde_json::to_value(self).map_err(codec::encoding)?;
        codec::omit_content_bytes(&mut v);
        codec::hash_value(REQUEST_FORMAT, &(actor, v))
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assessment {
    pub violations: Vec<Violation>,
    pub gaps: Vec<Gap>,
}
impl Assessment {
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }
    pub fn is_complete(&self) -> bool {
        self.is_valid() && self.gaps.is_empty()
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Difference {
    pub identity: String,
    pub before: Option<ExactRef>,
    pub after: Option<ExactRef>,
    pub properties_changed: bool,
    pub provenance_changed: bool,
    pub representation_changed: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "selection", content = "reference", deny_unknown_fields)]
pub enum Selection {
    Snapshot(SnapshotId),
    Baseline(ExactRef),
}
/// An owned exact original-content range. Partial chunk reads do not claim a full-byte hash check.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactRange {
    pub selection: Selection,
    pub reference: ExactRef,
    pub original: ContentRef,
    pub offset: u64,
    pub bytes: Vec<u8>,
    pub full_content_verified: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Forward,
    Reverse,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Query {
    pub selection: Selection,
    pub type_ref: Option<ExactRef>,
    pub ids: BTreeSet<String>,
    pub starts: Vec<ExactRef>,
    pub signatures: Vec<ExactRef>,
    pub direction: Direction,
    pub depth: usize,
    pub page_size: usize,
    pub subject_limit: usize,
    pub edge_limit: usize,
}
impl Query {
    pub fn select(selection: Selection) -> Self {
        Self {
            selection,
            type_ref: None,
            ids: BTreeSet::new(),
            starts: vec![],
            signatures: vec![],
            direction: Direction::Forward,
            depth: 1,
            page_size: 100,
            subject_limit: 10000,
            edge_limit: 10000,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page {
    pub selection: Selection,
    pub records: Vec<Record>,
    pub assertions: Vec<Record>,
    pub continuation: Option<String>,
    pub truncated: Option<String>,
    pub exhausted: bool,
}
