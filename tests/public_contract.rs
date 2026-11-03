//! Current default API checks, including the safety cases first exercised by the seed harness.
#[path = "../examples/support/mod.rs"]
mod seed;
mod support;
use knowledgehub::{backends::files::*, *};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
use support::Sandbox;

async fn registered(s: &Sandbox) -> KnowledgeHub {
    let h = s.create().await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut c = Change::at(&state, head, "catalogue");
    c.operations = builtin_catalogue().unwrap().operations();
    h.commit(seed::context(), c).await.unwrap();
    h
}
fn extra(state: &State, head: SnapshotId, id: &str) -> Change {
    let record = Record::new(
        ExactRef::new(&state.scope.model, id, "1"),
        type_ref("SourceArtifact"),
        RecordData::SourceArtifact(SourceArtifactData {
            origin: id.into(),
            media_type: "text/plain".into(),
            owner: state.scope.owner.clone(),
            status: "hypothesis".into(),
        }),
        vec![],
    )
    .unwrap();
    let mut c = Change::at(state, head, id);
    c.operations.push(Operation::Put(record));
    c
}
async fn walkthrough() {
    let s = Sandbox::new();
    let h = s.create().await;
    let context = seed::context();
    let (a, empty) = h.snapshot(context.clone()).await.unwrap();
    assert!(empty.records.is_empty());
    let mut c = Change::at(&empty, a.clone(), "catalogue");
    c.operations = builtin_catalogue().unwrap().operations();
    assert!(
        h.validate(context.clone(), c.clone())
            .await
            .unwrap()
            .is_valid()
    );
    assert_eq!(h.head(context.clone()).await.unwrap(), a);
    let receipt = h.commit(context.clone(), c.clone()).await.unwrap();
    assert_eq!(receipt.scope, seed::scope());
    assert!(receipt.affected.iter().all(ExactRef::is_valid));
    let selected = Selection::Snapshot(receipt.head.clone());
    assert_eq!(
        h.read(context.clone(), selected.clone(), type_ref("Requirement"))
            .await
            .unwrap()
            .type_ref,
        type_ref("ElementTypeDefinition")
    );
    let source = ExactRef::new("sdo", "urn:source:ontology:capture", "catalogue-3");
    assert_eq!(
        h.read_artifact(context.clone(), selected, source)
            .await
            .unwrap()
            .bytes,
        seed::ONTOLOGY
    );
    assert!(
        h.read_state(context.clone(), Selection::Snapshot(a))
            .await
            .unwrap()
            .records
            .is_empty()
    );
    let (head, state) = h.snapshot(context.clone()).await.unwrap();
    h.commit(context.clone(), extra(&state, head, "later"))
        .await
        .unwrap();
    h.drain().await;
    let reopened = s.open().await.unwrap();
    assert_eq!(reopened.commit(context.clone(), c).await.unwrap(), receipt);
    assert_eq!(
        reopened
            .operation_result(context, "catalogue".into())
            .await
            .unwrap(),
        OperationOutcome::Committed(Box::new(receipt))
    );
}
#[tokio::test]
async fn public_walkthrough_current_thread() {
    walkthrough().await;
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn public_walkthrough_multithreaded() {
    walkthrough().await;
}

#[tokio::test]
async fn canonical_retries_bytes_actor_and_operation_order() {
    let s = Sandbox::new();
    let h = registered(&s).await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut c = extra(&state, head, "canonical");
    c.operations
        .extend(extra(&state, c.expected_head.clone(), "second").operations);
    let fp = c.fingerprint("host:maintainer").unwrap();
    let mut reversed = c.clone();
    reversed.operations.reverse();
    assert_ne!(fp, reversed.fingerprint("host:maintainer").unwrap());
    assert_ne!(fp, c.fingerprint("host:another").unwrap());
    let value = serde_json::to_value(&c).unwrap();
    let equivalent = Change::from_json(&serde_json::to_vec(&value).unwrap()).unwrap();
    assert_eq!(fp, equivalent.fingerprint("host:maintainer").unwrap());
    let receipt = h.commit(seed::context(), c.clone()).await.unwrap();
    assert_eq!(
        h.commit(seed::context(), equivalent).await.unwrap(),
        receipt
    );
    assert!(matches!(
        h.commit(seed::context(), reversed).await,
        Err(Error::OperationReuse)
    ));
    assert!(matches!(
        h.commit(Context::new("host:another", Arc::new(seed::Allow)), c)
            .await,
        Err(Error::OperationReuse)
    ));
    // Authored bytes stay significant under current artifact intent.
    let record = Record::new(
        ExactRef::new("test", "source", "1"),
        type_ref("SourceRevision"),
        RecordData::SourceRevision(SourceRevisionData {
            source: ExactRef::new("test", "origin", "1"),
            original: ContentRef::of(b"one"),
            media_type: "text/plain".into(),
            derived_from: None,
            derivation: None,
        }),
        vec![],
    )
    .unwrap();
    let mut capture = Change::at(&state, receipt.head.clone(), "bytes");
    capture.operations = vec![Operation::Capture {
        record,
        content: Content::capture(b"one".to_vec()),
    }];
    let original = capture.fingerprint("host:maintainer").unwrap();
    let Operation::Capture { record, content } = &mut capture.operations[0] else {
        unreachable!()
    };
    *content = Content::capture(b"two".to_vec());
    let RecordData::SourceRevision(data) = &mut record.data else {
        unreachable!()
    };
    data.original = content.reference.clone();
    record.seal().unwrap();
    assert_ne!(original, capture.fingerprint("host:maintainer").unwrap());
}
struct Switch {
    allowed: AtomicBool,
    fail: AtomicBool,
}
impl Policy for Switch {
    fn allows(&self, _: &str, scope: &ModelScope, _: Action) -> Result<bool, PolicyFailure> {
        assert_eq!(scope, &seed::scope());
        if self.fail.load(Ordering::SeqCst) {
            Err(PolicyFailure)
        } else {
            Ok(self.allowed.load(Ordering::SeqCst))
        }
    }
}
#[tokio::test]
async fn default_denial_revocation_and_policy_failure() {
    let s = Sandbox::new();
    assert!(matches!(
        KnowledgeHub::create(
            s.backend(),
            Context::new("host:denied", Arc::new(DenyAll)),
            seed::scope()
        )
        .await,
        Err(Error::Denied { .. })
    ));
    assert!(!s.store.exists());
    let h = registered(&s).await;
    let policy = Arc::new(Switch {
        allowed: AtomicBool::new(true),
        fail: AtomicBool::new(false),
    });
    let c = Context::new("host:maintainer", policy.clone());
    h.head(c.clone()).await.unwrap();
    policy.allowed.store(false, Ordering::SeqCst);
    assert!(matches!(h.head(c.clone()).await, Err(Error::Denied { .. })));
    assert!(matches!(
        h.operation_result(c.clone(), "catalogue".into()).await,
        Err(Error::Denied { .. })
    ));
    policy.fail.store(true, Ordering::SeqCst);
    assert!(matches!(h.head(c).await, Err(Error::Unavailable(_))));
}
struct Revoke(AtomicUsize);
impl Policy for Revoke {
    fn allows(&self, _: &str, _: &ModelScope, action: Action) -> Result<bool, PolicyFailure> {
        Ok(action != Action::Commit || self.0.fetch_add(1, Ordering::SeqCst) == 0)
    }
}
#[tokio::test]
async fn recheck_policy_inside_publication_lock() {
    let s = Sandbox::new();
    let h = registered(&s).await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let c = extra(&state, head.clone(), "revoked");
    assert!(matches!(
        h.commit(
            Context::new("host:maintainer", Arc::new(Revoke(AtomicUsize::new(0)))),
            c
        )
        .await,
        Err(Error::Denied {
            action: Action::Commit
        })
    ));
    assert_eq!(h.head(seed::context()).await.unwrap(), head);
    assert!(matches!(
        h.operation_result(seed::context(), "revoked".into())
            .await
            .unwrap(),
        OperationOutcome::Absent { .. }
    ));
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn separately_opened_instances_race_and_later_stale_conflict() {
    let s = Sandbox::new();
    let a = registered(&s).await;
    let b = s.open().await.unwrap();
    let (head, state) = a.snapshot(seed::context()).await.unwrap();
    let left = extra(&state, head.clone(), "left");
    let right = extra(&state, head, "right");
    let (r1, r2) = tokio::join!(
        a.commit(seed::context(), left.clone()),
        b.commit(seed::context(), right.clone())
    );
    assert_eq!(usize::from(r1.is_ok()) + usize::from(r2.is_ok()), 1);
    let loser = if r1.is_ok() { right } else { left };
    assert!(matches!(
        a.commit(seed::context(), loser).await,
        Err(Error::Conflict { .. })
    ));
}
#[tokio::test]
async fn explicit_revision_and_absence_guards() {
    let s = Sandbox::new();
    let h = registered(&s).await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut c = extra(&state, head.clone(), "guarded");
    let r = type_ref("Requirement");
    c.guards.push(Guard {
        model: r.model.clone(),
        id: r.id.clone(),
        expected: None,
    });
    assert!(matches!(
        h.commit(seed::context(), c.clone()).await,
        Err(Error::GuardConflict { .. })
    ));
    c.guards[0].expected = Some(r);
    let committed = h.commit(seed::context(), c).await.unwrap();
    assert_ne!(committed.head, head);
}
#[tokio::test]
async fn invalid_current_changes_preserve_head_independently() {
    let s = Sandbox::new();
    let h = registered(&s).await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    for case in 0..3 {
        let mut c = extra(&state, head.clone(), &format!("invalid:{case}"));
        let Operation::Put(record) = &mut c.operations[0] else {
            unreachable!()
        };
        match case {
            0 => record.type_ref = ExactRef::new("missing", "type", "1"),
            1 => record
                .provenance
                .push(ExactRef::new("missing", "anchor", "1")),
            _ => record.sha256 = "bad".into(),
        };
        assert!(
            !h.validate(seed::context(), c.clone())
                .await
                .unwrap()
                .is_valid()
        );
        assert!(h.commit(seed::context(), c).await.is_err());
        assert_eq!(h.head(seed::context()).await.unwrap(), head);
    }
}
#[tokio::test]
async fn incompatible_and_damaged_stores_are_not_recreated() {
    let s = Sandbox::new();
    let h = registered(&s).await;
    h.drain().await;
    let path = s.store.join("hub.json");
    let mut meta: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    meta["format"] = "unsupported".into();
    let bytes = serde_json::to_vec(&meta).unwrap();
    std::fs::write(&path, &bytes).unwrap();
    assert!(matches!(s.open().await, Err(Error::Incompatible(_))));
    assert!(
        KnowledgeHub::create(s.backend(), seed::context(), seed::scope())
            .await
            .is_err()
    );
    assert_eq!(std::fs::read(path).unwrap(), bytes);
}
#[tokio::test]
async fn refuses_unsupported_declared_filesystem() {
    let s = Sandbox::new();
    let mut options = FileOptions::native_local().unwrap();
    options.support.coordinated_single_host = false;
    let b = FileBackend::new(&s.store, options).unwrap();
    assert!(matches!(
        KnowledgeHub::create(b, seed::context(), seed::scope()).await,
        Err(Error::Incompatible(_))
    ));
    assert!(!s.store.exists());
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn coordinated_exclusive_scope_creation() {
    let s = Sandbox::new();
    let (a, b) = tokio::join!(
        KnowledgeHub::create(s.backend(), seed::context(), seed::scope()),
        KnowledgeHub::create(s.backend(), seed::context(), seed::scope())
    );
    assert_eq!(usize::from(a.is_ok()) + usize::from(b.is_ok()), 1);
    assert!(matches!(
        KnowledgeHub::create(s.backend(), seed::context(), seed::scope()).await,
        Err(Error::AlreadyExists)
    ));
}
#[test]
fn file_adapter_requires_the_host_runtime_and_creates_none() {
    use std::{
        future::Future,
        task::{Context as TaskContext, Poll, Waker},
    };
    let s = Sandbox::new();
    let mut future = std::pin::pin!(KnowledgeHub::create(
        s.backend(),
        seed::context(),
        seed::scope()
    ));
    let mut c = TaskContext::from_waker(Waker::noop());
    assert!(matches!(
        future.as_mut().poll(&mut c),
        Poll::Ready(Err(Error::Unavailable(_)))
    ));
    assert!(!s.store.exists());
    assert_eq!(env!("CARGO_PKG_VERSION"), "0.10.0");
}
