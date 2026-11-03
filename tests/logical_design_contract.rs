#[path = "../examples/support/mod.rs"]
mod seed;
#[path = "../examples/ontology_support/mod.rs"]
mod specimens;
mod support;
use knowledgehub::{ontology::*, *};
use std::collections::BTreeMap;

fn r(id: &str) -> ExactRef {
    ExactRef::new("reservation", id, "1")
}
fn put(s: &mut State, v: Record) {
    specimens::install(s, v);
}
fn field(s: &mut State, reference: &ExactRef, key: &str, v: Option<Value>) {
    let rec = s.records.get_mut(&reference.key()).unwrap();
    let RecordData::Element(e) = &mut rec.data else {
        panic!()
    };
    if let Some(v) = v {
        e.properties.insert(key.into(), v);
    } else {
        e.properties.remove(key);
    }
    e.owners = ElementData::new(e.properties.clone()).owners;
    rec.seal().unwrap();
}
fn extension(
    s: &mut State,
    id: &str,
    parents: Vec<ExactRef>,
    properties: BTreeMap<String, Property>,
) -> ExactRef {
    let mut t = s.type_definition(&type_ref("Requirement")).unwrap().clone();
    t.name = id.into();
    t.coverage = format!("fixture:{id}");
    t.parents = parents;
    t.properties = properties;
    t.constraints.clear();
    let reference = r(id);
    put(
        s,
        Record::new(
            reference.clone(),
            type_ref("ElementTypeDefinition"),
            RecordData::Type(t),
            vec![specimens::anchor()],
        )
        .unwrap(),
    );
    reference
}
fn select_extensions(s: &mut State, types: Vec<ExactRef>, relationships: Vec<ExactRef>) {
    let RecordData::Ontology(mut o) = s.get(&ontology_ref()).unwrap().data.clone() else {
        panic!()
    };
    o.types.extend(types);
    o.relationships.extend(relationships);
    o.profiles = vec![r("profile")];
    let mut p = s.profile_definition().unwrap().clone();
    p.ontology = r("ontology");
    p.vocabulary = o.types.clone();
    put(
        s,
        Record::new(
            r("profile"),
            type_ref("ProfileDefinition"),
            RecordData::Profile(p),
            vec![specimens::anchor()],
        )
        .unwrap(),
    );
    put(
        s,
        Record::new(
            r("ontology"),
            type_ref("OntologyDefinition"),
            RecordData::Ontology(o),
            vec![specimens::anchor()],
        )
        .unwrap(),
    );
    s.profile = Some(r("profile"));
    s.ontology = Some(r("ontology"));
}
fn instance(s: &mut State, schema: ExactRef, id: &str, kind: &str) -> ExactRef {
    let mut rec = s.get(&specimens::reference(kind)).unwrap().clone();
    rec.reference = r(id);
    rec.type_ref = schema;
    rec.seal().unwrap();
    put(s, rec);
    r(id)
}

