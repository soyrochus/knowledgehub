use super::*;
use crate::{ontology::navigation::Cursor, seed, test_support::Sandbox};
use std::sync::atomic::Ordering;

async fn fixture() -> (Sandbox, FileBackend, KnowledgeHub, Receipt) {
    let s = Sandbox::new();
    let b = s.backend();
    let h = KnowledgeHub::create(b.clone(), seed::context(), seed::scope())
        .await
        .unwrap();
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut q = Change::at(&state, head, "history:catalogue");
    q.operations = builtin_catalogue().unwrap().operations();
    let r = h.commit(seed::context(), q).await.unwrap();
    (s, b, h, r)
}
#[tokio::test]
async fn shared_cursor_capacity_idle_expiry_cross_scope_and_empty_affected() {
    let (_s, _b, h, r) = fixture().await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut c = Change::at(&state, head, "history:empty");
    c.operations = vec![Operation::Gaps(state.gaps.clone())];
    let empty = h.commit(seed::context(), c).await.unwrap();
    assert!(empty.affected.is_empty());
    let p = h
        .publication_affected(
            seed::context(),
            PublicationAffected::new(empty.head.clone(), &empty.operation_id),
        )
        .await
        .unwrap();
    assert!(p.complete);
    assert!(matches!(
        p.items,
        HistoryItems::Affected {
            published: true,
            total: 0,
            ..
        }
    ));
    let mut q = Publications::new(empty.head.clone());
    q.limits.items = 1;
    let first = h.publications(seed::context(), q.clone()).await.unwrap();
    let token = first.continuation.unwrap();
    let mut other = h.clone();
    other.scope.branch = "other".into();
    assert_eq!(
        other
            .continue_history(
                seed::context(),
                HistoryRequest::Publications(q.clone()),
                token.clone()
            )
            .await,
        Err(Error::InvalidCursor)
    );
    let mut query = Query::select(Selection::Snapshot(empty.head));
    query.page_size = 1;
    let query_token = h
        .query(seed::context(), query.clone())
        .await
        .unwrap()
        .continuation
        .unwrap();
    assert_eq!(
        h.release_history(seed::context(), query_token.clone())
            .await,
        Err(Error::InvalidCursor)
    );
    {
        let mut pool = h.cursors.entries.lock().unwrap();
        for i in 2..64 {
            pool.insert(
                format!("ordinary-{i}"),
                Cursor {
                    actor: "host:maintainer".into(),
                    scope: h.scope.clone(),
                    fingerprint: String::new(),
                    query: query.clone(),
                    items: vec![],
                    index: 0,
                    truncated: None,
                    touched: h.cursors.now(),
                    neighbour: None,
                    impact: None,
                    history: None,
                },
            );
        }
    }
    assert_eq!(
        h.publications(seed::context(), q.clone()).await,
        Err(Error::Capacity)
    );
    assert_eq!(h.cursors.entries.lock().unwrap().len(), 64);
    h.cursors.offset.store(1799, Ordering::SeqCst);
    let mut changed = q.clone();
    changed.limits.nodes += 1;
    assert_eq!(
        h.continue_history(
            seed::context(),
            HistoryRequest::Publications(changed),
            token.clone()
        )
        .await,
        Err(Error::InvalidCursor)
    );
    h.cursors.offset.store(1801, Ordering::SeqCst);
    assert_eq!(
        h.continue_history(
            seed::context(),
            HistoryRequest::Publications(q.clone()),
            token.clone()
        )
        .await,
        Err(Error::InvalidCursor)
    );
    assert_eq!(
        h.release_history(seed::context(), token).await,
        Err(Error::InvalidCursor)
    );
    assert!(h.cursors.entries.lock().unwrap().is_empty());
    let p = h.publications(seed::context(), q).await.unwrap();
    assert!(p.continuation.is_some());
    assert_eq!(
        h.publication(
            seed::context(),
            Publication::new(r.previous_head, "history:empty")
        )
        .await
        .unwrap()
        .outcome,
        PublicationLookup::NotPublishedAt
    );
    h.drain().await;
}

