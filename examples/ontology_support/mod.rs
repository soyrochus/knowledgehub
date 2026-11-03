//! Explicitly synthetic, fully typed conformance specimens. These are not KnowledgeHub observations.
#![allow(dead_code)]
use knowledgehub::{Error, ontology::*};
use std::collections::BTreeMap;

pub fn reference(kind: &str) -> ExactRef {
    ExactRef::new("specimen", format!("specimen:{kind}"), "1")
}
pub fn anchor() -> ExactRef {
    ExactRef::new("sdo", "sdo:anchor:catalogue", "catalogue-3")
}
pub fn document() -> ExactRef {
    ExactRef::new("sdo", "urn:source:ontology:capture", "catalogue-3")
}
pub fn narrative() -> Value {
    Value::Narrative {
        document: document(),
        fragment: anchor(),
    }
}
pub fn body() -> Value {
    Value::Record(BTreeMap::from([
        ("form".into(), Value::enumeration("description")),
        ("description".into(), narrative()),
    ]))
}
pub fn scope() -> ModelScope {
    ModelScope {
        hub: "hub:conformance".into(),
        model: "model:conformance".into(),
        branch: "main".into(),
        owner: "owner:conformance".into(),
    }
}
fn target(name: &str) -> ExactRef {
    match name {
        "AnySubject" | "AnyElementRevision" | "ElementRevision" => reference("Requirement"),
        "ElementTypeDefinition" => type_ref("Requirement"),
        "OntologyDefinition" => ontology_ref(),
        "ProfileDefinition" => profile_ref(),
        "SourceRevision" => document(),
        "SourceArtifact" => ExactRef::new("sdo", "urn:source:ontology", "catalogue-3"),
        "SourceAnchor" => anchor(),
        "ConstraintDefinition" => ExactRef::new("sdo", "sdo:constraint:V-01", "catalogue-3"),
        "RelationshipTypeDefinition" => relationship_ref("UseCase", "satisfies"),
        _ => reference(name),
    }
}
fn specimen_value(k: &ValueKind, structures: &BTreeMap<String, ValueKind>) -> Value {
    match k {
        ValueKind::Text => Value::text(
            "Synthetic specimen: explicit authored contract; no execution or acceptance inferred",
        ),
        ValueKind::Bool => Value::Bool(false),
        ValueKind::Integer => Value::Integer("1".into()),
        ValueKind::Decimal => Value::Decimal {
            coefficient: "125".into(),
            exponent: "-2".into(),
        },
        ValueKind::Timestamp => Value::text("2026-10-31T12:00:00Z"),
        ValueKind::Enum(a) => Value::enumeration(&a[0]),
        ValueKind::Reference(a) => Value::Ref(match &a[0] {
            TypeSelector::Type(r) => target(r.id.strip_prefix("sdo:type:").unwrap()),
            TypeSelector::Role(_) => reference("Requirement"),
        }),
        ValueKind::Artifact => Value::Artifact(document()),
        ValueKind::Narrative => narrative(),
        ValueKind::List { item, min } => Value::List(
            (0..*min)
                .map(|_| specimen_value(item, structures))
                .collect(),
        ),
        ValueKind::Set { item, min } => Value::Set(
            (0..*min)
                .map(|_| specimen_value(item, structures))
                .collect(),
        ),
        ValueKind::Record(p) => Value::Record(
            p.iter()
                .filter(|(_, p)| p.required)
                .map(|(n, p)| (n.clone(), specimen_value(&p.value, structures)))
                .collect(),
        ),
        ValueKind::Structured(n) => match n.as_str() {
            "Body" => body(),
            "Cardinality" => Value::Record(BTreeMap::from([
                ("min".into(), Value::Integer("0".into())),
                ("max".into(), Value::Integer("1".into())),
                ("unbounded".into(), Value::Bool(false)),
            ])),
            "Limits" => Value::Record(BTreeMap::from([
                ("applicability".into(), Value::enumeration("bounded")),
                ("active_jobs".into(), Value::Integer("2".into())),
                ("waiting_jobs".into(), Value::Integer("8".into())),
            ])),
            "Selector" => Value::Record(BTreeMap::from([
                ("subject".into(), Value::Ref(reference("Requirement"))),
                ("path".into(), Value::text("statement")),
            ])),
            _ => specimen_value(&structures[n], structures),
        },
        ValueKind::Union(a) => specimen_value(&a[0], structures),
        ValueKind::Any => Value::text("typed text specimen"),
    }
}
pub fn put_property(record: &mut Record, key: &str, value: Value) {
    let RecordData::Element(e) = &mut record.data else {
        panic!("not element")
    };
    e.properties.insert(key.into(), value);
    e.owners = ElementData::new(e.properties.clone()).owners;
    record.seal().unwrap();
}
pub fn assertion(id: &str, source: &str, verb: &str, from: ExactRef, to: ExactRef) -> Record {
    Record::new(
        reference(id),
        type_ref("RelationshipAssertion"),
        RecordData::Assertion(AssertionData {
            signature: relationship_ref(source, verb),
            from,
            to,
            qualifiers: BTreeMap::new(),
        }),
        vec![anchor()],
    )
    .unwrap()
}
pub fn catalogue_state() -> State {
    let c = builtin_catalogue().unwrap();
    let mut s = State::empty(scope());
    for r in c.records {
        s.selected
            .insert(r.reference.identity(), r.reference.clone());
        s.records.insert(r.reference.key(), r);
    }
    s.artifacts = c.artifacts;
    s.ontology = Some(ontology_ref());
    s.profile = Some(profile_ref());
    s
}
pub fn install(s: &mut State, r: Record) {
    s.selected
        .insert(r.reference.identity(), r.reference.clone());
    s.records.insert(r.reference.key(), r);
}
/// Every family schema has all required properties; semantic overrides give connected roles real meaning.
pub fn specimens() -> Result<Vec<Operation>, Error> {
    let c = builtin_catalogue()?;
    let structures = c
        .records
        .iter()
        .find_map(|r| {
            if let RecordData::Ontology(o) = &r.data {
                Some(&o.structures)
            } else {
                None
            }
        })
        .unwrap();
    let mut records = BTreeMap::new();
    for r in &c.records {
        if let RecordData::Type(t) = &r.data {
            if !t.native {
                let mut p = t
                    .properties
                    .iter()
                    .filter(|(_, p)| p.required)
                    .map(|(k, p)| (k.clone(), specimen_value(&p.value, structures)))
                    .collect::<BTreeMap<_, _>>();
                p.insert(
                    "name".into(),
                    Value::text(format!("Synthetic {} conformance contract", t.name)),
                );
                records.insert(
                    t.name.clone(),
                    Record::new(
                        reference(&t.name),
                        r.reference.clone(),
                        RecordData::Element(ElementData::new(p)),
                        vec![anchor()],
                    )?,
                );
            }
        }
    }
    let mut set = |kind: &str, field: &str, value: Value| {
        put_property(records.get_mut(kind).unwrap(), field, value)
    };
    let one = |name: &str| Value::List(vec![Value::Ref(reference(name))]);
    set("DataSchema", "fields", one("FieldDefinition"));
    set("DataSchema", "identity_fields", one("FieldDefinition"));
    set("LifecycleDefinition", "states", one("State"));
    set("LifecycleDefinition", "transitions", one("Transition"));
    set(
        "Transition",
        "owner",
        Value::Ref(reference("LifecycleDefinition")),
    );
    set(
        "State",
        "owner",
        Value::Ref(reference("LifecycleDefinition")),
    );
    set("ProcessDefinition", "steps", one("ProcessStep"));
    set("ProcessDefinition", "initial_steps", one("ProcessStep"));
    set("RunLimit", "calls", Value::Integer("1".into()));
    set(
        "AcceptanceRecord",
        "assessor",
        Value::text("specimen:synthetic-assessor"),
    );
    set("ScreenDefinition", "states", one("ViewState"));
    set("DesignToken", "value", Value::text("synthetic-colour"));
    set("SoftwareComponent", "modules", one("Module"));
    set("ComponentPlacement", "bindings", one("ActionBinding"));
    set("StoryMap", "placements", one("StoryPlacement"));
    set("OutcomeCut", "prerequisites", Value::List(vec![]));
    let mut out = records
        .into_values()
        .filter(|r| {
            !matches!(
                r.type_ref.id.as_str(),
                "sdo:type:Release" | "sdo:type:TestRun" | "sdo:type:AcceptanceRecord"
            )
        })
        .map(Operation::Put)
        .collect::<Vec<_>>();
    let mut late = vec![];
    // Instantiate all declared signatures, choosing actual native subjects for the kernel roles.
    for r in &c.records {
        if let RecordData::Relationship(d) = &r.data {
            let source = match &d.from[0] {
                TypeSelector::Type(r) => r.id.strip_prefix("sdo:type:").unwrap(),
                TypeSelector::Role(SubjectRole::ElementRevision) => "ElementRevision",
                TypeSelector::Role(SubjectRole::AnySubject) => "AnySubject",
            };
            let mut from = target(source);
            let mut to = match &d.to[0] {
                TypeSelector::Type(r) => target(r.id.strip_prefix("sdo:type:").unwrap()),
                TypeSelector::Role(_) => reference("Requirement"),
            };
            match d.coverage.as_str() {
                "R03" => {
                    // Separate child avoids a meaningless self containment cycle.
                    let original = out
                        .iter()
                        .filter_map(Operation::record)
                        .find(|r| r.reference == reference("BusinessFunction"))
                        .unwrap();
                    let mut child = original.clone();
                    child.reference = reference("BusinessFunctionChild");
                    child.seal()?;
                    out.push(Operation::Put(child));
                    to = reference("BusinessFunctionChild");
                }
                "R16" => from = reference("Requirement"),
                "R30" => from = reference("RepresentationBinding"),
                "R31" => to = reference("edge:R08"),
                "R32" => {
                    from = reference("edge:R08");
                    to = reference("edge:R07")
                }
                "P02" => {
                    let original = out
                        .iter()
                        .filter_map(Operation::record)
                        .find(|r| r.reference == reference("ComponentPlacement"))
                        .unwrap();
                    let mut child = original.clone();
                    child.reference = reference("ComponentPlacementChild");
                    child.seal()?;
                    out.push(Operation::Put(child));
                    to = reference("ComponentPlacementChild");
                }
                "E03" => {
                    let mut child = out
                        .iter()
                        .filter_map(Operation::record)
                        .find(|r| r.reference == reference("UserStory"))
                        .unwrap()
                        .clone();
                    child.reference = reference("UserStoryPrerequisite");
                    child.seal()?;
                    to = child.reference.clone();
                    out.push(Operation::Put(child));
                }
                _ => {}
            }
            let mut edge = assertion(&format!("edge:{}", d.coverage), source, &d.name, from, to);
            if d.coverage == "E03" {
                if let RecordData::Assertion(a) = &mut edge.data {
                    a.qualifiers
                        .insert("dependency_kind".into(), Value::enumeration("required"));
                    a.qualifiers.insert("condition".into(), Value::Null);
                }
                edge.seal()?;
            }
            if d.coverage == "R28" {
                late.push(Operation::Put(edge))
            } else {
                out.push(Operation::Put(edge));
            }
        }
    }
    out.push(Operation::Put(Record::new(
        reference("KnowledgeHub"),
        type_ref("KnowledgeHub"),
        RecordData::Hub {
            owner: scope().owner,
            participants: vec!["specimen:author".into()],
            policy_references: vec![],
        },
        vec![],
    )?));
    out.push(Operation::Put(Record::new(
        reference("SystemModel"),
        type_ref("SystemModel"),
        RecordData::Model(ModelData {
            purpose: "Synthetic whole-ontology conformance model".into(),
            owner: scope().owner,
            ontology: ontology_ref(),
            profile: profile_ref(),
        }),
        vec![anchor()],
    )?));
    out.push(Operation::Put(Record::new(
        reference("RepresentationBinding"),
        type_ref("RepresentationBinding"),
        RecordData::Representation(RepresentationData {
            subject: reference("Requirement"),
            anchor: anchor(),
            role: RepresentationRole::Derived,
        }),
        vec![anchor()],
    )?));
    out.push(Operation::Put(Record::new(
        reference("Interpretation"),
        type_ref("Interpretation"),
        RecordData::Interpretation(InterpretationData {
            subject: reference("Requirement"),
            property: Some("statement".into()),
            anchor: anchor(),
            text: "Synthetic reading; not source adoption".into(),
            interpreter: "specimen:author".into(),
            method: "synthetic conformance".into(),
            review: None,
        }),
        vec![anchor()],
    )?));
    out.push(Operation::Put(Record::new(
        reference("ReviewDecision"),
        type_ref("ReviewDecision"),
        RecordData::Review(ReviewData {
            assessor: "specimen:assessor".into(),
            targets: vec![reference("Requirement")],
            decision: "proposed".into(),
            rationale: "Synthetic proposal; not actual project acceptance".into(),
            assessed_at: "2026-10-31T12:00:00Z".into(),
        }),
        vec![anchor()],
    )?));
    for (id, status, text) in [
        (
            "observed-build",
            "implementation-observation",
            "Synthetic conformance build manifest; not a project implementation claim",
        ),
        (
            "observed-log",
            "validation-result",
            "Synthetic conformance log; not an actual project execution",
        ),
        (
            "decision-table",
            "authored-requirement",
            "{\"rules\":[{\"current_authority\":true,\"expected_head_matches\":true,\"result\":\"admit\"},{\"current_authority\":false,\"result\":\"deny\"}]}",
        ),
    ] {
        let source = Record::new(
            reference(&format!("{id}:source")),
            type_ref("SourceArtifact"),
            RecordData::SourceArtifact(SourceArtifactData {
                origin: format!("synthetic:{id}"),
                media_type: "text/plain".into(),
                owner: "specimen:author".into(),
                status: status.into(),
            }),
            vec![],
        )?;
        let content = knowledgehub::Content::capture(text.as_bytes().to_vec());
        let capture = Record::new(
            reference(id),
            type_ref("SourceRevision"),
            RecordData::SourceRevision(SourceRevisionData {
                source: source.reference.clone(),
                original: content.reference.clone(),
                media_type: "text/plain".into(),
                derived_from: None,
                derivation: None,
            }),
            vec![],
        )?;
        out.push(Operation::Put(source));
        out.push(Operation::Capture {
            record: capture,
            content,
        });
    }
    let mut state = catalogue_state();
    for r in out.iter().filter_map(Operation::record) {
        install(&mut state, r.clone());
    }
    for op in &out {
        if let Operation::Capture { record, content } = op {
            state
                .artifacts
                .insert(record.reference.key(), content.clone());
        }
    }
    let baseline = Record::new(
        reference("ModelBaseline"),
        type_ref("ModelBaseline"),
        RecordData::Baseline(state.baseline_data()?),
        vec![anchor()],
    )?;
    out.push(Operation::Put(baseline));
    // Late observation records are expressly synthetic specimens, never loaded into the real self-model.
    let c = builtin_catalogue()?;
    for kind in ["Release", "TestRun", "AcceptanceRecord"] {
        let t = c
            .records
            .iter()
            .find_map(|r| {
                if let RecordData::Type(t) = &r.data {
                    (t.name == kind).then_some(t)
                } else {
                    None
                }
            })
            .unwrap();
        let mut p = t
            .properties
            .iter()
            .filter(|(_, p)| p.required)
            .map(|(n, p)| (n.clone(), specimen_value(&p.value, structures)))
            .collect::<BTreeMap<_, _>>();
        p.insert(
            "name".into(),
            Value::text(format!("Synthetic {kind}; not an observed project result")),
        );
        if kind == "TestRun" {
            p.insert("build".into(), Value::Artifact(reference("observed-build")));
            p.insert(
                "evidence".into(),
                Value::List(vec![Value::Artifact(reference("observed-log"))]),
            );
        }
        if kind == "AcceptanceRecord" {
            p.insert(
                "assessor".into(),
                Value::text("specimen:synthetic-assessor"),
            );
        }
        out.push(Operation::Put(Record::new(
            reference(kind),
            type_ref(kind),
            RecordData::Element(ElementData::new(p)),
            vec![anchor()],
        )?));
    }
    out.extend(late);
    Ok(out)
}
pub fn specimen_state() -> State {
    let mut s = catalogue_state();
    for op in specimens().unwrap() {
        if let Some(r) = op.record() {
            install(&mut s, r.clone());
        }
        if let Operation::Capture { record, content } = op {
            s.artifacts.insert(record.reference.key(), content);
        }
    }
    s
}

