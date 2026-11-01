#[path = "../examples/neighbour_support/mod.rs"]
mod fixture;
#[path = "../examples/rent_support/mod.rs"]
mod rent;
#[path = "../examples/support/mod.rs"]
mod seed;
#[path = "../examples/ontology_support/mod.rs"]
mod specimens;
mod support;
use knowledgehub::{Error, ontology::*};
#[tokio::test]
async fn narrative_rent_route_has_exact_original_bytes_seven_origins_and_ddl() {
    let s = support::Sandbox::new();
    let (b, h) = rent::create(&s.store).await.unwrap();
    let q = rent::query();
    let p = h.model_impact(fixture::context(), q.clone()).await.unwrap();
    assert_eq!(p.match_status, MatchStatus::Matched);
    assert_eq!(p.dependencies.len(), 1);
    assert_eq!(p.dependencies[0].witnesses[0].len(), 7);
    assert_eq!(p.records.len(), 1);
    assert_eq!(p.records[0].type_ref, rent::physical_type());
    assert_eq!(
        p.records[0].element().unwrap()["native_artifact"],
        Value::Artifact(rent::reference("ddl"))
    );
    let hit = &p.hits[0];
    let source = hit.source.as_ref().unwrap();
    assert_eq!(source.document, rent::reference("story-document"));
    let bytes = h
        .read_artifact_range(
            fixture::context(),
            p.selection.clone(),
            source.span_document.clone(),
            source.start,
            source.end - source.start,
        )
        .await
        .unwrap();
    assert_eq!(
        std::str::from_utf8(&bytes.bytes).unwrap(),
        "INCREASE THE RENT"
    );
    assert_eq!(hit.snippet.as_deref(), Some("INCREASE THE RENT"));
    assert_eq!(p.coverage.meaning, "candidate_dependency");
    assert!(!p.coverage.domain_complete);
    let mut compact = q.clone();
    compact.budgets.bytes = 4096;
    let compact = h.model_impact(fixture::context(), compact).await.unwrap();
    assert!(compact.dependencies[0].omitted);
    assert!(compact.continuation.is_none());
    assert!(serde_json::to_vec(&compact).unwrap().len() <= 4096);
    let mut pinned = q.clone();
    pinned.selection = Some(p.selection.clone());
    let before = b.file_work().state_materialisations;
    let again = h
        .model_impact(fixture::context(), pinned.clone())
        .await
        .unwrap();
    assert_eq!(p.dependencies, again.dependencies);
    assert_eq!(p.hits, again.hits);
    assert_eq!(b.file_work().state_materialisations, before);
    // Rebuild both derived indexes; accepted graph, exact snippets and coverage stay identical.
    b.clear_neighbour_cache(fixture::context(), rent::scope())
        .await
        .unwrap();
    std::fs::remove_dir_all(s.store.join("cache/text")).unwrap();
    let reopened = OntologyHub::open(
        knowledgehub::backends::files::FileBackend::new_shared_ontology(
            &s.store,
            knowledgehub::backends::files::FileOptions::local_ext4(),
        )
        .unwrap(),
        fixture::context(),
        rent::scope(),
    )
    .await
    .unwrap();
    let after = reopened
        .model_impact(fixture::context(), pinned)
        .await
        .unwrap();
    assert_eq!(p.dependencies, after.dependencies);
    assert_eq!(p.hits, after.hits);
    assert_eq!(p.coverage, after.coverage);
}
#[tokio::test]
async fn missing_mapping_gaps_wording_successor_and_late_alternate_mapping_are_honest() {
    let s = support::Sandbox::new();
    let (_, h) = rent::create(&s.store).await.unwrap();
    let q = rent::query();
    let original = h.model_impact(fixture::context(), q.clone()).await.unwrap();
    let (head, state) = h.snapshot(fixture::context()).await.unwrap();
    let mut story = state
        .get(&specimens::reference("UserStory"))
        .unwrap()
        .clone();
    story.predecessor = Some(story.reference.clone());
    story.reference.revision = "wording-2".into();
    specimens::put_property(
        &mut story,
        "name",
        Value::text("Clarify the wording of the rent story"),
    );
    let mut c = Change::at(&state, head, "wording-only");
    c.operations = specimens::successor_operations(&state, vec![story]).unwrap();
    h.commit(fixture::context(), c).await.unwrap();
    let wording = h.model_impact(fixture::context(), q.clone()).await.unwrap();
    assert_eq!(
        original.dependencies[0].target,
        wording.dependencies[0].target
    );
    assert_eq!(wording.coverage.meaning, "candidate_dependency");
    let (_, state) = h.snapshot(fixture::context()).await.unwrap();
    let (table, mapping) = rent::mapped_table(&state, "audit-table", "rent_audit");
    fixture::commit(
        &h,
        "late-map",
        vec![Operation::Put(table), Operation::Put(mapping)],
    )
    .await
    .unwrap();
    assert_eq!(
        h.model_impact(fixture::context(), q.clone())
            .await
            .unwrap()
            .dependencies
            .len(),
        2
    );
    // Remove selected mapping facts through the ordinary API; old baselines retain them.
    fixture::commit(
        &h,
        "unselect-physical-mappings",
        vec![
            Operation::Unselect(rent::reference("mapping:rent-table")),
            Operation::Unselect(rent::reference("mapping:audit-table")),
        ],
    )
    .await
    .unwrap();
    let p = h.model_impact(fixture::context(), q.clone()).await.unwrap();
    assert!(p.dependencies.is_empty());
    assert!(p.coverage.no_witnessed_dependency);
    assert!(!p.coverage.domain_complete);
    assert!(!p.coverage.dead_ends.is_empty());
    fixture::commit(
        &h,
        "gap",
        vec![Operation::Gaps(vec![Gap {
            subject: specimens::reference("EntityType"),
            path: "physical_mapping".into(),
            reason: "Fixture mapping coverage is intentionally incomplete".into(),
            provenance: vec![specimens::anchor()],
        }])],
    )
    .await
    .unwrap();
    assert_eq!(
        h.model_impact(fixture::context(), q)
            .await
            .unwrap()
            .coverage
            .gaps
            .len(),
        1
    );
}
#[tokio::test]
async fn text_readiness_corruption_policy_and_source_successor_remain_explicit() {
    let s = support::Sandbox::new();
    let (_, h) = rent::create(&s.store).await.unwrap();
    let q = rent::query();
    let ImpactStarts::Text { selector } = q.starts else {
        unreachable!()
    };
    let mut t = SearchText {
        selection: Selection::Snapshot(h.head(fixture::context()).await.unwrap()),
        text: selector,
        budgets: ReadBudgets::default(),
    };
    t.budgets.preparation.allow_rebuild = false;
    assert!(matches!(
        h.search_text(fixture::context(), t.clone()).await,
        Err(Error::IndexNotReady { .. })
    ));
    t.budgets.preparation.allow_rebuild = true;
    let p = h.search_text(fixture::context(), t.clone()).await.unwrap();
    assert_eq!(p.hits.len(), 1);
    let path = std::fs::read_dir(s.store.join("cache/text/views"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    std::fs::write(path, b"corrupt").unwrap();
    t.budgets.preparation.allow_rebuild = false;
    assert!(matches!(
        h.search_text(fixture::context(), t.clone()).await,
        Err(Error::IndexNotReady { .. })
    ));
    t.budgets.preparation.allow_rebuild = true;
    assert_eq!(
        h.search_text(fixture::context(), t.clone())
            .await
            .unwrap()
            .hits,
        p.hits
    );
    assert!(matches!(
        h.search_text(
            knowledgehub::Context::new(
                "fixture:neighbour-author",
                std::sync::Arc::new(knowledgehub::DenyAll)
            ),
            t.clone()
        )
        .await,
        Err(Error::Denied { .. })
    ));
    let (head, state) = h.snapshot(fixture::context()).await.unwrap();
    let content = knowledgehub::Content::capture(b"Replacement prose: decrease the rent".to_vec());
    let mut document = state
        .get(&rent::reference("story-document"))
        .unwrap()
        .clone();
    document.predecessor = Some(document.reference.clone());
    document.reference.revision = "2".into();
    let RecordData::SourceRevision(d) = &mut document.data else {
        unreachable!()
    };
    d.original = content.reference.clone();
    document.seal().unwrap();
    let mut anchor = state.get(&rent::reference("story-anchor")).unwrap().clone();
    anchor.predecessor = Some(anchor.reference.clone());
    anchor.reference.revision = "2".into();
    let RecordData::Anchor(a) = &mut anchor.data else {
        unreachable!()
    };
    a.source = document.reference.clone();
    a.locator = Locator::ByteRange {
        start: 0,
        end: content.bytes.len() as u64,
        quote: String::from_utf8(content.bytes.clone()).unwrap(),
    };
    anchor.seal().unwrap();
    let mut story = state
        .get(&specimens::reference("UserStory"))
        .unwrap()
        .clone();
    story.predecessor = Some(story.reference.clone());
    story.reference.revision = "source-successor".into();
    specimens::put_property(
        &mut story,
        "need",
        Value::Narrative {
            document: document.reference.clone(),
            fragment: anchor.reference.clone(),
        },
    );
    let mut c = Change::at(&state, head, "source-and-anchor-successors");
    c.operations = vec![
        Operation::Capture {
            record: document,
            content,
        },
        Operation::Put(anchor),
    ];
    c.operations
        .extend(specimens::successor_operations(&state, vec![story]).unwrap());
    h.commit(fixture::context(), c).await.unwrap();
    let mut current = t.clone();
    current.selection = Selection::Snapshot(h.head(fixture::context()).await.unwrap());
    current.budgets.preparation.allow_rebuild = false;
    assert!(
        h.search_text(fixture::context(), current)
            .await
            .unwrap()
            .hits
            .is_empty()
    );
    assert_eq!(
        h.search_text(fixture::context(), t).await.unwrap().hits,
        p.hits
    );
}
