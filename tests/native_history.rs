#[path = "../examples/history_support/mod.rs"]
mod fixture;
#[path = "../examples/support/mod.rs"]
mod seed;
mod support;
use knowledgehub::*;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use support::Sandbox;

fn output(page: &HistoryPage, limits: &HistoryLimits) {
    assert_eq!(
        serde_json::to_vec(page).unwrap().len() as u64,
        page.work.output_bytes
    );
    assert!(page.work.visited_nodes <= limits.nodes);
    assert!(page.work.storage_bytes <= limits.storage_bytes);
    assert!(page.work.output_bytes <= limits.output_bytes);
    assert_eq!(
        page.work.storage_bytes,
        page.work.encoded_bytes_read + page.work.decoded_bytes
    );
}
#[tokio::test]
async fn original_lookup_discovery_sources_baselines_and_recovery_survive_reopen_and_packing() {
    for domain in ["library-loans", "garden-irrigation"] {
        let s = Sandbox::new();
        let backend = s.backend();
        let h = KnowledgeHub::create(backend.clone(), seed::context(), seed::scope())
            .await
            .unwrap();
        let f = fixture::populate(&h, seed::context(), domain, 4)
            .await
            .unwrap();
        let b = fixture::reader(h.scope());
        let anchor = f.receipts.last().unwrap().head.clone();
        let mut q = Publications::new(anchor.clone());
        q.limits.items = 2;
        let before = backend.file_work();
        let found = fixture::headers(&h, b.clone(), q.clone()).await.unwrap();
        assert_eq!(
            found.iter().map(|r| &r.operation_id).collect::<Vec<_>>(),
            f.receipts
                .iter()
                .rev()
                .map(|r| &r.operation_id)
                .collect::<Vec<_>>()
        );
        for r in &f.receipts {
            let result = h
                .publication(b.clone(), Publication::new(anchor.clone(), &r.operation_id))
                .await
                .unwrap();
            assert_eq!(
                serde_json::to_vec(&result).unwrap().len() as u64,
                result.work.output_bytes
            );
            let PublicationLookup::Published(header) = result.outcome else {
                panic!()
            };
            let mut refs = PublicationAffected::new(anchor.clone(), &r.operation_id);
            refs.limits.items = if r.operation_id == "history:catalogue" {
                61
            } else {
                3
            };
            assert_eq!(
                header
                    .with_affected(fixture::affected(&h, b.clone(), refs).await.unwrap())
                    .unwrap(),
                *r
            );
        }
        let after = backend.file_work();
        assert_eq!(after.state_materialisations, before.state_materialisations);
        assert_eq!(after.objects_written, before.objects_written);
        fixture::meaning(&h, b.clone(), &f).await.unwrap();
        assert_eq!(
            h.operation_result(
                Context::new(b.actor.clone(), Arc::new(seed::Allow)),
                f.retry.operation_id.clone()
            )
            .await,
            Err(Error::OperationReuse)
        );
        assert_eq!(
            h.commit(
                Context::new(b.actor.clone(), Arc::new(seed::Allow)),
                f.retry.clone()
            )
            .await,
            Err(Error::OperationReuse)
        );
        assert_eq!(
            h.commit(seed::context(), f.retry.clone()).await.unwrap(),
            f.receipts[1]
        );
        h.drain().await;
        // Open and compact are measured separately; a brand new backend makes pack index reads cold.
        let maintenance = s.backend();
        let reopened = KnowledgeHub::open(maintenance.clone(), b.clone(), seed::scope())
            .await
            .unwrap();
        assert_eq!(
            fixture::headers(&reopened, b.clone(), q.clone())
                .await
                .unwrap(),
            found
        );
        maintenance
            .compact(seed::context(), seed::scope())
            .await
            .unwrap();
        assert_eq!(
            fixture::headers(&reopened, b.clone(), q.clone())
                .await
                .unwrap(),
            found
        );
        reopened.drain().await;
        let cold = s.backend();
        let h = KnowledgeHub::open(cold.clone(), b.clone(), seed::scope())
            .await
            .unwrap();
        let before = cold.file_work();
        assert_eq!(fixture::headers(&h, b.clone(), q).await.unwrap(), found);
        for r in &f.receipts {
            let PublicationLookup::Published(header) = h
                .publication(b.clone(), Publication::new(anchor.clone(), &r.operation_id))
                .await
                .unwrap()
                .outcome
            else {
                panic!()
            };
            let mut refs = PublicationAffected::new(anchor.clone(), &r.operation_id);
            refs.limits.items = if r.operation_id == "history:catalogue" {
                63
            } else {
                5
            };
            assert_eq!(
                header
                    .with_affected(fixture::affected(&h, b.clone(), refs).await.unwrap())
                    .unwrap(),
                *r
            );
        }
        assert_eq!(
            cold.file_work().state_materialisations,
            before.state_materialisations
        );
        fixture::meaning(&h, b, &f).await.unwrap();
        assert_eq!(h.head(seed::context()).await.unwrap(), anchor);
        assert_eq!(
            h.commit(seed::context(), f.retry).await.unwrap(),
            f.receipts[1]
        );
        cold.scrub(seed::context(), seed::scope()).await.unwrap();
        h.drain().await;
    }
}