#[tokio::test]
async fn accepted_anchors_exclude_staged_operations_and_foreign_snapshots() {
    let (s, b, h, receipt) = fixture().await;
    let o = Objects::new(&b.store);
    let (_, mut orphan) = generation::root(&b.store, &seed::scope(), &o).unwrap();
    let mut staged = receipt.clone();
    staged.operation_id = "staged-only".into();
    staged.head = b.store.allocate_id().unwrap();
    orphan.snapshot_id = staged.head.clone();
    let oid = projection::encode_receipt(&o, &staged).unwrap();
    orphan.receipt = Some(oid.clone());
    orphan.operations_root = (Tree { objects: &o })
        .update(
            &orphan.operations_root,
            "operations/1",
            value!(&staged.operation_id),
            Some(value!(oid)),
        )
        .unwrap();
    o.put_json("generation", &orphan).unwrap();
    fs::write(
        s.store.join("staging/unaccepted-receipt.json"),
        json(&staged).unwrap(),
    )
    .unwrap();
    assert_eq!(
        h.publication(
            seed::context(),
            Publication::new(receipt.head.clone(), &staged.operation_id)
        )
        .await
        .unwrap()
        .outcome,
        PublicationLookup::NotPublishedAt
    );
    let page = h
        .publications(seed::context(), Publications::new(receipt.head.clone()))
        .await
        .unwrap();
    let HistoryItems::Publications(headers) = page.items else {
        panic!()
    };
    assert_eq!(headers.len(), 1);
    assert_eq!(headers[0].operation_id, receipt.operation_id);
    assert_eq!(
        h.publication(
            seed::context(),
            Publication::new(staged.head.clone(), &staged.operation_id)
        )
        .await,
        Err(Error::HistoryAnchorUnavailable {
            anchor: staged.head
        })
    );
    let foreign = Sandbox::new();
    let mut scope = seed::scope();
    scope.model = "model:other-scope".into();
    let other = KnowledgeHub::create(foreign.backend(), seed::context(), scope)
        .await
        .unwrap();
    let foreign_anchor = other.head(seed::context()).await.unwrap();
    assert_eq!(
        h.publications(seed::context(), Publications::new(foreign_anchor.clone()))
            .await,
        Err(Error::HistoryAnchorUnavailable {
            anchor: foreign_anchor
        })
    );
    other.drain().await;
    h.drain().await;
}

#[tokio::test]
async fn complete_responses_fit_their_exact_encoded_budget() {
    let (_s, _b, h, receipt) = fixture().await;
    let mut q = Publications::new(receipt.head.clone());
    let expected = h.publications(seed::context(), q.clone()).await.unwrap();
    assert!(expected.complete);
    q.limits.output_bytes = expected.work.output_bytes;
    assert_eq!(
        h.publications(seed::context(), q.clone()).await.unwrap(),
        expected
    );
    q.limits.output_bytes -= 1;
    assert!(matches!(
        h.publications(seed::context(), q).await,
        Err(Error::HistoryLimit {
            resource: HistoryResource::OutputBytes,
            ..
        })
    ));
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut c = Change::at(&state, head, "one-reference");
    c.operations.push(Operation::Put(
        Record::new(
            ExactRef::new("history", "one", "1"),
            type_ref("SourceArtifact"),
            RecordData::SourceArtifact(SourceArtifactData {
                origin: "synthetic history source".into(),
                media_type: "text/plain".into(),
                owner: "host:maintainer".into(),
                status: "hypothesis".into(),
            }),
            vec![],
        )
        .unwrap(),
    ));
    let r = h.commit(seed::context(), c).await.unwrap();
    let mut q = PublicationAffected::new(r.head.clone(), &r.operation_id);
    let expected = h
        .publication_affected(seed::context(), q.clone())
        .await
        .unwrap();
    assert!(expected.complete);
    q.limits.output_bytes = expected.work.output_bytes;
    assert_eq!(
        h.publication_affected(seed::context(), q.clone())
            .await
            .unwrap(),
        expected
    );
    q.limits.output_bytes -= 1;
    assert!(matches!(
        h.publication_affected(seed::context(), q).await,
        Err(Error::HistoryLimit {
            resource: HistoryResource::OutputBytes,
            ..
        })
    ));
    h.drain().await;
}

