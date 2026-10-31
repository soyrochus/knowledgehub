#[path = "../examples/neighbour_support/mod.rs"]
mod fixture;
mod neighbour_oracle;
#[path = "../examples/support/mod.rs"]
mod seed;
mod support;
use knowledgehub::{Error, ontology::*};
use std::collections::BTreeSet;

#[tokio::test]
async fn paging_is_bounded_and_survives_head_movement_and_cache_eviction() {
    let s = support::Sandbox::new();
    let (b, h) = fixture::create(&s.store).await.unwrap();
    fixture::commit(
        &h,
        "fanout",
        (0..24)
            .map(|i| Operation::Put(fixture::edge(&format!("edge:{i:03}"), "a", "b")))
            .collect(),
    )
    .await
    .unwrap();
    let (head, state) = h.snapshot(fixture::context()).await.unwrap();
    let mut q = fixture::query(Selection::Snapshot(head.clone()), "b");
    q.page_size = 3;
    let first = h.neighbours(fixture::context(), q.clone()).await.unwrap();
    assert_eq!(first.neighbours.len(), 3);
    assert_eq!(first.reached, Some(NeighbourLimit::Page));
    let mut token = first.continuation;
    let mut all = first.neighbours;
    fixture::commit(
        &h,
        "future",
        (0..3)
            .map(|i| Operation::Put(fixture::edge(&format!("future:{i}"), "a", "b")))
            .collect(),
    )
    .await
    .unwrap();
    b.clear_neighbour_cache(fixture::context(), fixture::scope())
        .await
        .unwrap();
    while let Some(c) = token {
        let p = h
            .continue_neighbours(fixture::context(), q.clone(), c)
            .await
            .unwrap();
        assert_eq!(p.selection, q.selection);
        all.extend(p.neighbours);
        token = p.continuation;
    }
    assert_eq!(all, neighbour_oracle::neighbours(&state, &q));
    assert!(!all.iter().any(|n| n.origin.owner.id.starts_with("future:")));
    let mut filtered = q.clone();
    filtered.page_size = 1000;
    filtered.examined_limit = 1;
    filtered.opposite_type = Some(type_ref("SourceArtifact"));
    let mut p = h
        .neighbours(fixture::context(), filtered.clone())
        .await
        .unwrap();
    assert!(p.neighbours.is_empty());
    assert!(!p.exhausted);
    assert_eq!(p.reached, Some(NeighbourLimit::ExaminedPostings));
    let mut pages = 1;
    while let Some(c) = p.continuation {
        p = h
            .continue_neighbours(fixture::context(), filtered.clone(), c)
            .await
            .unwrap();
        assert!(p.neighbours.is_empty());
        assert!(p.work.examined_postings <= 1);
        pages += 1;
    }
    assert!(p.exhausted);
    assert!(pages > 24);
    let mut projected = q.clone();
    projected.page_size = 1000;
    projected.projection = NeighbourProjection::Records;
    projected.byte_limit = 8000;
    let p = h
        .neighbours(fixture::context(), projected.clone())
        .await
        .unwrap();
    assert_eq!(p.reached, Some(NeighbourLimit::ProjectedBytes));
    assert!(!p.records.is_empty());
    assert!(
        serde_json::to_vec(&(&p.neighbours, &p.vertices, &p.records))
            .unwrap()
            .len()
            <= 8000
    );
    h.release_cursor(fixture::context(), p.continuation.unwrap())
        .await
        .unwrap();
    projected.byte_limit = 1;
    assert!(matches!(
        h.neighbours(fixture::context(), projected).await,
        Err(Error::Capacity)
    ));
}

