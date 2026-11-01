use super::*;
use crate::{
    Action, Context, DenyAll, OperationOutcome, neighbour_test_support as f, test_support::Sandbox,
};
use std::sync::Arc;

#[tokio::test]
async fn cache_installation_faults_never_replace_the_committed_outcome() {
    let s = Sandbox::new();
    let (b, h) = f::create(&s.store).await.unwrap();
    let head = h.head(f::context()).await.unwrap();
    h.neighbours(f::context(), f::query(Selection::Snapshot(head), "b"))
        .await
        .unwrap();
    for (i, stage) in [
        "neighbour-nodes-synced",
        "neighbour-manifest-synced",
        "neighbour-cache-installed",
    ]
    .into_iter()
    .enumerate()
    {
        b.set_hook(move |p| {
            if p == stage {
                Err(storage("injected derived-cache install failure"))
            } else {
                Ok(())
            }
        });
        let before = b.file_work().neighbour_cache_failures;
        let (change, receipt) = f::commit(
            &h,
            &format!("cache-fault:{i}"),
            vec![ontology::Operation::Put(f::edge(
                &format!("edge:{i}"),
                "a",
                "b",
            ))],
        )
        .await
        .unwrap();
        b.set_hook(|_| Ok(()));
        assert_eq!(b.file_work().neighbour_cache_failures, before + 1);
        assert_eq!(
            h.operation_result(f::context(), format!("cache-fault:{i}"))
                .await
                .unwrap(),
            OperationOutcome::Committed(Box::new(receipt.clone()))
        );
        assert_eq!(h.commit(f::context(), change).await.unwrap(), receipt);
        let mut q = f::query(Selection::Snapshot(receipt.head), "b");
        q.preparation.allow_rebuild = false;
        if stage != "neighbour-cache-installed" {
            assert!(matches!(
                h.neighbours(f::context(), q.clone()).await,
                Err(Error::IndexNotReady { .. })
            ));
        }
        q.preparation.allow_rebuild = true;
        let page = h.neighbours(f::context(), q).await.unwrap();
        assert!(page.exhausted);
        assert_eq!(
            page.neighbours
                .iter()
                .filter(|n| n.category == NeighbourCategory::Association)
                .count(),
            i + 1
        );
    }
}

