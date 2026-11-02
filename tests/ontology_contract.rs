#[path = "../examples/support/mod.rs"]
mod seed;
mod support;
use knowledgehub::{backends::files::*, ontology::*, *};
use support::Sandbox;
fn model_scope() -> ModelScope {
    specimens::scope()
}
fn backend(s: &Sandbox) -> FileBackend {
    FileBackend::new(&s.store, FileOptions::native_local().unwrap()).unwrap()
}
async fn register(s: &Sandbox) -> (KnowledgeHub, Receipt) {
    let h = KnowledgeHub::create(backend(s), seed::context(), model_scope())
        .await
        .unwrap();
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut change = Change::at(&state, head, "register");
    change.operations = builtin_catalogue().unwrap().operations();
    let report = h.validate(seed::context(), change.clone()).await.unwrap();
    assert!(
        report.is_valid(),
        "{:#?}",
        report.violations.iter().take(15).collect::<Vec<_>>()
    );
    let r = h.commit(seed::context(), change).await.unwrap();
    (h, r)
}
#[tokio::test]
async fn full_catalogue_registration_reopen_and_original_receipt() {
    let s = Sandbox::new();
    let (h, r) = register(&s).await;
    let (_, state) = h.snapshot(seed::context()).await.unwrap();
    assert!(catalogue_coverage(&state).is_empty());
    assert_eq!(
        state
            .records
            .values()
            .filter(|r| matches!(r.data, RecordData::Type(_)))
            .count(),
        145
    );
    assert_eq!(
        state
            .records
            .values()
            .filter(|r| matches!(r.data, RecordData::Relationship(_)))
            .count(),
        38
    );
    let md = ExactRef::new("sdo", "urn:source:ontology:capture", "catalogue-1");
    assert_eq!(
        h.read_artifact(seed::context(), Selection::Snapshot(r.head.clone()), md)
            .await
            .unwrap()
            .bytes,
        seed::ONTOLOGY
    );
    h.drain().await;
    let h = KnowledgeHub::open(backend(&s), seed::context(), model_scope())
        .await
        .unwrap();
    assert_eq!(
        h.operation_result(seed::context(), "register".into())
            .await
            .unwrap(),
        OperationOutcome::Committed(Box::new(r))
    );
}

#[path = "../examples/ontology_support/mod.rs"]
mod specimens;

#[test]
fn every_family_has_a_complete_connected_typed_specimen() {
    let state = specimens::specimen_state();
    let report = assess(&state);
    assert!(report.is_complete(), "{:#?}", report);
    assert_eq!(
        state
            .selected_records()
            .filter(|r| r.element().is_some())
            .map(|r| r.type_ref.clone())
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        128
    );
    assert_eq!(
        state
            .selected_records()
            .filter(|r| matches!(r.data, RecordData::Assertion(_)))
            .count(),
        38
    );
}

#[tokio::test]
async fn all_families_and_kernel_relationships_commit_and_reopen() {
    let sandbox = Sandbox::new();
    let (hub, _) = register(&sandbox).await;
    let (head, state) = hub.snapshot(seed::context()).await.unwrap();
    let mut change = Change::at(&state, head, "all-families");
    change.operations = specimens::specimens().unwrap();
    let report = hub.validate(seed::context(), change.clone()).await.unwrap();
    assert!(report.is_complete(), "{report:#?}");
    let receipt = hub.commit(seed::context(), change).await.unwrap();
    let (_, state) = hub.snapshot(seed::context()).await.unwrap();
    let mut q = Query::select(Selection::Snapshot(receipt.head.clone()));
    q.page_size = 1000;
    let page = hub.query(seed::context(), q).await.unwrap();
    let public = page
        .records
        .into_iter()
        .chain(page.assertions)
        .map(|r| (r.reference.key(), r))
        .collect::<std::collections::BTreeMap<_, _>>();
    assert_eq!(
        public,
        state
            .selected_records()
            .map(|r| (r.reference.key(), r.clone()))
            .collect()
    );
    for reference in [
        specimens::reference("Requirement"),
        specimens::reference("edge:R31"),
        state
            .identity_reference(&specimens::reference("Requirement"))
            .unwrap(),
    ] {
        assert_eq!(
            hub.read(
                seed::context(),
                Selection::Snapshot(receipt.head.clone()),
                reference.clone()
            )
            .await
            .unwrap(),
            state.resolve(&reference).unwrap()
        );
    }
    hub.drain().await;
    let reopened = KnowledgeHub::open(backend(&sandbox), seed::context(), model_scope())
        .await
        .unwrap();
    let (_, loaded) = reopened.snapshot(seed::context()).await.unwrap();
    assert_eq!(state, loaded);
}

#[test]
fn every_declared_property_has_independent_missing_and_invalid_evidence() {
    let accepted = specimens::specimen_state();
    let catalogue = builtin_catalogue().unwrap();
    for missing in [true, false] {
        let mut candidate = accepted.clone();
        let mut expected = vec![];
        for schema in &catalogue.records {
            if let RecordData::Type(t) = &schema.data {
                if !t.native {
                    let reference = specimens::reference(&t.name);
                    let record = candidate.records.get_mut(&reference.key()).unwrap();
                    let RecordData::Element(e) = &mut record.data else {
                        unreachable!()
                    };
                    for (name, property) in &t.properties {
                        if !missing || property.required {
                            if missing {
                                e.properties.remove(name);
                            } else {
                                e.properties.insert(name.clone(), Value::Null);
                            }
                            expected.push((reference.key(), format!("/{name}:")));
                        }
                    }
                    e.owners = ElementData::new(e.properties.clone()).owners;
                    record.seal().unwrap();
                }
            }
        }
        let report = assess(&candidate);
        for (subject, path) in &expected {
            assert!(
                report
                    .violations
                    .iter()
                    .any(|v| &v.subject == subject && v.message.starts_with(path)),
                "no {} evidence for {subject} {path}",
                if missing { "absence" } else { "nullability" }
            );
        }
        assert!(
            expected.len() > 700,
            "inventory unexpectedly shrank: {}",
            expected.len()
        );
    }
    // Each schema independently rejects an unknown normative property, even if the value is valid text.
    let mut candidate = accepted.clone();
    for r in candidate.records.values_mut() {
        if let RecordData::Element(e) = &mut r.data {
            e.properties.insert(
                "unregistered_normative_field".into(),
                Value::text("not admitted"),
            );
            e.owners = ElementData::new(e.properties.clone()).owners;
            r.seal().unwrap();
        }
    }
    let report = assess(&candidate);
    for r in candidate
        .selected_records()
        .filter(|r| r.element().is_some())
    {
        assert!(
            report
                .violations
                .iter()
                .any(|v| v.subject == r.reference.key()
                    && v.message.starts_with("/unregistered_normative_field:"))
        );
    }
    assert!(assess(&accepted).is_complete());
}

#[test]
fn scalar_encodings_closed_requests_and_revision_tokens_are_lossless() {
    let scope = specimens::scope();
    let mut c = Change::new(scope, SnapshotId("head".into()), "intent");
    let mut r = specimens::specimen_state()
        .get(&specimens::reference("Measure"))
        .unwrap()
        .clone();
    let first = r.sha256.clone();
    specimens::put_property(
        &mut r,
        "target",
        Value::Decimal {
            coefficient: "90071992547409931234567890123456789".into(),
            exponent: "-10000".into(),
        },
    );
    assert_ne!(first, r.sha256);
    assert_eq!(r.reference.revision, "1");
    c.operations.push(knowledgehub::ontology::Operation::Put(r));
    let bytes = serde_json::to_vec(&c).unwrap();
    assert_eq!(Change::from_json(&bytes).unwrap(), c);
    let mut json = serde_json::to_value(&c).unwrap();
    json.as_object_mut()
        .unwrap()
        .insert("unchecked".into(), serde_json::json!(true));
    assert!(Change::from_json(&serde_json::to_vec(&json).unwrap()).is_err());
    for bad in ["01", "-0", "1.0", "1e3", "+2"] {
        let mut s = specimens::specimen_state();
        let r = s
            .records
            .get_mut(&specimens::reference("RunLimit").key())
            .unwrap();
        specimens::put_property(r, "calls", Value::Integer(bad.into()));
        assert!(!assess(&s).is_valid(), "accepted {bad}");
    }
    let mut s = specimens::specimen_state();
    let r = s
        .records
        .get_mut(&specimens::reference("Measure").key())
        .unwrap();
    specimens::put_property(
        r,
        "target",
        Value::Decimal {
            coefficient: "100".into(),
            exponent: "0".into(),
        },
    );
    assert!(!assess(&s).is_valid());
}