/// Explicit authoring helper: caller retains the complete coherent intent before commit.
/// Does not change reviews or observed evidence attached to earlier exact revisions.
pub fn successor_operations(
    state: &State,
    mut replacements: Vec<Record>,
) -> Result<Vec<Operation>, Error> {
    let mut mapping = replacements
        .iter()
        .filter_map(|r| r.predecessor.clone().map(|p| (p, r.reference.clone())))
        .collect::<BTreeMap<_, _>>();
    loop {
        let mut changed = false;
        for r in state.selected_records() {
            if mapping.contains_key(&r.reference) {
                continue;
            }
            let affected = match &r.data {
                RecordData::Assertion(a) => {
                    mapping.contains_key(&a.from) || mapping.contains_key(&a.to)
                }
                RecordData::Representation(b) => mapping.contains_key(&b.subject),
                _ => false,
            };
            if affected {
                let mut next = r.clone();
                next.reference.revision = format!("{}-successor", r.reference.revision);
                next.predecessor = Some(r.reference.clone());
                mapping.insert(r.reference.clone(), next.reference.clone());
                replacements.push(next);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    for r in &mut replacements {
        match &mut r.data {
            RecordData::Assertion(a) => {
                if let Some(v) = mapping.get(&a.from) {
                    a.from = v.clone();
                }
                if let Some(v) = mapping.get(&a.to) {
                    a.to = v.clone();
                }
            }
            RecordData::Representation(b) => {
                if let Some(v) = mapping.get(&b.subject) {
                    b.subject = v.clone();
                }
            }
            _ => {}
        }
        r.seal()?;
    }
    Ok(replacements.into_iter().map(Operation::Put).collect())
}
