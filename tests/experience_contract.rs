#[path = "../examples/support/mod.rs"]
mod seed;
#[path = "../examples/ontology_support/mod.rs"]
mod specimens;
mod support;
use knowledgehub::{ontology::*, *};
use std::{collections::BTreeMap, sync::Arc};

// Small equipment-reservation model. Source content and domain names are fixture
// data; records are built using the same public schemas as catalogue conformance.
fn equipment() -> (State, Vec<Operation>) {
    let mut state = specimens::catalogue_state();
    let mut ops = specimens::specimens()
        .unwrap()
        .into_iter()
        .filter(|o| {
            o.record().is_some_and(|r| {
                [
                    "SystemModel",
                    "Actor",
                    "Journey",
                    "UserStory",
                    "UserActivity",
                    "UserTask",
                    "StoryMap",
                    "StoryPlacement",
                    "OutcomeCut",
                    "Scenario",
                    "Requirement",
                ]
                .iter()
                .any(|n| r.type_ref == type_ref(n))
            })
        })
        .collect::<Vec<_>>();
    let source = specimens::reference("equipment-source");
    let capture = specimens::reference("equipment-capture");
    let anchor = specimens::reference("equipment-anchor");
    let text = "Equipment reservation: a borrower finds and reserves equipment, then returns to inspect the booking. Shared stories appear in two tasks and cumulative cuts. A denied booking preserves the existing reservation. This is an authored synthetic design, not execution evidence.";
    let content = Content::capture(text.as_bytes().to_vec());
    ops.push(Operation::Put(
        Record::new(
            source.clone(),
            type_ref("SourceArtifact"),
            RecordData::SourceArtifact(SourceArtifactData {
                origin: "fixture:equipment-reservation".into(),
                media_type: "text/plain".into(),
                owner: "fixture:author".into(),
                status: "authored-requirement".into(),
            }),
            vec![],
        )
        .unwrap(),
    ));
    ops.push(Operation::Capture {
        record: Record::new(
            capture.clone(),
            type_ref("SourceRevision"),
            RecordData::SourceRevision(SourceRevisionData {
                source,
                original: content.reference.clone(),
                media_type: "text/plain".into(),
                derived_from: None,
                derivation: None,
            }),
            vec![],
        )
        .unwrap(),
        content,
    });
    ops.push(Operation::Put(
        Record::new(
            anchor.clone(),
            type_ref("SourceAnchor"),
            RecordData::Anchor(AnchorData {
                source: capture.clone(),
                label: "Reservation design".into(),
                locator: Locator::ByteRange {
                    start: 0,
                    end: text.len() as u64,
                    quote: text.into(),
                },
            }),
            vec![],
        )
        .unwrap(),
    ));
    fn rebind(v: &mut Value, capture: &ExactRef, anchor: &ExactRef) {
        match v {
            Value::Narrative { document, fragment } => {
                *document = capture.clone();
                *fragment = anchor.clone();
            }
            Value::Record(p) => p.values_mut().for_each(|v| rebind(v, capture, anchor)),
            Value::Set(p) | Value::List(p) => p.iter_mut().for_each(|v| rebind(v, capture, anchor)),
            _ => {}
        }
    }
    for op in &mut ops {
        if let Operation::Put(r) = op {
            if let RecordData::Element(e) = &mut r.data {
                e.properties
                    .values_mut()
                    .for_each(|v| rebind(v, &capture, &anchor));
                e.properties.insert(
                    "name".into(),
                    Value::text(format!("Equipment {}", r.reference.id)),
                );
                e.owners = ElementData::new(e.properties.clone()).owners;
                r.provenance = vec![anchor.clone()];
                r.seal().unwrap();
            }
        }
    }
    // A second journey and task reuse one story through a distinct placement.
    for (kind, id) in [
        ("Journey", "return-journey"),
        ("UserTask", "return-task"),
        ("StoryPlacement", "return-placement"),
        ("OutcomeCut", "later-cut"),
    ] {
        let mut r = ops
            .iter()
            .filter_map(Operation::record)
            .find(|r| r.reference == specimens::reference(kind))
            .unwrap()
            .clone();
        r.reference = specimens::reference(id);
        if kind == "StoryPlacement" {
            specimens::put_property(
                &mut r,
                "task",
                Value::Ref(specimens::reference("return-task")),
            );
        }
        if kind == "OutcomeCut" {
            specimens::put_property(
                &mut r,
                "prerequisites",
                Value::List(vec![Value::Ref(specimens::reference("OutcomeCut"))]),
            );
        }
        r.seal().unwrap();
        ops.push(Operation::Put(r));
    }
    for op in &mut ops {
        if let Operation::Put(r) = op {
            let n = r.reference.id.as_str();
            if n == "specimen:StoryMap" || n == "specimen:UserActivity" {
                specimens::put_property(
                    r,
                    "journeys",
                    Value::Set(vec![
                        Value::Ref(specimens::reference("Journey")),
                        Value::Ref(specimens::reference("return-journey")),
                    ]),
                );
            }
            match r.reference.id.as_str() {
                "specimen:StoryMap" => {
                    specimens::put_property(
                        r,
                        "placements",
                        Value::List(vec![
                            Value::Ref(specimens::reference("StoryPlacement")),
                            Value::Ref(specimens::reference("return-placement")),
                        ]),
                    );
                    specimens::put_property(
                        r,
                        "cuts",
                        Value::List(vec![
                            Value::Ref(specimens::reference("OutcomeCut")),
                            Value::Ref(specimens::reference("later-cut")),
                        ]),
                    );
                }
                "specimen:UserActivity" => specimens::put_property(
                    r,
                    "tasks",
                    Value::List(vec![
                        Value::Ref(specimens::reference("UserTask")),
                        Value::Ref(specimens::reference("return-task")),
                    ]),
                ),
                _ => {}
            }
        }
    }
    ops.push(Operation::Put(specimens::assertion(
        "scenario-trace",
        "Scenario",
        "illustrates",
        specimens::reference("Scenario"),
        specimens::reference("Requirement"),
    )));
    for op in &ops {
        if let Some(r) = op.record() {
            specimens::install(&mut state, r.clone());
        }
        if let Operation::Capture { record, content } = op {
            state
                .artifacts
                .insert(record.reference.key(), content.clone());
        }
    }
    (state, ops)
}