#[test]
fn exact_selectors_diamonds_conflicts_cycles_and_native_boundaries() {
    let mut s = specimens::specimen_state();
    let base = extension(
        &mut s,
        "base",
        vec![type_ref("Requirement")],
        BTreeMap::new(),
    );
    let left = extension(&mut s, "left", vec![base.clone()], BTreeMap::new());
    let right = extension(&mut s, "right", vec![base.clone()], BTreeMap::new());
    let diamond = extension(
        &mut s,
        "diamond",
        vec![left.clone(), right.clone()],
        BTreeMap::new(),
    );
    let holder = extension(
        &mut s,
        "holder",
        vec![],
        BTreeMap::from([(
            "target".into(),
            Property::required(ValueKind::Reference(vec![TypeSelector::Type(base.clone())])),
        )]),
    );
    select_extensions(
        &mut s,
        vec![
            base.clone(),
            left.clone(),
            right.clone(),
            diamond.clone(),
            holder.clone(),
        ],
        vec![],
    );
    let value = instance(&mut s, diamond.clone(), "booking-rule", "Requirement");
    put(
        &mut s,
        Record::new(
            r("holder-value"),
            holder.clone(),
            RecordData::Element(ElementData::new(BTreeMap::from([(
                "target".into(),
                Value::Ref(value.clone()),
            )]))),
            vec![specimens::anchor()],
        )
        .unwrap(),
    );
    assert!(assess(&s).is_complete(), "{:#?}", assess(&s));
    for schema in [base.clone(), left.clone(), diamond.clone()] {
        let mut q = s.clone();
        let v = instance(&mut q, schema, "candidate", "Requirement");
        field(&mut q, &r("holder-value"), "target", Some(Value::Ref(v)));
        assert!(assess(&q).is_complete());
    }
    let mut q = s.clone();
    field(
        &mut q,
        &r("holder-value"),
        "target",
        Some(Value::Ref(specimens::reference("Requirement"))),
    );
    assert!(!assess(&q).is_valid()); // reverse base-for-descendant assignment
    let mut q = s.clone();
    let same = extension(&mut q, "same-name", vec![], BTreeMap::new());
    if let RecordData::Type(t) = &mut q.records.get_mut(&same.key()).unwrap().data {
        t.name = "base".into();
    }
    q.records.get_mut(&same.key()).unwrap().seal().unwrap();
    select_extensions(
        &mut q,
        vec![
            base.clone(),
            left.clone(),
            right.clone(),
            diamond.clone(),
            holder.clone(),
            same.clone(),
        ],
        vec![],
    );
    put(
        &mut q,
        Record::new(
            r("impostor"),
            same,
            RecordData::Element(ElementData::new(BTreeMap::new())),
            vec![specimens::anchor()],
        )
        .unwrap(),
    );
    field(
        &mut q,
        &r("holder-value"),
        "target",
        Some(Value::Ref(r("impostor"))),
    );
    assert!(!assess(&q).is_valid());
    for (name, parents, props) in [
        (
            "shadow",
            vec![base.clone()],
            BTreeMap::from([("name".into(), Property::required(ValueKind::Text))]),
        ),
        ("unresolved", vec![r("missing")], BTreeMap::new()),
        (
            "native-child",
            vec![type_ref("SourceAnchor")],
            BTreeMap::new(),
        ),
        ("cycle", vec![r("cycle")], BTreeMap::new()),
    ] {
        let mut q = s.clone();
        let t = extension(&mut q, name, parents, props);
        assert!(!assess(&q).is_valid(), "accepted {t:?}");
    }
    let mut q = s.clone();
    let a = extension(
        &mut q,
        "conflict-a",
        vec![],
        BTreeMap::from([("same".into(), Property::required(ValueKind::Text))]),
    );
    let b = extension(
        &mut q,
        "conflict-b",
        vec![],
        BTreeMap::from([("same".into(), Property::required(ValueKind::Text))]),
    );
    extension(&mut q, "conflict", vec![a, b], BTreeMap::new());
    assert!(!assess(&q).is_valid());
    let mut q = s.clone();
    field(&mut q, &value, "name", None);
    assert!(!assess(&q).is_valid());
    let obsolete = br#"{"kind":"Reference","schema":["Requirement"]}"#;
    assert!(serde_json::from_slice::<ValueKind>(obsolete).is_err());
}

