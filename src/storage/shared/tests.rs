use super::*;
use super::{generation, objects::Objects, trees::Tree};
use crate::{
    Context, DenyAll, OperationOutcome, ontology::Operation, ontology::*, seed,
    test_support::Sandbox,
};
use serde_json::json as value;
use std::sync::Arc;
struct RevokedOnAdmission(std::sync::atomic::AtomicUsize);
impl crate::Policy for RevokedOnAdmission {
    fn allows(
        &self,
        _: &str,
        _: &ModelScope,
        action: crate::Action,
    ) -> Result<bool, crate::PolicyFailure> {
        Ok(action != crate::Action::Commit || self.0.fetch_add(1, Ordering::SeqCst) == 0)
    }
}

fn backend(s: &Sandbox) -> FileBackend {
    FileBackend::new(&s.store, FileOptions::local_ext4()).unwrap()
}
async fn create(s: &Sandbox) -> (FileBackend, KnowledgeHub) {
    let b = backend(s);
    let h = KnowledgeHub::create(b.clone(), seed::context(), specimens::scope())
        .await
        .unwrap();
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut c = Change::at(&state, head, "catalogue");
    c.operations = builtin_catalogue().unwrap().operations();
    h.commit(seed::context(), c).await.unwrap();
    (b, h)
}
fn extra(state: &State, head: SnapshotId, id: &str) -> Change {
    let r = Record::new(
        ExactRef::new("test", id, "1"),
        type_ref("SourceArtifact"),
        RecordData::SourceArtifact(SourceArtifactData {
            origin: "synthetic storage test".into(),
            media_type: "application/octet-stream".into(),
            owner: "test:host".into(),
            status: "observation".into(),
        }),
        vec![],
    )
    .unwrap();
    let mut c = Change::at(state, head, id);
    c.operations.push(Operation::Put(r));
    c
}

#[tokio::test]
async fn current_default_guards_policy_and_same_base_race() {
    let s = Sandbox::new();
    let (_b, h) = create(&s).await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let denied = Context::new("test:denied", Arc::new(DenyAll));
    assert!(matches!(
        h.read(
            denied.clone(),
            Selection::Snapshot(head.clone()),
            ontology_ref()
        )
        .await,
        Err(Error::Denied { .. })
    ));
    assert!(matches!(
        h.commit(denied, extra(&state, head.clone(), "denied"))
            .await,
        Err(Error::Denied { .. })
    ));
    let mut invalid = extra(&state, head.clone(), "invalid");
    if let Operation::Put(r) = &mut invalid.operations[0] {
        r.type_ref = ExactRef::new("test", "missing-type", "1");
        r.seal().unwrap();
    }
    assert!(matches!(
        h.commit(seed::context(), invalid).await,
        Err(Error::Invalid(_))
    ));
    assert_eq!(h.head(seed::context()).await.unwrap(), head);
    let revoked = Context::new(
        "test:host",
        Arc::new(RevokedOnAdmission(std::sync::atomic::AtomicUsize::new(0))),
    );
    assert!(matches!(
        h.commit(revoked, extra(&state, head.clone(), "revoked"))
            .await,
        Err(Error::Denied {
            action: crate::Action::Commit
        })
    ));
    let mut guarded = extra(&state, head.clone(), "wrong-guard");
    guarded.guards.push(Guard {
        model: ontology_ref().model,
        id: ontology_ref().id,
        expected: None,
    });
    assert!(h.commit(seed::context(), guarded).await.is_err());
    assert_eq!(h.head(seed::context()).await.unwrap(), head);
    let first = extra(&state, head.clone(), "first");
    let second = extra(&state, head.clone(), "second");
    let (a, b) = tokio::join!(
        h.commit(seed::context(), first.clone()),
        h.commit(seed::context(), second.clone())
    );
    let (loser, result) = if a.is_ok() {
        assert!(b.is_err());
        (second, b)
    } else {
        assert!(b.is_ok());
        (first, a)
    };
    assert!(matches!(result, Err(Error::Busy | Error::Conflict { .. })));
    assert!(matches!(
        h.commit(seed::context(), loser).await,
        Err(Error::Conflict { .. })
    ));
    let (_, after) = h.snapshot(seed::context()).await.unwrap();
    assert_ne!(
        after.get(&ExactRef::new("test", "first", "1")).is_some(),
        after.get(&ExactRef::new("test", "second", "1")).is_some()
    );
    // A separately constructed default backend reopens the same current state.
    let reopened = KnowledgeHub::open(backend(&s), seed::context(), specimens::scope())
        .await
        .unwrap();
    assert_eq!(
        reopened.head(seed::context()).await.unwrap(),
        h.head(seed::context()).await.unwrap()
    );
}