#[test]
fn specialised_integrity_draft_gaps_and_native_roles() {
    fn invalid(kind: &str, field: &str, value: Value, rule: &str) {
        let mut s = specimens::specimen_state();
        let r = s
            .records
            .get_mut(&specimens::reference(kind).key())
            .unwrap();
        specimens::put_property(r, field, value);
        let result = assess(&s);
        assert!(
            result
                .violations
                .iter()
                .any(|v| v.subject == specimens::reference(kind).key() && v.rule == rule),
            "{kind}/{field}: {result:#?}"
        );
    }
    invalid(
        "Transition",
        "to",
        Value::Ref(specimens::reference("Requirement")),
        "V-02",
    );
    invalid(
        "DataSchema",
        "identity_fields",
        Value::List(vec![Value::Ref(type_ref("Requirement"))]),
        "V-12",
    );
    invalid(
        "EventHandler",
        "input_schema",
        Value::Ref(type_ref("Requirement")),
        "V-11",
    );
    invalid(
        "ActionBinding",
        "command",
        Value::Ref(specimens::reference("Requirement")),
        "V-13",
    );
    invalid(
        "ComponentPlacement",
        "screen",
        Value::Ref(specimens::reference("Requirement")),
        "V-13",
    );
    invalid(
        "AcceptanceRecord",
        "assessor",
        Value::text("unqualified"),
        "V-16",
    );
    let mut s = specimens::specimen_state();
    // A reasoned missing root is a valid incomplete draft. It never admits a second root.
    let root = specimens::reference("edge:R14");
    s.selected.remove(&root.identity());
    s.gaps.push(Gap {
        subject: specimens::reference("AggregateDefinition"),
        path: "/hasRoot".into(),
        reason: "Draft analysis has not selected its root".into(),
        provenance: vec![specimens::anchor()],
    });
    let report = assess(&s);
    assert!(report.is_valid(), "{report:#?}");
    assert!(!report.is_complete());
    let duplicate = specimens::assertion(
        "second-root",
        "AggregateDefinition",
        "hasRoot",
        specimens::reference("AggregateDefinition"),
        specimens::reference("EntityType"),
    );
    specimens::install(&mut s, duplicate);
    // Restore the original selected root, giving two independent root assertions.
    s.selected.insert(root.identity(), root);
    assert!(!assess(&s).is_valid());
    for (id, bad) in [
        ("edge:R30", specimens::reference("edge:R08")),
        ("edge:R31", specimens::reference("Requirement")),
        ("edge:R32", specimens::reference("Requirement")),
    ] {
        let mut s = specimens::specimen_state();
        let r = s.records.get_mut(&specimens::reference(id).key()).unwrap();
        let RecordData::Assertion(a) = &mut r.data else {
            unreachable!()
        };
        a.to = bad;
        r.seal().unwrap();
        assert!(
            assess(&s)
                .violations
                .iter()
                .any(|v| v.subject == specimens::reference(id).key() && v.rule == "V-03")
        );
    }
    let s = specimens::specimen_state();
    let identity = s
        .identity_reference(&specimens::reference("Requirement"))
        .unwrap();
    assert_eq!(s.kind(&identity), Some("ElementIdentity"));
    assert!(!s.records.contains_key(&identity.key()));
    assert_eq!(
        s.kind(&specimens::reference("edge:R08")),
        Some("RelationshipAssertion")
    );
}

#[test]
fn every_declared_signature_rejects_wrong_direction_and_targets() {
    let accepted = specimens::specimen_state();
    let mut candidate = accepted.clone();
    let mut expected = vec![];
    for record in candidate.records.values_mut() {
        if let RecordData::Assertion(a) = &mut record.data {
            a.to = if a.signature == relationship_ref("RepresentationBinding", "represents") {
                specimens::reference("edge:R08")
            } else {
                type_ref("Requirement")
            };
            expected.push(record.reference.clone());
            record.seal().unwrap();
        }
    }
    let report = assess(&candidate);
    for r in &expected {
        assert!(
            report
                .violations
                .iter()
                .any(|v| v.subject == r.key() && v.rule == "V-03"),
            "wrong target accepted for {r:?}"
        );
    }
    assert_eq!(expected.len(), 38);
}