#[test]
fn header_count_is_selective_and_affected_ranges_preserve_repetitions() {
    let (_s, b) = super::super::object_backend("history-selective");
    let o = Objects::new(&b.store);
    let mut receipt = Receipt {
        scope: seed::scope(),
        operation_id: "repeated".into(),
        actor: "host:original".into(),
        fingerprint_version: ontology::REQUEST_FORMAT.into(),
        fingerprint: "a".repeat(64),
        previous_head: SnapshotId("b".repeat(64)),
        head: SnapshotId("c".repeat(64)),
        sequence: 1,
        affected: (0..400)
            .map(|i| ExactRef::new("repeat", format!("reference:{}", i % 7), "1"))
            .collect(),
    };
    let oid = projection::encode_receipt(&o, &receipt).unwrap();
    let cold = Objects::new(&b.store);
    let before = b.file_work();
    let (header, root) = projection::receipt_header(&cold, &oid).unwrap();
    assert_eq!(header.affected_count, 400);
    assert_eq!(b.file_work().object_reads - before.object_reads, 2);
    let mut references = vec![];
    for offset in (0..400).step_by(61) {
        references.extend(
            (Tree { objects: &cold })
                .seq_range(&root, "exact-refs/1", offset, 61.min(400 - offset))
                .unwrap()
                .into_iter()
                .map(|v| serde_json::from_value::<ExactRef>(v).unwrap()),
        );
    }
    assert!(
        (Tree { objects: &cold })
            .seq_range(&root, "exact-refs/1", 400, 0)
            .unwrap()
            .is_empty()
    );
    assert_eq!(header.with_affected(references).unwrap(), receipt);
    receipt.affected.clear();
    let oid = projection::encode_receipt(&o, &receipt).unwrap();
    assert_eq!(
        projection::receipt_header(&cold, &oid)
            .unwrap()
            .0
            .affected_count,
        0
    );
}

#[tokio::test]
async fn inconsistent_receipt_and_ancestry_bindings_never_report_absence() {
    let (_s, b, h, r) = fixture().await;
    let o = Objects::new(&b.store);
    let (_, g) = generation::root(&b.store, &seed::scope(), &o).unwrap();
    let reference_path = generation::head_path(&b.store, &seed::scope()).unwrap();
    let original_ref = fs::read(&reference_path).unwrap();
    let original: serde_json::Value = o.get_json(g.receipt.as_ref().unwrap(), "receipt").unwrap();
    for (field, bad) in [
        (
            "scope",
            value!(ModelScope {
                branch: "wrong".into(),
                ..seed::scope()
            }),
        ),
        ("operation_id", value!("wrong")),
        ("fingerprint", value!("bad")),
        ("fingerprint_version", value!("old/2")),
        ("sequence", value!(99)),
        ("head", value!(r.previous_head)),
        ("previous_head", value!(r.head)),
        ("actor", value!("")),
    ] {
        let mut changed = original.clone();
        changed["fields"][field] = bad;
        let oid = o.put_json("receipt", &changed).unwrap();
        let mut candidate = g.clone();
        candidate.receipt = Some(oid.clone());
        candidate.operations_root = (Tree { objects: &o })
            .update(
                &g.operations_root,
                "operations/1",
                value!(&r.operation_id),
                Some(value!(oid)),
            )
            .unwrap();
        let generation = o.put_json("generation", &candidate).unwrap();
        let mut accepted: serde_json::Value = parse(&original_ref).unwrap();
        accepted["generation"] = value!(generation);
        fs::write(&reference_path, json(&accepted).unwrap()).unwrap();
        assert!(
            matches!(
                h.publication(
                    seed::context(),
                    Publication::new(r.head.clone(), &r.operation_id)
                )
                .await,
                Err(Error::Integrity(_))
            ),
            "{field}"
        );
        assert!(
            h.publications(seed::context(), Publications::new(r.head.clone()))
                .await
                .is_err(),
            "{field}"
        );
        fs::write(&reference_path, &original_ref).unwrap();
    }
    let mut candidate = g.clone();
    candidate.ancestors_root = (Tree { objects: &o })
        .update(
            &g.ancestors_root,
            "ancestors/1",
            value!(&r.previous_head.0),
            Some(value!({"generation":g.parent,"sequence":"99"})),
        )
        .unwrap();
    let generation = o.put_json("generation", &candidate).unwrap();
    let mut accepted: serde_json::Value = parse(&original_ref).unwrap();
    accepted["generation"] = value!(generation);
    fs::write(&reference_path, json(&accepted).unwrap()).unwrap();
    assert!(matches!(
        h.publication(
            seed::context(),
            Publication::new(r.previous_head.clone(), "never")
        )
        .await,
        Err(Error::Integrity(_))
    ));
    fs::write(&reference_path, &original_ref).unwrap();
    let path = o.path(g.receipt.as_ref().unwrap()).unwrap();
    let saved = fs::read(&path).unwrap();
    fs::remove_file(&path).unwrap();
    assert!(
        h.publication(
            seed::context(),
            Publication::new(r.head.clone(), &r.operation_id)
        )
        .await
        .is_err()
    );
    fs::write(path, saved).unwrap();
    b.set_hook(|stage| {
        if stage == "read-object" {
            Err(Error::Storage {
                stage: "injected unavailable history".into(),
            })
        } else {
            Ok(())
        }
    });
    assert!(matches!(
        h.publication(seed::context(), Publication::new(r.head.clone(), "never"))
            .await,
        Err(Error::Storage { .. })
    ));
    b.set_hook(|stage| {
        if stage == "read-object" {
            Err(Error::Uncertain {
                operation_id: None,
                stage: "interrupted history".into(),
            })
        } else {
            Ok(())
        }
    });
    assert!(matches!(
        h.publications(seed::context(), Publications::new(r.head.clone()))
            .await,
        Err(Error::Uncertain { .. })
    ));
    b.set_hook(|_| Ok(()));
    assert_eq!(h.head(seed::context()).await.unwrap(), r.head);
    assert_eq!(
        h.operation_result(seed::context(), r.operation_id.clone())
            .await
            .unwrap(),
        OperationOutcome::Committed(Box::new(r))
    );
    h.drain().await;
}