#[tokio::test]
async fn ranges_skip_unrelated_content_and_scrub_finds_retained_corruption() {
    let s = Sandbox::new();
    let (b, h) = create(&s).await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut c = extra(&state, head, "binary");
    let content = Content::capture((0..2_100_013).map(|i| (i % 251) as u8).collect());
    let source = ExactRef::new("test", "binary:capture", "1");
    let r = Record::new(
        source.clone(),
        type_ref("SourceRevision"),
        RecordData::SourceRevision(SourceRevisionData {
            source: ExactRef::new("test", "binary", "1"),
            original: content.reference.clone(),
            media_type: "application/octet-stream".into(),
            derived_from: None,
            derivation: None,
        }),
        vec![],
    )
    .unwrap();
    c.operations.push(Operation::Capture {
        record: r,
        content: content.clone(),
    });
    let receipt = h.commit(seed::context(), c).await.unwrap();
    let result = h
        .read_artifact_range(
            seed::context(),
            Selection::Snapshot(receipt.head.clone()),
            source.clone(),
            1_048_570,
            30,
        )
        .await
        .unwrap();
    assert_eq!(result.bytes, content.bytes[1_048_570..1_048_600]);
    assert!(!result.full_content_verified);
    assert_eq!(result.original, content.reference);
    let o = Objects::new(&b.store);
    let oid = super::objects::oid("blob", &content.bytes[1_048_576..2_097_152]).unwrap();
    let path = o.path(&oid).unwrap();
    let saved = fs::read(&path).unwrap();
    fs::remove_file(&path).unwrap();
    h.read(
        seed::context(),
        Selection::Snapshot(receipt.head.clone()),
        ontology_ref(),
    )
    .await
    .unwrap();
    h.read_artifact_range(
        seed::context(),
        Selection::Snapshot(receipt.head.clone()),
        source.clone(),
        0,
        64,
    )
    .await
    .unwrap();
    assert!(b.scrub(seed::context(), specimens::scope()).await.is_err());
    fs::write(path, saved).unwrap();
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut unselect = Change::at(&state, head, "unselect");
    unselect
        .operations
        .push(Operation::Unselect(source.clone()));
    h.commit(seed::context(), unselect).await.unwrap();
    b.compact(seed::context(), specimens::scope())
        .await
        .unwrap();
    assert_eq!(
        h.read_artifact(seed::context(), Selection::Snapshot(receipt.head), source)
            .await
            .unwrap(),
        content
    );
}