struct Access {
    allowed: std::sync::atomic::AtomicBool,
    failed: std::sync::atomic::AtomicBool,
    registration: bool,
}
impl Policy for Access {
    fn allows(&self, _: &str, _: &ModelScope, action: Action) -> Result<bool, PolicyFailure> {
        use std::sync::atomic::Ordering::SeqCst;
        if self.failed.load(SeqCst) {
            return Err(PolicyFailure);
        }
        Ok(self.allowed.load(SeqCst) && (self.registration || action != Action::RegisterOntology))
    }
}
fn extension(state: &State, head: SnapshotId) -> Change {
    let original = state.type_definition(&type_ref("Requirement")).unwrap();
    let citation = original.citation.clone();
    let a = ExactRef::new("extension", "type:Alpha", "1");
    let b = ExactRef::new("extension", "type:Beta", "1");
    let mut properties = std::collections::BTreeMap::from([(
        "partner".into(),
        Property::required(ValueKind::Reference(vec!["Beta".into()])),
    )]);
    properties.insert(
        "values".into(),
        Property::required(ValueKind::Record(std::collections::BTreeMap::from([
            (
                "ordered".into(),
                Property::required(ValueKind::List {
                    item: Box::new(ValueKind::Text),
                    min: 0,
                }),
            ),
            (
                "distinct".into(),
                Property::required(ValueKind::Set {
                    item: Box::new(ValueKind::Text),
                    min: 0,
                }),
            ),
            (
                "nullable".into(),
                Property {
                    value: ValueKind::Bool,
                    required: true,
                    nullable: true,
                    owner: PropertyOwner::Graph,
                },
            ),
        ]))),
    );
    let alpha = TypeDefinition {
        name: "Alpha".into(),
        family: "extension".into(),
        coverage: "extension:A".into(),
        native: false,
        parents: vec![],
        properties,
        constraints: vec![],
        citation: citation.clone(),
    };
    let beta = TypeDefinition {
        name: "Beta".into(),
        properties: std::collections::BTreeMap::from([(
            "partner".into(),
            Property::required(ValueKind::Reference(vec!["Alpha".into()])),
        )]),
        ..alpha.clone()
    };
    let mut ontology = if let RecordData::Ontology(o) = &state.get(&ontology_ref()).unwrap().data {
        o.clone()
    } else {
        unreachable!()
    };
    ontology.types.extend([a.clone(), b.clone()]);
    let o = ExactRef::new("extension", "ontology", "1");
    let p = ExactRef::new("extension", "profile", "1");
    ontology.profiles = vec![p.clone()];
    let profile = ProfileDefinition {
        name: "recursive pair".into(),
        ontology: o.clone(),
        vocabulary: ontology.types.clone(),
        allow_gaps: true,
        required_types: vec![],
        inapplicable: Default::default(),
        citation,
    };
    let mut change = Change::at(state, head, "recursive-registration");
    for (reference, kind, data) in [
        (a, "ElementTypeDefinition", RecordData::Type(alpha)),
        (b, "ElementTypeDefinition", RecordData::Type(beta)),
        (
            o.clone(),
            "OntologyDefinition",
            RecordData::Ontology(ontology),
        ),
        (p.clone(), "ProfileDefinition", RecordData::Profile(profile)),
    ] {
        change
            .operations
            .push(knowledgehub::ontology::Operation::Put(
                Record::new(reference, type_ref(kind), data, vec![specimens::anchor()]).unwrap(),
            ));
    }
    change
        .operations
        .push(knowledgehub::ontology::Operation::SelectProfile {
            ontology: o,
            profile: p,
        });
    change
}
#[tokio::test]
async fn registration_is_atomic_recursive_guarded_and_currently_authorised() {
    use std::sync::{Arc, atomic::Ordering::SeqCst};
    let s = Sandbox::new();
    let (h, _) = register(&s).await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let request = extension(&state, head.clone());
    let policy = Arc::new(Access {
        allowed: true.into(),
        failed: false.into(),
        registration: false,
    });
    let denied = Context::new(seed::context().actor, policy.clone());
    assert!(matches!(
        h.commit(denied.clone(), request.clone()).await,
        Err(Error::Denied { .. })
    ));
    policy.failed.store(true, SeqCst);
    assert!(matches!(
        h.commit(denied, request.clone()).await,
        Err(Error::Unavailable(_))
    ));
    assert_eq!(h.head(seed::context()).await.unwrap(), head);
    let mut bad = request.clone();
    if let Some(knowledgehub::ontology::Operation::Put(r)) = bad.operations.first_mut() {
        let RecordData::Type(t) = &mut r.data else {
            unreachable!()
        };
        t.parents = vec![r.reference.clone()];
        r.seal().unwrap();
    }
    assert!(!h.validate(seed::context(), bad).await.unwrap().is_valid());
    let mut shadow = request.clone();
    if let knowledgehub::ontology::Operation::Put(record) = &mut shadow.operations[0] {
        let RecordData::Type(t) = &mut record.data else {
            unreachable!()
        };
        t.name = "SourceAnchor".into();
        record.seal().unwrap();
    }
    assert!(
        !h.validate(seed::context(), shadow)
            .await
            .unwrap()
            .is_valid()
    );
    let mut bad_citation = request.clone();
    if let knowledgehub::ontology::Operation::Put(record) = &mut bad_citation.operations[0] {
        let RecordData::Type(t) = &mut record.data else {
            unreachable!()
        };
        t.citation.original = ContentRef::of(b"not the cited source");
        record.seal().unwrap();
    }
    assert!(
        !h.validate(seed::context(), bad_citation)
            .await
            .unwrap()
            .is_valid()
    );
    let receipt = h.commit(seed::context(), request.clone()).await.unwrap();
    assert_eq!(h.commit(seed::context(), request).await.unwrap(), receipt);
    let (_, state) = h.snapshot(seed::context()).await.unwrap();
    assert!(
        state
            .type_definition(&ExactRef::new("extension", "type:Alpha", "1"))
            .is_some()
    );
    let mut c = Change::at(&state, receipt.head, "pair-values");
    let ar = ExactRef::new("extension", "alpha", "token");
    let br = ExactRef::new("extension", "beta", "token");
    let values = Value::Record(std::collections::BTreeMap::from([
        (
            "ordered".into(),
            Value::List(vec![Value::text("b"), Value::text("a"), Value::text("a")]),
        ),
        (
            "distinct".into(),
            Value::Set(vec![Value::text("a"), Value::text("b")]),
        ),
        ("nullable".into(), Value::Null),
    ]));
    for (reference, kind, properties) in [
        (
            ar.clone(),
            "Alpha",
            std::collections::BTreeMap::from([
                ("partner".into(), Value::Ref(br.clone())),
                ("values".into(), values),
            ]),
        ),
        (
            br,
            "Beta",
            std::collections::BTreeMap::from([("partner".into(), Value::Ref(ar))]),
        ),
    ] {
        c.operations.push(knowledgehub::ontology::Operation::Put(
            Record::new(
                reference,
                ExactRef::new("extension", format!("type:{kind}"), "1"),
                RecordData::Element(ElementData::new(properties)),
                vec![specimens::anchor()],
            )
            .unwrap(),
        ));
    }
    assert!(
        h.validate(seed::context(), c.clone())
            .await
            .unwrap()
            .is_complete()
    );
    let mut invalid = c.clone();
    let knowledgehub::ontology::Operation::Put(r) = &mut invalid.operations[0] else {
        unreachable!()
    };
    let RecordData::Element(e) = &mut r.data else {
        unreachable!()
    };
    let Value::Record(m) = e.properties.get_mut("values").unwrap() else {
        unreachable!()
    };
    m.insert(
        "distinct".into(),
        Value::Set(vec![Value::text("b"), Value::text("a"), Value::text("a")]),
    );
    e.owners = ElementData::new(e.properties.clone()).owners;
    r.seal().unwrap();
    assert!(
        !h.validate(seed::context(), invalid)
            .await
            .unwrap()
            .is_valid()
    );
    h.commit(seed::context(), c).await.unwrap();
}

async fn full_model(s: &Sandbox) -> KnowledgeHub {
    let (h, _) = register(s).await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut c = Change::at(&state, head, "model");
    c.operations = specimens::specimens().unwrap();
    h.commit(seed::context(), c).await.unwrap();
    h
}
async fn baseline(h: &KnowledgeHub, name: &str) -> (ExactRef, Receipt) {
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let reference = ExactRef::new("conformance", format!("baseline:{name}"), "1");
    let record = Record::new(
        reference.clone(),
        type_ref("ModelBaseline"),
        RecordData::Baseline(state.baseline_data().unwrap()),
        vec![specimens::anchor()],
    )
    .unwrap();
    let mut c = Change::at(&state, head, format!("baseline:{name}"));
    c.operations
        .push(knowledgehub::ontology::Operation::Put(record));
    let receipt = h.commit(seed::context(), c).await.unwrap();
    (reference, receipt)
}