fn edit(s: &mut State, kind: &str, field: &str, v: Value) {
    specimens::put_property(
        s.records
            .get_mut(&specimens::reference(kind).key())
            .unwrap(),
        field,
        v,
    );
}

#[test]
fn equipment_map_shared_stories_cuts_and_all_structural_rejections() {
    let (accepted, _) = equipment();
    assert!(assess(&accepted).is_complete(), "{:#?}", assess(&accepted));
    for (kind, field, v) in [
        (
            "StoryMap",
            "journey",
            Value::Ref(specimens::reference("Journey")),
        ),
        (
            "OutcomeCut",
            "stories",
            Value::List(vec![Value::Ref(specimens::reference("UserStory"))]),
        ),
        ("UserActivity", "journeys", Value::Set(vec![])),
        (
            "UserActivity",
            "tasks",
            Value::List(vec![Value::Ref(specimens::reference("UserTask")); 2]),
        ),
        (
            "StoryMap",
            "activities",
            Value::List(vec![Value::Ref(specimens::reference("UserActivity")); 2]),
        ),
        (
            "StoryMap",
            "placements",
            Value::List(vec![Value::Ref(specimens::reference("StoryPlacement")); 2]),
        ),
        ("StoryPlacement", "order", Value::Integer("-1".into())),
        (
            "return-placement",
            "task",
            Value::Ref(specimens::reference("UserTask")),
        ),
        (
            "OutcomeCut",
            "prerequisites",
            Value::List(vec![Value::Ref(specimens::reference("later-cut"))]),
        ),
        (
            "UserTask",
            "activity",
            Value::Ref(specimens::reference("Requirement")),
        ),
    ] {
        let mut candidate = accepted.clone();
        edit(&mut candidate, kind, field, v);
        assert!(
            !assess(&candidate).is_valid(),
            "accepted invalid {kind}.{field}"
        );
    }
    // Valid but foreign objects expose ownership errors independently of typing.
    for field in ["journeys", "activities", "placements", "cuts"] {
        let mut s = accepted.clone();
        let mut foreign = s.get(&specimens::reference("StoryMap")).unwrap().clone();
        foreign.reference = specimens::reference("foreign-map");
        foreign.seal().unwrap();
        specimens::install(&mut s, foreign);
        if field == "journeys" {
            edit(
                &mut s,
                "StoryMap",
                field,
                Value::Set(vec![Value::Ref(specimens::reference("Journey"))]),
            );
        } else if field == "activities" {
            edit(&mut s, "foreign-map", "activities", Value::List(vec![]));
            edit(
                &mut s,
                "StoryPlacement",
                "map",
                Value::Ref(specimens::reference("foreign-map")),
            );
        } else {
            edit(
                &mut s,
                if field == "cuts" {
                    "later-cut"
                } else {
                    "StoryPlacement"
                },
                "map",
                Value::Ref(specimens::reference("foreign-map")),
            );
        }
        assert!(!assess(&s).is_valid(), "foreign {field}");
    }
}

