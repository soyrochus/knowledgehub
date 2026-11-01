#[path = "../examples/neighbour_support/mod.rs"]
mod fixture;
mod impact_oracle;
#[path = "../examples/support/mod.rs"]
mod seed;
mod support;
use knowledgehub::{Error, ontology::*};
use std::collections::BTreeSet;
fn selector() -> NeighbourSelector {
    NeighbourSelector::Assertion {
        definition: fixture::relation(),
    }
}
fn step(direction: NeighbourDirection) -> TraversalStep {
    TraversalStep {
        direction,
        selectors: BTreeSet::from([selector()]),
        categories: BTreeSet::from([NeighbourCategory::Association]),
        opposite_type: None,
        include_subtypes: false,
    }
}
fn request() -> ModelImpact {
    ModelImpact {
        selection: None,
        starts: ImpactStarts::Exact {
            references: vec![fixture::reference("a")],
        },
        match_policy: MatchPolicy::AllGrouped,
        plan: TraversalPlan::Repeated {
            alternatives: vec![step(NeighbourDirection::Outgoing)],
        },
        target: TargetPredicate {
            type_ref: Some(fixture::node_type()),
            ..Default::default()
        },
        fact_scope: FactScope::Selected,
        projection: ImpactProjection::default(),
        budgets: ReadBudgets::default(),
    }
}
fn text(selection: Selection, phrase: &str) -> SearchText {
    SearchText {
        selection,
        text: TextSelector {
            phrase: phrase.into(),
            mode: LiteralMode::AsciiFold,
            fields: BTreeSet::from([TextField {
                type_ref: fixture::node_type(),
                path: "/name".into(),
            }]),
        },
        budgets: ReadBudgets::default(),
    }
}
async fn collect(h: &OntologyHub, q: ModelImpact) -> Vec<Dependency> {
    let mut p = h.model_impact(fixture::context(), q.clone()).await.unwrap();
    let mut all = p.dependencies;
    while let Some(c) = p.continuation {
        p = h
            .continue_model_impact(fixture::context(), q.clone(), c)
            .await
            .unwrap();
        all.extend(p.dependencies);
    }
    all
}
#[tokio::test]
async fn literal_modes_exact_fields_candidate_verification_and_incremental_successors() {
    let s = support::Sandbox::new();
    let (_, h) = fixture::create(&s.store).await.unwrap();
    let mut a = fixture::node("text:a", "b", false);
    let RecordData::Element(e) = &mut a.data else {
        unreachable!()
    };
    e.properties.insert(
        "name".into(),
        Value::text("é INCREASE THE RENT ÄBC abcXXbcd"),
    );
    e.properties
        .insert("literal".into(), Value::text("cross boundary"));
    a.seal().unwrap();
    fixture::commit(&h, "text", vec![Operation::Put(a.clone())])
        .await
        .unwrap();
    let head = h.head(fixture::context()).await.unwrap();
    let q = text(Selection::Snapshot(head.clone()), "increase the rent");
    let p = h.search_text(fixture::context(), q.clone()).await.unwrap();
    assert_eq!(p.hits.len(), 1);
    assert_eq!(p.hits[0].snippet.as_deref(), Some("INCREASE THE RENT"));
    assert_eq!(p.hits[0].start, 3);
    assert!(p.exhausted);
    let mut exact = q.clone();
    exact.text.mode = LiteralMode::Utf8;
    assert!(
        h.search_text(fixture::context(), exact)
            .await
            .unwrap()
            .hits
            .is_empty()
    );
    let mut nonascii = q.clone();
    nonascii.text.phrase = "äbc".into();
    assert!(
        h.search_text(fixture::context(), nonascii.clone())
            .await
            .unwrap()
            .hits
            .is_empty()
    );
    nonascii.text.phrase = "Äbc".into();
    assert_eq!(
        h.search_text(fixture::context(), nonascii)
            .await
            .unwrap()
            .hits
            .len(),
        1
    );
    let mut grams = q.clone();
    grams.text.phrase = "abcd".into();
    let p = h.search_text(fixture::context(), grams).await.unwrap();
    assert!(p.hits.is_empty());
    assert!(p.work.candidates_verified > 0);
    let mut cross = q.clone();
    cross.text.phrase = "RENT cross".into();
    cross.text.fields.insert(TextField {
        type_ref: fixture::node_type(),
        path: "/literal".into(),
    });
    assert!(
        h.search_text(fixture::context(), cross)
            .await
            .unwrap()
            .hits
            .is_empty()
    );
    for phrase in ["".into(), "ab".into(), "x".repeat(4097)] {
        let mut bad = q.clone();
        bad.text.phrase = phrase;
        assert!(matches!(
            h.search_text(fixture::context(), bad).await,
            Err(Error::InvalidInput(_))
        ));
    }
    let mut bad = q.clone();
    bad.text.fields = BTreeSet::from([TextField {
        type_ref: fixture::node_type(),
        path: "/any".into(),
    }]);
    assert!(matches!(
        h.search_text(fixture::context(), bad).await,
        Err(Error::InvalidInput(_))
    ));
    let mut a2 = a.clone();
    a2.predecessor = Some(a.reference);
    a2.reference.revision = "2".into();
    let RecordData::Element(e) = &mut a2.data else {
        unreachable!()
    };
    e.properties
        .insert("name".into(), Value::text("INCREASE THE RENT wording only"));
    a2.seal().unwrap();
    fixture::commit(&h, "successor", vec![Operation::Put(a2.clone())])
        .await
        .unwrap();
    let new = h.head(fixture::context()).await.unwrap();
    let mut next = q.clone();
    next.selection = Selection::Snapshot(new);
    next.budgets.preparation.allow_rebuild = false;
    let p = h.search_text(fixture::context(), next).await.unwrap();
    assert_eq!(p.hits[0].record, a2.reference);
    assert_eq!(p.work.prepared_owners, 0);
    assert_eq!(
        h.search_text(fixture::context(), q).await.unwrap().hits[0]
            .record
            .revision,
        "1"
    );
}
#[test]
fn strict_owned_contracts_reject_unknown_modes_fields_and_unbounded_plans() {
    let q = request();
    let bytes = serde_json::to_vec(&q).unwrap();
    assert_eq!(ModelImpact::from_json(&bytes).unwrap(), q);
    let mut v = serde_json::to_value(&q).unwrap();
    v["agent"] = "infer".into();
    assert!(ModelImpact::from_json(&serde_json::to_vec(&v).unwrap()).is_err());
    let mut v = serde_json::to_value(text(
        Selection::Snapshot(knowledgehub::SnapshotId("s".into())),
        "abc",
    ))
    .unwrap();
    v["text"]["mode"] = "semantic/1".into();
    assert!(serde_json::from_value::<SearchText>(v).is_err());
    let mut q = q;
    q.budgets.hops = 33;
    assert!(ModelImpact::from_json(&serde_json::to_vec(&q).unwrap()).is_err());
}
#[tokio::test]
async fn mixed_seven_hop_plan_cycles_parallel_origins_and_nearer_targets() {
    let s = support::Sandbox::new();
    let (_, h) = fixture::create(&s.store).await.unwrap();
    let mut ops = (0..7)
        .map(|i| Operation::Put(fixture::node(&format!("v{i}"), "b", false)))
        .collect::<Vec<_>>();
    let mut from = "a".to_owned();
    for i in 0..7 {
        let to = format!("v{i}");
        let (f, t) = if i % 2 == 0 {
            (&from, &to)
        } else {
            (&to, &from)
        };
        ops.push(Operation::Put(fixture::edge(&format!("e{i}"), f, t)));
        from = to;
    }
    ops.push(Operation::Put(fixture::edge("parallel", "a", "v0")));
    ops.push(Operation::Put(fixture::edge("cycle", "v6", "a")));
    fixture::commit(&h, "chain", ops).await.unwrap();
    let mut q = request();
    q.plan = TraversalPlan::Ordered {
        steps: (0..7)
            .map(|i| {
                vec![step(if i % 2 == 0 {
                    NeighbourDirection::Outgoing
                } else {
                    NeighbourDirection::Incoming
                })]
            })
            .collect(),
    };
    q.budgets.witnesses = 2;
    let p = h.model_impact(fixture::context(), q.clone()).await.unwrap();
    assert_eq!(p.dependencies.len(), 8);
    let far = p
        .dependencies
        .iter()
        .find(|d| d.target == fixture::reference("v6"))
        .unwrap();
    assert_eq!(far.witnesses[0].len(), 7);
    assert_eq!(far.witnesses.len(), 2);
    assert_eq!(
        p.dependencies
            .iter()
            .find(|d| d.target == fixture::reference("v0"))
            .unwrap()
            .witnesses
            .len(),
        2
    );
    let (_, state) = h.snapshot(fixture::context()).await.unwrap();
    let directions = (0..7)
        .map(|i| {
            if i % 2 == 0 {
                NeighbourDirection::Outgoing
            } else {
                NeighbourDirection::Incoming
            }
        })
        .collect::<Vec<_>>();
    let oracle = impact_oracle::paths(
        &state,
        &[fixture::reference("a")],
        &directions,
        &fixture::relation(),
    );
    assert_eq!(
        impact_oracle::memberships(&p),
        oracle.keys().cloned().collect()
    );
    for d in &p.dependencies {
        let observed = d
            .witnesses
            .iter()
            .map(|path| {
                path.iter()
                    .map(|n| n.origin.owner.clone())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert!(observed.iter().all(|path| oracle[&d.target].contains(path)));
    }
    q.plan = TraversalPlan::Repeated {
        alternatives: vec![step(NeighbourDirection::Both)],
    };
    q.budgets.hops = 10;
    let p = h.model_impact(fixture::context(), q).await.unwrap();
    assert!(p.work.visited_states <= 8);
    assert!(p.exhausted);
}
#[tokio::test]
async fn composite_ambiguity_grouping_paging_full_intent_and_pin() {
    let s = support::Sandbox::new();
    let (b, h) = fixture::create(&s.store).await.unwrap();
    fixture::commit(
        &h,
        "links",
        vec![
            Operation::Put(fixture::edge("ab", "a", "b")),
            Operation::Put(fixture::edge("cb", "c", "b")),
        ],
    )
    .await
    .unwrap();
    let mut q = request();
    q.starts = ImpactStarts::Text {
        selector: text(
            Selection::Snapshot(h.head(fixture::context()).await.unwrap()),
            "Synthetic node",
        )
        .text,
    };
    q.match_policy = MatchPolicy::RequireOne;
    let p = h.model_impact(fixture::context(), q.clone()).await.unwrap();
    assert_eq!(p.match_status, MatchStatus::Ambiguous);
    assert_eq!(p.starts.len(), 3);
    assert!(p.dependencies.is_empty());
    q.match_policy = MatchPolicy::AllGrouped;
    q.target.ids = BTreeSet::from([fixture::reference("b")]);
    q.projection.target_records = true;
    let p = h.model_impact(fixture::context(), q.clone()).await.unwrap();
    assert_eq!(p.dependencies.len(), 2);
    assert_eq!(p.records.len(), 1);
    q.budgets.page_size = 1;
    let p = h.model_impact(fixture::context(), q.clone()).await.unwrap();
    let token = p.continuation.unwrap();
    let mut changed = q.clone();
    changed.projection.snippets = false;
    assert_eq!(
        h.continue_model_impact(fixture::context(), changed, token.clone())
            .await
            .unwrap_err(),
        Error::InvalidCursor
    );
    fixture::commit(
        &h,
        "later",
        vec![Operation::Put(fixture::edge("future", "b", "a"))],
    )
    .await
    .unwrap();
    let second = h
        .continue_model_impact(fixture::context(), q.clone(), token.clone())
        .await
        .unwrap();
    assert_eq!(p.selection, second.selection);
    assert!(second.continuation.is_none());
    assert_eq!(
        h.continue_model_impact(fixture::context(), q.clone(), token)
            .await
            .unwrap_err(),
        Error::InvalidCursor
    );
    let first = h.model_impact(fixture::context(), q.clone()).await.unwrap();
    let token = first.continuation.unwrap();
    let reopened = OntologyHub::open(
        fixture::backend(&s.store),
        fixture::context(),
        fixture::scope(),
    )
    .await
    .unwrap();
    assert_eq!(
        reopened
            .continue_model_impact(fixture::context(), q, token)
            .await
            .unwrap_err(),
        Error::InvalidCursor
    );
    assert!(b.file_work().state_materialisations > 0);
}
#[tokio::test]
async fn finite_work_unknown_selectors_unresolved_starts_and_compact_large_targets() {
    let s = support::Sandbox::new();
    let (_, h) = fixture::create(&s.store).await.unwrap();
    let mut large = fixture::node("large", "b", false);
    let RecordData::Element(e) = &mut large.data else {
        unreachable!()
    };
    e.properties
        .insert("name".into(), Value::text("x".repeat(100000)));
    large.seal().unwrap();
    fixture::commit(
        &h,
        "large",
        vec![
            Operation::Put(large),
            Operation::Put(fixture::edge("large-edge", "a", "large")),
        ],
    )
    .await
    .unwrap();
    let mut q = request();
    q.projection.target_records = true;
    q.budgets.bytes = 4096;
    let p = h.model_impact(fixture::context(), q.clone()).await.unwrap();
    assert_eq!(p.dependencies.len(), 1);
    assert_eq!(p.omitted_records, vec![fixture::reference("large")]);
    assert!(serde_json::to_vec(&p).unwrap().len() <= 4096);
    q.budgets.examined = 1;
    let p = h.model_impact(fixture::context(), q.clone()).await.unwrap();
    assert!(p.reached.contains(&ReadLimit::Examined));
    assert!(!p.exhausted);
    q.starts = ImpactStarts::Exact {
        references: vec![ExactRef::new("external", "unavailable", "1")],
    };
    let p = h.model_impact(fixture::context(), q).await.unwrap();
    assert_eq!(p.match_status, MatchStatus::Incomplete);
    assert_eq!(p.coverage.unresolved.len(), 1);
    assert!(!p.coverage.domain_complete);
    let mut q = request();
    let TraversalPlan::Repeated { alternatives } = &mut q.plan else {
        unreachable!()
    };
    alternatives[0].selectors = BTreeSet::from([NeighbourSelector::Assertion {
        definition: ExactRef::new("external", "unknown", "1"),
    }]);
    assert!(matches!(
        h.model_impact(fixture::context(), q).await,
        Err(Error::InvalidInput(_))
    ));
}
#[tokio::test]
async fn pinned_results_survive_reopen_rebuild_and_late_link_without_full_state_loads() {
    let s = support::Sandbox::new();
    let (b, h) = fixture::create(&s.store).await.unwrap();
    fixture::commit(
        &h,
        "first",
        vec![Operation::Put(fixture::edge("ab", "a", "b"))],
    )
    .await
    .unwrap();
    let mut q = request();
    q.selection = Some(Selection::Snapshot(
        h.head(fixture::context()).await.unwrap(),
    ));
    let expected = collect(&h, q.clone()).await;
    let before = b.file_work().state_materialisations;
    let p = h.model_impact(fixture::context(), q.clone()).await.unwrap();
    assert_eq!(b.file_work().state_materialisations, before);
    assert_eq!(p.dependencies, expected);
    fixture::commit(
        &h,
        "late",
        vec![Operation::Put(fixture::edge("ac", "a", "c"))],
    )
    .await
    .unwrap();
    b.clear_neighbour_cache(fixture::context(), fixture::scope())
        .await
        .unwrap();
    let h = OntologyHub::open(
        fixture::backend(&s.store),
        fixture::context(),
        fixture::scope(),
    )
    .await
    .unwrap();
    assert_eq!(collect(&h, q.clone()).await, expected);
    q.selection = None;
    assert_eq!(collect(&h, q).await.len(), 2);
}