#[test]
fn inherited_semantics_both_endpoints_and_local_constraints() {
    for kind in [
        "AggregateDefinition",
        "LifecycleDefinition",
        "ActionBinding",
        "DataSchema",
    ] {
        let mut s = specimens::specimen_state();
        let t = extension(
            &mut s,
            &format!("special-{kind}"),
            vec![type_ref(kind)],
            BTreeMap::new(),
        );
        select_extensions(&mut s, vec![t.clone()], vec![]);
        let v = instance(&mut s, t, "special-value", kind);
        if kind == "ActionBinding" {
            // A valid placement with its binding list empty fails reciprocal semantics.
            field(
                &mut s,
                &specimens::reference("ComponentPlacement"),
                "bindings",
                Some(Value::List(vec![])),
            );
        }
        assert!(!assess(&s).is_valid(), "subtype escaped {kind}: {v:?}");
    }
    let mut s = specimens::specimen_state();
    let sub = extension(
        &mut s,
        "requirement-subtype",
        vec![type_ref("Requirement")],
        BTreeMap::new(),
    );
    let local = r("local-constraint");
    let citation = s
        .type_definition(&type_ref("Requirement"))
        .unwrap()
        .citation
        .clone();
    put(
        &mut s,
        Record::new(
            local.clone(),
            type_ref("ConstraintDefinition"),
            RecordData::Constraint(ConstraintDefinition {
                name: "maximum-rank".into(),
                coverage: "fixture".into(),
                rule: Constraint::Range {
                    property: "rank".into(),
                    min: "0".into(),
                    max: "2".into(),
                },
                citation: citation.clone(),
            }),
            vec![specimens::anchor()],
        )
        .unwrap(),
    );
    let ranked = extension(
        &mut s,
        "ranked",
        vec![sub.clone()],
        BTreeMap::from([("rank".into(), Property::required(ValueKind::Integer))]),
    );
    if let RecordData::Type(t) = &mut s.records.get_mut(&ranked.key()).unwrap().data {
        t.constraints.push(local);
    }
    s.records.get_mut(&ranked.key()).unwrap().seal().unwrap();
    let descendant = extension(
        &mut s,
        "ranked-descendant",
        vec![ranked.clone()],
        BTreeMap::new(),
    );
    let signature = r("links");
    put(
        &mut s,
        Record::new(
            signature.clone(),
            type_ref("RelationshipTypeDefinition"),
            RecordData::Relationship(RelationshipDefinition {
                name: "links".into(),
                coverage: "fixture".into(),
                from: vec![TypeSelector::Type(sub.clone())],
                to: vec![TypeSelector::Type(sub.clone())],
                min_complete: 0,
                max: None,
                acyclic: false,
                qualifiers: BTreeMap::new(),
                citation,
            }),
            vec![specimens::anchor()],
        )
        .unwrap(),
    );
    select_extensions(
        &mut s,
        vec![sub, ranked, descendant.clone()],
        vec![signature.clone()],
    );
    let a = instance(&mut s, descendant.clone(), "a", "Requirement");
    let b = instance(&mut s, descendant, "b", "Requirement");
    for v in [&a, &b] {
        field(&mut s, v, "rank", Some(Value::Integer("1".into())));
    }
    put(
        &mut s,
        Record::new(
            r("edge"),
            type_ref("RelationshipAssertion"),
            RecordData::Assertion(AssertionData {
                signature,
                from: a.clone(),
                to: b.clone(),
                qualifiers: BTreeMap::new(),
            }),
            vec![specimens::anchor()],
        )
        .unwrap(),
    );
    assert!(assess(&s).is_complete(), "{:#?}", assess(&s));
    let mut invalid = s.clone();
    field(&mut invalid, &b, "rank", Some(Value::Integer("3".into())));
    assert!(!assess(&invalid).is_valid());
    for source in [true, false] {
        let mut invalid = s.clone();
        let record = invalid.records.get_mut(&r("edge").key()).unwrap();
        if let RecordData::Assertion(a) = &mut record.data {
            if source {
                a.from = specimens::reference("Actor");
            } else {
                a.to = specimens::reference("Actor");
            }
        }
        record.seal().unwrap();
        assert!(!assess(&invalid).is_valid());
    }
}