#[tokio::test]
async fn successors_selection_and_schema_versions_equal_clean_rebuilds() {
    let s = support::Sandbox::new();
    let (b, h) = fixture::create(&s.store).await.unwrap();
    let head = h.head(fixture::context()).await.unwrap();
    let pinned = fixture::query(Selection::Snapshot(head), "b");
    let original = h
        .neighbours(fixture::context(), pinned.clone())
        .await
        .unwrap()
        .neighbours;
    let base = fixture::baseline(&h, "original").await.unwrap();
    let (head, state) = h.snapshot(fixture::context()).await.unwrap();
    let mut next = state.get(&fixture::reference("a")).unwrap().clone();
    next.reference.revision = "2".into();
    next.predecessor = Some(fixture::reference("a"));
    if let RecordData::Element(e) = &mut next.data {
        e.properties.insert("links".into(), Value::List(vec![]));
        e.properties.insert("any".into(), Value::Null);
        e.properties.insert("choice".into(), Value::text("removed"));
        e.properties.insert(
            "nested".into(),
            Value::Record(std::collections::BTreeMap::from([(
                "next".into(),
                Value::Ref(fixture::reference("c")),
            )])),
        );
        e.owners = ElementData::new(e.properties.clone()).owners;
    }
    next.seal().unwrap();
    let mut c = Change::at(&state, head, "successor:a");
    c.operations.push(Operation::Put(next.clone()));
    h.commit(fixture::context(), c).await.unwrap();
    for (id, ops) in [
        (
            "unselect",
            vec![Operation::Unselect(next.reference.clone())],
        ),
        (
            "select-original",
            vec![Operation::Select(fixture::reference("a"))],
        ),
    ] {
        fixture::commit(&h, id, ops).await.unwrap();
        let (head, state) = h.snapshot(fixture::context()).await.unwrap();
        let q = fixture::query(Selection::Snapshot(head), "b");
        let (actual, _) = fixture::collect(&h, q.clone()).await.unwrap();
        assert_eq!(actual, neighbour_oracle::neighbours(&state, &q));
    }
    let (head, state) = h.snapshot(fixture::context()).await.unwrap();
    let mut definition = state.get(&fixture::node_type()).unwrap().clone();
    definition.reference.revision = "2".into();
    definition.predecessor = Some(fixture::node_type());
    if let RecordData::Type(t) = &mut definition.data {
        t.properties.insert(
            "extra".into(),
            Property {
                value: ValueKind::Reference(vec!["AnySubject".into()]),
                required: false,
                nullable: false,
                owner: PropertyOwner::Graph,
            },
        );
    }
    definition.seal().unwrap();
    let mut ontology = state.get(&fixture::reference("ontology")).unwrap().clone();
    ontology.reference.revision = "2".into();
    ontology.predecessor = Some(fixture::reference("ontology"));
    if let RecordData::Ontology(o) = &mut ontology.data {
        o.types.retain(|r| *r != fixture::node_type());
        o.types.push(definition.reference.clone());
        o.profiles = vec![ExactRef::new("neighbour-fixture", "profile", "2")];
    }
    ontology.seal().unwrap();
    let mut profile = state.get(&fixture::reference("profile")).unwrap().clone();
    profile.reference.revision = "2".into();
    profile.predecessor = Some(fixture::reference("profile"));
    if let RecordData::Profile(p) = &mut profile.data {
        p.ontology = ontology.reference.clone();
        p.vocabulary.retain(|r| *r != fixture::node_type());
        p.vocabulary.push(definition.reference.clone());
    }
    profile.seal().unwrap();
    let mut new_owner = fixture::node("new-schema-owner", "b", false);
    new_owner.type_ref = definition.reference.clone();
    if let RecordData::Element(e) = &mut new_owner.data {
        e.properties
            .insert("extra".into(), Value::Ref(fixture::reference("b")));
        e.owners = ElementData::new(e.properties.clone()).owners;
    }
    new_owner.seal().unwrap();
    let mut c = Change::at(&state, head, "schema:2");
    for name in ["a", "b", "c"] {
        let old = state.current("neighbour-fixture", name).unwrap();
        let mut owner = old.clone();
        owner.reference.revision = "schema-2".into();
        owner.predecessor = Some(old.reference.clone());
        owner.type_ref = definition.reference.clone();
        owner.seal().unwrap();
        c.operations.push(Operation::Put(owner));
    }
    c.operations.extend([
        Operation::Put(definition),
        Operation::Put(ontology.clone()),
        Operation::Put(profile.clone()),
        Operation::Put(new_owner.clone()),
        Operation::SelectProfile {
            ontology: ontology.reference,
            profile: profile.reference,
        },
    ]);
    h.commit(fixture::context(), c).await.unwrap();
    let (head, state) = h.snapshot(fixture::context()).await.unwrap();
    let q = fixture::query(Selection::Snapshot(head), "b");
    let p = h.neighbours(fixture::context(), q.clone()).await.unwrap();
    assert!(p.work.index_rebuilt);
    assert_eq!(p.neighbours, neighbour_oracle::neighbours(&state, &q));
    assert!(p.neighbours.iter().any(|n|n.origin.owner==new_owner.reference&&matches!(&n.selector,NeighbourSelector::Property{definition,..}if definition.revision=="2")));
    b.clear_neighbour_cache(fixture::context(), fixture::scope())
        .await
        .unwrap();
    assert_eq!(
        h.neighbours(fixture::context(), q)
            .await
            .unwrap()
            .neighbours,
        p.neighbours
    );
    assert_eq!(
        h.neighbours(
            fixture::context(),
            fixture::query(Selection::Baseline(base), "b")
        )
        .await
        .unwrap()
        .neighbours,
        original
    );
    assert_eq!(
        h.neighbours(fixture::context(), pinned)
            .await
            .unwrap()
            .neighbours,
        original
    );
}