#[tokio::test]
async fn corrupt_derived_data_rebuilds_but_authoritative_corruption_is_integrity() {
    let s = Sandbox::new();
    let (b, h) = f::create(&s.store).await.unwrap();
    let head = h.head(f::context()).await.unwrap();
    let q = f::query(Selection::Snapshot(head), "b");
    let expected = h
        .neighbours(f::context(), q.clone())
        .await
        .unwrap()
        .neighbours;
    let c = cache(&b.store);
    let manifest_path = fs::read_dir(c.root.join("views"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let original = fs::read(&manifest_path).unwrap();
    let mut bad: Json = parse(&original).unwrap();
    bad["binding"]["scope"]["branch"] = "wrong-view".into();
    fs::write(&manifest_path, json(&bad).unwrap()).unwrap();
    let mut only_ready = q.clone();
    only_ready.preparation.allow_rebuild = false;
    assert!(matches!(
        h.neighbours(f::context(), only_ready.clone()).await,
        Err(Error::IndexNotReady { .. })
    ));
    let page = h.neighbours(f::context(), q.clone()).await.unwrap();
    assert!(page.work.index_rebuilt);
    assert_eq!(page.neighbours, expected);
    let objects = Objects::new(&b.store);
    let (id, g) = generation::root(&b.store, &f::scope(), &objects).unwrap();
    let view = generation::view(&objects, &id, &g, &q.selection).unwrap();
    let m = manifest(&c, &Binding::view(&view, q.fact_scope)).unwrap();
    let index = Objects::new(&c);
    fs::remove_file(index.path(&m.incoming).unwrap()).unwrap();
    let page = h.neighbours(f::context(), q.clone()).await.unwrap();
    assert!(page.work.index_rebuilt);
    assert_eq!(page.neighbours, expected);
    let table = Tree { objects: &objects };
    let oid = table
        .get(
            &g.revisions_root,
            "revisions/1",
            &ref_key(&f::reference("a")),
        )
        .unwrap()
        .unwrap();
    let record: Json = objects.get_json(oid.as_str().unwrap(), "record").unwrap();
    let data = objects.path(record["data_oid"].as_str().unwrap()).unwrap();
    let bytes = fs::read(&data).unwrap();
    fs::remove_file(&data).unwrap();
    assert!(matches!(
        h.neighbours(f::context(), q.clone()).await,
        Err(Error::Integrity(_))
    ));
    assert!(matches!(
        h.neighbours(f::context(), only_ready).await,
        Err(Error::Integrity(_))
    ));
    fs::write(data, bytes).unwrap();
    assert_eq!(
        h.neighbours(f::context(), q).await.unwrap().neighbours,
        expected
    );
}

#[tokio::test]
async fn ready_lookup_ignores_unrelated_artifacts_and_reports_no_state_materialisation() {
    let s = Sandbox::new();
    let (b, h) = f::create(&s.store).await.unwrap();
    let head = h.head(f::context()).await.unwrap();
    h.neighbours(f::context(), f::query(Selection::Snapshot(head), "b"))
        .await
        .unwrap();
    let (_, receipt) = f::commit(
        &h,
        "unrelated-binary",
        f::binary("unrelated", vec![7; 3_000_000]),
    )
    .await
    .unwrap();
    let objects = Objects::new(&b.store);
    let oid = super::super::objects::oid("blob", &vec![7; 1_048_576]).unwrap();
    let path = objects.path(&oid).unwrap();
    let bytes = fs::read(&path).unwrap();
    fs::remove_file(&path).unwrap();
    let before = b.file_work();
    let page = h
        .neighbours(
            f::context(),
            f::query(Selection::Snapshot(receipt.head), "b"),
        )
        .await
        .unwrap();
    let after = b.file_work();
    assert!(!page.work.index_rebuilt);
    assert!(page.exhausted);
    assert_eq!(before.state_materialisations, after.state_materialisations);
    assert!(page.work.record_reads < 10);
    assert!(after.encoded_bytes_read - before.encoded_bytes_read < 100_000);
    fs::write(path, bytes).unwrap();
}

#[test]
fn readers_coordinate_cache_eviction_on_both_runtime_configurations() {
    for multi in [false, true] {
        let mut builder = if multi {
            tokio::runtime::Builder::new_multi_thread()
        } else {
            tokio::runtime::Builder::new_current_thread()
        };
        builder.worker_threads(2).build().unwrap().block_on(async {
            let s = Sandbox::new();
            let (b, h) = f::create(&s.store).await.unwrap();
            let head = h.head(f::context()).await.unwrap();
            let q = f::query(Selection::Snapshot(head), "b");
            let expected = h
                .neighbours(f::context(), q.clone())
                .await
                .unwrap()
                .neighbours;
            let (started, rx) = std::sync::mpsc::channel();
            let (release, wait) = std::sync::mpsc::channel();
            let wait = std::sync::Mutex::new(wait);
            let once = std::sync::atomic::AtomicBool::new(false);
            b.set_hook(move |p| {
                if p == "read-object" && !once.swap(true, Ordering::SeqCst) {
                    started.send(()).unwrap();
                    wait.lock().unwrap().recv().unwrap();
                }
                Ok(())
            });
            let reader = h.clone();
            let request = q.clone();
            let task = tokio::spawn(async move { reader.neighbours(f::context(), request).await });
            while rx.try_recv().is_err() {
                tokio::task::yield_now().await;
            }
            assert!(matches!(
                f::backend(&s.store)
                    .clear_neighbour_cache(f::context(), f::scope())
                    .await,
                Err(Error::Busy)
            ));
            release.send(()).unwrap();
            assert_eq!(task.await.unwrap().unwrap().neighbours, expected);
            b.set_hook(|_| Ok(()));
            b.clear_neighbour_cache(f::context(), f::scope())
                .await
                .unwrap();
            assert_eq!(
                h.neighbours(f::context(), q).await.unwrap().neighbours,
                expected
            );
            h.drain().await;
        });
    }
}

#[tokio::test]
async fn finite_cursor_pool_full_intent_and_idle_expiry_are_shared_with_query() {
    let s = Sandbox::new();
    let (_, h) = f::create(&s.store).await.unwrap();
    let head = h.head(f::context()).await.unwrap();
    let mut q = f::query(Selection::Snapshot(head.clone()), "b");
    q.page_size = 1;
    let token = h
        .neighbours(f::context(), q.clone())
        .await
        .unwrap()
        .continuation
        .unwrap();
    let denied = Context::new("fixture:neighbour-author", Arc::new(DenyAll));
    assert!(matches!(
        h.continue_neighbours(denied, q.clone(), token.clone())
            .await,
        Err(Error::Denied {
            action: Action::Read
        })
    ));
    let mut other = f::context();
    other.actor = "fixture:other".into();
    assert!(matches!(
        h.continue_neighbours(other, q.clone(), token.clone()).await,
        Err(Error::InvalidCursor)
    ));
    for changed in 0..5 {
        let mut request = q.clone();
        match changed {
            0 => request.fact_scope = FactScope::Closure,
            1 => request.projection = NeighbourProjection::Records,
            2 => request.categories.clear(),
            3 => request.direction = NeighbourDirection::Outgoing,
            _ => request.byte_limit += 1,
        };
        assert!(matches!(
            h.continue_neighbours(f::context(), request, token.clone())
                .await,
            Err(Error::InvalidCursor)
        ));
    }
    h.cursors.offset.store(1799, Ordering::SeqCst);
    let next = h
        .continue_neighbours(f::context(), q.clone(), token)
        .await
        .unwrap()
        .continuation
        .unwrap();
    h.cursors.offset.store(3599, Ordering::SeqCst);
    assert!(matches!(
        h.continue_neighbours(f::context(), q.clone(), next).await,
        Err(Error::InvalidCursor)
    ));
    let now = h.cursors.now();
    {
        let mut pool = h.cursors.entries.lock().unwrap();
        for i in 0..64 {
            pool.insert(
                format!("legacy-{i}"),
                crate::ontology::navigation::Cursor {
                    actor: f::context().actor,
                    scope: f::scope(),
                    fingerprint: "fixture:old-query".into(),
                    query: Query::select(Selection::Snapshot(head.clone())),
                    items: vec![],
                    index: 0,
                    truncated: None,
                    touched: now,
                    neighbour: None,
                    impact: None,
                },
            );
        }
    }
    assert!(matches!(
        h.neighbours(f::context(), q).await,
        Err(Error::Capacity)
    ));
    let mut legacy = Query::select(Selection::Snapshot(head));
    legacy.page_size = 1;
    assert!(matches!(
        h.query(f::context(), legacy).await,
        Err(Error::Capacity)
    ));
    for i in 0..64 {
        h.release_cursor(f::context(), format!("legacy-{i}"))
            .await
            .unwrap();
    }
}

#[tokio::test]
async fn preparation_budgets_quota_and_view_eviction_preserve_knowledge() {
    let s = Sandbox::new();
    let (b, h) = f::create(&s.store).await.unwrap();
    let head = h.head(f::context()).await.unwrap();
    let q = f::query(Selection::Snapshot(head), "b");
    for which in 0..3 {
        let mut limited = q.clone();
        match which {
            0 => limited.preparation.owner_limit = 1,
            1 => limited.preparation.fact_limit = 1,
            _ => limited.preparation.byte_limit = 100,
        };
        assert!(matches!(
            h.neighbours(f::context(), limited).await,
            Err(Error::IndexNotReady { .. } | Error::Capacity)
        ));
    }
    let expected = h
        .neighbours(f::context(), q.clone())
        .await
        .unwrap()
        .neighbours;
    let mut c = cache(&b.store);
    c.loose_budget = Some(AtomicU64::new(0));
    assert!(matches!(
        Objects::new(&c).put("blob", b"unique quota fixture"),
        Err(Error::Capacity)
    ));
    let advertised = fs::read_dir(c.root.join("views"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let bytes = fs::read(advertised).unwrap();
    for i in 0..32 {
        fs::write(
            c.root.join("views").join(format!("quota-view-{i}.json")),
            &bytes,
        )
        .unwrap();
    }
    let (_, receipt) = f::commit(
        &h,
        "evict-after-commit",
        vec![ontology::Operation::Put(f::edge("eviction:edge", "a", "b"))],
    )
    .await
    .unwrap();
    assert_eq!(
        h.operation_result(f::context(), "evict-after-commit".into())
            .await
            .unwrap(),
        OperationOutcome::Committed(Box::new(receipt))
    );
    let rebuilt = h.neighbours(f::context(), q).await.unwrap();
    assert!(rebuilt.work.index_rebuilt);
    assert_eq!(rebuilt.neighbours, expected);
}