fn dependency(id: &str, from: ExactRef, to: ExactRef, conditional: bool) -> Record {
    let mut r = specimens::assertion(id, "UserStory", "dependsOn", from, to);
    if let RecordData::Assertion(a) = &mut r.data {
        a.qualifiers = BTreeMap::from([
            (
                "dependency_kind".into(),
                Value::enumeration(if conditional {
                    "conditional"
                } else {
                    "required"
                }),
            ),
            (
                "condition".into(),
                if conditional {
                    specimens::narrative()
                } else {
                    Value::Null
                },
            ),
        ]);
    }
    r.seal().unwrap();
    r
}

#[tokio::test]
async fn guarded_publication_navigation_baselines_reopen_and_recovery() {
    let sandbox = support::Sandbox::new();
    let hub = KnowledgeHub::create(sandbox.backend(), seed::context(), specimens::scope())
        .await
        .unwrap();
    let (head, s) = hub.snapshot(seed::context()).await.unwrap();
    let mut registration = Change::at(&s, head, "equipment:register");
    registration.operations = builtin_catalogue().unwrap().operations();
    struct Editor;
    impl Policy for Editor {
        fn allows(&self, _: &str, _: &ModelScope, a: Action) -> Result<bool, PolicyFailure> {
            Ok(a != Action::RegisterOntology)
        }
    }
    assert!(matches!(
        hub.commit(
            Context::new("host:editor", Arc::new(Editor)),
            registration.clone()
        )
        .await,
        Err(Error::Denied { .. })
    ));
    hub.commit(seed::context(), registration).await.unwrap();
    let (head, s) = hub.snapshot(seed::context()).await.unwrap();
    let (model, ops) = equipment();
    let baseline = specimens::reference("equipment-A");
    let mut change = Change::at(&s, head, "equipment:publish");
    change.operations = ops;
    change.operations.push(Operation::Put(
        Record::new(
            baseline.clone(),
            type_ref("ModelBaseline"),
            RecordData::Baseline(model.baseline_data().unwrap()),
            vec![specimens::anchor()],
        )
        .unwrap(),
    ));
    assert!(
        hub.validate(seed::context(), change.clone())
            .await
            .unwrap()
            .is_complete()
    );
    let receipt = hub.commit(seed::context(), change.clone()).await.unwrap();
    // Lost response: exact original intent recovers before the stale-base check.
    assert_eq!(
        receipt,
        hub.commit(seed::context(), change.clone()).await.unwrap()
    );
    let mut changed = change.clone();
    changed.operations.pop();
    assert!(matches!(
        hub.commit(seed::context(), changed).await,
        Err(Error::OperationReuse)
    ));
    let mut stale = change;
    stale.operation_id = "equipment:stale".into();
    assert!(matches!(
        hub.commit(seed::context(), stale).await,
        Err(Error::Conflict { .. })
    ));
    let (head, s) = hub.snapshot(seed::context()).await.unwrap();
    let mut next = s.get(&specimens::reference("UserStory")).unwrap().clone();
    next.reference.revision = "2".into();
    next.predecessor = Some(specimens::reference("UserStory"));
    specimens::put_property(&mut next, "name", Value::text("Revised reservation story"));
    let mut successor = Change::at(&s, head, "equipment:B");
    successor.operations = vec![Operation::Put(next)];
    let b = hub.commit(seed::context(), successor).await.unwrap();
    for direction in [Direction::Forward, Direction::Reverse] {
        let mut q = Query::select(Selection::Baseline(baseline.clone()));
        q.starts = vec![specimens::reference(if direction == Direction::Forward {
            "Scenario"
        } else {
            "Requirement"
        })];
        q.signatures = vec![relationship_ref("Scenario", "illustrates")];
        q.direction = direction;
        q.depth = 2;
        let p = hub.query(seed::context(), q).await.unwrap();
        assert!(p.exhausted);
        assert_eq!(p.assertions.len(), 1);
        assert_eq!(p.records.len(), 2);
        assert!(
            p.records
                .iter()
                .any(|r| r.reference == specimens::reference("Scenario"))
        );
        assert!(
            p.records
                .iter()
                .any(|r| r.reference == specimens::reference("Requirement"))
        );
    }
    for (id, conditional) in [("required", false), ("conditional", true)] {
        let (head, s) = hub.snapshot(seed::context()).await.unwrap();
        let mut story = s.get(&specimens::reference("UserStory")).unwrap().clone();
        story.reference = specimens::reference("prerequisite");
        story.seal().unwrap();
        let a = ExactRef::new("specimen", "specimen:UserStory", "2");
        let mut invalid = Change::at(&s, head.clone(), format!("invalid:{id}"));
        invalid.operations = vec![
            Operation::Put(story.clone()),
            Operation::Put(dependency("a", a.clone(), story.reference.clone(), false)),
            Operation::Put(dependency("b", story.reference, a, conditional)),
        ];
        if conditional {
            // Informational conditional return edge is legitimate.
            assert!(
                hub.validate(seed::context(), invalid.clone())
                    .await
                    .unwrap()
                    .is_complete()
            );
            if let Operation::Put(r) = &mut invalid.operations[2] {
                if let RecordData::Assertion(a) = &mut r.data {
                    a.qualifiers.insert("condition".into(), Value::Null);
                }
                r.seal().unwrap();
            }
        }
        assert!(
            !hub.validate(seed::context(), invalid.clone())
                .await
                .unwrap()
                .is_valid()
        );
        assert!(hub.commit(seed::context(), invalid).await.is_err());
        assert_eq!(head, hub.snapshot(seed::context()).await.unwrap().0);
    }
    hub.drain().await;
    let reopened = KnowledgeHub::open(sandbox.backend(), seed::context(), specimens::scope())
        .await
        .unwrap();
    assert_eq!(
        reopened
            .operation_result(seed::context(), "equipment:publish".into())
            .await
            .unwrap(),
        OperationOutcome::Committed(Box::new(receipt))
    );
    assert_eq!(reopened.snapshot(seed::context()).await.unwrap().0, b.head);
    let a = reopened
        .read(
            seed::context(),
            Selection::Baseline(baseline.clone()),
            specimens::reference("UserStory"),
        )
        .await
        .unwrap();
    assert_ne!(
        a.element().unwrap()["name"],
        Value::text("Revised reservation story")
    );
    let placements = reopened
        .read(
            seed::context(),
            Selection::Baseline(baseline),
            specimens::reference("StoryMap"),
        )
        .await
        .unwrap();
    assert_eq!(placements.element().unwrap()["placements"].refs().len(), 2);
    let bytes = reopened
        .read_artifact(
            seed::context(),
            Selection::Snapshot(b.head),
            specimens::reference("equipment-capture"),
        )
        .await
        .unwrap();
    assert!(
        std::str::from_utf8(&bytes.bytes)
            .unwrap()
            .starts_with("Equipment reservation:")
    );
}
