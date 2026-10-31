//! Host authoring and exact source inventory. Uses only the public ontology API.
#![allow(dead_code)]
use knowledgehub::{Content, ContentRef, Error, ontology::*};
use serde::Deserialize;
use std::{collections::BTreeMap, path::Path};
#[derive(Deserialize)]
pub struct InputFile {
    pub path: String,
    pub reference: ContentRef,
    pub media_type: String,
    pub derived_from: Option<String>,
    pub derivation: Option<String>,
}
#[derive(Deserialize)]
pub struct Field {
    pub name: String,
    pub rust_type: String,
}
#[derive(Deserialize)]
pub struct Unit {
    pub path: String,
    pub kind: String,
    pub name: String,
    pub start: u64,
    pub end: u64,
    pub fields: Option<Vec<Field>>,
    pub signature: Option<String>,
    pub asynchronous: Option<bool>,
    pub rust_kind: Option<String>,
    pub value: Option<serde_json::Value>,
    pub has_body: Option<bool>,
    pub parameters: Option<Vec<Field>>,
    pub result: Option<String>,
    pub text: Option<String>,
}
#[derive(Deserialize)]
pub struct PdfPage {
    pub index: u64,
    pub start: u64,
    pub end: u64,
    pub label: String,
}
#[derive(Deserialize)]
pub struct Capture {
    pub captured_at: String,
    pub source_status: String,
    pub compiler: String,
    pub filesystem: String,
    pub kernel: String,
    pub extractor: String,
    pub files: Vec<InputFile>,
    pub units: Vec<Unit>,
    pub pdf_pages: Vec<PdfPage>,
    pub checked: Option<serde_json::Value>,
    pub log: String,
}
pub fn scope() -> ModelScope {
    ModelScope {
        hub: "kh:hub:self-description".into(),
        model: "urn:knowledgehub:library".into(),
        branch: "main".into(),
        owner: "host:maintainer".into(),
    }
}
// Explicit semantic bindings. Update an origin key when a file moves; retain its value.
pub fn module_id(path: &str) -> &'static str {
    match path {
        "src/lib.rs" => "lib",
        "src/application.rs" => "application",
        "src/contracts.rs" => "contracts",
        "src/model.rs" => "model",
        "src/storage.rs" => "storage",
        "src/codec.rs" => "codec",
        "src/jobs.rs" => "jobs",
        "src/ontology/mod.rs" => "ontology",
        "src/ontology/api.rs" => "ontology-application",
        "src/ontology/catalogue.rs" => "ontology-catalogue",
        "src/ontology/navigation.rs" => "ontology-navigation",
        "src/ontology/types.rs" => "ontology-contracts",
        "src/ontology/validation.rs" => "ontology-validation",
        _ => panic!("declare semantic module binding for {path}"),
    }
}
fn namespace(path: &str) -> String {
    match path {
        "src/contracts.rs" | "src/application.rs" => "knowledgehub".into(),
        "src/storage.rs" => "knowledgehub::backends::files".into(),
        "src/ontology/types.rs"
        | "src/ontology/api.rs"
        | "src/ontology/navigation.rs"
        | "src/ontology/catalogue.rs"
        | "src/ontology/validation.rs" => "knowledgehub::ontology".into(),
        _ => format!("knowledgehub::internal::{}", module_id(path)),
    }
}
fn source_key(path: &str) -> String {
    if path.starts_with("src/") {
        return if path == "src/ontology/catalogue.json" {
            "ontology-catalogue-data".into()
        } else {
            format!("library-source:{}", module_id(path))
        };
    }
    if path.starts_with("openspec/changes/") {
        let parts = path.split('/').collect::<Vec<_>>();
        return if parts.get(3) == Some(&"specs") {
            format!("change:{}:specification:{}", parts[2], parts[4])
        } else {
            format!("change:{}:{}", parts[2], parts[3].trim_end_matches(".md"))
        };
    }
    let key = match path {
        "README.md" => "project-overview",
        "Cargo.toml" => "crate-manifest",
        "Cargo.lock" => "dependency-lock",
        "source/system-definition-ontology-proposal.md" => "ontology-original-markdown",
        "source/system-definition-ontology-proposal.pdf" => "ontology-original-pdf",
        "source/practical-design-manual.md" => "design-method-markdown",
        "source/practical-design-manual.pdf" => "design-method-pdf",
        "design/domain-analysis.md" => "domain-analysis",
        "design/self-hosting-plan.md" => "self-hosting-plan",
        "design/full-ontology-implementation.md" => "ontology-implementation-record",
        "design/implementation-record.md" => "seed-implementation-record",
        "design/typed-ontology-api.md" => "typed-api-contract",
        "design/file-backend.md" => "seed-file-contract",
        "design/self-model-consumer.md" => "self-model-consumer-guide",
        "delivery/slice-plan.md" => "slice-plan",
        "examples/catalogue_conformance.rs" => "catalogue-consumer",
        "examples/full_self_description.rs" => "self-model-consumer",
        "examples/inspect_self_model.rs" => "self-model-inspector",
        "examples/seed.rs" => "seed-consumer",
        "examples/support/mod.rs" => "seed-consumer-fixture",
        "examples/ontology_support/mod.rs" => "catalogue-specimens",
        "examples/self_model_support/mod.rs" => "self-model-author",
        "examples/capture_self_model.py" => "host-source-capture",
        "tests/public_contract.rs" => "seed-api-cases",
        "tests/ontology_contract.rs" => "ontology-api-cases",
        "tests/support/mod.rs" => "test-support",
        "tests/internal/faults.rs" => "seed-publication-cases",
        "tests/internal/ontology_faults.rs" => "ontology-publication-cases",
        "delivery/evidence/kh-sl-001-run.txt" => "historical-seed-log",
        "delivery/evidence/full-ontology-start.txt" => "ontology-start-log",
        "delivery/evidence/full-ontology-start.json" => "ontology-start-inventory",
        "delivery/evidence/full-ontology-start.tar.gz" => "ontology-start-snapshot",
        "delivery/evidence/full-ontology-run.txt" => "native-conformance-log",
        "target/ontology-pdf-extracted.txt" => "ontology-pdf-extraction",
        "ontology-pdf-pages.json" => "ontology-pdf-locator-manifest",
        "host-inventory.json" => "host-inventory",
        "urn:document:knowledgehub-seed" => "urn:document:knowledgehub-seed",
        _ => panic!("declare semantic source binding for {path}"),
    };
    key.into()
}
pub fn capture_ref(path: &str) -> ExactRef {
    r(&format!("kh:capture:{}", source_key(path)))
}
pub fn file_anchor_ref(path: &str) -> ExactRef {
    r(&format!("kh:anchor:source:{}", source_key(path)))
}
fn unit_scope(unit: &Unit) -> String {
    if unit.path.starts_with("src/") {
        namespace(&unit.path)
    } else {
        source_key(&unit.path)
    }
}
pub fn r(id: &str) -> ExactRef {
    ExactRef::new("knowledgehub", id, "self-1")
}
pub fn list(values: impl IntoIterator<Item = ExactRef>) -> Value {
    Value::List(values.into_iter().map(Value::Ref).collect())
}
pub fn artifact_list(values: impl IntoIterator<Item = ExactRef>) -> Value {
    Value::List(values.into_iter().map(Value::Artifact).collect())
}
pub fn properties(
    values: impl IntoIterator<Item = (&'static str, Value)>,
) -> BTreeMap<String, Value> {
    values.into_iter().map(|(k, v)| (k.into(), v)).collect()
}
pub fn slug(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '.' {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}
pub struct Builder {
    pub state: State,
    pub operations: Vec<Operation>,
    pub sources: BTreeMap<String, ExactRef>,
    pub anchors: BTreeMap<String, ExactRef>,
    pub groups: BTreeMap<String, Vec<ExactRef>>,
}
impl Builder {
    pub fn new(state: State) -> Self {
        Self {
            state,
            operations: vec![],
            sources: BTreeMap::new(),
            anchors: BTreeMap::new(),
            groups: BTreeMap::new(),
        }
    }
    pub fn add(&mut self, record: Record, content: Option<Content>) {
        if let Some(existing) = self.state.get(&record.reference) {
            assert_eq!(existing, &record, "conflicting host-generated identity");
            return;
        }
        self.state
            .selected
            .insert(record.reference.identity(), record.reference.clone());
        self.state
            .records
            .insert(record.reference.key(), record.clone());
        if let Some(content) = content {
            self.state
                .artifacts
                .insert(record.reference.key(), content.clone());
            self.operations.push(Operation::Capture { record, content });
        } else {
            self.operations.push(Operation::Put(record));
        }
    }
    pub fn native(
        &mut self,
        id: &str,
        kind: &str,
        data: RecordData,
        anchor: Option<ExactRef>,
    ) -> Result<ExactRef, Error> {
        let reference = r(id);
        self.add(
            Record::new(
                reference.clone(),
                type_ref(kind),
                data,
                anchor.into_iter().collect(),
            )?,
            None,
        );
        Ok(reference)
    }
    pub fn anchor(
        &mut self,
        id: &str,
        source: ExactRef,
        start: u64,
        end: u64,
    ) -> Result<ExactRef, Error> {
        let c = self.state.artifact(&source).ok_or(Error::NotFound)?;
        let quote = std::str::from_utf8(&c.bytes)
            .map_err(|_| Error::InvalidInput("anchor source is not UTF-8".into()))?
            .get(start as usize..end as usize)
            .ok_or_else(|| Error::InvalidInput("anchor boundary".into()))?
            .to_owned();
        self.native(
            id,
            "SourceAnchor",
            RecordData::Anchor(AnchorData {
                source,
                label: id.into(),
                locator: Locator::ByteRange { start, end, quote },
            }),
            None,
        )
    }
    pub fn capture(
        &mut self,
        path: &str,
        bytes: Vec<u8>,
        media: &str,
        status: &str,
        derived_from: Option<ExactRef>,
        derivation: Option<String>,
    ) -> Result<ExactRef, Error> {
        let source = self.native(
            &if path == "urn:document:knowledgehub-seed" {
                path.into()
            } else {
                format!("kh:source:{}", source_key(path))
            },
            "SourceArtifact",
            RecordData::SourceArtifact(SourceArtifactData {
                origin: path.into(),
                media_type: media.into(),
                owner: "repository:project".into(),
                status: status.into(),
            }),
            None,
        )?;
        let content = Content::capture(bytes);
        let reference = capture_ref(path);
        let record = Record::new(
            reference.clone(),
            type_ref("SourceRevision"),
            RecordData::SourceRevision(SourceRevisionData {
                source,
                original: content.reference.clone(),
                media_type: media.into(),
                derived_from,
                derivation,
            }),
            vec![],
        )?;
        self.add(record, Some(content));
        self.sources.insert(path.into(), reference.clone());
        Ok(reference)
    }
    pub fn capture_successor(&mut self, path: &str, bytes: Vec<u8>) -> Result<ExactRef, Error> {
        let old = capture_ref(path);
        let mut record = self.state.get(&old).ok_or(Error::NotFound)?.clone();
        record.reference.revision = "self-2".into();
        record.predecessor = Some(old);
        let content = Content::capture(bytes);
        let RecordData::SourceRevision(source) = &mut record.data else {
            return Err(Error::InvalidInput("not a source revision".into()));
        };
        source.original = content.reference.clone();
        record.seal()?;
        let reference = record.reference.clone();
        self.add(record, Some(content));
        Ok(reference)
    }
    pub fn anchor_successor(
        &mut self,
        id: &str,
        source: ExactRef,
        end: u64,
    ) -> Result<ExactRef, Error> {
        let old = r(id);
        let mut record = self.state.get(&old).ok_or(Error::NotFound)?.clone();
        record.reference.revision = "self-2".into();
        record.predecessor = Some(old);
        let quote = String::from_utf8(
            self.state
                .artifact(&source)
                .ok_or(Error::NotFound)?
                .bytes
                .clone(),
        )
        .map_err(|_| Error::InvalidInput("not UTF-8".into()))?;
        record.data = RecordData::Anchor(AnchorData {
            source,
            label: id.into(),
            locator: Locator::ByteRange {
                start: 0,
                end,
                quote,
            },
        });
        record.seal()?;
        let reference = record.reference.clone();
        self.add(record, None);
        Ok(reference)
    }
    pub fn narrative(&self, a: &ExactRef) -> Value {
        let RecordData::Anchor(anchor) = &self.state.get(a).unwrap().data else {
            panic!("not anchor")
        };
        Value::Narrative {
            document: anchor.source.clone(),
            fragment: a.clone(),
        }
    }
    pub fn body(&self, a: &ExactRef) -> Value {
        Value::Record(properties([
            ("form", Value::enumeration("description")),
            ("description", self.narrative(a)),
        ]))
    }
    pub fn make(
        &mut self,
        group: &str,
        kind: &str,
        id: &str,
        name: &str,
        a: ExactRef,
        mut values: BTreeMap<String, Value>,
    ) -> Result<ExactRef, Error> {
        let reference = r(id);
        values.insert("name".into(), Value::text(name));
        let t = self
            .state
            .type_definition(&type_ref(kind))
            .ok_or(Error::NotFound)?
            .clone();
        for (field, p) in &t.properties {
            if p.required && !values.contains_key(field) {
                match &p.value{
   ValueKind::Narrative=>{values.insert(field.clone(),self.narrative(&a));},ValueKind::Structured(n)if n=="Body"=>{values.insert(field.clone(),self.body(&a));},
   _=>self.state.gaps.push(Gap{subject:reference.clone(),path:format!("/{field}"),reason:format!("The retained source does not yet resolve the {kind}.{field} refinement. This is an explicit analysis gap, not a claim of a complete or executed contract."),provenance:vec![a.clone()]})
  }
            }
        }
        self.add(
            Record::new(
                reference.clone(),
                type_ref(kind),
                RecordData::Element(ElementData::new(values)),
                vec![a],
            )?,
            None,
        );
        self.groups
            .entry(group.into())
            .or_default()
            .push(reference.clone());
        Ok(reference)
    }
    pub fn patch(&mut self, reference: &ExactRef, field: &str, value: Value) {
        let record = self.state.records.get_mut(&reference.key()).unwrap();
        let RecordData::Element(e) = &mut record.data else {
            panic!("not element")
        };
        e.properties.insert(field.into(), value);
        e.owners = ElementData::new(e.properties.clone()).owners;
        record.seal().unwrap();
        let replacement = record.clone();
        for op in &mut self.operations {
            if let Operation::Put(r) = op {
                if r.reference == *reference {
                    *r = replacement.clone();
                }
            }
        }
        self.state
            .gaps
            .retain(|g| !(g.subject == *reference && g.path == format!("/{field}")));
    }
    pub fn edge(
        &mut self,
        id: &str,
        source_kind: &str,
        verb: &str,
        from: ExactRef,
        to: ExactRef,
        a: ExactRef,
    ) -> Result<ExactRef, Error> {
        self.native(
            id,
            "RelationshipAssertion",
            RecordData::Assertion(AssertionData {
                signature: relationship_ref(source_kind, verb),
                from,
                to,
                qualifiers: BTreeMap::new(),
            }),
            Some(a),
        )
    }
    pub fn finish(mut self) -> (Vec<Operation>, BTreeMap<String, Vec<ExactRef>>) {
        self.operations.push(Operation::Gaps(self.state.gaps));
        (self.operations, self.groups)
    }
}
fn datatype_kind(rust: &str) -> &'static str {
    let s = rust.trim();
    if s.starts_with("Option<") {
        return datatype_kind(&s[7..s.len() - 1]);
    }
    if s == "bool" {
        "boolean"
    } else if matches!(s, "u64" | "u32" | "usize" | "i64" | "u16" | "u8") {
        "integer"
    } else if s.contains("Vec<") || s.contains('[') {
        "list"
    } else if s.contains("Set<") {
        "set"
    } else if s.contains("Map<") {
        "record"
    } else if s.contains("String") || s.contains("str") {
        "text"
    } else {
        "record"
    }
}
fn contract_id(path: &str, name: &str) -> String {
    format!("kh:schema:{}::{name}", namespace(path))
}
fn signature_schema(
    b: &mut Builder,
    id: &str,
    name: &str,
    a: ExactRef,
    fields: &[Field],
) -> Result<ExactRef, Error> {
    let schema = b.make(
        "information",
        "DataSchema",
        id,
        &format!("{name} exact Rust signature"),
        a.clone(),
        properties([
            ("fields", list([])),
            ("identity_fields", list([])),
            (
                "compatibility",
                Value::Record(properties([
                    ("policy", Value::enumeration("exact")),
                    ("schemas", list([])),
                ])),
            ),
        ]),
    )?;
    let mut members = vec![];
    for f in fields {
        let datatype = b.make(
            "information",
            "DataType",
            &format!("{id}:datatype:{}", f.name),
            &f.rust_type,
            a.clone(),
            properties([
                (
                    "value_kind",
                    Value::enumeration(datatype_kind(&f.rust_type)),
                ),
                ("value_constraints", list([])),
            ]),
        )?;
        members.push(b.make(
            "information",
            "FieldDefinition",
            &format!("{id}:field:{}", f.name),
            &f.name,
            a.clone(),
            properties([
                ("schema", Value::Ref(schema.clone())),
                ("datatype", Value::Ref(datatype)),
                (
                    "nullable",
                    Value::Bool(f.rust_type.trim().starts_with("Option<")),
                ),
                (
                    "cardinality",
                    Value::Record(properties([
                        ("min", Value::Integer("1".into())),
                        ("max", Value::Integer("1".into())),
                        ("unbounded", Value::Bool(false)),
                    ])),
                ),
                ("constraints", list([])),
                ("reference_targets", list([])),
            ]),
        )?);
    }
    b.patch(&schema, "fields", list(members));
    Ok(schema)
}
pub fn author(
    root: &Path,
    capture: &Capture,
    initial: State,
    input_bytes: Vec<u8>,
) -> Result<Builder, Box<dyn std::error::Error>> {
    let mut b = Builder::new(initial);
    for file in &capture.files {
        let bytes = std::fs::read(root.join(&file.path))?;
        if ContentRef::of(&bytes) != file.reference {
            return Err(format!("captured file changed: {}", file.path).into());
        }
        let existing = match file.path.as_str() {
            "source/system-definition-ontology-proposal.md" => Some(ExactRef::new(
                "sdo",
                "urn:source:ontology:capture",
                "catalogue-1",
            )),
            "source/system-definition-ontology-proposal.pdf" => Some(ExactRef::new(
                "sdo",
                "sdo:source:ontology-pdf:capture",
                "catalogue-1",
            )),
            _ => None,
        };
        let source = if let Some(existing) = existing {
            if b.state.artifact(&existing).unwrap().reference != file.reference {
                return Err("built-in source differs from host original".into());
            }
            b.sources.insert(file.path.clone(), existing.clone());
            existing
        } else {
            let derived = file.derived_from.as_ref().map(|p| b.sources[p].clone());
            b.capture(
                &file.path,
                bytes.clone(),
                &file.media_type,
                if file.path == capture.log
                    || (file.path.starts_with("delivery/evidence/") && file.path.ends_with(".txt"))
                {
                    "validation-result"
                } else if file.path.ends_with(".rs")
                    || file.path.starts_with("Cargo.")
                    || file.derived_from.is_some()
                    || file.path.starts_with("delivery/evidence/")
                {
                    "implementation-observation"
                } else if file.path.starts_with("design/") || file.path.starts_with("delivery/") {
                    "design-decision"
                } else {
                    "authored-requirement"
                },
                derived,
                file.derivation.clone(),
            )?
        };
        if std::str::from_utf8(&bytes).is_ok() && !bytes.is_empty() {
            let a = b.anchor(
                &file_anchor_ref(&file.path).id,
                source,
                0,
                bytes.len() as u64,
            )?;
            b.anchors.insert(file.path.clone(), a);
        }
    }
    let build = b.capture(
        "host-inventory.json",
        input_bytes,
        "application/json",
        "implementation-observation",
        None,
        None,
    )?;
    let root_anchor = b.anchors["design/domain-analysis.md"].clone();
    let plan = b.anchors["design/self-hosting-plan.md"].clone();
    let source = b.anchors["source/system-definition-ontology-proposal.md"].clone();
    // Retain every physical PDF page against actual pdftotext output and original PDF digest.
    let pdf = b.sources["source/system-definition-ontology-proposal.pdf"].clone();
    let derived = b.sources["target/ontology-pdf-extracted.txt"].clone();
    let manifest = LocatorManifest {
        source: pdf.clone(),
        original: b.state.artifact(&pdf).unwrap().reference.clone(),
        tool: "pdftotext -layout".into(),
        version: capture.extractor.clone(),
        entries: capture
            .pdf_pages
            .iter()
            .map(|p| {
                (
                    format!("page:{}", p.index + 1),
                    LocatorEntry {
                        kind: LocatorKind::PdfPage,
                        label: p.label.clone(),
                        location: SourceLocation::PdfPage {
                            page: p.index + 1,
                            pages: capture.pdf_pages.len() as u64,
                        },
                        derived: derived.clone(),
                        start: p.start,
                        end: p.end,
                    },
                )
            })
            .collect(),
    };
    let m = b.capture(
        "ontology-pdf-pages.json",
        serde_json::to_vec(&manifest)?,
        "application/json",
        "implementation-observation",
        None,
        None,
    )?;
    for page in &capture.pdf_pages {
        b.native(
            &format!("kh:anchor:pdf:{}", page.index + 1),
            "SourceAnchor",
            RecordData::Anchor(AnchorData {
                source: pdf.clone(),
                label: page.label.clone(),
                locator: Locator::Manifest {
                    locator: LocatorKind::PdfPage,
                    manifest: m.clone(),
                    entry: format!("page:{}", page.index + 1),
                    quote: None,
                },
            }),
            None,
        )?;
    }
    b.native(
        "kh:hub:self-description",
        "KnowledgeHub",
        RecordData::Hub {
            owner: scope().owner,
            participants: vec!["host:maintainer".into()],
            policy_references: vec![
                "trusted host Context/Policy; no privilege from stored records".into(),
            ],
        },
        None,
    )?;
    b.native("urn:knowledgehub:library","SystemModel",RecordData::Model(ModelData{purpose:"Explain the Rust System Definition Ontology library, its governed definitions, implemented boundaries and retained evidence".into(),owner:scope().owner,ontology:ontology_ref(),profile:self_profile_ref()}),Some(source.clone()))?;
    let actor = b.make(
        "participants",
        "Actor",
        "kh:actor:maintainer",
        "Maintainer",
        source.clone(),
        properties([
            ("actor_kind", Value::enumeration("human")),
            ("system_model", Value::Ref(r("urn:knowledgehub:library"))),
        ]),
    )?;
    for (name, kind) in [("host", "host"), ("reader", "human")] {
        b.make(
            "participants",
            "Actor",
            &format!("kh:actor:{name}"),
            name,
            source.clone(),
            properties([
                ("actor_kind", Value::enumeration(kind)),
                ("system_model", Value::Ref(r("urn:knowledgehub:library"))),
            ]),
        )?;
    }
    let goal = b.make(
        "purpose",
        "BusinessGoal",
        "kh:goal:explain-system-definition",
        "Explain a system from governed exact definitions",
        source.clone(),
        properties([("measures", list([]))]),
    )?;
    let outcome = b.make(
        "purpose",
        "DesiredOutcome",
        "kh:outcome:recover-library-explanation",
        "Recover exact library explanations",
        source.clone(),
        properties([
            ("beneficiary", list([r("kh:actor:reader")])),
            ("measures", list([])),
        ]),
    )?;
    let capability = b.make(
        "purpose",
        "BusinessCapability",
        "kh:capability:govern-definitions",
        "Govern system definitions",
        source.clone(),
        BTreeMap::new(),
    )?;
    b.make(
        "purpose",
        "Stakeholder",
        "kh:stakeholder:maintainer",
        "Project maintainer",
        source.clone(),
        properties([
            (
                "involvement",
                Value::text(
                    "Author and inspect source definitions; human acceptance remains separate",
                ),
            ),
            (
                "accountable_scope",
                Value::Ref(r("urn:knowledgehub:library")),
            ),
        ]),
    )?;
    b.make(
        "purpose",
        "Measure",
        "kh:measure:catalogue-coverage",
        "Registered ontology coverage",
        source.clone(),
        properties([(
            "unit",
            Value::text("exact catalogue rows / applicable inventory rows"),
        )]),
    )?;
    b.make(
        "domain",
        "Domain",
        "kh:domain:governed-system-definition",
        "Governed system definition",
        root_anchor.clone(),
        properties([("subdomains", list([r("kh:subdomain:coherent-definitions")]))]),
    )?;
    b.make(
        "domain",
        "Subdomain",
        "kh:subdomain:coherent-definitions",
        "Coherent definitions and provenance",
        root_anchor.clone(),
        properties([
            (
                "domain",
                Value::Ref(r("kh:domain:governed-system-definition")),
            ),
            ("classification", Value::enumeration("core")),
        ]),
    )?;
    for name in [
        "definition-modelling",
        "evidence-provenance",
        "change-review",
        "interchange",
    ] {
        b.make(
            "contexts",
            "BoundedContext",
            &format!("kh:context:{name}"),
            &format!("Candidate context: {name}"),
            root_anchor.clone(),
            properties([
                (
                    "model_owner",
                    Value::text("repository:project; proposed responsibility boundary"),
                ),
                ("language", list([])),
                ("hypotheses", list([])),
            ]),
        )?;
    }
    let schema = r(&contract_id("src/ontology/types.rs", "Change"));
    let result_schema = r(&contract_id("src/contracts.rs", "Receipt"));
    let datatype = b.make(
        "information",
        "DataType",
        "kh:datatype:source-defined",
        "Exact source-defined owned record",
        source.clone(),
        properties([
            ("value_kind", Value::enumeration("record")),
            ("value_constraints", list([])),
        ]),
    )?;
    // Inventory public Rust contracts and every declared public field/enum alternative separately.
    let mut schema_fields: BTreeMap<ExactRef, Vec<ExactRef>> = BTreeMap::new();
    for unit in capture.units.iter().filter(|u| {
        matches!(u.kind.as_str(), "contract" | "native-contract") && u.path.starts_with("src/")
    }) {
        let a = b.anchor(
            &format!(
                "kh:anchor:{}:{}:{}",
                source_key(&unit.path),
                unit.start,
                unit.name
            ),
            b.sources[&unit.path].clone(),
            unit.start,
            unit.end,
        )?;
        let id = contract_id(&unit.path, &unit.name);
        let s = b.make(
            "information",
            "DataSchema",
            &id,
            &format!("{}::{}", unit.path, unit.name),
            a.clone(),
            properties([
                ("fields", list([])),
                ("identity_fields", list([])),
                (
                    "compatibility",
                    Value::Record(properties([
                        ("policy", Value::enumeration("exact")),
                        ("schemas", list([])),
                    ])),
                ),
                (
                    "native_artifact",
                    Value::Artifact(b.sources[&unit.path].clone()),
                ),
            ]),
        )?;
        for field in unit.fields.as_deref().unwrap_or(&[]) {
            let dt = b.make(
                "information",
                "DataType",
                &format!(
                    "kh:datatype:{}::{}::{}",
                    namespace(&unit.path),
                    unit.name,
                    field.name
                ),
                &field.rust_type,
                a.clone(),
                properties([
                    (
                        "value_kind",
                        Value::enumeration(datatype_kind(&field.rust_type)),
                    ),
                    ("value_constraints", list([])),
                ]),
            )?;
            let cardinality = Value::Record(properties([
                (
                    "min",
                    Value::Integer(
                        if field.rust_type.starts_with("Option<")
                            || unit.rust_kind.as_deref() == Some("enum")
                        {
                            "0"
                        } else {
                            "1"
                        }
                        .into(),
                    ),
                ),
                ("max", Value::Integer("1".into())),
                ("unbounded", Value::Bool(false)),
            ]));
            let f = b.make(
                "information",
                "FieldDefinition",
                &format!(
                    "kh:field:{}::{}::{}",
                    namespace(&unit.path),
                    unit.name,
                    field.name
                ),
                &field.name,
                a.clone(),
                properties([
                    ("schema", Value::Ref(s.clone())),
                    ("datatype", Value::Ref(dt)),
                    (
                        "nullable",
                        Value::Bool(field.rust_type.starts_with("Option<")),
                    ),
                    ("cardinality", cardinality),
                    ("constraints", list([])),
                    ("reference_targets", list([])),
                ]),
            )?;
            schema_fields.entry(s.clone()).or_default().push(f);
        }
    }
    for (s, fields) in schema_fields {
        b.patch(&s, "fields", list(fields));
    }
    let component = b.make(
        "architecture",
        "SoftwareComponent",
        "kh:component:knowledgehub-crate",
        "knowledgehub Rust crate",
        b.anchors["Cargo.toml"].clone(),
        properties([
            ("modules", list([])),
            ("interfaces", list([r("kh:interface:public-library")])),
            (
                "artifacts",
                artifact_list([build.clone(), b.sources["Cargo.toml"].clone()]),
            ),
        ]),
    )?;
    let mut modules = vec![];
    for file in capture
        .files
        .iter()
        .filter(|f| f.path.starts_with("src/") && f.path.ends_with(".rs"))
    {
        modules.push(b.make(
            "architecture",
            "Module",
            &format!("kh:module:{}", module_id(&file.path)),
            &file.path,
            b.anchors[&file.path].clone(),
            properties([
                ("component", Value::Ref(component.clone())),
                ("artifacts", artifact_list([b.sources[&file.path].clone()])),
                ("interfaces", list([])),
            ]),
        )?);
    }
    b.patch(&component, "modules", list(modules));
    let policy = b.make(
        "policy",
        "AuthorizationPolicy",
        "kh:policy:current-host-access",
        "Current trusted-host authority; deny by default",
        plan.clone(),
        properties([
            ("rules", Value::List(vec![b.body(&plan)])),
            ("evaluation_failure", Value::enumeration("unavailable")),
        ]),
    )?;
    b.make(
        "policy",
        "TrustBoundary",
        "kh:boundary:trusted-host",
        "Trusted host supplies actor and current policy",
        plan.clone(),
        properties([("inside", list([component.clone()])), ("outside", list([]))]),
    )?;
    let aggregate = b.make(
        "consistency",
        "AggregateDefinition",
        "kh:aggregate:model-branch",
        "Proposed model branch consistency responsibility",
        root_anchor.clone(),
        properties([("members", list([r("kh:entity:model-branch")]))]),
    )?;
    b.make(
        "consistency",
        "EntityType",
        "kh:entity:model-branch",
        "Proposed model branch root role",
        root_anchor.clone(),
        properties([
            (
                "identity_contract",
                Value::Ref(r(&contract_id("src/ontology/types.rs", "ModelScope"))),
            ),
            ("fields", list([])),
        ]),
    )?;
    let invariant = b.make(
        "consistency",
        "Invariant",
        "kh:invariant:atomic-publication",
        "Atomic definition, artifact, history and original receipt publication",
        root_anchor.clone(),
        properties([("valid_examples", list([])), ("invalid_examples", list([]))]),
    )?;
    b.edge(
        "kh:assertion:aggregate-root",
        "AggregateDefinition",
        "hasRoot",
        aggregate.clone(),
        r("kh:entity:model-branch"),
        root_anchor.clone(),
    )?;
    b.edge(
        "kh:assertion:aggregate-invariant",
        "AggregateDefinition",
        "protects",
        aggregate.clone(),
        invariant,
        root_anchor.clone(),
    )?;
    let explanation=b"# Atomic governed change\n\nAn admitted change publishes its definitions, exact source captures, selected revisions and original operation receipt together. The trusted host supplies current authority. This explanation is derived from ontology section 21.4; project acceptance is separate.\n".to_vec();
    let explanation_source=b.capture("urn:document:knowledgehub-seed",explanation.clone(),"text/markdown","design-decision",None,Some("Repository-owned walkthrough explanation derived from ontology section 21.4; fresh files/2 replay domain".into()))?;
    let explanation_anchor = b.anchor(
        "kh:anchor:atomic-explanation",
        explanation_source,
        0,
        explanation.len() as u64,
    )?;
    let requirement = b.make(
        "requirements",
        "Requirement",
        "urn:requirement:atomic-change",
        "Atomic governed change",
        source.clone(),
        properties([
            ("statement", b.narrative(&explanation_anchor)),
            ("rationale", list([])),
            ("claim_kind", Value::enumeration("authored-requirement")),
        ]),
    )?;
    let usecase = b.make(
        "behaviour",
        "UseCase",
        "urn:use-case:commit-change-set",
        "Commit a governed change set",
        source.clone(),
        properties([
            ("actor", list([r("kh:actor:host")])),
            ("preconditions", list([])),
            ("errors", list([])),
        ]),
    )?;
    b.make(
        "behaviour",
        "CommandDefinition",
        "kh:command:commit-change-set",
        "Commit exact retained intent",
        source.clone(),
        properties([
            ("input", Value::Ref(schema.clone())),
            ("result", Value::Ref(result_schema.clone())),
            ("guards", list([])),
            ("errors", list([])),
        ]),
    )?;
    let limits = Value::Record(properties([
        ("applicability", Value::enumeration("bounded")),
        ("active_jobs", Value::Integer("4".into())),
        ("waiting_jobs", Value::Integer("32".into())),
    ]));
    let admit = b.make(
        "behaviour",
        "FunctionDefinition",
        "kh:function:admit-change",
        "Guard and atomically admit a typed change",
        b.anchors["src/ontology/api.rs"].clone(),
        properties([
            ("input", Value::Ref(schema.clone())),
            ("output", Value::Ref(result_schema.clone())),
            ("preconditions", list([])),
            ("postconditions", list([])),
            ("rules", list([])),
            ("reads", list([aggregate.clone()])),
            ("effects", list([])),
            ("errors", list([])),
            ("determinism", Value::enumeration("conditional")),
            ("limits", limits.clone()),
            ("verification", list([])),
        ]),
    )?;
    let mut operations = vec![r("kh:command:commit-change-set")];
    for unit in &capture.units {
        if matches!(unit.kind.as_str(), "contract" | "function" | "operation")
            && !unit.path.starts_with("src/")
        {
            continue;
        }
        let a = b.anchor(
            &format!("kh:anchor:unit:{}:{}", source_key(&unit.path), unit.start),
            b.sources[&unit.path].clone(),
            unit.start,
            unit.end,
        )?;
        let semantic_name = if unit.kind == "task" {
            unit.name.split_whitespace().next().unwrap_or(&unit.name)
        } else {
            &unit.name
        };
        let id = format!(
            "kh:{}:{}:{}",
            unit.kind,
            unit_scope(unit),
            slug(semantic_name)
        );
        match unit.kind.as_str() {
            "term" => {
                b.make(
                    "language",
                    "UbiquitousLanguageTerm",
                    &format!("kh:term:{}", slug(&unit.name)),
                    &unit.name,
                    a,
                    properties([
                        ("term", Value::text(&unit.name)),
                        ("context", Value::Ref(r("kh:context:definition-modelling"))),
                        ("distinctions", Value::List(vec![])),
                    ]),
                )?;
            }
            "rule" => {
                for kind in ["BusinessRule", "Invariant", "Constraint", "Rationale"] {
                    b.make(
                        "rules",
                        kind,
                        &format!("kh:rule:{}:{kind}", unit.name),
                        &unit.name,
                        a.clone(),
                        BTreeMap::new(),
                    )?;
                }
            }
            "decision" => {
                let status = if unit.text.as_ref().is_some_and(|s| s.contains("Adopted:")) {
                    "adopted"
                } else {
                    "proposed"
                };
                let rationale = b.make(
                    "decisions",
                    "Rationale",
                    &format!("kh:rationale:{}", unit.name),
                    &unit.name,
                    a.clone(),
                    properties([("supporting_subjects", list([]))]),
                )?;
                b.make(
                    "decisions",
                    "DesignDecision",
                    &format!("kh:decision:{}", unit.name),
                    &unit.name,
                    a,
                    properties([
                        ("rationale", list([rationale])),
                        ("alternatives", Value::List(vec![])),
                        (
                            "decision_owner",
                            Value::text("repository:project; recorded source status"),
                        ),
                        ("assessment_status", Value::enumeration(status)),
                    ]),
                )?;
            }
            "requirement" => {
                b.make(
                    "requirements",
                    "Requirement",
                    &id,
                    &unit.name,
                    a,
                    properties([
                        ("claim_kind", Value::enumeration("authored-requirement")),
                        ("rationale", list([])),
                    ]),
                )?;
            }
            "constant" => {
                let schema_id = format!("kh:constant-schema:{}", unit.name);
                let constant_schema = signature_schema(
                    &mut b,
                    &schema_id,
                    &unit.name,
                    a.clone(),
                    unit.fields.as_deref().unwrap_or(&[]),
                )?;
                let value = match &unit.value {
                    Some(serde_json::Value::String(s)) => Value::text(s),
                    Some(serde_json::Value::Array(values)) => Value::List(
                        values
                            .iter()
                            .map(|v| Value::text(v.as_str().expect("literal string")))
                            .collect(),
                    ),
                    _ => return Err("unsupported public constant".into()),
                };
                let bound = Value::Record(properties([
                    (
                        "parameter",
                        Value::Ref(r(&format!("{schema_id}:field:value"))),
                    ),
                    ("value", value),
                ]));
                b.make(
                    "configuration",
                    "ConfigurationDefinition",
                    &format!("kh:constant:{}", unit.name),
                    &unit.name,
                    a,
                    properties([
                        ("schema", Value::Ref(constant_schema)),
                        ("defaults", Value::List(vec![bound])),
                        ("secret_references", Value::List(vec![])),
                    ]),
                )?;
            }
            "operation" | "function" => {
                let input_schema = signature_schema(
                    &mut b,
                    &format!("{id}:input"),
                    &unit.name,
                    a.clone(),
                    unit.parameters.as_deref().unwrap_or(&[]),
                )?;
                let output_fields = [Field {
                    name: "result".into(),
                    rust_type: unit.result.clone().unwrap_or("()".into()),
                }];
                let output_schema = signature_schema(
                    &mut b,
                    &format!("{id}:output"),
                    &unit.name,
                    a.clone(),
                    &output_fields,
                )?;
                let mut function_values = properties([
                    ("input", Value::Ref(input_schema.clone())),
                    ("output", Value::Ref(output_schema.clone())),
                    ("determinism", Value::enumeration("conditional")),
                    (
                        "limits",
                        if unit.asynchronous == Some(true) || unit.path == "src/storage.rs" {
                            limits.clone()
                        } else {
                            Value::Record(properties([
                                ("applicability", Value::enumeration("not-applicable")),
                                ("reason", b.narrative(&a)),
                            ]))
                        },
                    ),
                    (
                        "body",
                        if unit.has_body == Some(false) {
                            b.body(&a)
                        } else {
                            Value::Record(properties([
                                ("form", Value::enumeration("native")),
                                ("artifact", Value::Artifact(b.sources[&unit.path].clone())),
                                ("language", Value::text("Rust")),
                                (
                                    "entrypoint",
                                    Value::text(
                                        unit.name.trim_start_matches("module.").replace('.', "::"),
                                    ),
                                ),
                            ]))
                        },
                    ),
                ]);
                let function = b.make(
                    "behaviour",
                    "FunctionDefinition",
                    &id,
                    unit.signature.as_deref().unwrap_or(&unit.name),
                    a.clone(),
                    std::mem::take(&mut function_values),
                )?;
                if unit.kind == "operation" && unit.asynchronous == Some(true) {
                    let query = unit.name.contains("read")
                        || unit.name.contains("query")
                        || unit.name.contains("snapshot")
                        || unit.name.contains("head")
                        || unit.name.contains("compare")
                        || unit.name.contains("operation_result");
                    let kind = if query {
                        "QueryDefinition"
                    } else {
                        "CommandDefinition"
                    };
                    let mut values = properties([
                        ("input", Value::Ref(input_schema.clone())),
                        ("result", Value::Ref(output_schema.clone())),
                    ]);
                    if query {
                        values.insert("limits".into(), limits.clone());
                    }
                    let op = b.make(
                        "operations",
                        kind,
                        &format!("{id}:contract"),
                        &unit.name,
                        a.clone(),
                        values,
                    )?;
                    operations.push(op.clone());
                    if !query {
                        let handler = b.make(
                            "behaviour",
                            "UseCase",
                            &format!("{id}:handler"),
                            &unit.name,
                            a.clone(),
                            properties([("actor", list([r("kh:actor:host")]))]),
                        )?;
                        b.edge(
                            &format!("{id}:handledBy"),
                            "CommandDefinition",
                            "handledBy",
                            op,
                            handler.clone(),
                            a.clone(),
                        )?;
                        b.edge(
                            &format!("{id}:invokes"),
                            "UseCase",
                            "invokes",
                            handler,
                            function,
                            a,
                        )?;
                    }
                }
            }
            "slice" => {
                b.make(
                    "delivery",
                    "DeliverySlice",
                    &format!("kh:slice:{}", unit.name),
                    &unit.name,
                    a,
                    properties([
                        ("actor", Value::Ref(actor.clone())),
                        ("safeguards", list([requirement.clone()])),
                        (
                            "evidence_obligations",
                            list([r("kh:criterion:recover-original-receipt")]),
                        ),
                        ("prerequisites", list([])),
                    ]),
                )?;
            }
            "task" => {
                b.make(
                    "delivery",
                    "ImplementationTask",
                    &id,
                    &unit.name,
                    a,
                    properties([
                        (
                            "change",
                            Value::Ref(r(if unit.path.contains("retain-first-self-definition") {
                                "kh:change:retain-first-self-definition"
                            } else {
                                "kh:change:express-full-ontology-and-self-model"
                            })),
                        ),
                        ("implementation_references", artifact_list([])),
                        (
                            "implementation_status",
                            Value::enumeration(
                                if unit.text.as_ref().is_some_and(|s| s.starts_with("- [x]")) {
                                    "implemented"
                                } else {
                                    "planned"
                                },
                            ),
                        ),
                    ]),
                )?;
            }
            "test" => {
                b.make(
                    "verification",
                    "TestCase",
                    &id,
                    &unit.name,
                    a,
                    properties([
                        ("inputs", artifact_list([])),
                        (
                            "expected",
                            Value::List(vec![b.narrative(&b.anchors[&unit.path])]),
                        ),
                    ]),
                )?;
            }
            _ => {}
        }
    }
    b.make(
        "architecture",
        "Interface",
        "kh:interface:public-library",
        "Owned storage-independent public library API",
        source.clone(),
        properties([
            ("operations", list(operations)),
            ("contract", list([schema.clone(), result_schema.clone()])),
            (
                "compatibility",
                Value::Record(properties([
                    ("policy", Value::enumeration("exact")),
                    ("schemas", list([schema.clone(), result_schema.clone()])),
                ])),
            ),
        ]),
    )?;
    let port = b.make(
        "architecture",
        "Port",
        "kh:port:guarded-storage",
        "Guarded storage publication and recovery",
        b.anchors["src/storage.rs"].clone(),
        properties([
            ("interface", Value::Ref(r("kh:interface:public-library"))),
            ("owner_component", Value::Ref(component.clone())),
        ]),
    )?;
    let adapter = b.make(
        "architecture",
        "Adapter",
        "kh:adapter:local-files",
        "Default Linux/ext4 file adapter",
        b.anchors["src/storage.rs"].clone(),
        properties([
            ("port", Value::Ref(port)),
            (
                "implementation",
                artifact_list([b.sources["src/storage.rs"].clone()]),
            ),
        ]),
    )?;
    let integration = b.make(
        "architecture",
        "IntegrationContract",
        "kh:integration:guarded-file-storage",
        "Versioned immutable generations and atomic head replacement",
        b.anchors["src/storage.rs"].clone(),
        properties([
            ("schema", Value::Ref(schema.clone())),
            (
                "protocol",
                Value::text("knowledgehub-files/2; file and directory sync, coordinated writers"),
            ),
            ("translation", list([])),
        ]),
    )?;
    b.make(
        "operation",
        "Environment",
        "kh:environment:verified-host",
        "Observed native verification environment",
        plan.clone(),
        properties([
            ("configuration", list([])),
            ("assumptions", Value::List(vec![b.narrative(&plan)])),
            ("captured_facts", artifact_list([build.clone()])),
        ]),
    )?;
    b.make(
        "operation",
        "RecoveryProcedure",
        "kh:recovery:file-publication",
        "Recover accepted history and original operation outcomes",
        b.anchors["src/storage.rs"].clone(),
        properties([("preconditions", list([])), ("verification", list([]))]),
    )?;
    b.make(
        "delivery",
        "AcceptanceCriterion",
        "kh:criterion:recover-original-receipt",
        "Resolve original receipt after response loss",
        source.clone(),
        properties([
            ("scope", list([requirement.clone()])),
            (
                "decision_owner",
                Value::text("project:review; human acceptance remains pending"),
            ),
        ]),
    )?;
    b.make(
        "delivery",
        "ChangeDefinition",
        "kh:change:express-full-ontology-and-self-model",
        "Express full ontology and self-model",
        plan.clone(),
        properties([
            ("affected_subjects", list([r("urn:knowledgehub:library")])),
            ("rationale", list([])),
            ("slice", Value::Ref(r("kh:slice:KH-SL-008"))),
            (
                "specifications",
                artifact_list([
                    b.sources["openspec/changes/express-full-ontology-and-self-model/proposal.md"]
                        .clone(),
                ]),
            ),
        ]),
    )?;
    let test_source = capture
        .units
        .iter()
        .find(|u| u.kind == "test" && u.name.contains("sigkill_typed_artifact"))
        .ok_or("missing actual files/2 interruption test")?;
    let a = b.anchor(
        "kh:anchor:interruption-case",
        b.sources[&test_source.path].clone(),
        test_source.start,
        test_source.end,
    )?;
    let case = b.make(
        "verification",
        "TestCase",
        "kh:test:kill-after-head-replacement",
        "Interrupt typed publication after head replacement and recover original B receipt",
        a.clone(),
        properties([
            ("inputs", artifact_list([build.clone()])),
            ("expected", Value::List(vec![b.narrative(&a)])),
        ]),
    )?;
    b.edge(
        "kh:assertion:interruption-checks",
        "TestCase",
        "checks",
        case,
        requirement.clone(),
        a.clone(),
    )?;
    for (id, kind, verb, from, to) in [
        (
            "kh:assertion:capability-goal",
            "BusinessCapability",
            "contributesTo",
            capability,
            goal,
        ),
        (
            "kh:assertion:commit-handler",
            "CommandDefinition",
            "handledBy",
            r("kh:command:commit-change-set"),
            usecase.clone(),
        ),
        (
            "urn:assertion:commit-satisfies-atomic-change",
            "UseCase",
            "satisfies",
            usecase.clone(),
            requirement.clone(),
        ),
        (
            "kh:assertion:commit-invokes",
            "UseCase",
            "invokes",
            usecase.clone(),
            admit.clone(),
        ),
        (
            "kh:assertion:admit-aggregate",
            "FunctionDefinition",
            "operatesOn",
            admit.clone(),
            aggregate,
        ),
        (
            "kh:assertion:crate-implements",
            "SoftwareComponent",
            "implements",
            component,
            usecase.clone(),
        ),
        (
            "kh:assertion:adapter-translates",
            "Adapter",
            "translates",
            adapter,
            integration,
        ),
        (
            "kh:assertion:policy-governs",
            "AuthorizationPolicy",
            "governs",
            policy,
            usecase,
        ),
    ] {
        b.edge(id, kind, verb, from, to, source.clone())?;
    }
    let journey = b.make(
        "journeys",
        "Journey",
        "kh:journey:describe-library",
        "Describe, commit, inspect and recover the library model",
        b.anchors["delivery/slice-plan.md"].clone(),
        properties([
            ("actor", Value::Ref(actor)),
            ("alternatives", Value::List(vec![])),
            ("interruptions", Value::List(vec![])),
        ]),
    )?;
    b.edge(
        "kh:assertion:journey-outcome",
        "Journey",
        "pursues",
        journey,
        outcome,
        source.clone(),
    )?;
    b.edge(
        "kh:assertion:source-supports",
        "SourceAnchor",
        "supportsAssertion",
        source.clone(),
        r("urn:assertion:commit-satisfies-atomic-change"),
        source.clone(),
    )?;
    let binding = b.native(
        "kh:representation:admission-code",
        "RepresentationBinding",
        RecordData::Representation(RepresentationData {
            subject: admit.clone(),
            anchor: b.anchors["src/ontology/api.rs"].clone(),
            role: RepresentationRole::Implementation,
        }),
        Some(source.clone()),
    )?;
    b.edge(
        "kh:assertion:admission-represents",
        "RepresentationBinding",
        "represents",
        binding,
        admit,
        source.clone(),
    )?;
    // Expand every candidate responsibility and all source journeys/stories, preserving analysis status.
    for name in [
        "element-identity",
        "relationship-identity",
        "source-artifact",
        "model-baseline",
        "ontology-definition",
        "review-decision",
    ] {
        let entity = b.make(
            "consistency",
            "EntityType",
            &format!("kh:entity:{name}"),
            &format!("Proposed {name} root responsibility"),
            root_anchor.clone(),
            properties([("fields", list([]))]),
        )?;
        let agg = b.make(
            "consistency",
            "AggregateDefinition",
            &format!("kh:aggregate:{name}"),
            &format!("Candidate aggregate: {name}; not a claimed Rust struct"),
            root_anchor.clone(),
            properties([("members", list([entity.clone()]))]),
        )?;
        b.edge(
            &format!("kh:assertion:{name}:root"),
            "AggregateDefinition",
            "hasRoot",
            agg.clone(),
            entity,
            root_anchor.clone(),
        )?;
        b.edge(
            &format!("kh:assertion:{name}:protects"),
            "AggregateDefinition",
            "protects",
            agg,
            r(&format!(
                "kh:rule:{}:Invariant",
                match name {
                    "element-identity" => "KH-DR-01",
                    "relationship-identity" => "KH-DR-03",
                    "source-artifact" => "KH-DR-05",
                    "model-baseline" => "KH-DR-04",
                    "ontology-definition" => "KH-DR-16",
                    _ => "KH-DR-13",
                }
            )),
            root_anchor.clone(),
        )?;
    }
    for unit in capture
        .units
        .iter()
        .filter(|u| u.kind == "journey" || u.kind == "story")
    {
        let a = b.anchor(
            &format!("kh:anchor:journey:{}:{}", source_key(&unit.path), unit.name),
            b.sources[&unit.path].clone(),
            unit.start,
            unit.end,
        )?;
        if unit.kind == "story" {
            b.make(
                "journeys",
                "UserStory",
                &format!("kh:story:{}", unit.name),
                &unit.name,
                a,
                properties([("actor", Value::Ref(r("kh:actor:maintainer")))]),
            )?;
            continue;
        }
        let id = match unit.name.as_str() {
            "KH-J-01" => "kh:journey:retain-sourced-definition",
            "KH-J-06" => "kh:journey:express-full-ontology",
            _ => "",
        };
        let id = if id.is_empty() {
            format!("kh:journey:{}", unit.name)
        } else {
            id.into()
        };
        let journey = b.make(
            "journeys",
            "Journey",
            &id,
            &unit.name,
            a.clone(),
            properties([
                ("actor", Value::Ref(r("kh:actor:maintainer"))),
                ("alternatives", Value::List(vec![])),
                ("interruptions", Value::List(vec![])),
            ]),
        )?;
        let stages = unit
            .text
            .as_deref()
            .unwrap_or("")
            .split('|')
            .nth(3)
            .unwrap_or("")
            .split('→')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        for (i, stage) in stages.iter().enumerate() {
            let next = if i + 1 < stages.len() {
                vec![r(&format!("{id}:step:{}", i + 1))]
            } else {
                vec![]
            };
            let step = b.make(
                "journeys",
                "JourneyStep",
                &format!("{id}:step:{i}"),
                stage,
                a.clone(),
                properties([
                    ("journey", Value::Ref(journey.clone())),
                    ("next_steps", list(next)),
                    ("alternatives", list([])),
                    ("handoffs", Value::List(vec![])),
                ]),
            )?;
            b.edge(
                &format!("{id}:contains:{i}"),
                "Journey",
                "containsStep",
                journey.clone(),
                step,
                a.clone(),
            )?;
        }
    }
    let journey = r("kh:journey:describe-library");
    let mut activities = vec![];
    for name in [
        "establish-scope",
        "understand-meaning",
        "propose-and-commit",
        "review-and-explain",
        "resume-and-exchange",
    ] {
        let activity = r(&format!("kh:activity:{name}"));
        let task = b.make(
            "journeys",
            "UserTask",
            &format!("kh:task:{name}"),
            name,
            b.anchors["delivery/slice-plan.md"].clone(),
            properties([("activity", Value::Ref(activity.clone()))]),
        )?;
        activities.push(b.make(
            "journeys",
            "UserActivity",
            &format!("kh:activity:{name}"),
            name,
            b.anchors["delivery/slice-plan.md"].clone(),
            properties([
                ("journey", Value::Ref(journey.clone())),
                ("tasks", list([task])),
            ]),
        )?);
    }
    let map = b.make(
        "journeys",
        "StoryMap",
        "kh:story-map:library",
        "Library source story map",
        b.anchors["delivery/slice-plan.md"].clone(),
        properties([
            ("journey", Value::Ref(journey)),
            ("activities", list(activities)),
            (
                "cuts",
                list([r("kh:cut:full-ontology"), r("kh:cut:self-description")]),
            ),
        ]),
    )?;
    for (name, story) in [
        ("full-ontology", "KH-US-19"),
        ("self-description", "KH-US-23"),
    ] {
        b.make(
            "journeys",
            "OutcomeCut",
            &format!("kh:cut:{name}"),
            name,
            b.anchors["delivery/slice-plan.md"].clone(),
            properties([
                ("map", Value::Ref(map.clone())),
                ("stories", list([r(&format!("kh:story:{story}"))])),
                ("prerequisites", list([])),
                ("exclusions", Value::List(vec![])),
            ]),
        )?;
    }
    for name in [
        "definition-modelling",
        "evidence-provenance",
        "change-review",
        "interchange",
    ] {
        let context = r(&format!("kh:context:{name}"));
        b.make(
            "contexts",
            "PublishedLanguage",
            &format!("kh:published-language:{name}"),
            &format!("Candidate collaboration contract: {name}"),
            root_anchor.clone(),
            properties([
                ("owner_context", Value::Ref(context)),
                ("definitions", list([schema.clone(), result_schema.clone()])),
            ]),
        )?;
    }
    b.make(
        "contexts",
        "ContextRelationship",
        "kh:context-relationship:governance-definitions",
        "Proposed admission to definition validation collaboration",
        root_anchor.clone(),
        properties([
            ("upstream", Value::Ref(r("kh:context:definition-modelling"))),
            ("downstream", Value::Ref(r("kh:context:change-review"))),
            (
                "contract",
                Value::Ref(r("kh:published-language:definition-modelling")),
            ),
        ]),
    )?;
    b.make(
        "consistency",
        "ValueObjectType",
        "kh:value-object:exact-reference",
        "Qualified identity and immutable revision value",
        b.anchors["src/ontology/types.rs"].clone(),
        properties([
            (
                "value_contract",
                Value::Ref(r(&contract_id("src/ontology/types.rs", "ExactRef"))),
            ),
            ("fields", list([])),
        ]),
    )?;
    b.make(
        "consistency",
        "DomainService",
        "kh:service:assess-model",
        "Deterministic selected-model assessment",
        b.anchors["src/ontology/validation.rs"].clone(),
        properties([
            ("functions", list([r("kh:function:admit-change")])),
            ("rules", list([r("kh:invariant:atomic-publication")])),
        ]),
    )?;
    b.make(
        "participants",
        "BusinessRole",
        "kh:business-role:maintainer",
        "Maintainer authoring and review responsibility",
        plan.clone(),
        BTreeMap::new(),
    )?;
    let principal = b.make(
        "participants",
        "PrincipalType",
        "kh:principal-type:host-actor",
        "Stable host-qualified actor identity",
        plan.clone(),
        properties([(
            "identity_contract",
            Value::Ref(r(&contract_id("src/contracts.rs", "Context"))),
        )]),
    )?;
    let permission = b.make(
        "policy",
        "Permission",
        "kh:permission:commit",
        "Commit under current host policy",
        plan.clone(),
        properties([
            ("action", Value::text("Commit")),
            (
                "resource_scope",
                Value::Record(properties([
                    ("subject", Value::Ref(r("urn:knowledgehub:library"))),
                    ("path", Value::text("purpose")),
                ])),
            ),
        ]),
    )?;
    b.make(
        "participants",
        "Role",
        "kh:role:maintainer",
        "Descriptive maintainer role; grants no runtime privilege",
        plan.clone(),
        properties([
            ("principal_types", list([principal])),
            ("permissions", list([permission])),
        ]),
    )?;
    for name in [
        "Precondition",
        "Postcondition",
        "Effect",
        "ErrorDefinition",
        "Parameter",
    ] {
        let mut fields = BTreeMap::new();
        match name {
            "Parameter" => {
                fields.insert("datatype".into(), Value::Ref(datatype.clone()));
                fields.insert("direction".into(), Value::enumeration("input"));
                fields.insert("nullable".into(), Value::Bool(false));
                fields.insert(
                    "cardinality".into(),
                    Value::Record(properties([
                        ("min", Value::Integer("1".into())),
                        ("max", Value::Integer("1".into())),
                        ("unbounded", Value::Bool(false)),
                    ])),
                );
                fields.insert("constraints".into(), list([]));
            }
            "Effect" => {
                fields.insert("effect_kind".into(), Value::enumeration("write"));
                fields.insert("target".into(), Value::Ref(r("urn:knowledgehub:library")));
            }
            "ErrorDefinition" => {
                fields.insert("code".into(), Value::text("Uncertain"));
                fields.insert("outcome".into(), Value::enumeration("uncertain"));
            }
            _ => {}
        }
        b.make(
            "behaviour",
            name,
            &format!("kh:contract:{}", slug(name)),
            &format!("Guarded publication {name}"),
            source.clone(),
            fields,
        )?;
    }
    b.patch(
        &r("kh:function:admit-change"),
        "preconditions",
        list([r("kh:contract:precondition")]),
    );
    b.patch(
        &r("kh:function:admit-change"),
        "postconditions",
        list([r("kh:contract:postcondition")]),
    );
    b.patch(
        &r("kh:function:admit-change"),
        "effects",
        list([r("kh:contract:effect")]),
    );
    b.patch(
        &r("kh:function:admit-change"),
        "errors",
        list([r("kh:contract:errordefinition")]),
    );
    for name in [
        "canonical-request",
        "semantic-revision",
        "snapshot-encoding",
    ] {
        let inputs = match name {
            "canonical-request" => vec![
                Field {
                    name: "change".into(),
                    rust_type: "Change".into(),
                },
                Field {
                    name: "actor".into(),
                    rust_type: "String".into(),
                },
            ],
            "semantic-revision" => vec![Field {
                name: "record".into(),
                rust_type: "Record".into(),
            }],
            _ => vec![
                Field {
                    name: "snapshot".into(),
                    rust_type: "Snapshot".into(),
                },
                Field {
                    name: "receipt".into(),
                    rust_type: "Receipt".into(),
                },
            ],
        };
        let codec_anchor = b.anchors["src/codec.rs"].clone();
        let input_contract = signature_schema(
            &mut b,
            &format!("kh:transformation:{name}:input"),
            name,
            codec_anchor.clone(),
            &inputs,
        )?;
        let output_contract = if name == "snapshot-encoding" {
            r(&contract_id("src/storage.rs", "Manifest"))
        } else {
            signature_schema(
                &mut b,
                &format!("kh:transformation:{name}:output"),
                name,
                codec_anchor.clone(),
                &[Field {
                    name: "sha256".into(),
                    rust_type: "String".into(),
                }],
            )?
        };
        let mapping = b.make(
            "transformations",
            "MappingRule",
            &format!("kh:mapping:{name}"),
            name,
            codec_anchor.clone(),
            properties([
                (
                    "source",
                    Value::Record(properties([
                        ("subject", Value::Ref(input_contract.clone())),
                        (
                            "path",
                            Value::text(match name {
                                "canonical-request" => "change",
                                "semantic-revision" => "record",
                                _ => "snapshot",
                            }),
                        ),
                    ])),
                ),
                (
                    "target",
                    Value::Record(properties([
                        ("subject", Value::Ref(output_contract.clone())),
                        (
                            "path",
                            Value::text(if name == "snapshot-encoding" {
                                "files"
                            } else {
                                "sha256"
                            }),
                        ),
                    ])),
                ),
            ]),
        )?;
        let transformation = b.make(
            "transformations",
            "TransformationDefinition",
            &format!("kh:transformation:{name}"),
            name,
            codec_anchor.clone(),
            properties([
                (
                    "kind",
                    Value::enumeration(if name == "snapshot-encoding" {
                        "artifact"
                    } else {
                        "data"
                    }),
                ),
                ("mapping_rules", list([mapping])),
                ("preconditions", list([])),
                ("postconditions", list([])),
                ("effects", list([])),
                ("errors", list([])),
            ]),
        )?;
        for (verb, to) in [
            ("hasInputContract", input_contract.clone()),
            ("hasOutputContract", output_contract.clone()),
        ] {
            b.edge(
                &format!("kh:assertion:{name}:{verb}"),
                "TransformationDefinition",
                verb,
                transformation.clone(),
                to,
                source.clone(),
            )?;
        }
        b.make(
            "transformations",
            "ProjectionDefinition",
            &format!("kh:projection:{name}"),
            name,
            source.clone(),
            properties([
                ("output_schema", Value::Ref(output_contract)),
                ("mapping", Value::Ref(transformation)),
            ]),
        )?;
    }
    b.make(
        "architecture",
        "ApplicationSystem",
        "kh:application-system:knowledgehub",
        "KnowledgeHub embedded library",
        source.clone(),
        properties([
            ("components", list([r("kh:component:knowledgehub-crate")])),
            ("interfaces", list([r("kh:interface:public-library")])),
            ("external_systems", list([])),
        ]),
    )?;
    b.make(
        "architecture",
        "DataStoreDefinition",
        "kh:data-store:files",
        "Versioned Markdown, graph JSON and original artifacts",
        b.anchors["src/storage.rs"].clone(),
        properties([("schemas", list([schema.clone()])), ("retention", list([]))]),
    )?;
    b.make(
        "policy",
        "RetentionPolicy",
        "kh:retention:original-history",
        "Committed generations and receipts retained for store lifetime",
        plan.clone(),
        properties([(
            "scope",
            Value::Record(properties([
                ("subject", Value::Ref(r("urn:knowledgehub:library"))),
                ("path", Value::text("purpose")),
            ])),
        )]),
    )?;
    b.make(
        "policy",
        "AuditRequirement",
        "kh:audit:original-receipts",
        "Exact actor-bound intent and original operation outcome",
        plan.clone(),
        properties([
            (
                "scope",
                Value::Record(properties([
                    ("subject", Value::Ref(r("urn:knowledgehub:library"))),
                    ("path", Value::text("purpose")),
                ])),
            ),
            ("retained_facts", Value::List(vec![b.narrative(&plan)])),
            ("retention", Value::Ref(r("kh:retention:original-history"))),
        ]),
    )?;
    b.make(
        "decisions",
        "Assumption",
        "kh:assumption:platform-compliance",
        "User-supplied platform compliance and existing Rust compiler",
        plan.clone(),
        properties([("resolution_status", Value::enumeration("open"))]),
    )?;
    b.make(
        "decisions",
        "OpenQuestion",
        "kh:question:authority-handover",
        "Exact authority handover scope and accepted baseline remain undecided",
        plan.clone(),
        properties([("decision_owner", Value::text("project:review"))]),
    )?;
    b.make(
        "operation",
        "Resource",
        "kh:resource:blocking-jobs",
        "Finite active and waiting blocking jobs",
        b.anchors["src/jobs.rs"].clone(),
        properties([
            (
                "resource_kind",
                Value::text("host-owned Tokio blocking jobs"),
            ),
            ("capacity", Value::List(vec![])),
            ("constraints", list([])),
        ]),
    )?;
    b.make(
        "operation",
        "ConfigurationDefinition",
        "kh:configuration:file-options",
        "File support and bounded job options",
        b.anchors["src/storage.rs"].clone(),
        properties([
            (
                "schema",
                Value::Ref(r(&contract_id("src/storage.rs", "FileOptions"))),
            ),
            ("defaults", Value::List(vec![])),
            ("secret_references", Value::List(vec![])),
        ]),
    )?;
    b.make(
        "operation",
        "QualityScenario",
        "kh:quality:interrupted-publication",
        "Process interruption preserves complete prior or new generation",
        plan.clone(),
        properties([
            ("environment", Value::Ref(r("kh:environment:verified-host"))),
            ("measure", Value::Ref(r("kh:measure:catalogue-coverage"))),
            (
                "target",
                Value::Decimal {
                    coefficient: "1".into(),
                    exponent: "0".into(),
                },
            ),
        ]),
    )?;
    b.make(
        "operation",
        "ObservabilityRequirement",
        "kh:observability:operation-outcomes",
        "Expose exact original outcomes and safe uncertainty",
        plan.clone(),
        properties([
            ("subject", Value::Ref(r("urn:knowledgehub:library"))),
            ("signals", Value::List(vec![b.narrative(&plan)])),
            (
                "verification",
                list([r("kh:test:kill-after-head-replacement")]),
            ),
        ]),
    )?;
    b.make(
        "delivery",
        "ChangeDefinition",
        "kh:change:retain-first-self-definition",
        "Historical first seed change; separate review status",
        plan.clone(),
        properties([
            ("affected_subjects", list([r("urn:knowledgehub:library")])),
            ("rationale", list([])),
            ("slice", Value::Ref(r("kh:slice:KH-SL-001"))),
            (
                "specifications",
                artifact_list([
                    b.sources["openspec/changes/retain-first-self-definition/proposal.md"].clone(),
                ]),
            ),
        ]),
    )?;
    b.make(
        "delivery",
        "ReleaseWorkPackage",
        "kh:work-package:file-conformance",
        "File-backend conformance evidence; acceptance pending",
        plan.clone(),
        properties([
            (
                "slices",
                list([
                    r("kh:slice:KH-SL-001"),
                    r("kh:slice:KH-SL-007"),
                    r("kh:slice:KH-SL-008"),
                ]),
            ),
            (
                "obligations",
                list([r("kh:criterion:recover-original-receipt")]),
            ),
        ]),
    )?;
    b.native("kh:interpretation:source-extraction","Interpretation",RecordData::Interpretation(InterpretationData{subject:r("urn:knowledgehub:library"),property:None,anchor:root_anchor.clone(),text:"This sourced self-model is a host extraction/authoring interpretation of exact repository inputs. Candidate responsibilities and unresolved refinements retain explicit status; repository authority and human acceptance remain separate.".into(),interpreter:"host:maintainer".into(),method:"capture_self_model.py inventory and typed public API authoring".into(),review:None}),Some(root_anchor.clone()))?;

    let terms = b.groups.get("language").cloned().unwrap_or_default();
    b.patch(
        &r("kh:context:definition-modelling"),
        "language",
        list(terms.clone()),
    );
    for subject in terms {
        b.edge(
            &format!("{}:ownedIn", subject.id),
            "ElementRevision",
            "ownedIn",
            subject,
            r("kh:context:definition-modelling"),
            root_anchor.clone(),
        )?;
    }
    for name in [
        "model-branch",
        "element-identity",
        "relationship-identity",
        "source-artifact",
        "model-baseline",
        "ontology-definition",
        "review-decision",
    ] {
        let context = match name {
            "model-branch" | "review-decision" => "change-review",
            "source-artifact" => "evidence-provenance",
            _ => "definition-modelling",
        };
        for prefix in ["kh:aggregate:", "kh:entity:"] {
            let subject = r(&format!("{prefix}{name}"));
            b.edge(
                &format!("{}:ownedIn", subject.id),
                "ElementRevision",
                "ownedIn",
                subject,
                r(&format!("kh:context:{context}")),
                root_anchor.clone(),
            )?;
        }
    }
    for path in [
        "delivery/evidence/kh-sl-001-run.txt",
        "delivery/evidence/full-ontology-start.txt",
    ] {
        if let (Some(source), Some(anchor)) =
            (b.sources.get(path).cloned(), b.anchors.get(path).cloned())
        {
            b.make(
                "historical-evidence",
                "EvidenceArtifact",
                &format!("kh:evidence:historical:{}", source_key(path)),
                "Historical native run: its captured scope is not current-code success",
                anchor,
                properties([
                    ("artifact", Value::Artifact(source.clone())),
                    ("observed_scope", list([source])),
                    ("observation_kind", Value::enumeration("validation-result")),
                    ("captured_at", Value::text(&capture.captured_at)),
                ]),
            )?;
        }
    }
    let _ = (datatype, result_schema);
    Ok(b)
}