#[tokio::test]
async fn legacy_assertion_traversal_order_filters_and_limits_match_files2() {
    let old = support::Sandbox::new();
    let (_, legacy) = fixture::populate(
        knowledgehub::backends::files::FileBackend::new_ontology(
            &old.store,
            knowledgehub::backends::files::FileOptions::local_ext4(),
        )
        .unwrap(),
    )
    .await
    .unwrap();
    let new = support::Sandbox::new();
    let (b, indexed) = fixture::create(&new.store).await.unwrap();
    let operations = vec![
        Operation::Put(fixture::edge("z:a-b", "a", "b")),
        Operation::Put(fixture::edge("a:a-c", "a", "c")),
        Operation::Put(fixture::edge("b:c-a", "c", "a")),
        Operation::Put(fixture::edge("c:self", "b", "b")),
        Operation::Put(fixture::edge("d:parallel", "a", "b")),
    ];
    fixture::commit(&legacy, "edges", operations.clone())
        .await
        .unwrap();
    fixture::commit(&indexed, "edges", operations)
        .await
        .unwrap();
    let old_head = legacy.head(fixture::context()).await.unwrap();
    let new_head = indexed.head(fixture::context()).await.unwrap();
    for case in 0..14 {
        let mut q = Query::select(Selection::Snapshot(old_head.clone()));
        q.starts = vec![fixture::reference("a"), fixture::reference("b")];
        q.page_size = 2;
        q.depth = case % 5;
        q.direction = if case % 2 == 0 {
            Direction::Forward
        } else {
            Direction::Reverse
        };
        if case % 3 == 0 {
            q.signatures = vec![fixture::relation()];
        }
        if case % 4 == 0 {
            q.type_ref = Some(fixture::node_type());
            q.ids.insert("a".into());
        }
        if case > 8 {
            q.subject_limit = 2;
            q.edge_limit = 2;
        }
        let mut modern = q.clone();
        modern.selection = Selection::Snapshot(new_head.clone());
        let before = b.file_work();
        async fn pages(
            h: &OntologyHub,
            q: Query,
        ) -> (Vec<Record>, Vec<Record>, Option<String>, bool) {
            let mut p = h.query(fixture::context(), q.clone()).await.unwrap();
            let mut records = p.records;
            let mut assertions = p.assertions;
            while let Some(c) = p.continuation {
                p = h
                    .continue_query(fixture::context(), q.clone(), c)
                    .await
                    .unwrap();
                records.extend(p.records);
                assertions.extend(p.assertions);
            }
            (records, assertions, p.truncated, p.exhausted)
        }
        assert_eq!(pages(&legacy, q).await, pages(&indexed, modern).await);
        assert_eq!(
            before.state_materialisations,
            b.file_work().state_materialisations
        );
    }
}