#[tokio::test]
async fn pinned_append_restart_and_anchor_relative_absence_are_explicit() {
    let s = Sandbox::new();
    let h = s.create().await;
    let initial = h.head(seed::context()).await.unwrap();
    let empty = h
        .publications(seed::context(), Publications::new(initial.clone()))
        .await
        .unwrap();
    assert!(empty.complete);
    assert_eq!(empty.initial_boundary, Some(initial.clone()));
    let f = fixture::populate(&h, seed::context(), "pinned", 1)
        .await
        .unwrap();
    let b = fixture::reader(h.scope());
    let anchor = h.head(b.clone()).await.unwrap();
    let mut q = Publications::new(anchor.clone());
    q.limits.items = 1;
    let first = h.publications(b.clone(), q.clone()).await.unwrap();
    output(&first, &q.limits);
    let token = first.continuation.clone().unwrap();
    let appended = fixture::append(&h, seed::context(), "after-anchor")
        .await
        .unwrap();
    let rest = h
        .continue_history(
            b.clone(),
            HistoryRequest::Publications(q.clone()),
            token.clone(),
        )
        .await
        .unwrap();
    let HistoryItems::Publications(items) = rest.items else {
        panic!()
    };
    assert_eq!(items[0].operation_id, f.receipts[2].operation_id);
    assert_eq!(
        h.publication(
            b.clone(),
            Publication::new(anchor.clone(), &appended.operation_id)
        )
        .await
        .unwrap()
        .outcome,
        PublicationLookup::NotPublishedAt
    );
    assert_eq!(
        h.publication(b.clone(), Publication::new(initial, "history:catalogue"))
            .await
            .unwrap()
            .outcome,
        PublicationLookup::NotPublishedAt
    );
    assert_eq!(
        h.publication(
            b.clone(),
            Publication::new(anchor.clone(), "never-published")
        )
        .await
        .unwrap()
        .outcome,
        PublicationLookup::NotPublishedAt
    );
    let missing = h
        .publication_affected(
            b.clone(),
            PublicationAffected::new(anchor.clone(), "never-published"),
        )
        .await
        .unwrap();
    assert!(matches!(
        missing.items,
        HistoryItems::Affected {
            published: false,
            total: 0,
            ..
        }
    ));
    let unknown = SnapshotId("f".repeat(64));
    assert_eq!(
        h.publication(
            b.clone(),
            Publication::new(unknown.clone(), "history:catalogue")
        )
        .await,
        Err(Error::HistoryAnchorUnavailable { anchor: unknown })
    );
    for id in ["", "\n"] {
        assert!(matches!(
            h.publication(b.clone(), Publication::new(anchor.clone(), id))
                .await,
            Err(Error::InvalidInput(_))
        ));
    }
    assert!(matches!(
        h.publication(
            b.clone(),
            Publication::new(SnapshotId("malformed".into()), "x")
        )
        .await,
        Err(Error::InvalidInput(_))
    ));
    h.drain().await;
    let h = s.open().await.unwrap();
    assert_eq!(
        h.continue_history(b.clone(), HistoryRequest::Publications(q.clone()), token)
            .await,
        Err(Error::InvalidCursor)
    );
    assert_eq!(
        fixture::headers(&h, b, q)
            .await
            .unwrap()
            .iter()
            .map(|h| &h.operation_id)
            .collect::<Vec<_>>(),
        f.receipts
            .iter()
            .rev()
            .map(|h| &h.operation_id)
            .collect::<Vec<_>>()
    );
    h.drain().await;
}