#[test]
fn logical_constructors_subjects_planned_realization_and_precise_gaps() {
    let accepted = specimens::specimen_state();
    let lifecycle = specimens::reference("LifecycleDefinition");
    for v in [
        Value::Ref(type_ref("EntityType")),
        Value::List(vec![]),
        Value::Ref(r("missing")),
        Value::Ref(specimens::reference("DataSchema")),
    ] {
        let mut s = accepted.clone();
        field(&mut s, &lifecycle, "subject", Some(v));
        assert!(!assess(&s).is_valid());
    }
    for target in ["EntityType", "AggregateDefinition"] {
        let mut s = accepted.clone();
        field(
            &mut s,
            &lifecycle,
            "subject",
            Some(Value::Ref(specimens::reference(target))),
        );
        assert!(assess(&s).is_complete());
    }
    for (kind, field_name, value) in [
        (
            "enumeration",
            "enum_values",
            Value::Set(vec![Value::text("available"), Value::text("booked")]),
        ),
        (
            "record",
            "record_schema",
            Value::Ref(specimens::reference("DataSchema")),
        ),
        (
            "list",
            "item_type",
            Value::Ref(specimens::reference("DataType")),
        ),
        (
            "set",
            "item_type",
            Value::Ref(specimens::reference("DataType")),
        ),
        (
            "reference",
            "reference_targets",
            Value::Set(vec![Value::Ref(specimens::reference("EntityType"))]),
        ),
    ] {
        let mut s = accepted.clone();
        let datatype = instance(&mut s, type_ref("DataType"), "constructor", "DataType");
        field(
            &mut s,
            &datatype,
            "value_kind",
            Some(Value::enumeration(kind)),
        );
        field(&mut s, &datatype, field_name, Some(value));
        assert!(assess(&s).is_complete(), "{kind}: {:#?}", assess(&s));
        let mut missing = s.clone();
        field(&mut missing, &datatype, field_name, None);
        assert!(!assess(&missing).is_valid());
        let mut wrong = s;
        field(&mut wrong, &datatype, field_name, Some(Value::Null));
        assert!(!assess(&wrong).is_valid());
    }
    for kind in ["date", "timestamp", "duration"] {
        let mut s = accepted.clone();
        let dt = instance(&mut s, type_ref("DataType"), "temporal", "DataType");
        field(&mut s, &dt, "value_kind", Some(Value::enumeration(kind)));
        assert!(assess(&s).is_complete());
    }
    let mut s = accepted.clone();
    let datatype = specimens::reference("DataType");
    field(
        &mut s,
        &specimens::reference("DesignToken"),
        "value",
        Some(Value::Ref(specimens::reference("EntityType"))),
    );
    field(
        &mut s,
        &datatype,
        "value_kind",
        Some(Value::enumeration("reference")),
    );
    field(
        &mut s,
        &datatype,
        "reference_targets",
        Some(Value::Set(vec![Value::Ref(specimens::reference(
            "EntityType",
        ))])),
    );
    field(
        &mut s,
        &specimens::reference("FieldDefinition"),
        "datatype",
        Some(Value::Ref(datatype.clone())),
    );
    field(
        &mut s,
        &specimens::reference("FieldDefinition"),
        "reference_targets",
        Some(Value::Set(vec![Value::Ref(specimens::reference(
            "EntityType",
        ))])),
    );
    assert!(assess(&s).is_complete(), "{:#?}", assess(&s));
    field(
        &mut s,
        &specimens::reference("FieldDefinition"),
        "reference_targets",
        Some(Value::Set(vec![Value::Ref(specimens::reference(
            "ValueObjectType",
        ))])),
    );
    assert!(!assess(&s).is_valid());
    for (kind, artifact) in [("Module", "artifacts"), ("Adapter", "implementation")] {
        let mut s = accepted.clone();
        let reference = specimens::reference(kind);
        field(&mut s, &reference, artifact, None);
        assert!(assess(&s).is_complete()); // planned is the catalogue specimen default
        field(
            &mut s,
            &reference,
            "implementation_status",
            Some(Value::enumeration("implemented")),
        );
        assert!(!assess(&s).is_valid());
        s.gaps.push(Gap {
            subject: reference.clone(),
            path: format!("/{artifact}"),
            reason: "Source supplies design only".into(),
            provenance: vec![specimens::anchor()],
        });
        let gaps = s.gaps.clone();
        let baseline = s
            .records
            .get_mut(&specimens::reference("ModelBaseline").key())
            .unwrap();
        if let RecordData::Baseline(b) = &mut baseline.data {
            b.gaps = gaps;
        }
        baseline.seal().unwrap();
        assert!(assess(&s).is_valid());
        assert!(!assess(&s).is_complete());
        let valid = s.clone();
        for malformed in [
            Value::Null,
            Value::List(vec![]),
            Value::List(vec![Value::Artifact(r("missing"))]),
            Value::text("wrong type"),
        ] {
            let mut invalid = valid.clone();
            field(&mut invalid, &reference, artifact, Some(malformed));
            assert!(!assess(&invalid).is_valid());
        }
        for mutation in 0..4 {
            let mut invalid = valid.clone();
            let gap = invalid.gaps.last_mut().unwrap();
            match mutation {
                0 => gap.path = "/not-required".into(),
                1 => gap.subject = specimens::reference("Actor"),
                2 => gap.reason = " ".into(),
                _ => gap.provenance = vec![specimens::reference("Requirement")],
            };
            assert!(!assess(&invalid).is_valid());
        }
    }
}

