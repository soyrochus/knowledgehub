//! Fictional rent model, isolated from the canonical self-description.
#![allow(dead_code)]
use crate::{fixture, specimens};
use knowledgehub::{Content, Error, backends::files::*, ontology::*};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
pub fn reference(id: &str) -> ExactRef {
    ExactRef::new("rent-fixture", id, "1")
}
pub fn physical_type() -> ExactRef {
    reference("type:PhysicalTable")
}
pub fn scope() -> ModelScope {
    ModelScope {
        hub: "hub:rent-fixture".into(),
        model: "model:rent-fixture".into(),
        branch: "main".into(),
        owner: "owner:conformance".into(),
    }
}
pub fn vocabulary() -> Vec<Operation> {
    let catalogue = builtin_catalogue().unwrap();
    let mut o = catalogue
        .records
        .iter()
        .find_map(|r| {
            if let RecordData::Ontology(o) = &r.data {
                Some(o.clone())
            } else {
                None
            }
        })
        .unwrap();
    let t = TypeDefinition {
        name: "RentFixturePhysicalTable".into(),
        family: "fixture physical classification".into(),
        coverage: "fixture:physical-table".into(),
        native: false,
        parents: vec![type_ref("DataSchema")],
        properties: ["database", "schema", "table"]
            .into_iter()
            .map(|p| (p.into(), Property::required(ValueKind::Text)))
            .collect(),
        constraints: vec![],
        citation: o.citation.clone(),
    };
    o.types.push(physical_type());
    o.profiles = vec![reference("profile")];
    o.name = "Isolated rent traceability fixture".into();
    let p = ProfileDefinition {
        name: "rent-fixture".into(),
        ontology: reference("ontology"),
        vocabulary: o.types.clone(),
        allow_gaps: true,
        required_types: vec![],
        inapplicable: BTreeMap::new(),
        citation: o.citation.clone(),
    };
    [
        (
            physical_type(),
            "ElementTypeDefinition",
            RecordData::Type(t),
        ),
        (
            reference("ontology"),
            "OntologyDefinition",
            RecordData::Ontology(o),
        ),
        (
            reference("profile"),
            "ProfileDefinition",
            RecordData::Profile(p),
        ),
    ]
    .into_iter()
    .map(|(r, t, d)| {
        Operation::Put(Record::new(r, type_ref(t), d, vec![specimens::anchor()]).unwrap())
    })
    .chain([Operation::SelectProfile {
        ontology: reference("ontology"),
        profile: reference("profile"),
    }])
    .collect()
}
pub fn capture(id: &str, text: &str) -> Vec<Operation> {
    let source = Record::new(
        reference(&format!("source:{id}")),
        type_ref("SourceArtifact"),
        RecordData::SourceArtifact(SourceArtifactData {
            origin: format!("fictional:{id}"),
            media_type: "text/plain".into(),
            owner: "fixture:author".into(),
            status: "authored-requirement".into(),
        }),
        vec![],
    )
    .unwrap();
    let content = Content::capture(text.as_bytes().to_vec());
    let r = Record::new(
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
    )
    .unwrap();
    vec![
        Operation::Put(source),
        Operation::Capture { record: r, content },
    ]
}
pub fn selector(subject: ExactRef, path: &str) -> Value {
    Value::Record(BTreeMap::from([
        ("subject".into(), Value::Ref(subject)),
        ("path".into(), Value::text(path)),
    ]))
}
pub fn mapped_table(state: &State, id: &str, table: &str) -> (Record, Record) {
    let mut r = state
        .get(&specimens::reference("DataSchema"))
        .unwrap()
        .clone();
    r.reference = reference(id);
    r.type_ref = physical_type();
    for (k, v) in [
        ("name", table),
        ("database", "rental"),
        ("schema", "public"),
        ("table", table),
    ] {
        specimens::put_property(&mut r, k, Value::text(v));
    }
    specimens::put_property(&mut r, "native_artifact", Value::Artifact(reference("ddl")));
    let mut m = state
        .get(&specimens::reference("MappingRule"))
        .unwrap()
        .clone();
    m.reference = reference(&format!("mapping:{id}"));
    specimens::put_property(
        &mut m,
        "source",
        selector(specimens::reference("EntityType"), "name"),
    );
    specimens::put_property(&mut m, "target", selector(r.reference.clone(), "table"));
    (r, m)
}
pub async fn create(path: &Path) -> Result<(FileBackend, OntologyHub), Error> {
    let b = FileBackend::new_shared_ontology(path, FileOptions::local_ext4())?;
    let h = OntologyHub::create(b.clone(), fixture::context(), scope()).await?;
    fixture::commit(&h, "catalogue", builtin_catalogue()?.operations()).await?;
    fixture::commit(&h, "fixture-vocabulary", vocabulary()).await?;
    let mut ops = specimens::specimens()?;
    // Real captured fixture prose, searched through its exact narrative anchor.
    let prose = "Préface\nAs a User I want to be able to INCREASE THE RENT, so agreed rent remains accurate.\n";
    ops.extend(capture("story-document", prose));
    ops.extend(capture(
        "ddl",
        "CREATE TABLE public.rent_schedule (rent_amount DECIMAL NOT NULL);",
    ));
    let start = "Préface\n".len() as u64;
    let end = prose.len() as u64;
    ops.push(Operation::Put(Record::new(
        reference("story-anchor"),
        type_ref("SourceAnchor"),
        RecordData::Anchor(AnchorData {
            source: reference("story-document"),
            label: "fictional rent story".into(),
            locator: Locator::ByteRange {
                start,
                end,
                quote: prose[start as usize..].into(),
            },
        }),
        vec![],
    )?));
    for op in &mut ops {
        if let Operation::Put(r) = op {
            if r.reference == specimens::reference("UserStory") {
                specimens::put_property(
                    r,
                    "need",
                    Value::Narrative {
                        document: reference("story-document"),
                        fragment: reference("story-anchor"),
                    },
                );
            }
        }
    }
    let state = specimens::specimen_state();
    let (table, mapping) = mapped_table(&state, "rent-table", "rent_schedule");
    ops.extend([Operation::Put(table), Operation::Put(mapping)]);
    // The specimen baseline must include our changed narrative and custom vocabulary.
    let (_, mut baseline_state) = h.snapshot(fixture::context()).await?;
    for op in &ops {
        if let Some(r) = op.record() {
            if r.reference != specimens::reference("edge:R28")
                && !["ModelBaseline", "TestRun", "Release", "AcceptanceRecord"]
                    .iter()
                    .any(|k| r.type_ref == type_ref(k))
            {
                specimens::install(&mut baseline_state, r.clone());
            }
        }
        if let Operation::Capture { record, content } = op {
            baseline_state
                .artifacts
                .insert(record.reference.key(), content.clone());
        }
    }
    for op in &mut ops {
        if let Operation::Put(r) = op {
            if r.reference == specimens::reference("ModelBaseline") {
                r.data = RecordData::Baseline(baseline_state.baseline_data()?);
                r.seal()?;
            }
        }
    }
    fixture::commit(&h, "rent-model", ops).await?;
    Ok((b, h))
}
fn assertion(source: &str, verb: &str, direction: NeighbourDirection) -> TraversalStep {
    TraversalStep {
        direction,
        selectors: BTreeSet::from([NeighbourSelector::Assertion {
            definition: relationship_ref(source, verb),
        }]),
        categories: BTreeSet::from([NeighbourCategory::Association]),
        opposite_type: None,
        include_subtypes: false,
    }
}
fn property(path: &str, direction: NeighbourDirection) -> TraversalStep {
    TraversalStep {
        direction,
        selectors: BTreeSet::from([NeighbourSelector::Property {
            definition: type_ref("MappingRule"),
            path: path.into(),
        }]),
        categories: BTreeSet::from([NeighbourCategory::Property]),
        opposite_type: None,
        include_subtypes: false,
    }
}
pub fn query() -> ModelImpact {
    ModelImpact {
        selection: None,
        starts: ImpactStarts::Text {
            selector: TextSelector {
                phrase: "increase the rent".into(),
                mode: LiteralMode::AsciiFold,
                fields: BTreeSet::from([TextField {
                    type_ref: type_ref("UserStory"),
                    path: "/need".into(),
                }]),
            },
        },
        match_policy: MatchPolicy::RequireOne,
        plan: TraversalPlan::Ordered {
            steps: vec![
                vec![assertion(
                    "Requirement",
                    "formalises",
                    NeighbourDirection::Incoming,
                )],
                vec![assertion(
                    "UseCase",
                    "satisfies",
                    NeighbourDirection::Incoming,
                )],
                vec![assertion(
                    "UseCase",
                    "invokes",
                    NeighbourDirection::Outgoing,
                )],
                vec![assertion(
                    "FunctionDefinition",
                    "operatesOn",
                    NeighbourDirection::Outgoing,
                )],
                vec![assertion(
                    "AggregateDefinition",
                    "hasRoot",
                    NeighbourDirection::Outgoing,
                )],
                vec![property("/source/subject", NeighbourDirection::Incoming)],
                vec![property("/target/subject", NeighbourDirection::Outgoing)],
            ],
        },
        target: TargetPredicate {
            type_ref: Some(physical_type()),
            ..Default::default()
        },
        fact_scope: FactScope::Selected,
        projection: ImpactProjection {
            target_records: true,
            witness_records: false,
            snippets: true,
        },
        budgets: ReadBudgets::default(),
    }
}