#[tokio::test]
async fn loose_and_cold_packed_history_work_is_measured_and_never_rebuilds() {
    let (s, b, h, r) = fixture().await;
    // Many zero-affected accepted generations isolate history cost from model/artifact size.
    let (mut head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut receipts = vec![r];
    let mut rejected = Change::at(&state, head.clone(), "rejected");
    rejected.operations = vec![Operation::Put(
        Record::new(
            ExactRef::new("x", "invalid", "1"),
            ExactRef::new("x", "absent-type", "1"),
            RecordData::Element(ElementData::new(BTreeMap::new())),
            vec![],
        )
        .unwrap(),
    )];
    assert!(h.commit(seed::context(), rejected).await.is_err());
    for i in 0..24 {
        let mut c = Change::at(&state, head, format!("empty:{i}"));
        c.operations = vec![Operation::Gaps(state.gaps.clone())];
        let r = h.commit(seed::context(), c).await.unwrap();
        head = r.head.clone();
        receipts.push(r);
    }
    for packed in [false, true] {
        if packed {
            b.compact(seed::context(), seed::scope()).await.unwrap();
        }
        let cold = s.backend();
        let h = KnowledgeHub::open(cold.clone(), seed::context(), seed::scope())
            .await
            .unwrap();
        // Opening is measured separately. Force request-time cold pack discovery.
        *cold.store.work.locations.lock().unwrap() = None;
        let mut q = Publications::new(head.clone());
        q.limits.items = 2;
        let first_before = cold.file_work();
        let mut page = h.publications(seed::context(), q.clone()).await.unwrap();
        let first_after = cold.file_work();
        assert!(
            first_after.encoded_bytes_read - first_before.encoded_bytes_read
                + first_after.index_bytes_read
                - first_before.index_bytes_read
                <= page.work.encoded_bytes_read
        );
        if packed {
            assert!(first_after.index_bytes_read > first_before.index_bytes_read);
        }
        let mut all = vec![];
        loop {
            assert!(page.work.visited_nodes <= q.limits.nodes);
            assert!(page.work.storage_bytes <= q.limits.storage_bytes);
            assert_eq!(
                page.work.output_bytes,
                serde_json::to_vec(&page).unwrap().len() as u64
            );
            let HistoryItems::Publications(headers) = page.items else {
                panic!()
            };
            assert!(headers.len() <= 2);
            all.extend(headers);
            if let Some(t) = page.continuation {
                let before = cold.file_work();
                page = h
                    .continue_history(seed::context(), HistoryRequest::Publications(q.clone()), t)
                    .await
                    .unwrap();
                let after = cold.file_work();
                assert!(after.object_reads - before.object_reads <= page.work.visited_nodes);
                assert!(
                    after.encoded_bytes_read - before.encoded_bytes_read + after.index_bytes_read
                        - before.index_bytes_read
                        <= page.work.encoded_bytes_read
                );
                assert_eq!(before.state_materialisations, after.state_materialisations);
                assert_eq!(before.index_rebuilds, after.index_rebuilds);
            } else {
                assert!(page.complete);
                break;
            }
        }
        assert_eq!(
            all.iter().map(|h| &h.operation_id).collect::<Vec<_>>(),
            receipts
                .iter()
                .rev()
                .map(|r| &r.operation_id)
                .collect::<Vec<_>>()
        );
        h.drain().await;
    }
    h.drain().await;
    // Missing derived offsets require explicit maintenance, never a hidden full-pack scan.
    for e in fs::read_dir(s.store.join("objects/packs")).unwrap() {
        let p = e.unwrap().path();
        if p.extension().is_some_and(|v| v == "idx") {
            fs::remove_file(p).unwrap();
        }
    }
    let cold = s.backend();
    let error = cold
        .store
        .publication(&seed::scope(), &Publication::new(head, "history:catalogue"))
        .unwrap_err();
    assert!(matches!(error, Error::IndexNotReady { .. }));
    assert_eq!(cold.file_work().index_rebuilds, 0);
}