struct Switch {
    read: AtomicBool,
    history: AtomicBool,
    failed: AtomicBool,
}
impl Policy for Switch {
    fn allows(&self, _: &str, scope: &ModelScope, action: Action) -> Result<bool, PolicyFailure> {
        assert_eq!(scope, &seed::scope());
        if self.failed.load(Ordering::SeqCst) {
            return Err(PolicyFailure);
        }
        Ok(match action {
            Action::Read => self.read.load(Ordering::SeqCst),
            Action::ReadHistory => self.history.load(Ordering::SeqCst),
            _ => false,
        })
    }
}
#[tokio::test]
async fn authority_cursors_and_all_intent_are_checked_before_disclosure() {
    let s = Sandbox::new();
    let backend = s.backend();
    let h = KnowledgeHub::create(backend.clone(), seed::context(), seed::scope())
        .await
        .unwrap();
    let f = fixture::populate(&h, seed::context(), "authority", 0)
        .await
        .unwrap();
    let anchor = f.receipts.last().unwrap().head.clone();
    let policy = Arc::new(Switch {
        read: AtomicBool::new(true),
        history: AtomicBool::new(false),
        failed: AtomicBool::new(false),
    });
    let b = Context::new("host:reader", policy.clone());
    let mut q = Publications::new(anchor.clone());
    q.limits.items = 1;
    for c in [Context::new("host:denied", Arc::new(DenyAll)), b.clone()] {
        let work = backend.file_work();
        assert!(matches!(
            h.publication(
                c.clone(),
                Publication::new(anchor.clone(), "history:catalogue")
            )
            .await,
            Err(Error::Denied { .. })
        ));
        assert!(matches!(
            h.publications(c.clone(), q.clone()).await,
            Err(Error::Denied { .. })
        ));
        assert!(matches!(
            h.publication_affected(
                c,
                PublicationAffected::new(anchor.clone(), "history:catalogue")
            )
            .await,
            Err(Error::Denied { .. })
        ));
        assert_eq!(backend.file_work(), work);
    }
    policy.history.store(true, Ordering::SeqCst);
    let page = h.publications(b.clone(), q.clone()).await.unwrap();
    let token = page.continuation.unwrap();
    let original = HistoryRequest::Publications(q.clone());
    let mut altered = q.clone();
    altered.limits.items = 2;
    let mut other_anchor = q.clone();
    other_anchor.anchor = f.initial;
    for request in [
        HistoryRequest::Publications(altered),
        HistoryRequest::Publications(other_anchor),
        HistoryRequest::Affected(PublicationAffected::new(
            anchor.clone(),
            "history:catalogue",
        )),
    ] {
        assert_eq!(
            h.continue_history(b.clone(), request, token.clone()).await,
            Err(Error::InvalidCursor)
        );
    }
    assert_eq!(
        h.continue_history(
            Context::new("host:other", policy.clone()),
            original.clone(),
            token.clone()
        )
        .await,
        Err(Error::InvalidCursor)
    );
    assert_eq!(
        h.continue_history(b.clone(), original.clone(), format!("{token}tampered"))
            .await,
        Err(Error::InvalidCursor)
    );
    for fail in [false, true] {
        policy.history.store(false, Ordering::SeqCst);
        policy.failed.store(fail, Ordering::SeqCst);
        let before = backend.file_work();
        let error = h
            .continue_history(b.clone(), original.clone(), token.clone())
            .await
            .unwrap_err();
        assert!(if fail {
            matches!(error, Error::Unavailable(_))
        } else {
            matches!(
                error,
                Error::Denied {
                    action: Action::ReadHistory
                }
            )
        });
        assert!(h.release_history(b.clone(), token.clone()).await.is_err());
        assert!(h.release_cursor(b.clone(), token.clone()).await.is_err());
        assert_eq!(backend.file_work(), before);
    }
    policy.failed.store(false, Ordering::SeqCst);
    policy.history.store(true, Ordering::SeqCst);
    policy.read.store(false, Ordering::SeqCst);
    assert_eq!(
        h.continue_history(b.clone(), original.clone(), token.clone())
            .await,
        Err(Error::Denied {
            action: Action::Read
        })
    );
    policy.read.store(true, Ordering::SeqCst);
    h.release_history(b.clone(), token.clone()).await.unwrap();
    assert_eq!(
        h.release_history(b.clone(), token.clone()).await,
        Err(Error::InvalidCursor)
    );
    assert_eq!(
        h.continue_history(b, original, token).await,
        Err(Error::InvalidCursor)
    );
    h.drain().await;
}