async fn revision_walkthrough() {
    let s = Sandbox::new();
    let h = full_model(&s).await;
    let (a, _) = baseline(&h, "A").await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let old = specimens::reference("Requirement");
    let mut next = state.get(&old).unwrap().clone();
    next.reference.revision = "2".into();
    next.predecessor = Some(old.clone());
    specimens::put_property(
        &mut next,
        "name",
        Value::text("Renamed requirement — stable semantic identity"),
    );
    let mut c = Change::at(&state, head.clone(), "successor");
    c.operations
        .push(knowledgehub::ontology::Operation::Put(next.clone()));
    assert!(
        !h.validate(seed::context(), c.clone())
            .await
            .unwrap()
            .is_valid()
    );
    assert_eq!(h.head(seed::context()).await.unwrap(), head);
    c.operations = specimens::successor_operations(&state, vec![next.clone()]).unwrap();
    let report = h.validate(seed::context(), c.clone()).await.unwrap();
    assert!(report.is_complete(), "{report:#?}");
    let receipt = h.commit(seed::context(), c.clone()).await.unwrap();
    let (b, _) = baseline(&h, "B").await;
    let changes = h
        .compare(seed::context(), a.clone(), b.clone())
        .await
        .unwrap();
    assert!(
        changes
            .iter()
            .any(|d| d.before == Some(old.clone()) && d.after == Some(next.reference.clone()))
    );
    let sa = h
        .read_state(seed::context(), Selection::Baseline(a.clone()))
        .await
        .unwrap();
    let sb = h
        .read_state(seed::context(), Selection::Baseline(b.clone()))
        .await
        .unwrap();
    assert_eq!(
        sa.current(&old.model, &old.id).unwrap(),
        state.get(&old).unwrap()
    );
    assert_eq!(sb.current(&old.model, &old.id).unwrap(), &next);
    assert!(sa.get(&next.reference).is_none());
    assert_eq!(h.commit(seed::context(), c.clone()).await.unwrap(), receipt);
    let mut reused = c.clone();
    reused.operations.reverse();
    assert!(matches!(
        h.commit(seed::context(), reused).await,
        Err(Error::OperationReuse)
    ));
    let (head, current) = h.snapshot(seed::context()).await.unwrap();
    let mut rewrite = Change::at(&current, head, "rewrite-token");
    let mut bad = next.clone();
    specimens::put_property(&mut bad, "name", Value::text("different immutable content"));
    rewrite
        .operations
        .push(knowledgehub::ontology::Operation::Put(bad));
    assert!(
        !h.validate(seed::context(), rewrite)
            .await
            .unwrap()
            .is_valid()
    );
    h.drain().await;
    let reopened = KnowledgeHub::open(backend(&s), seed::context(), model_scope())
        .await
        .unwrap();
    assert_eq!(
        reopened.compare(seed::context(), a, b).await.unwrap(),
        changes
    );
    assert_eq!(reopened.commit(seed::context(), c).await.unwrap(), receipt);
}
#[tokio::test(flavor = "current_thread")]
async fn coherent_successors_baselines_and_replay_current_thread() {
    revision_walkthrough().await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn coherent_successors_baselines_and_replay_multithreaded() {
    revision_walkthrough().await;
}

#[tokio::test]
async fn bounded_navigation_cursors_policy_and_selection_binding() {
    use std::sync::{Arc, atomic::Ordering::SeqCst};
    let s = Sandbox::new();
    let (h, r) = register(&s).await;
    let mut q = Query::select(Selection::Snapshot(r.head));
    q.page_size = 1;
    let first = h.query(seed::context(), q.clone()).await.unwrap();
    let cursor = first.continuation.unwrap();
    let policy = Arc::new(Access {
        allowed: true.into(),
        failed: false.into(),
        registration: true,
    });
    let context = Context::new(seed::context().actor, policy.clone());
    policy.allowed.store(false, SeqCst);
    assert!(matches!(
        h.continue_query(context.clone(), q.clone(), cursor.clone())
            .await,
        Err(Error::Denied { .. })
    ));
    policy.allowed.store(true, SeqCst);
    let mut other = q.clone();
    other.type_ref = Some(type_ref("Requirement"));
    assert!(matches!(
        h.continue_query(context.clone(), other, cursor.clone())
            .await,
        Err(Error::InvalidCursor)
    ));
    let other_actor = Context::new("actor:other", policy);
    assert!(matches!(
        h.continue_query(other_actor, q.clone(), cursor.clone())
            .await,
        Err(Error::InvalidCursor)
    ));
    let second = h
        .continue_query(context, q.clone(), cursor.clone())
        .await
        .unwrap();
    assert_ne!(first.records, second.records);
    h.release_cursor(seed::context(), cursor.clone())
        .await
        .unwrap();
    assert!(matches!(
        h.continue_query(seed::context(), q.clone(), cursor).await,
        Err(Error::InvalidCursor)
    ));
    let mut excessive = q.clone();
    excessive.depth = 33;
    assert!(matches!(
        h.query(seed::context(), excessive).await,
        Err(Error::InvalidInput(_))
    ));
    let mut bounded = q.clone();
    bounded.subject_limit = 2;
    bounded.page_size = 100;
    let p = h.query(seed::context(), bounded).await.unwrap();
    assert_eq!(p.records.len() + p.assertions.len(), 2);
    assert!(!p.exhausted);
    assert_eq!(p.truncated.as_deref(), Some("subject budget"));
    let mut cursors = vec![];
    for _ in 0..64 {
        cursors.push(
            h.query(seed::context(), q.clone())
                .await
                .unwrap()
                .continuation
                .unwrap(),
        );
    }
    assert!(matches!(
        h.query(seed::context(), q.clone()).await,
        Err(Error::Capacity)
    ));
    h.release_cursor(seed::context(), cursors.pop().unwrap())
        .await
        .unwrap();
    let cursor = h
        .query(seed::context(), q.clone())
        .await
        .unwrap()
        .continuation
        .unwrap();
    h.drain().await;
    let reopened = KnowledgeHub::open(backend(&s), seed::context(), model_scope())
        .await
        .unwrap();
    assert!(matches!(
        reopened.continue_query(seed::context(), q, cursor).await,
        Err(Error::InvalidCursor)
    ));
}

#[tokio::test]
async fn binary_originals_and_every_locator_kind_resolve_and_reject_bad_quotes() {
    let s = Sandbox::new();
    let (h, _) = register(&s).await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let pdf = ExactRef::new("sdo", "sdo:source:ontology-pdf:capture", "catalogue-1");
    assert_eq!(
        h.read_artifact(
            seed::context(),
            Selection::Snapshot(head.clone()),
            pdf.clone()
        )
        .await
        .unwrap()
        .bytes,
        include_bytes!("../source/system-definition-ontology-proposal.pdf")
    );
    let derived = ExactRef::new("locator", "derived", "1");
    let manifest = ExactRef::new("locator", "manifest", "1");
    let source = ExactRef::new("locator", "manifest-source", "1");
    let mut c = Change::at(&state, head, "locators");
    let content=Content::capture("Section 21.4; requirement KH-DR-08; table row Atomic changes; JSON /scope; symbol commit; UI Save; image region publication diagram; time segment replacement; PDF page source excerpt.".as_bytes().to_vec());
    let mut entries = std::collections::BTreeMap::new();
    for (i, (kind, label)) in [
        (LocatorKind::Section, "Section 21.4"),
        (LocatorKind::RequirementId, "requirement KH-DR-08"),
        (LocatorKind::TableRow, "table row Atomic changes"),
        (LocatorKind::JsonPointer, "JSON /scope"),
        (LocatorKind::CodeSymbol, "symbol commit"),
        (LocatorKind::UiElement, "UI Save"),
        (LocatorKind::ImageRegion, "image region publication diagram"),
        (LocatorKind::TimeRange, "time segment replacement"),
        (LocatorKind::PdfPage, "PDF page source excerpt"),
    ]
    .into_iter()
    .enumerate()
    {
        let text = std::str::from_utf8(&content.bytes).unwrap();
        let start = text.find(label).unwrap() as u64;
        entries.insert(
            format!("entry:{i}"),
            LocatorEntry {
                kind: kind.clone(),
                label: label.into(),
                location: match kind {
                    LocatorKind::Section => SourceLocation::Section {
                        path: vec!["21".into(), "21.4".into()],
                    },
                    LocatorKind::RequirementId => SourceLocation::RequirementId {
                        id: "KH-DR-08".into(),
                    },
                    LocatorKind::TableRow => SourceLocation::TableRow {
                        table: "21.3 capabilities".into(),
                        row: "Atomic changes".into(),
                    },
                    LocatorKind::JsonPointer => SourceLocation::JsonPointer {
                        pointer: "/scope".into(),
                    },
                    LocatorKind::CodeSymbol => SourceLocation::CodeSymbol {
                        language: "Rust".into(),
                        symbol: "KnowledgeHub::commit".into(),
                    },
                    LocatorKind::UiElement => SourceLocation::UiElement {
                        screen: "Publication form".into(),
                        element: "Save".into(),
                    },
                    LocatorKind::ImageRegion => SourceLocation::ImageRegion {
                        x: 0,
                        y: 0,
                        width: 64,
                        height: 64,
                        image_width: 128,
                        image_height: 128,
                    },
                    LocatorKind::TimeRange => SourceLocation::TimeRange {
                        start_ms: 3000,
                        end_ms: 6000,
                        duration_ms: 9000,
                    },
                    LocatorKind::PdfPage => SourceLocation::PdfPage { page: 1, pages: 26 },
                },
                derived: derived.clone(),
                start,
                end: start + label.len() as u64,
            },
        );
        c.operations.push(knowledgehub::ontology::Operation::Put(
            Record::new(
                ExactRef::new("locator", format!("anchor:{i}"), "1"),
                type_ref("SourceAnchor"),
                RecordData::Anchor(AnchorData {
                    source: pdf.clone(),
                    label: format!("Synthetic locator contract fixture: {label}"),
                    locator: Locator::Manifest {
                        locator: kind,
                        manifest: manifest.clone(),
                        entry: format!("entry:{i}"),
                        quote: Some(label.into()),
                    },
                }),
                vec![],
            )
            .unwrap(),
        ));
    }
    // This is a synthetic extraction observation used to verify binding mechanics, not a claim about PDF text accuracy.
    let m = LocatorManifest {
        source: pdf.clone(),
        original: state.artifact(&pdf).unwrap().reference.clone(),
        tool: "synthetic locator-contract fixture".into(),
        version: "1".into(),
        entries,
    };
    let mcontent = Content::capture(serde_json::to_vec(&m).unwrap());
    c.operations.push(knowledgehub::ontology::Operation::Put(
        Record::new(
            source.clone(),
            type_ref("SourceArtifact"),
            RecordData::SourceArtifact(SourceArtifactData {
                origin: "synthetic locator contract mapping".into(),
                media_type: "application/json".into(),
                owner: "test:host".into(),
                status: "validation-result".into(),
            }),
            vec![],
        )
        .unwrap(),
    ));
    for (reference, bytes, derived_from, media) in [
        (derived.clone(), content, Some(pdf.clone()), "text/plain"),
        (manifest.clone(), mcontent, None, "application/json"),
    ] {
        c.operations
            .push(knowledgehub::ontology::Operation::Capture {
                record: Record::new(
                    reference,
                    type_ref("SourceRevision"),
                    RecordData::SourceRevision(SourceRevisionData {
                        source: source.clone(),
                        original: bytes.reference.clone(),
                        media_type: media.into(),
                        derived_from,
                        derivation: Some(
                            "Explicitly synthetic fixture mapping, no PDF extraction claim".into(),
                        ),
                    }),
                    vec![],
                )
                .unwrap(),
                content: bytes,
            });
    }
    let report = h.validate(seed::context(), c.clone()).await.unwrap();
    assert!(report.is_complete(), "{report:#?}");
    let mut bad = c.clone();
    for op in &mut bad.operations {
        if let Some(r) = match op {
            knowledgehub::ontology::Operation::Put(r) => Some(r),
            _ => None,
        } {
            if let RecordData::Anchor(a) = &mut r.data {
                if let Locator::Manifest { quote, .. } = &mut a.locator {
                    *quote = Some("incorrect quotation".into());
                    r.seal().unwrap();
                }
            }
        }
    }
    let invalid = h.validate(seed::context(), bad).await.unwrap();
    for i in 0..9 {
        assert!(
            invalid
                .violations
                .iter()
                .any(|v| v.subject == ExactRef::new("locator", format!("anchor:{i}"), "1").key())
        );
    }
    let receipt = h.commit(seed::context(), c).await.unwrap();
    h.drain().await;
    let reopened = KnowledgeHub::open(backend(&s), seed::context(), model_scope())
        .await
        .unwrap();
    assert_eq!(
        reopened
            .read_artifact(seed::context(), Selection::Snapshot(receipt.head), pdf)
            .await
            .unwrap()
            .bytes,
        include_bytes!("../source/system-definition-ontology-proposal.pdf")
    );
}

#[test]
fn cycles_process_membership_transformation_kinds_and_contracts() {
    let mut s = specimens::specimen_state();
    let cycle = specimens::assertion(
        "cycle",
        "BusinessFunction",
        "decomposesInto",
        specimens::reference("BusinessFunctionChild"),
        specimens::reference("BusinessFunction"),
    );
    specimens::install(&mut s, cycle);
    assert!(assess(&s).violations.iter().any(|v| v.rule == "V-03"));
    let mut s = specimens::specimen_state();
    let ps = specimens::reference("process-state");
    let pt = specimens::reference("process-transition");
    for (kind, new) in [("State", ps.clone()), ("Transition", pt.clone())] {
        let mut r = s.get(&specimens::reference(kind)).unwrap().clone();
        r.reference = new;
        specimens::put_property(
            &mut r,
            "owner",
            Value::Ref(specimens::reference("ProcessDefinition")),
        );
        if kind == "Transition" {
            specimens::put_property(&mut r, "from", Value::Ref(ps.clone()));
            specimens::put_property(&mut r, "to", Value::Ref(ps.clone()));
        }
        specimens::install(&mut s, r);
    }
    let p = s
        .records
        .get_mut(&specimens::reference("ProcessDefinition").key())
        .unwrap();
    for (field, value) in [
        ("states", ps.clone()),
        ("transitions", pt.clone()),
        ("allowed_loops", pt.clone()),
    ] {
        specimens::put_property(p, field, Value::List(vec![Value::Ref(value)]));
    }
    specimens::install(
        &mut s,
        specimens::assertion(
            "loop-trigger",
            "Transition",
            "triggeredBy",
            pt.clone(),
            specimens::reference("CommandDefinition"),
        ),
    );
    // This unit specimen is a fresh candidate; its named baseline includes the newly declared process subjects.
    let additions = [ps.clone(), pt.clone(), specimens::reference("loop-trigger")];
    let baseline = s
        .records
        .get_mut(&specimens::reference("ModelBaseline").key())
        .unwrap();
    let RecordData::Baseline(data) = &mut baseline.data else {
        unreachable!()
    };
    data.selected.extend(additions.clone());
    data.supporting.extend(additions);
    baseline.seal().unwrap();
    assert!(assess(&s).is_complete(), "{:#?}", assess(&s));
    let p = s
        .records
        .get_mut(&specimens::reference("ProcessDefinition").key())
        .unwrap();
    specimens::put_property(p, "allowed_loops", Value::List(vec![]));
    assert!(assess(&s).violations.iter().any(|v| v.rule == "V-10"));
    for kind in ["data", "state", "schema", "model", "artifact"] {
        let mut s = specimens::specimen_state();
        specimens::put_property(
            s.records
                .get_mut(&specimens::reference("TransformationDefinition").key())
                .unwrap(),
            "kind",
            Value::enumeration(kind),
        );
        assert!(assess(&s).is_complete(), "{kind}");
    }
    let mut s = specimens::specimen_state();
    let selector = Value::Record(std::collections::BTreeMap::from([
        (
            "subject".into(),
            Value::Ref(specimens::reference("Requirement")),
        ),
        ("path".into(), Value::text("nonexistent")),
    ]));
    specimens::put_property(
        s.records
            .get_mut(&specimens::reference("MappingRule").key())
            .unwrap(),
        "source",
        selector,
    );
    assert!(assess(&s).violations.iter().any(|v| v.rule == "V-12"));
    let mut s = specimens::specimen_state();
    let missing = s
        .records
        .get_mut(&specimens::reference("edge:R21").key())
        .unwrap();
    let RecordData::Assertion(a) = &mut missing.data else {
        unreachable!()
    };
    a.to = specimens::reference("Requirement");
    missing.seal().unwrap();
    assert!(!assess(&s).is_valid());
}

#[test]
fn kernel_codecs_are_closed_and_missing_catalogue_family_is_reported() {
    let s = specimens::specimen_state();
    let mut kinds = std::collections::BTreeSet::new();
    for r in s.records.values() {
        if let Some(kind) = r.data.native_kind() {
            kinds.insert(kind);
            let mut v = serde_json::to_value(r).unwrap();
            let data = v
                .get_mut("data")
                .unwrap()
                .get_mut("data")
                .unwrap()
                .as_object_mut()
                .unwrap();
            data.insert("unregistered_field".into(), serde_json::json!(true));
            assert!(
                serde_json::from_value::<Record>(v).is_err(),
                "open native codec: {kind}"
            );
        }
    }
    assert_eq!(kinds.len(), 15); // Identity is a resolver projection; ElementRevision is the actual envelope.
    let identity = s
        .resolve(
            &s.identity_reference(&specimens::reference("Requirement"))
                .unwrap(),
        )
        .unwrap();
    assert_eq!(identity.data.native_kind(), Some("ElementIdentity"));
    let mut missing = s.clone();
    let removed = missing
        .records
        .values()
        .filter(|r| matches!(&r.data,RecordData::Type(t) if t.family.contains("AI")))
        .map(|r| r.reference.key())
        .collect::<Vec<_>>();
    assert_eq!(removed.len(), 9);
    for key in removed {
        missing.records.remove(&key);
    }
    assert_eq!(catalogue_coverage(&missing).len(), 9);
    assert!(!assess(&missing).is_complete());
}

#[tokio::test]
async fn each_family_retains_an_explicit_valid_draft_and_gaps_never_waive_integrity() {
    let s = Sandbox::new();
    let h = full_model(&s).await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut c = Change::at(&state, head, "family-drafts");
    let mut gaps = vec![];
    for kind in [
        "Stakeholder",
        "Requirement",
        "UserStory",
        "Subdomain",
        "ValueObjectType",
        "FunctionDefinition",
        "EventOccurrence",
        "DataType",
        "Asset",
        "Dependency",
        "Permission",
        "QualityScenario",
        "ModelConfiguration",
        "AcceptanceCriterion",
    ] {
        let mut r = state.get(&specimens::reference(kind)).unwrap().clone();
        r.reference.id.push_str(":draft");
        let RecordData::Element(e) = &mut r.data else {
            unreachable!()
        };
        e.properties.remove("name");
        e.owners = ElementData::new(e.properties.clone()).owners;
        r.seal().unwrap();
        gaps.push(Gap {
            subject: r.reference.clone(),
            path: "/name".into(),
            reason:
                "Draft label awaits editorial review; semantic contract and source binding retained"
                    .into(),
            provenance: r.provenance.clone(),
        });
        c.operations.push(knowledgehub::ontology::Operation::Put(r));
    }
    c.operations
        .push(knowledgehub::ontology::Operation::Gaps(gaps));
    let report = h.validate(seed::context(), c.clone()).await.unwrap();
    assert!(report.is_valid(), "{report:#?}");
    assert_eq!(report.gaps.len(), 14);
    assert!(!report.is_complete());
    let mut bad = c.clone();
    let knowledgehub::ontology::Operation::Put(r) = &mut bad.operations[0] else {
        unreachable!()
    };
    specimens::put_property(
        r,
        "accountable_scope",
        Value::Ref(ExactRef::new("missing", "unresolved", "1")),
    );
    assert!(!h.validate(seed::context(), bad).await.unwrap().is_valid());
    h.commit(seed::context(), c).await.unwrap();
}

#[tokio::test]
async fn superseded_formats_are_refused_without_mutation() {
    for version in [1, 2, 3] {
        let s = Sandbox::new();
        std::fs::create_dir(&s.store).unwrap();
        let path = s.store.join("hub.json");
        let bytes = format!("{{\"format\":\"knowledgehub-files/{version}\"}}").into_bytes();
        std::fs::write(&path, &bytes).unwrap();
        std::fs::write(s.store.join("retained-data"), b"unchanged").unwrap();
        assert!(matches!(
            KnowledgeHub::open(backend(&s), seed::context(), model_scope()).await,
            Err(Error::Incompatible(_))
        ));
        assert!(
            KnowledgeHub::create(backend(&s), seed::context(), model_scope())
                .await
                .is_err()
        );
        assert_eq!(std::fs::read(path).unwrap(), bytes);
        assert_eq!(
            std::fs::read(s.store.join("retained-data")).unwrap(),
            b"unchanged"
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_same_base_root_and_baseline_race_has_one_winner() {
    let s = Sandbox::new();
    let h = full_model(&s).await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut draft = Change::at(&state, head, "draft-root");
    let mut aggregate = state
        .get(&specimens::reference("AggregateDefinition"))
        .unwrap()
        .clone();
    aggregate.reference.revision = "draft".into();
    aggregate.predecessor = Some(specimens::reference("AggregateDefinition"));
    let mut member = state
        .get(&specimens::reference("EntityType"))
        .unwrap()
        .clone();
    member.reference = specimens::reference("SecondEntityType");
    member.seal().unwrap();
    specimens::put_property(
        &mut aggregate,
        "members",
        Value::List(vec![
            Value::Ref(specimens::reference("EntityType")),
            Value::Ref(member.reference.clone()),
        ]),
    );
    draft.operations = specimens::successor_operations(&state, vec![aggregate.clone()]).unwrap();
    draft
        .operations
        .push(knowledgehub::ontology::Operation::Put(member.clone()));
    draft
        .operations
        .push(knowledgehub::ontology::Operation::Unselect(
            specimens::reference("edge:R14"),
        ));
    // Its successor root assertion is explicitly removed from normative selection as well.
    let successor = ExactRef::new("specimen", "specimen:edge:R14", "1-successor");
    draft
        .operations
        .push(knowledgehub::ontology::Operation::Unselect(successor));
    draft
        .operations
        .push(knowledgehub::ontology::Operation::Gaps(vec![Gap {
            subject: aggregate.reference.clone(),
            path: "/hasRoot".into(),
            reason: "Draft has not chosen between two members".into(),
            provenance: vec![specimens::anchor()],
        }]));
    // The original root is already superseded; remove only the selected successor.
    draft.operations.retain(|op|!matches!(op,knowledgehub::ontology::Operation::Unselect(r) if *r==specimens::reference("edge:R14")));
    h.commit(seed::context(), draft).await.unwrap();
    let second = KnowledgeHub::open(backend(&s), seed::context(), model_scope())
        .await
        .unwrap();
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let candidate = |id: &str, target: ExactRef| {
        let mut c = Change::at(&state, head.clone(), id);
        let root = specimens::assertion(
            id,
            "AggregateDefinition",
            "hasRoot",
            aggregate.reference.clone(),
            target,
        );
        c.operations
            .push(knowledgehub::ontology::Operation::Put(root));
        c.operations
            .push(knowledgehub::ontology::Operation::Gaps(vec![]));
        c
    };
    let x = candidate("root-X", specimens::reference("EntityType"));
    let y = candidate("root-Y", member.reference);
    assert!(
        h.validate(seed::context(), x.clone())
            .await
            .unwrap()
            .is_complete()
    );
    assert!(
        h.validate(seed::context(), y.clone())
            .await
            .unwrap()
            .is_complete()
    );
    let (rx, ry) = tokio::join!(
        h.commit(seed::context(), x.clone()),
        second.commit(seed::context(), y.clone())
    );
    assert_ne!(rx.is_ok(), ry.is_ok());
    let loser = if rx.is_ok() {
        assert!(matches!(ry, Err(Error::Busy | Error::Conflict { .. })));
        y
    } else {
        assert!(matches!(rx, Err(Error::Busy | Error::Conflict { .. })));
        x
    };
    assert!(matches!(
        h.commit(seed::context(), loser).await,
        Err(Error::Conflict { .. })
    ));
    let (_, state) = h.snapshot(seed::context()).await.unwrap();
    assert_eq!(state.selected_records().filter(|r|matches!(&r.data,RecordData::Assertion(a) if a.signature==relationship_ref("AggregateDefinition","hasRoot"))).count(),1);
    baseline(&h, "race-winner").await;
}

#[tokio::test]
async fn damaged_typed_binary_is_integrity_failure_and_never_operation_absence() {
    let s = Sandbox::new();
    let (h, _) = register(&s).await;
    let denied = Context::new("host:denied", std::sync::Arc::new(DenyAll));
    assert!(matches!(
        h.snapshot(denied.clone()).await,
        Err(Error::Denied { .. })
    ));
    let q = Query::select(Selection::Snapshot(h.head(seed::context()).await.unwrap()));
    assert!(matches!(
        h.query(denied, q).await,
        Err(Error::Denied { .. })
    ));
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let (key, content) = state
        .artifacts
        .iter()
        .find(|(_, c)| c.bytes.starts_with(b"%PDF"))
        .expect("retained original PDF");
    let reference = state.records[key].reference.clone();
    let bytes = &content.bytes[..content.bytes.len().min(1_048_576)];
    use sha2::{Digest, Sha256};
    let mut hash = Sha256::new();
    hash.update(format!("knowledgehub-object/1\nblob\n{}\n", bytes.len()).as_bytes());
    hash.update(bytes);
    let oid = format!("{:x}", hash.finalize());
    let path = s
        .store
        .join("objects/loose")
        .join(&oid[..2])
        .join(format!("{}.obj", &oid[2..]));
    let mut frame = std::fs::read(&path).unwrap();
    frame[0] ^= 1;
    std::fs::write(path, frame).unwrap();
    assert!(matches!(
        h.read_artifact(
            seed::context(),
            Selection::Snapshot(head.clone()),
            reference
        )
        .await,
        Err(Error::Integrity(_))
    ));
    assert!(matches!(
        h.read_state(seed::context(), Selection::Snapshot(head))
            .await,
        Err(Error::Integrity(_))
    ));
    // Selective receipt resolution never turns this known committed operation into absence.
    assert!(matches!(
        h.operation_result(seed::context(), "register".into())
            .await
            .unwrap(),
        OperationOutcome::Committed(_)
    ));
    assert!(matches!(
        backend(&s).scrub(seed::context(), model_scope()).await,
        Err(Error::Integrity(_))
    ));
}

#[test]
fn invalid_baseline_closures_and_competing_ownership_are_rejected() {
    for mode in [
        "duplicate",
        "missing-support",
        "missing-selected-endpoint",
        "wrong-profile",
    ] {
        let mut s = specimens::specimen_state();
        let record = s
            .records
            .get_mut(&specimens::reference("ModelBaseline").key())
            .unwrap();
        let RecordData::Baseline(b) = &mut record.data else {
            unreachable!()
        };
        match mode {
            "duplicate" => b.selected.push(b.selected[0].clone()),
            "missing-support" => {
                b.selected.retain(|r| *r != specimens::anchor());
                b.supporting.retain(|r| *r != specimens::anchor());
            }
            "missing-selected-endpoint" => b
                .selected
                .retain(|r| *r != specimens::reference("Requirement")),
            _ => b.profile = ontology_ref(),
        };
        record.seal().unwrap();
        assert!(!assess(&s).is_valid(), "{mode}");
    }
    let mut s = specimens::specimen_state();
    let r = s
        .records
        .get_mut(&specimens::reference("Requirement").key())
        .unwrap();
    let RecordData::Element(e) = &mut r.data else {
        unreachable!()
    };
    e.owners.push(Ownership {
        path: "/statement".into(),
        owner: PropertyOwner::Graph,
    });
    r.seal().unwrap();
    assert!(assess(&s).violations.iter().any(|v| v.rule == "V-06"));
    let mut s = specimens::specimen_state();
    let r = s
        .records
        .get_mut(&specimens::reference("DesignToken").key())
        .unwrap();
    specimens::put_property(r, "value", Value::Bool(true));
    assert!(assess(&s).violations.iter().any(|v| v.rule == "V-13"));
}

#[tokio::test]
async fn baseline_comparison_reports_unicode_additions_removals_and_stable_order() {
    let s = Sandbox::new();
    let (h, _) = register(&s).await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let template = specimens::specimen_state()
        .get(&specimens::reference("Requirement"))
        .unwrap()
        .clone();
    let mut c = Change::at(&state, head, "unicode-A");
    for id in ["é", "😀"] {
        let mut record = template.clone();
        record.reference = ExactRef::new("unicode", id, "1");
        record.seal().unwrap();
        c.operations
            .push(knowledgehub::ontology::Operation::Put(record));
    }
    h.commit(seed::context(), c).await.unwrap();
    let (a, _) = baseline(&h, "unicode-A").await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut c = Change::at(&state, head, "unicode-B");
    c.operations
        .push(knowledgehub::ontology::Operation::Unselect(ExactRef::new(
            "unicode", "é", "1",
        )));
    let mut record = template;
    record.reference = ExactRef::new("unicode", "中", "1");
    record.seal().unwrap();
    c.operations
        .push(knowledgehub::ontology::Operation::Put(record));
    h.commit(seed::context(), c).await.unwrap();
    let (b, _) = baseline(&h, "unicode-B").await;
    let changes = h.compare(seed::context(), a, b).await.unwrap();
    let unicode = changes
        .iter()
        .filter(|d| d.identity.starts_with("unicode\0"))
        .collect::<Vec<_>>();
    assert_eq!(unicode.len(), 2);
    assert!(unicode[0].identity.ends_with('é') && unicode[0].after.is_none());
    assert!(unicode[1].identity.ends_with('中') && unicode[1].before.is_none());
    assert!(changes.windows(2).all(|a| a[0].identity < a[1].identity));
}

#[test]
fn native_decision_tables_remain_exact_and_definitions_do_not_become_observations() {
    let mut s = specimens::specimen_state();
    let table = specimens::reference("decision-table");
    let value = Value::Record(std::collections::BTreeMap::from([
        ("form".into(), Value::enumeration("decision-table")),
        ("artifact".into(), Value::Artifact(table.clone())),
        (
            "input".into(),
            Value::Ref(specimens::reference("DataSchema")),
        ),
        (
            "output".into(),
            Value::Ref(specimens::reference("DataSchema")),
        ),
    ]));
    specimens::put_property(
        s.records
            .get_mut(&specimens::reference("FunctionDefinition").key())
            .unwrap(),
        "body",
        value,
    );
    assert!(assess(&s).is_complete());
    assert!(
        String::from_utf8_lossy(&s.artifact(&table).unwrap().bytes).contains("current_authority")
    );
    // A planned definition is not an observed build or a captured execution log.
    specimens::put_property(
        s.records
            .get_mut(&specimens::reference("TestRun").key())
            .unwrap(),
        "evidence",
        Value::List(vec![Value::Artifact(specimens::document())]),
    );
    assert!(assess(&s).violations.iter().any(|v| v.rule == "V-15"));
}

#[test]
fn retained_definitions_do_not_hide_a_missing_active_family() {
    let mut state = specimens::catalogue_state();
    let o = ExactRef::new("subset", "ontology", "1");
    let p = ExactRef::new("subset", "profile", "1");
    let RecordData::Ontology(mut ontology) = state.get(&ontology_ref()).unwrap().data.clone()
    else {
        unreachable!()
    };
    ontology
        .types
        .retain(|t| !state.type_definition(t).unwrap().family.contains("AI"));
    ontology.profiles = vec![p.clone()];
    let vocabulary = ontology.types.clone();
    let RecordData::Profile(mut profile) = state.get(&profile_ref()).unwrap().data.clone() else {
        unreachable!()
    };
    profile.ontology = o.clone();
    profile.vocabulary = vocabulary;
    specimens::install(
        &mut state,
        Record::new(
            o.clone(),
            type_ref("OntologyDefinition"),
            RecordData::Ontology(ontology),
            vec![specimens::anchor()],
        )
        .unwrap(),
    );
    specimens::install(
        &mut state,
        Record::new(
            p.clone(),
            type_ref("ProfileDefinition"),
            RecordData::Profile(profile),
            vec![specimens::anchor()],
        )
        .unwrap(),
    );
    state.ontology = Some(o);
    state.profile = Some(p);
    assert!(assess(&state).is_valid(), "{:#?}", assess(&state));
    assert_eq!(catalogue_coverage(&state).len(), 9);
}

#[test]
fn nested_owner_paths_distinguish_literal_slashes_and_tildes() {
    let data = ElementData::new(std::collections::BTreeMap::from([
        ("a/b".into(), Value::Bool(true)),
        (
            "a".into(),
            Value::Record(std::collections::BTreeMap::from([(
                "b".into(),
                Value::Bool(true),
            )])),
        ),
        ("~".into(), Value::Bool(false)),
    ]));
    let paths = data
        .owners
        .iter()
        .map(|o| o.path.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(paths.len(), data.owners.len());
    assert!(paths.contains("/a~1b") && paths.contains("/a/b") && paths.contains("/~0"));
}

#[tokio::test]
async fn selection_restores_exact_history_and_refuses_a_second_origin() {
    let s = Sandbox::new();
    let (h, _) = register(&s).await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut original = specimens::specimen_state()
        .get(&specimens::reference("Requirement"))
        .unwrap()
        .clone();
    original.reference = ExactRef::new("selection", "requirement", "1");
    original.seal().unwrap();
    let mut c = Change::at(&state, head, "selection:first");
    c.operations
        .push(knowledgehub::ontology::Operation::Put(original.clone()));
    h.commit(seed::context(), c).await.unwrap();
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut c = Change::at(&state, head, "selection:remove");
    c.operations
        .push(knowledgehub::ontology::Operation::Unselect(
            original.reference.clone(),
        ));
    h.commit(seed::context(), c).await.unwrap();
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut successor = original.clone();
    successor.reference.revision = "2".into();
    successor.seal().unwrap();
    let mut c = Change::at(&state, head, "selection:successor");
    c.operations
        .push(knowledgehub::ontology::Operation::Put(successor.clone()));
    assert!(
        !h.validate(seed::context(), c.clone())
            .await
            .unwrap()
            .is_valid()
    );
    successor.predecessor = Some(original.reference.clone());
    successor.seal().unwrap();
    c.operations = vec![
        knowledgehub::ontology::Operation::Select(original.reference),
        knowledgehub::ontology::Operation::Put(successor),
    ];
    assert!(
        h.validate(seed::context(), c.clone())
            .await
            .unwrap()
            .is_complete()
    );
    h.commit(seed::context(), c).await.unwrap();
}

#[tokio::test]
async fn directional_kernel_traversal_keeps_cycles_budgets_and_old_pages_exact() {
    let sandbox = Sandbox::new();
    let hub = full_model(&sandbox).await;
    let (head, state) = hub.snapshot(seed::context()).await.unwrap();
    let a = specimens::reference("edge:R08");
    let mut b = state.get(&a).unwrap().clone();
    b.reference = ExactRef::new("retained:external", "assertion:B", "1");
    b.seal().unwrap();
    let mut c = b.clone();
    c.reference = specimens::reference("assertion:C");
    c.seal().unwrap();
    let mut change = Change::at(&state, head, "navigation:cycle");
    change.operations = vec![
        knowledgehub::ontology::Operation::Put(b.clone()),
        knowledgehub::ontology::Operation::Put(c.clone()),
        knowledgehub::ontology::Operation::Unselect(specimens::reference("edge:R32")),
    ];
    for (id, from, to) in [
        ("cycle:AB", a.clone(), b.reference.clone()),
        ("cycle:BC", b.reference.clone(), c.reference.clone()),
        ("cycle:CA", c.reference.clone(), a.clone()),
    ] {
        change
            .operations
            .push(knowledgehub::ontology::Operation::Put(
                specimens::assertion(id, "RelationshipAssertion", "contradicts", from, to),
            ));
    }
    let receipt = hub.commit(seed::context(), change).await.unwrap();
    let mut query = Query::select(Selection::Snapshot(receipt.head.clone()));
    assert_eq!(
        (
            query.page_size,
            query.depth,
            query.subject_limit,
            query.edge_limit
        ),
        (100, 1, 10000, 10000)
    );
    query.starts = vec![a.clone()];
    query.signatures = vec![relationship_ref("RelationshipAssertion", "contradicts")];
    query.depth = 3;
    query.page_size = 1000;
    let forward = hub.query(seed::context(), query.clone()).await.unwrap();
    assert!(forward.exhausted);
    assert_eq!(
        forward
            .assertions
            .iter()
            .map(|r| r.reference.clone())
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        6
    );
    assert!(
        forward
            .assertions
            .iter()
            .any(|r| r.reference == b.reference)
    );
    let mut reverse = query.clone();
    reverse.direction = Direction::Reverse;
    let backward = hub.query(seed::context(), reverse).await.unwrap();
    assert!(backward.exhausted);
    assert_eq!(
        backward.assertions[1].reference,
        specimens::reference("cycle:CA")
    );
    assert_eq!(
        forward.assertions[1].reference,
        specimens::reference("cycle:AB")
    );
    for (index, mut excessive) in [query.clone(), query.clone(), query.clone()]
        .into_iter()
        .enumerate()
    {
        match index {
            0 => excessive.page_size = 1001,
            1 => excessive.subject_limit = 10001,
            _ => excessive.edge_limit = 10001,
        }
        assert!(matches!(
            hub.query(seed::context(), excessive).await,
            Err(Error::InvalidInput(_))
        ));
    }
    let mut select = Query::select(Selection::Snapshot(receipt.head));
    select.type_ref = Some(type_ref("Requirement"));
    select.ids.insert(specimens::reference("Requirement").id);
    let selected = hub.query(seed::context(), select).await.unwrap();
    assert_eq!(selected.records.len(), 1);
    assert_eq!(
        selected.records[0].reference,
        specimens::reference("Requirement")
    );
    let mut bounded = query.clone();
    bounded.edge_limit = 2;
    bounded.page_size = 2;
    let first = hub.query(seed::context(), bounded.clone()).await.unwrap();
    let mut rows = first.assertions;
    let mut cursor = first.continuation;
    let (head, state) = hub.snapshot(seed::context()).await.unwrap();
    let mut next = b.clone();
    next.reference.revision = "2".into();
    next.predecessor = Some(b.reference.clone());
    next.seal().unwrap();
    let mut revision = Change::at(&state, head, "navigation:move-head");
    revision.operations = specimens::successor_operations(&state, vec![next.clone()]).unwrap();
    hub.commit(seed::context(), revision).await.unwrap();
    loop {
        let token = cursor.take().expect("remaining bounded page");
        let page = hub
            .continue_query(seed::context(), bounded.clone(), token)
            .await
            .unwrap();
        assert!(page.records.len() + page.assertions.len() <= 2);
        rows.extend(page.assertions);
        cursor = page.continuation;
        if cursor.is_none() {
            assert!(!page.exhausted);
            assert_eq!(page.truncated.as_deref(), Some("edge/work budget"));
            break;
        }
    }
    assert_eq!(rows.len(), 5);
    assert!(rows.iter().any(|r| r.reference == b.reference));
    assert!(!rows.iter().any(|r| r.reference == next.reference));
    let mut depth = query.clone();
    depth.depth = 0;
    let limited = hub.query(seed::context(), depth).await.unwrap();
    assert_eq!(limited.truncated.as_deref(), Some("depth bound"));
    let mut subjects = query;
    subjects.subject_limit = 1;
    let limited = hub.query(seed::context(), subjects).await.unwrap();
    assert_eq!(limited.truncated.as_deref(), Some("subject budget"));
}