#[tokio::test]
async fn generic_registered_nested_native_and_parallel_facts_match_independent_oracle() {
    let s = support::Sandbox::new();
    let (_, h) = fixture::create(&s.store).await.unwrap();
    let mut special = fixture::node("special", "a", true);
    special.type_ref = fixture::special_type();
    if let RecordData::Element(e) = &mut special.data {
        e.properties.insert(
            "any".into(),
            Value::Record(std::collections::BTreeMap::from([
                (
                    "identity".into(),
                    Value::Ref(ExactRef::new("neighbour-fixture", "a", "@identity")),
                ),
                (
                    "artifact".into(),
                    Value::Artifact(ExactRef::new(
                        "sdo",
                        "urn:source:ontology:capture",
                        "catalogue-1",
                    )),
                ),
                (
                    "narrative".into(),
                    Value::Narrative {
                        document: ExactRef::new(
                            "sdo",
                            "urn:source:ontology:capture",
                            "catalogue-1",
                        ),
                        fragment: fixture::anchor(),
                    },
                ),
                (
                    "set".into(),
                    Value::Set(vec![
                        Value::Ref(fixture::reference("a")),
                        Value::Ref(fixture::reference("b")),
                    ]),
                ),
            ])),
        );
        e.owners = ElementData::new(e.properties.clone()).owners;
    }
    special.seal().unwrap();
    let mut qualified = fixture::edge("qualified", "a", "b");
    if let RecordData::Assertion(a) = &mut qualified.data {
        a.qualifiers.insert(
            "context".into(),
            Value::List(vec![
                Value::Ref(fixture::reference("c")),
                Value::Ref(fixture::reference("c")),
            ]),
        );
    }
    qualified.seal().unwrap();
    fixture::commit(
        &h,
        "more-facts",
        vec![
            Operation::Put(special),
            Operation::Put(qualified),
            Operation::Put(fixture::edge("parallel:1", "a", "b")),
            Operation::Put(fixture::edge("parallel:2", "a", "b")),
            Operation::Put(fixture::edge("self", "a", "a")),
        ],
    )
    .await
    .unwrap();
    let (head, state) = h.snapshot(fixture::context()).await.unwrap();
    for direction in [
        NeighbourDirection::Outgoing,
        NeighbourDirection::Incoming,
        NeighbourDirection::Both,
    ] {
        let mut q = Neighbours::new(
            Selection::Snapshot(head.clone()),
            vec![
                fixture::reference("a"),
                fixture::reference("b"),
                fixture::reference("special"),
                fixture::relation(),
            ],
        );
        q.direction = direction;
        q.page_size = 3;
        q.categories = BTreeSet::from([
            NeighbourCategory::Association,
            NeighbourCategory::Property,
            NeighbourCategory::Representation,
            NeighbourCategory::Provenance,
            NeighbourCategory::Administrative,
        ]);
        q.byte_limit = 1_048_576;
        let expected = neighbour_oracle::neighbours(&state, &q);
        let (actual, _) = fixture::collect(&h, q.clone()).await.unwrap();
        assert_eq!(actual, expected);
    }
    let mut every = Neighbours::new(
        Selection::Snapshot(head.clone()),
        state
            .records
            .values()
            .map(|r| r.reference.clone())
            .chain([ExactRef::new("neighbour-fixture", "a", "@identity")])
            .collect(),
    );
    every.categories = [
        NeighbourCategory::Association,
        NeighbourCategory::Property,
        NeighbourCategory::Representation,
        NeighbourCategory::Provenance,
        NeighbourCategory::Administrative,
    ]
    .into_iter()
    .collect();
    every.page_size = 1000;
    every.byte_limit = 1_048_576;
    let (actual, _) = fixture::collect(&h, every.clone()).await.unwrap();
    let expected = neighbour_oracle::neighbours(&state, &every);
    let mismatch = actual.iter().zip(&expected).position(|(a, b)| a != b);
    assert!(
        actual == expected,
        "all-fact oracle: actual={}, expected={}, first_mismatch={:?}, actual_item={:?}, expected_item={:?}",
        actual.len(),
        expected.len(),
        mismatch,
        mismatch.and_then(|i| actual.get(i)),
        mismatch.and_then(|i| expected.get(i))
    );
    let mut q = fixture::query(Selection::Snapshot(head), "a");
    q.direction = NeighbourDirection::Incoming;
    q.opposite_type = Some(fixture::node_type());
    q.include_subtypes = true;
    let (actual, _) = fixture::collect(&h, q.clone()).await.unwrap();
    assert_eq!(actual, neighbour_oracle::neighbours(&state, &q));
    assert!(
        actual
            .iter()
            .any(|n| n.opposite == fixture::reference("special"))
    );
    for selector in [
        NeighbourSelector::Property {
            definition: fixture::node_type(),
            path: "/not-declared".into(),
        },
        NeighbourSelector::Property {
            definition: fixture::node_type(),
            path: "/links/0/ref~1key".into(),
        },
        NeighbourSelector::Native {
            definition: fixture::node_type(),
            path: "/selected/*".into(),
        },
    ] {
        let mut unsupported = q.clone();
        unsupported.selectors = BTreeSet::from([selector]);
        assert!(matches!(
            h.neighbours(fixture::context(), unsupported).await,
            Err(Error::InvalidInput(_))
        ));
    }
    q.include_subtypes = false;
    let (actual, _) = fixture::collect(&h, q.clone()).await.unwrap();
    assert_eq!(actual, neighbour_oracle::neighbours(&state, &q));
    assert!(
        !actual
            .iter()
            .any(|n| n.opposite == fixture::reference("special"))
    );
}

