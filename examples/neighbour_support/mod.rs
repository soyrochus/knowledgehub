//! Synthetic registered vocabulary for neighbour conformance. No project acceptance is modelled.
#![allow(dead_code)]
use knowledgehub::{
    Action, Content, Context, Error, Policy, PolicyFailure, Receipt, Scope, backends::files::*,
    ontology::*,
};
use std::{collections::BTreeMap, path::Path, sync::Arc};
pub struct Permit;
impl Policy for Permit {
    fn allows(&self, _: &str, _: &Scope, _: Action) -> Result<bool, PolicyFailure> {
        Ok(true)
    }
}
pub fn context() -> Context {
    Context::new("fixture:neighbour-author", Arc::new(Permit))
}
pub fn scope() -> ModelScope {
    ModelScope {
        hub: "hub:neighbour-fixture".into(),
        model: "model:neighbour-fixture".into(),
        branch: "main".into(),
        owner: "owner:neighbour-fixture".into(),
    }
}
pub fn reference(id: &str) -> ExactRef {
    ExactRef::new("neighbour-fixture", id, "1")
}
pub fn node_type() -> ExactRef {
    reference("type:Node")
}
pub fn special_type() -> ExactRef {
    reference("type:Special")
}
pub fn relation() -> ExactRef {
    reference("signature:links")
}
pub fn anchor() -> ExactRef {
    ExactRef::new("sdo", "sdo:anchor:catalogue", "catalogue-1")
}
pub fn backend(path: &Path) -> FileBackend {
    FileBackend::new_shared_ontology(path, FileOptions::local_ext4()).unwrap()
}
pub async fn commit(
    h: &OntologyHub,
    id: &str,
    operations: Vec<Operation>,
) -> Result<(Change, Receipt), Error> {
    let (head, state) = h.snapshot(context()).await?;
    let mut c = Change::at(&state, head, id);
    c.operations = operations;
    let receipt = h.commit(context(), c.clone()).await?;
    Ok((c, receipt))
}
pub fn vocabulary() -> Vec<Operation> {
    let catalogue = builtin_catalogue().unwrap();
    let mut ontology = catalogue
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
    let citation = ontology.citation.clone();
    let properties = BTreeMap::from([
        ("name".into(), Property::required(ValueKind::Text)),
        (
            "links".into(),
            Property::required(ValueKind::List {
                item: Box::new(ValueKind::Record(BTreeMap::from([
                    (
                        "ref/key".into(),
                        Property::required(ValueKind::Reference(vec!["AnySubject".into()])),
                    ),
                    ("tagged".into(), Property::required(ValueKind::Any)),
                ]))),
                min: 0,
            }),
        ),
        (
            "any".into(),
            Property {
                nullable: true,
                ..Property::required(ValueKind::Any)
            },
        ),
        (
            "nested".into(),
            Property::required(ValueKind::Structured("NeighbourBox".into())),
        ),
        (
            "choice".into(),
            Property::required(ValueKind::Union(vec![
                ValueKind::Text,
                ValueKind::Record(BTreeMap::from([(
                    "target".into(),
                    Property::required(ValueKind::Reference(vec!["AnySubject".into()])),
                )])),
            ])),
        ),
        ("literal".into(), Property::required(ValueKind::Text)),
    ]);
    let t = TypeDefinition {
        name: "NeighbourNode".into(),
        family: "neighbour-fixture".into(),
        coverage: "fixture:node".into(),
        native: false,
        parents: vec![],
        properties,
        constraints: vec![],
        citation: citation.clone(),
    };
    let special = TypeDefinition {
        name: "SpecialNeighbourNode".into(),
        coverage: "fixture:special".into(),
        parents: vec![node_type()],
        properties: BTreeMap::new(),
        ..t.clone()
    };
    ontology.types.extend([node_type(), special_type()]);
    ontology.relationships.push(relation());
    ontology.structures.insert(
        "NeighbourBox".into(),
        ValueKind::Record(BTreeMap::from([(
            "next".into(),
            Property::required(ValueKind::Reference(vec!["AnySubject".into()])),
        )])),
    );
    ontology.name = "Synthetic neighbour vocabulary".into();
    ontology.profiles = vec![reference("profile")];
    let profile = ProfileDefinition {
        name: "synthetic-neighbours".into(),
        ontology: reference("ontology"),
        vocabulary: ontology.types.clone(),
        allow_gaps: true,
        required_types: vec![],
        inapplicable: BTreeMap::new(),
        citation: citation.clone(),
    };
    let signature = RelationshipDefinition {
        name: "recordedLink".into(),
        coverage: "fixture:link".into(),
        from: vec!["NeighbourNode".into()],
        to: vec!["NeighbourNode".into()],
        min_complete: 0,
        max: None,
        acyclic: false,
        qualifiers: BTreeMap::from([(
            "context".into(),
            Property {
                required: false,
                nullable: true,
                value: ValueKind::Any,
                owner: PropertyOwner::Graph,
            },
        )]),
        citation,
    };
    [
        (node_type(), "ElementTypeDefinition", RecordData::Type(t)),
        (
            special_type(),
            "ElementTypeDefinition",
            RecordData::Type(special),
        ),
        (
            relation(),
            "RelationshipTypeDefinition",
            RecordData::Relationship(signature),
        ),
        (
            reference("ontology"),
            "OntologyDefinition",
            RecordData::Ontology(ontology),
        ),
        (
            reference("profile"),
            "ProfileDefinition",
            RecordData::Profile(profile),
        ),
    ]
    .into_iter()
    .map(|(r, k, d)| Operation::Put(Record::new(r, type_ref(k), d, vec![anchor()]).unwrap()))
    .chain([Operation::SelectProfile {
        ontology: reference("ontology"),
        profile: reference("profile"),
    }])
    .collect()
}
pub fn node(id: &str, target: &str, linked: bool) -> Record {
    let target = reference(target);
    let properties = BTreeMap::from([
        ("name".into(), Value::text(format!("Synthetic node {id}"))),
        (
            "links".into(),
            Value::List(if linked {
                vec![
                    Value::Record(BTreeMap::from([
                        ("ref/key".into(), Value::Ref(target.clone())),
                        ("tagged".into(), Value::text(target.id.clone()))
                    ]));
                    2
                ]
            } else {
                vec![]
            }),
        ),
        (
            "any".into(),
            if linked {
                Value::List(vec![
                    Value::Ref(target.clone()),
                    Value::Ref(target.clone()),
                    Value::text(target.id.clone()),
                ])
            } else {
                Value::Null
            },
        ),
        (
            "nested".into(),
            Value::Record(BTreeMap::from([(
                "next".into(),
                Value::Ref(target.clone()),
            )])),
        ),
        (
            "choice".into(),
            if linked {
                Value::Record(BTreeMap::from([(
                    "target".into(),
                    Value::Ref(target.clone()),
                )]))
            } else {
                Value::text("no reference in this union alternative")
            },
        ),
        ("literal".into(), Value::text(target.id)),
    ]);
    Record::new(
        reference(id),
        node_type(),
        RecordData::Element(ElementData::new(properties)),
        vec![anchor()],
    )
    .unwrap()
}
pub fn edge(id: &str, from: &str, to: &str) -> Record {
    Record::new(
        reference(id),
        type_ref("RelationshipAssertion"),
        RecordData::Assertion(AssertionData {
            signature: relation(),
            from: reference(from),
            to: reference(to),
            qualifiers: BTreeMap::new(),
        }),
        vec![anchor()],
    )
    .unwrap()
}
pub async fn create(path: &Path) -> Result<(FileBackend, OntologyHub), Error> {
    populate(backend(path)).await
}
pub async fn populate(b: FileBackend) -> Result<(FileBackend, OntologyHub), Error> {
    let h = OntologyHub::create(b.clone(), context(), scope()).await?;
    commit(&h, "catalogue", builtin_catalogue()?.operations()).await?;
    commit(&h, "vocabulary", vocabulary()).await?;
    commit(
        &h,
        "endpoints",
        vec![
            Operation::Put(node("a", "b", true)),
            Operation::Put(node("b", "b", false)),
            Operation::Put(node("c", "a", false)),
        ],
    )
    .await?;
    Ok((b, h))
}
pub async fn baseline(h: &OntologyHub, name: &str) -> Result<ExactRef, Error> {
    let (head, state) = h.snapshot(context()).await?;
    let reference = reference(&format!("baseline:{name}"));
    let mut c = Change::at(&state, head, format!("baseline:{name}"));
    c.operations.push(Operation::Put(Record::new(
        reference.clone(),
        type_ref("ModelBaseline"),
        RecordData::Baseline(state.baseline_data()?),
        vec![anchor()],
    )?));
    h.commit(context(), c).await?;
    Ok(reference)
}
pub fn query(selection: Selection, subject: &str) -> Neighbours {
    Neighbours::new(selection, vec![reference(subject)])
}
pub async fn collect(
    h: &OntologyHub,
    q: Neighbours,
) -> Result<(Vec<Neighbour>, NeighbourWork), Error> {
    let mut p = h.neighbours(context(), q.clone()).await?;
    let mut all = p.neighbours;
    let mut work = p.work;
    while let Some(cursor) = p.continuation {
        p = h.continue_neighbours(context(), q.clone(), cursor).await?;
        all.extend(p.neighbours);
        work.index_objects_read += p.work.index_objects_read;
        work.index_bytes_read += p.work.index_bytes_read;
        work.record_reads += p.work.record_reads;
        work.examined_postings += p.work.examined_postings;
        work.postings_read += p.work.postings_read;
    }
    Ok((all, work))
}
pub fn binary(id: &str, bytes: Vec<u8>) -> Vec<Operation> {
    let source = Record::new(
        reference(&format!("source:{id}")),
        type_ref("SourceArtifact"),
        RecordData::SourceArtifact(SourceArtifactData {
            origin: "synthetic neighbour fixture".into(),
            media_type: "application/octet-stream".into(),
            owner: "fixture:host".into(),
            status: "observation".into(),
        }),
        vec![],
    )
    .unwrap();
    let content = Content::capture(bytes);
    let capture = Record::new(
        reference(&format!("capture:{id}")),
        type_ref("SourceRevision"),
        RecordData::SourceRevision(SourceRevisionData {
            source: source.reference.clone(),
            original: content.reference.clone(),
            media_type: "application/octet-stream".into(),
            derived_from: None,
            derivation: None,
        }),
        vec![],
    )
    .unwrap();
    vec![
        Operation::Put(source),
        Operation::Capture {
            record: capture,
            content,
        },
    ]
}