#[tokio::test]
async fn finite_limits_account_for_envelope_and_oversized_values() {
    let s = Sandbox::new();
    let h = s.create().await;
    let f = fixture::populate(&h, seed::context(), "budgets", 6)
        .await
        .unwrap();
    let anchor = f.receipts.last().unwrap().head.clone();
    let b = fixture::reader(h.scope());
    for field in 0..4 {
        for bad in [0, u64::MAX] {
            let mut q = Publication::new(anchor.clone(), "history:original-A");
            match field {
                0 => q.limits.items = bad,
                1 => q.limits.nodes = bad,
                2 => q.limits.storage_bytes = bad,
                _ => q.limits.output_bytes = bad,
            }
            assert!(matches!(
                h.publication(b.clone(), q).await,
                Err(Error::InvalidInput(_))
            ));
        }
    }
    for resource in [
        HistoryResource::Nodes,
        HistoryResource::StorageBytes,
        HistoryResource::OutputBytes,
    ] {
        let mut q = Publication::new(anchor.clone(), "history:original-A");
        match resource {
            HistoryResource::Nodes => q.limits.nodes = 1,
            HistoryResource::StorageBytes => q.limits.storage_bytes = 1,
            _ => q.limits.output_bytes = 1,
        }
        assert!(
            matches!(h.publication(b.clone(), q).await, Err(Error::HistoryLimit{resource:r,..}) if r==resource)
        );
    }
    let mut q = Publications::new(anchor.clone());
    q.limits.output_bytes = 1800;
    let p = h.publications(b.clone(), q.clone()).await.unwrap();
    output(&p, &q.limits);
    assert!(!p.complete);
    let HistoryItems::Publications(v) = p.items else {
        panic!()
    };
    assert!(!v.is_empty());
    assert!(v.len() < f.receipts.len());
    let mut refs = PublicationAffected::new(anchor, "history:catalogue");
    refs.limits.items = 1000;
    let mut p = h
        .publication_affected(b.clone(), refs.clone())
        .await
        .unwrap();
    let mut total = 0;
    loop {
        output(&p, &refs.limits);
        let HistoryItems::Affected {
            references,
            offset,
            total: count,
            ..
        } = &p.items
        else {
            panic!()
        };
        assert_eq!(*offset, total);
        assert!(!references.is_empty());
        total += references.len() as u64;
        assert_eq!(*count, f.receipts[0].affected.len() as u64);
        if let Some(t) = p.continuation {
            p = h
                .continue_history(b.clone(), HistoryRequest::Affected(refs.clone()), t)
                .await
                .unwrap();
        } else {
            assert!(p.complete);
            break;
        }
    }
    assert_eq!(total, f.receipts[0].affected.len() as u64);
    // A single valid original identifier is never shortened to satisfy an output cap.
    let id = format!("history:{}", "long".repeat(2048));
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut c = Change::at(&state, head, &id);
    c.operations = vec![Operation::Gaps(state.gaps.clone())];
    let large = h.commit(seed::context(), c).await.unwrap();
    let mut lookup = Publication::new(large.head.clone(), &id);
    lookup.limits.output_bytes = 1000;
    assert!(matches!(
        h.publication(b.clone(), lookup.clone()).await,
        Err(Error::HistoryLimit {
            resource: HistoryResource::OutputBytes,
            ..
        })
    ));
    lookup.limits.output_bytes = 65536;
    let PublicationLookup::Published(original) = h.publication(b, lookup).await.unwrap().outcome
    else {
        panic!()
    };
    assert_eq!(original.operation_id, id);
    h.drain().await;
}