#[tokio::test]
async fn strict_contract_bounds_and_explicit_legacy_capability() {
    let s = support::Sandbox::new();
    let h = OntologyHub::create(
        fixture::backend(&s.store),
        fixture::context(),
        fixture::scope(),
    )
    .await
    .unwrap();
    let q = fixture::query(
        Selection::Snapshot(h.head(fixture::context()).await.unwrap()),
        "absent",
    );
    assert_eq!(
        Neighbours::from_json(&serde_json::to_vec(&q).unwrap()).unwrap(),
        q
    );
    let mut json = serde_json::to_value(&q).unwrap();
    json["unrecognised"] = true.into();
    assert!(Neighbours::from_json(&serde_json::to_vec(&json).unwrap()).is_err());
    for (field, value) in [
        ("direction", "Sideways"),
        ("fact_scope", "Latest"),
        ("projection", "DriverCursor"),
    ] {
        let mut v = serde_json::to_value(&q).unwrap();
        v[field] = value.into();
        assert!(Neighbours::from_json(&serde_json::to_vec(&v).unwrap()).is_err());
    }
    for which in 0..6 {
        let mut bad = q.clone();
        match which {
            0 => bad.page_size = 1001,
            1 => bad.starts.clear(),
            2 => bad.examined_limit = 10001,
            3 => bad.byte_limit = 1_048_577,
            4 => bad.include_subtypes = true,
            _ => bad
                .selectors
                .insert(NeighbourSelector::Property {
                    definition: fixture::node_type(),
                    path: "/bad~escape".into(),
                })
                .then_some(())
                .unwrap(),
        };
        assert!(matches!(
            h.neighbours(fixture::context(), bad).await,
            Err(Error::InvalidInput(_))
        ));
    }
    let mut ready = q.clone();
    ready.preparation.allow_rebuild = false;
    assert!(matches!(
        h.neighbours(fixture::context(), ready.clone()).await,
        Err(Error::IndexNotReady { .. })
    ));
    let page = h.neighbours(fixture::context(), q.clone()).await.unwrap();
    assert!(page.exhausted);
    assert!(!page.vertices[0].available);
    assert!(page.work.index_rebuilt);
    assert!(
        !h.neighbours(fixture::context(), ready)
            .await
            .unwrap()
            .work
            .index_rebuilt
    );
    let old = support::Sandbox::new();
    let h = OntologyHub::create(
        knowledgehub::backends::files::FileBackend::new_ontology(
            &old.store,
            knowledgehub::backends::files::FileOptions::local_ext4(),
        )
        .unwrap(),
        fixture::context(),
        fixture::scope(),
    )
    .await
    .unwrap();
    let head = h.head(fixture::context()).await.unwrap();
    assert!(matches!(
        h.neighbours(
            fixture::context(),
            fixture::query(Selection::Snapshot(head.clone()), "a")
        )
        .await,
        Err(Error::Incompatible(_))
    ));
    let query = Query::select(Selection::Snapshot(head));
    let fields = serde_json::to_value(&query)
        .unwrap()
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        fields,
        [
            "selection",
            "type_ref",
            "ids",
            "starts",
            "signatures",
            "direction",
            "depth",
            "page_size",
            "subject_limit",
            "edge_limit"
        ]
        .map(String::from)
        .into_iter()
        .collect()
    );
    assert!(h.query(fixture::context(), query).await.unwrap().exhausted);
}