#[tokio::test]
async fn extension_registration_paged_inclusive_reads_exact_baselines_and_receipts() {
    let sandbox = support::Sandbox::new();
    let hub = KnowledgeHub::create(sandbox.backend(), seed::context(), specimens::scope())
        .await
        .unwrap();
    let (head, s) = hub.snapshot(seed::context()).await.unwrap();
    let mut c = Change::at(&s, head, "register");
    c.operations = builtin_catalogue().unwrap().operations();
    hub.commit(seed::context(), c).await.unwrap();
    let (head, s) = hub.snapshot(seed::context()).await.unwrap();
    let mut c = Change::at(&s, head, "model");
    c.operations = specimens::specimens().unwrap();
    hub.commit(seed::context(), c).await.unwrap();
    let (head, before) = hub.snapshot(seed::context()).await.unwrap();
    let mut after = before.clone();
    let sub = extension(
        &mut after,
        "booking-rule",
        vec![type_ref("Requirement")],
        BTreeMap::new(),
    );
    let left = extension(
        &mut after,
        "booking-left",
        vec![sub.clone()],
        BTreeMap::new(),
    );
    let right = extension(
        &mut after,
        "booking-right",
        vec![sub.clone()],
        BTreeMap::new(),
    );
    let diamond = extension(
        &mut after,
        "booking-diamond",
        vec![left.clone(), right.clone()],
        BTreeMap::new(),
    );
    select_extensions(&mut after, vec![sub, left, right, diamond.clone()], vec![]);
    instance(&mut after, diamond, "booking-requirement", "Requirement");
    let mut c = Change::at(&before, head, "extensions");
    c.operations = after
        .selected_records()
        .filter(|r| before.get(&r.reference).is_none())
        .cloned()
        .map(Operation::Put)
        .collect();
    c.operations.push(Operation::SelectProfile {
        ontology: r("ontology"),
        profile: r("profile"),
    });
    let a = r("A");
    c.operations.push(Operation::Put(
        Record::new(
            a.clone(),
            type_ref("ModelBaseline"),
            RecordData::Baseline(after.baseline_data().unwrap()),
            vec![specimens::anchor()],
        )
        .unwrap(),
    ));
    let receipt = hub.commit(seed::context(), c.clone()).await.unwrap();
    // Index preparation must resolve the same diamond as native assessment.
    let neighbours = Neighbours::new(
        Selection::Baseline(a.clone()),
        vec![r("booking-requirement")],
    );
    hub.neighbours(seed::context(), neighbours).await.unwrap();
    for inclusive in [false, true] {
        let mut q = Query::select(Selection::Baseline(a.clone()));
        q.type_ref = Some(type_ref("Requirement"));
        q.include_subtypes = inclusive;
        q.page_size = 1;
        let mut page = hub.query(seed::context(), q.clone()).await.unwrap();
        assert_eq!(page.include_subtypes, inclusive);
        let mut refs = vec![];
        loop {
            refs.extend(page.records.iter().map(|r| r.reference.clone()));
            if let Some(cursor) = page.continuation {
                page = hub
                    .continue_query(seed::context(), q.clone(), cursor)
                    .await
                    .unwrap();
            } else {
                assert!(page.exhausted);
                break;
            }
        }
        assert_eq!(refs.len(), if inclusive { 2 } else { 1 });
    }
    let (head, s) = hub.snapshot(seed::context()).await.unwrap();
    let mut revision = s
        .get(&specimens::reference("LifecycleDefinition"))
        .unwrap()
        .clone();
    revision.reference.revision = "2".into();
    revision.predecessor = Some(specimens::reference("LifecycleDefinition"));
    specimens::put_property(
        &mut revision,
        "subject",
        Value::Ref(specimens::reference("AggregateDefinition")),
    );
    let mut successor = Change::at(&s, head, "B");
    let mut replacements = vec![revision.clone()];
    for kind in ["State", "Transition"] {
        let mut next = s.get(&specimens::reference(kind)).unwrap().clone();
        next.reference.revision = "2".into();
        next.predecessor = Some(specimens::reference(kind));
        replacements.push(next);
    }
    let mapping = replacements
        .iter()
        .map(|r| (r.predecessor.clone().unwrap(), r.reference.clone()))
        .collect::<BTreeMap<_, _>>();
    fn rewrite(v: &mut Value, m: &BTreeMap<ExactRef, ExactRef>) {
        match v {
            Value::Ref(r) => {
                if let Some(n) = m.get(r) {
                    *r = n.clone();
                }
            }
            Value::List(v) | Value::Set(v) => v.iter_mut().for_each(|v| rewrite(v, m)),
            Value::Record(p) => p.values_mut().for_each(|v| rewrite(v, m)),
            _ => {}
        }
    }
    for next in &mut replacements {
        if let RecordData::Element(e) = &mut next.data {
            e.properties.values_mut().for_each(|v| rewrite(v, &mapping));
            e.owners = ElementData::new(e.properties.clone()).owners;
        }
        next.seal().unwrap();
    }
    revision = replacements[0].clone();
    successor.operations = specimens::successor_operations(&s, replacements).unwrap();
    hub.commit(seed::context(), successor).await.unwrap();
    hub.drain().await;
    let hub = KnowledgeHub::open(sandbox.backend(), seed::context(), specimens::scope())
        .await
        .unwrap();
    assert_eq!(
        hub.operation_result(seed::context(), "extensions".into())
            .await
            .unwrap(),
        OperationOutcome::Committed(Box::new(receipt.clone()))
    );
    assert_eq!(hub.commit(seed::context(), c).await.unwrap(), receipt);
    let original = hub
        .read(
            seed::context(),
            Selection::Baseline(a),
            specimens::reference("LifecycleDefinition"),
        )
        .await
        .unwrap();
    assert_eq!(
        original.element().unwrap()["subject"],
        Value::Ref(specimens::reference("EntityType"))
    );
    let (head, _) = hub.snapshot(seed::context()).await.unwrap();
    assert_eq!(
        hub.read(
            seed::context(),
            Selection::Snapshot(head),
            revision.reference.clone()
        )
        .await
        .unwrap(),
        revision
    );
}