#[tokio::test]
async fn original_retry_absence_orphans_missing_receipts_and_point_read_work() {
    let s = Sandbox::new();
    let (b, h) = create(&s).await;
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let c = extra(&state, head, "first");
    let receipt = h.commit(seed::context(), c.clone()).await.unwrap();
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    h.commit(seed::context(), extra(&state, head, "later"))
        .await
        .unwrap();
    assert_eq!(h.commit(seed::context(), c.clone()).await.unwrap(), receipt);
    let mut different = c.clone();
    different.operations.clear();
    assert!(matches!(
        h.commit(seed::context(), different).await,
        Err(Error::OperationReuse)
    ));
    let mut actor = seed::context();
    actor.actor = "other:actor".into();
    assert!(matches!(
        h.commit(actor.clone(), c).await,
        Err(Error::OperationReuse)
    ));
    assert!(matches!(
        h.operation_result(actor, "first".into()).await,
        Err(Error::OperationReuse)
    ));
    let objects = Objects::new(&b.store);
    let (id, g) = generation::root(&b.store, &specimens::scope(), &objects).unwrap();
    let mut orphan = g.clone();
    orphan.snapshot_id = SnapshotId("a".repeat(64));
    objects.put_json("generation", &orphan).unwrap();
    assert!(matches!(
        h.read_state(seed::context(), Selection::Snapshot(orphan.snapshot_id))
            .await,
        Err(Error::NotFound)
    ));
    assert!(matches!(
        h.operation_result(seed::context(), "orphan".into())
            .await
            .unwrap(),
        OperationOutcome::Absent { .. }
    ));
    let before = b.file_work();
    let r = h
        .read(
            seed::context(),
            Selection::Snapshot(g.snapshot_id.clone()),
            ontology_ref(),
        )
        .await
        .unwrap();
    assert_eq!(r.reference, ontology_ref());
    let work = b.file_work();
    assert!(work.object_reads - before.object_reads < 12);
    assert!(work.encoded_bytes_read - before.encoded_bytes_read < 100_000);
    // Required receipt corruption is not absence; unrelated point/head reads still have bounded scope.
    let v = Tree { objects: &objects }
        .get(&g.operations_root, "operations/1", &value!("first"))
        .unwrap()
        .unwrap();
    let rid = v.as_str().unwrap();
    let p = objects.path(rid).unwrap();
    let saved = fs::read(&p).unwrap();
    fs::remove_file(&p).unwrap();
    assert!(matches!(
        h.operation_result(seed::context(), "first".into()).await,
        Err(Error::Integrity(_))
    ));
    assert!(b.scrub(seed::context(), specimens::scope()).await.is_err());
    fs::write(&p, saved).unwrap();
    let p = objects.path(&g.operations_root).unwrap();
    let saved = fs::read(&p).unwrap();
    fs::remove_file(&p).unwrap();
    assert!(matches!(
        h.operation_result(seed::context(), "never-committed".into())
            .await,
        Err(Error::Integrity(_))
    ));
    fs::write(p, saved).unwrap();
    assert_eq!(
        generation::root(&b.store, &specimens::scope(), &Objects::new(&b.store))
            .unwrap()
            .0,
        id
    );
    b.compact(seed::context(), specimens::scope())
        .await
        .unwrap();
    assert_eq!(
        h.operation_result(seed::context(), "first".into())
            .await
            .unwrap(),
        OperationOutcome::Committed(Box::new(receipt))
    );
}

#[test]
fn coordinated_reader_and_maintenance_on_both_host_runtimes() {
    for multi in [false, true] {
        let mut builder = if multi {
            tokio::runtime::Builder::new_multi_thread()
        } else {
            tokio::runtime::Builder::new_current_thread()
        };
        let rt = builder.worker_threads(2).build().unwrap();
        rt.block_on(async {
            let s = Sandbox::new();
            let (b, h) = create(&s).await;
            let head = h.head(seed::context()).await.unwrap();
            let (started, rx) = std::sync::mpsc::channel();
            let (release, wait) = std::sync::mpsc::channel();
            let wait = std::sync::Mutex::new(wait);
            let once = std::sync::atomic::AtomicBool::new(false);
            b.set_hook(move |stage| {
                if stage == "read-object" && !once.swap(true, Ordering::SeqCst) {
                    started.send(()).unwrap();
                    wait.lock().unwrap().recv().unwrap();
                }
                Ok(())
            });
            let reader = h.clone();
            let task = tokio::spawn(async move {
                reader
                    .read(seed::context(), Selection::Snapshot(head), ontology_ref())
                    .await
            });
            while rx.try_recv().is_err() {
                tokio::task::yield_now().await;
            }
            assert!(matches!(
                backend(&s)
                    .compact(seed::context(), specimens::scope())
                    .await,
                Err(Error::Busy)
            ));
            release.send(()).unwrap();
            task.await.unwrap().unwrap();
            b.set_hook(|_| Ok(()));
            b.compact(seed::context(), specimens::scope())
                .await
                .unwrap();
            h.drain().await;
        });
    }
}