#[tokio::test]
async fn late_links_exact_scopes_and_ready_point_reads() {
    let s = support::Sandbox::new();
    let (b, h) = fixture::create(&s.store).await.unwrap();
    let old = h.head(fixture::context()).await.unwrap();
    let mut q = fixture::query(Selection::Snapshot(old.clone()), "b");
    q.direction = NeighbourDirection::Incoming;
    let p = h.neighbours(fixture::context(), q.clone()).await.unwrap();
    assert!(p.exhausted);
    assert!(p.work.index_rebuilt);
    assert_eq!(
        p.neighbours
            .iter()
            .filter(|n| n.origin.owner == fixture::reference("a"))
            .count(),
        6
    );
    let a = fixture::baseline(&h, "A").await.unwrap();
    let (q0, _) = fixture::collect(&h, fixture::query(Selection::Baseline(a.clone()), "b"))
        .await
        .unwrap();
    assert!(
        !q0.iter()
            .any(|n| n.category == NeighbourCategory::Association)
    );
    let (change, receipt) = fixture::commit(
        &h,
        "late-links",
        vec![
            Operation::Put(fixture::edge("edge:ab", "a", "b")),
            Operation::Put(fixture::edge("edge:parallel", "a", "b")),
            Operation::Put(fixture::edge("edge:self", "b", "b")),
        ],
    )
    .await
    .unwrap();
    let q = fixture::query(Selection::Snapshot(receipt.head.clone()), "b");
    let before = b.file_work();
    let p = h.neighbours(fixture::context(), q.clone()).await.unwrap();
    let after = b.file_work();
    assert!(!p.work.index_rebuilt);
    assert_eq!(before.state_materialisations, after.state_materialisations);
    assert!(p.work.record_reads < 15);
    assert_eq!(
        p.neighbours
            .iter()
            .filter(|n| n.category == NeighbourCategory::Association)
            .count(),
        3
    );
    let self_loop = p
        .neighbours
        .iter()
        .find(|n| n.origin.owner == fixture::reference("edge:self"))
        .unwrap();
    assert_eq!(
        self_loop.directions,
        BTreeSet::from([NeighbourDirection::Outgoing, NeighbourDirection::Incoming])
    );
    let (old_result, _) = fixture::collect(&h, fixture::query(Selection::Baseline(a), "b"))
        .await
        .unwrap();
    assert!(
        !old_result
            .iter()
            .any(|n| n.category == NeighbourCategory::Association)
    );
    assert_eq!(h.commit(fixture::context(), change).await.unwrap(), receipt);
    fixture::commit(
        &h,
        "unselect-old",
        vec![Operation::Unselect(fixture::reference("edge:ab"))],
    )
    .await
    .unwrap();
    let head = h.head(fixture::context()).await.unwrap();
    let q = fixture::query(Selection::Snapshot(head.clone()), "b");
    let selected = h.neighbours(fixture::context(), q.clone()).await.unwrap();
    assert!(
        !selected
            .neighbours
            .iter()
            .any(|n| n.origin.owner == fixture::reference("edge:ab"))
    );
    let mut closure = q;
    closure.fact_scope = FactScope::Closure;
    let retained = h.neighbours(fixture::context(), closure).await.unwrap();
    assert!(
        retained
            .neighbours
            .iter()
            .any(|n| n.origin.owner == fixture::reference("edge:ab"))
    );
    h.drain().await;
    let reopened = OntologyHub::open(
        fixture::backend(&s.store),
        fixture::context(),
        fixture::scope(),
    )
    .await
    .unwrap();
    assert_eq!(reopened.head(fixture::context()).await.unwrap(), head);
}
