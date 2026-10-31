#[path = "../examples/support/mod.rs"]
mod seed;
mod support;
use knowledgehub::{backends::files::*, *};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
use support::Sandbox;

async fn walkthrough() {
    let sandbox = Sandbox::new();
    let hub = sandbox.create().await;
    let c = seed::context();
    let a = hub.head(c.clone()).await.unwrap();
    assert!(
        !hub.read_snapshot(c.clone(), a.clone())
            .await
            .unwrap()
            .seed_complete()
    );
    let request = seed::seed(seed::scope(), a.clone(), "seed");
    assert!(
        hub.validate(c.clone(), request.clone())
            .await
            .unwrap()
            .is_valid()
    );
    assert_eq!(hub.head(c.clone()).await.unwrap(), a);
    let b = hub.commit(c.clone(), request.clone()).await.unwrap();
    let snapshot = hub.read_snapshot(c.clone(), b.head.clone()).await.unwrap();
    assert!(snapshot.seed_complete());
    assert_eq!(snapshot.sources[0].content.bytes, seed::ONTOLOGY);
    assert_eq!(
        snapshot.sources[0].reference,
        seed::scope().profile.ontology
    );
    let source = &snapshot.sources[0];
    let anchor = &source.anchors[0];
    assert_eq!(
        &source.content.bytes[anchor.start as usize..anchor.end as usize],
        anchor.quote.as_bytes()
    );
    assert_eq!(
        hub.read_element(
            c.clone(),
            b.head.clone(),
            snapshot.elements[0].reference.clone()
        )
        .await
        .unwrap(),
        snapshot.elements[0]
    );
    assert_eq!(
        hub.read_assertion(
            c.clone(),
            b.head.clone(),
            snapshot.assertions[0].reference.clone()
        )
        .await
        .unwrap(),
        snapshot.assertions[0]
    );
    assert_eq!(
        hub.read_document(
            c.clone(),
            b.head.clone(),
            snapshot.documents[0].reference.clone()
        )
        .await
        .unwrap(),
        snapshot.documents[0]
    );
    assert_eq!(
        hub.read_source(c.clone(), b.head.clone(), source.reference.clone())
            .await
            .unwrap(),
        *source
    );
    // Snapshot A still resolves empty after B and after later movement.
    assert!(
        !hub.read_snapshot(c.clone(), a.clone())
            .await
            .unwrap()
            .seed_complete()
    );
    let mut current = b.head.clone();
    for n in 0..5 {
        current = hub
            .commit(
                c.clone(),
                seed::additional_document(seed::scope(), current, n),
            )
            .await
            .unwrap()
            .head;
    }
    assert_eq!(hub.commit(c.clone(), request.clone()).await.unwrap(), b);
    assert_eq!(hub.head(c.clone()).await.unwrap(), current);
    hub.drain().await;
    drop(hub);
    let hub = sandbox.open().await.unwrap();
    assert_eq!(
        hub.operation_result(c.clone(), "seed".into())
            .await
            .unwrap(),
        OperationOutcome::Committed(Box::new(b.clone()))
    );
    assert_eq!(
        hub.read_snapshot(c.clone(), b.head.clone()).await.unwrap(),
        snapshot
    );
    assert_eq!(hub.read_snapshot(c.clone(), a).await.unwrap().sequence, 0);
    assert_eq!(
        hub.operation_result(c, "never-committed".into())
            .await
            .unwrap(),
        OperationOutcome::Absent {
            recovered_head: current
        }
    );
    hub.drain().await;
}
#[tokio::test(flavor = "current_thread")]
async fn public_walkthrough_current_thread() {
    walkthrough().await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn public_walkthrough_multithreaded() {
    walkthrough().await;
}

#[tokio::test]
async fn invalid_seed_examples_preserve_head_independently() {
    let s = Sandbox::new();
    let hub = s.create().await;
    let c = seed::context();
    let a = hub.head(c.clone()).await.unwrap();
    for case in 0..12 {
        let mut request = seed::seed(seed::scope(), a.clone(), &format!("invalid:{case}"));
        match case {
            0 => {
                if let Operation::CaptureSource(r) = &mut request.operations[0] {
                    r.content.reference.sha256 = "0".repeat(64);
                }
            }
            1 => {
                if let Operation::CaptureSource(r) = &mut request.operations[0] {
                    r.anchors[0].start += 1;
                }
            }
            2 => {
                if let Operation::CaptureSource(r) = &mut request.operations[0] {
                    r.anchors.clear();
                }
            }
            3 => {
                if let Operation::Interpret(r) = &mut request.operations[2] {
                    r.source.revision = "0".repeat(64);
                    *r = r.clone().seal().unwrap();
                }
            }
            4 => {
                if let Operation::Define(r) = &mut request.operations[3] {
                    r.element_type.id = "UnsupportedType".into();
                    *r = r.clone().seal().unwrap();
                }
            }
            5 => {
                if let Operation::Assert(r) = &mut request.operations[5] {
                    r.to = r.from.clone();
                    *r = r.clone().seal().unwrap();
                }
            }
            6 => {
                if let Operation::Assert(r) = &mut request.operations[5] {
                    r.to.revision = "0".repeat(64);
                    *r = r.clone().seal().unwrap();
                }
            }
            7 => {
                if let Operation::Define(r) = &mut request.operations[4] {
                    r.authority.retain(|b| b.field != "actor");
                    *r = r.clone().seal().unwrap();
                }
            }
            8 => {
                if let Operation::Define(r) = &mut request.operations[3] {
                    r.authority.push(AuthorityBinding {
                        field: "title".into(),
                        owner: Owner::Graph,
                    });
                    *r = r.clone().seal().unwrap();
                }
            }
            9 => {
                if let Operation::Assert(r) = &mut request.operations[5] {
                    r.interpretation.id = "missing".into();
                    *r = r.clone().seal().unwrap();
                }
            }
            10 => {
                if let Operation::Define(r) = &mut request.operations[3] {
                    r.authority.push(AuthorityBinding {
                        field: "arbitrary".into(),
                        owner: Owner::Graph,
                    });
                    *r = r.clone().seal().unwrap();
                }
            }
            11 => {
                if let Operation::AddDocument(r) = &mut request.operations[1] {
                    r.fragments[0].start = u64::MAX;
                }
            }
            _ => unreachable!(),
        }
        let report = hub.validate(c.clone(), request.clone()).await.unwrap();
        assert!(!report.is_valid(), "case {case}");
        assert!(
            matches!(
                hub.commit(c.clone(), request.clone()).await,
                Err(Error::Invalid(_) | Error::InvalidInput(_))
            ),
            "case {case}"
        );
        assert_eq!(hub.head(c.clone()).await.unwrap(), a);
        assert!(matches!(
            hub.operation_result(c.clone(), request.operation_id)
                .await
                .unwrap(),
            OperationOutcome::Absent { .. }
        ));
    }
    let valid = seed::seed(seed::scope(), a.clone(), "invalid:0");
    assert!(hub.commit(c, valid).await.is_ok()); // rejected attempts never reserve a key
}

#[tokio::test]
async fn canonical_retries_bytes_actor_and_operation_order() {
    let s = Sandbox::new();
    let hub = s.create().await;
    let c = seed::context();
    let a = hub.head(c.clone()).await.unwrap();
    let request = seed::seed(seed::scope(), a, "seed");
    let receipt = hub.commit(c.clone(), request.clone()).await.unwrap();
    let mut value = serde_json::to_value(&request).unwrap();
    // Alternate JSON formatting and field order is parsed back to equivalent typed intent.
    let raw = serde_json::to_string_pretty(&value).unwrap();
    let parsed = ChangeSet::from_json(raw.as_bytes()).unwrap();
    assert_eq!(hub.commit(c.clone(), parsed).await.unwrap(), receipt);
    let mut changed = request.clone();
    if let Operation::AddDocument(d) = &mut changed.operations[1] {
        d.content.bytes.push(b' ');
        d.content.reference = ContentRef::of(&d.content.bytes);
    }
    assert_eq!(
        hub.commit(c.clone(), changed).await.unwrap_err(),
        Error::OperationReuse
    );
    let mut changed = request.clone();
    if let Operation::CaptureSource(s) = &mut changed.operations[0] {
        s.content.bytes.push(b' ');
        s.content.reference = ContentRef::of(&s.content.bytes);
        s.reference.revision = s.content.reference.sha256.clone();
    }
    assert_eq!(
        hub.commit(c.clone(), changed).await.unwrap_err(),
        Error::OperationReuse
    );
    let mut changed = request.clone();
    changed.operations.swap(3, 4);
    assert_eq!(
        hub.commit(c.clone(), changed).await.unwrap_err(),
        Error::OperationReuse
    );
    let other = Context::new("host:other", Arc::new(seed::Allow));
    assert_eq!(
        hub.commit(other.clone(), request.clone())
            .await
            .unwrap_err(),
        Error::OperationReuse
    );
    assert_eq!(
        hub.operation_result(other, "seed".into())
            .await
            .unwrap_err(),
        Error::OperationReuse
    );
    value["unrecognised"] = true.into();
    assert!(ChangeSet::from_json(&serde_json::to_vec(&value).unwrap()).is_err());
    value.as_object_mut().unwrap().remove("unrecognised");
    value["operations"][3]["record"]["unknownNarrativeCopy"] = "competing value".into();
    assert!(ChangeSet::from_json(&serde_json::to_vec(&value).unwrap()).is_err());
    let text = serde_json::to_string(&request).unwrap();
    let duplicate = text.replacen(
        "\"operation_id\":\"seed\"",
        "\"operation_id\":\"seed\",\"operation_id\":\"seed\"",
        1,
    );
    assert_ne!(duplicate, text);
    assert!(ChangeSet::from_json(duplicate.as_bytes()).is_err());
    assert_eq!(hub.head(c).await.unwrap(), receipt.head);
}

struct CurrentPolicy {
    allowed: AtomicBool,
    failed: AtomicBool,
}
impl Policy for CurrentPolicy {
    fn allows(&self, _: &str, _: &Scope, _: Action) -> Result<bool, PolicyFailure> {
        if self.failed.load(Ordering::SeqCst) {
            Err(PolicyFailure)
        } else {
            Ok(self.allowed.load(Ordering::SeqCst))
        }
    }
}
#[tokio::test]
async fn default_denial_revocation_and_policy_failure() {
    let s = Sandbox::new();
    let denied = Context::new("host:denied", Arc::new(DenyAll));
    assert!(matches!(
        KnowledgeHub::create(s.backend(), denied.clone(), seed::scope()).await,
        Err(Error::Denied { .. })
    ));
    assert!(!s.store.exists());
    let hub = s.create().await;
    let c = seed::context();
    let a = hub.head(c.clone()).await.unwrap();
    let request = seed::seed(seed::scope(), a.clone(), "seed");
    assert!(matches!(
        hub.validate(denied.clone(), request.clone()).await,
        Err(Error::Denied { .. })
    ));
    let policy = Arc::new(CurrentPolicy {
        allowed: AtomicBool::new(true),
        failed: AtomicBool::new(false),
    });
    let current = Context::new(c.actor.clone(), policy.clone());
    assert!(
        hub.validate(current.clone(), request.clone())
            .await
            .unwrap()
            .is_valid()
    );
    policy.allowed.store(false, Ordering::SeqCst);
    assert!(matches!(
        hub.commit(current.clone(), request.clone()).await,
        Err(Error::Denied { .. })
    ));
    assert_eq!(hub.head(c.clone()).await.unwrap(), a);
    policy.allowed.store(true, Ordering::SeqCst);
    let receipt = hub.commit(current.clone(), request.clone()).await.unwrap();
    policy.allowed.store(false, Ordering::SeqCst);
    assert!(matches!(
        hub.commit(current.clone(), request).await,
        Err(Error::Denied { .. })
    ));
    assert!(matches!(
        hub.operation_result(current.clone(), "seed".into()).await,
        Err(Error::Denied { .. })
    ));
    assert!(matches!(
        hub.read_snapshot(current.clone(), receipt.head).await,
        Err(Error::Denied { .. })
    ));
    policy.failed.store(true, Ordering::SeqCst);
    assert!(matches!(
        hub.head(current).await,
        Err(Error::Unavailable(_))
    ));
    assert!(matches!(
        KnowledgeHub::open(s.backend(), denied, seed::scope()).await,
        Err(Error::Denied { .. })
    ));
}

struct RevokeAtAdmission(AtomicUsize);
impl Policy for RevokeAtAdmission {
    fn allows(&self, _: &str, _: &Scope, action: Action) -> Result<bool, PolicyFailure> {
        Ok(action != Action::Commit || self.0.fetch_add(1, Ordering::SeqCst) == 0)
    }
}
#[tokio::test]
async fn recheck_policy_inside_publication_lock() {
    let s = Sandbox::new();
    let hub = s.create().await;
    let c = seed::context();
    let a = hub.head(c.clone()).await.unwrap();
    let current = Context::new(
        c.actor.clone(),
        Arc::new(RevokeAtAdmission(AtomicUsize::new(0))),
    );
    assert!(matches!(
        hub.commit(current, seed::seed(seed::scope(), a.clone(), "seed"))
            .await,
        Err(Error::Denied { .. })
    ));
    assert_eq!(hub.head(c).await.unwrap(), a);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn separately_opened_instances_race_and_later_stale_conflict() {
    let s = Sandbox::new();
    let first = s.create().await;
    let second = s.open().await.unwrap();
    let c = seed::context();
    let a = first.head(c.clone()).await.unwrap();
    let x = seed::seed(seed::scope(), a.clone(), "x");
    let y = seed::seed(seed::scope(), a.clone(), "y");
    let (rx, ry) = tokio::join!(
        first.commit(c.clone(), x.clone()),
        second.commit(c.clone(), y.clone())
    );
    assert_ne!(rx.is_ok(), ry.is_ok());
    let (winner, loser) = if let Ok(w) = rx {
        assert!(matches!(ry, Err(Error::Busy | Error::Conflict { .. })));
        (w, y)
    } else {
        assert!(matches!(rx, Err(Error::Busy | Error::Conflict { .. })));
        (ry.unwrap(), x)
    };
    assert!(matches!(
        second.commit(c.clone(), loser).await,
        Err(Error::Conflict { .. })
    ));
    let state = first.read_snapshot(c, winner.head).await.unwrap();
    assert_eq!(state.sequence, 1);
    assert_eq!(state.elements.len(), 2);
}

#[tokio::test]
async fn explicit_revision_and_absence_guards() {
    let s = Sandbox::new();
    let h = s.create().await;
    let c = seed::context();
    let a = h.head(c.clone()).await.unwrap();
    let receipt = h
        .commit(c.clone(), seed::seed(seed::scope(), a, "seed"))
        .await
        .unwrap();
    let snapshot = h
        .read_snapshot(c.clone(), receipt.head.clone())
        .await
        .unwrap();
    let r = &snapshot.elements[0].reference;
    let mut request = seed::additional_document(seed::scope(), receipt.head.clone(), 10);
    request.guards.push(RevisionGuard {
        id: r.id.clone(),
        expected: None,
    });
    assert!(matches!(
        h.commit(c.clone(), request.clone()).await,
        Err(Error::GuardConflict { .. })
    ));
    request.guards[0].expected = Some(r.revision.clone());
    assert!(h.commit(c, request).await.is_ok());
}

#[tokio::test]
async fn incompatible_and_damaged_stores_are_not_recreated() {
    let s = Sandbox::new();
    let h = s.create().await;
    let c = seed::context();
    assert!(matches!(
        KnowledgeHub::create(s.backend(), c.clone(), seed::scope()).await,
        Err(Error::AlreadyExists)
    ));
    let bytes = std::fs::read(s.store.join("hub.json")).unwrap();
    let mut meta: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    meta["format"] = "knowledgehub-files/99".into();
    std::fs::write(s.store.join("hub.json"), serde_json::to_vec(&meta).unwrap()).unwrap();
    assert!(matches!(s.open().await, Err(Error::Incompatible(_))));
    assert!(matches!(
        KnowledgeHub::create(s.backend(), c.clone(), seed::scope()).await,
        Err(Error::Incompatible(_))
    ));
    std::fs::write(s.store.join("hub.json"), &bytes).unwrap();
    let head = h.head(c.clone()).await.unwrap();
    std::fs::write(
        s.store.join("generations").join(&head.0).join("graph.json"),
        b"corrupt",
    )
    .unwrap();
    assert!(matches!(s.open().await, Err(Error::Integrity(_))));
    assert!(matches!(
        h.operation_result(c.clone(), "missing".into()).await,
        Err(Error::Integrity(_))
    ));
    assert!(matches!(
        KnowledgeHub::create(s.backend(), c, seed::scope()).await,
        Err(Error::Integrity(_))
    ));
}

#[tokio::test]
async fn refuses_unsupported_profile_and_detected_filesystem() {
    let s = Sandbox::new();
    let mut scope = seed::scope();
    scope.profile.version = "knowledgehub-seed/99".into();
    assert!(matches!(
        KnowledgeHub::create(s.backend(), seed::context(), scope).await,
        Err(Error::Incompatible(_))
    ));
    assert!(!s.store.exists());
    let mut options = FileOptions::local_ext4();
    options.support.coordinated_single_host = false;
    let backend = FileBackend::new(&s.store, options).unwrap();
    assert!(matches!(
        KnowledgeHub::create(backend, seed::context(), seed::scope()).await,
        Err(Error::Incompatible(_))
    ));
    // /tmp is tmpfs on this test host; no store should be created there.
    if std::process::Command::new("stat")
        .args(["-f", "-c", "%T", "/tmp"])
        .output()
        .is_ok_and(|o| o.stdout.starts_with(b"tmpfs"))
    {
        let path = std::path::PathBuf::from(format!("/tmp/kh-rejected-{}", std::process::id()));
        let backend = FileBackend::new(&path, FileOptions::local_ext4()).unwrap();
        assert!(matches!(
            KnowledgeHub::create(backend, seed::context(), seed::scope()).await,
            Err(Error::Incompatible(_))
        ));
        assert!(!path.exists());
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn coordinated_exclusive_scope_creation() {
    let s = Sandbox::new();
    let (x, y) = tokio::join!(
        KnowledgeHub::create(s.backend(), seed::context(), seed::scope()),
        KnowledgeHub::create(s.backend(), seed::context(), seed::scope())
    );
    assert_ne!(x.is_ok(), y.is_ok());
    let failed = if x.is_ok() { y } else { x };
    assert!(matches!(failed, Err(Error::AlreadyExists | Error::Busy)));
    let h = s.open().await.unwrap();
    let a = h.head(seed::context()).await.unwrap();
    assert_eq!(
        h.read_snapshot(seed::context(), a).await.unwrap().sequence,
        0
    );
}

#[test]
fn file_adapter_requires_the_host_runtime_and_creates_none() {
    use std::{
        future::Future,
        task::{Context as TaskContext, Poll, Waker},
    };
    let s = Sandbox::new();
    let mut future = Box::pin(KnowledgeHub::create(
        s.backend(),
        seed::context(),
        seed::scope(),
    ));
    let mut context = TaskContext::from_waker(Waker::noop());
    assert!(matches!(
        future.as_mut().poll(&mut context),
        Poll::Ready(Err(Error::Unavailable(_)))
    ));
    assert!(!s.store.exists());
}
