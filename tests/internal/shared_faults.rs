//! Current full-model publication fault checks. Hooks are compiled only into the native test harness.
use crate::test_specimens as specimens;
use crate::test_support as support;
use crate::{backends::files::*, *};
use std::{
    path::Path,
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};
use support::Sandbox;
const STAGES: [&str; 8] = [
    "staging",
    "content-file",
    "content-synced",
    "generation-installed",
    "reference-synced",
    "head-replaced",
    "head-synced",
    "acknowledgement",
];
fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
}
fn backend(s: &Sandbox) -> FileBackend {
    FileBackend::new(&s.store, FileOptions::local_ext4()).unwrap()
}
async fn create(s: &Sandbox) -> KnowledgeHub {
    let h = KnowledgeHub::create(backend(s), seed::context(), specimens::scope())
        .await
        .unwrap();
    let (head, state) = h.snapshot(seed::context()).await.unwrap();
    let mut c = Change::at(&state, head, "catalogue");
    c.operations = builtin_catalogue().unwrap().operations();
    h.commit(seed::context(), c).await.unwrap();
    h
}
fn request(state: &State, head: SnapshotId, id: &str) -> Change {
    let mut c = Change::at(state, head, id);
    let mut next = state.clone();
    // Non-text original bytes and a complete named baseline publish with the typed generation.
    let source = ExactRef::new("fault", "binary", "1");
    let capture = ExactRef::new("fault", "binary:capture", "1");
    let bytes = Content::capture(vec![0, 255, 1, 2, 128]);
    let records = [
        Record::new(
            source.clone(),
            type_ref("SourceArtifact"),
            RecordData::SourceArtifact(SourceArtifactData {
                origin: "fault fixture".into(),
                media_type: "application/octet-stream".into(),
                owner: "test:host".into(),
                status: "observation".into(),
            }),
            vec![],
        )
        .unwrap(),
        Record::new(
            capture.clone(),
            type_ref("SourceRevision"),
            RecordData::SourceRevision(SourceRevisionData {
                source,
                original: bytes.reference.clone(),
                media_type: "application/octet-stream".into(),
                derived_from: None,
                derivation: None,
            }),
            vec![],
        )
        .unwrap(),
    ];
    for r in records {
        specimens::install(&mut next, r.clone());
        if r.reference == capture {
            next.artifacts.insert(r.reference.key(), bytes.clone());
            c.operations.push(ontology::Operation::Capture {
                record: r,
                content: bytes.clone(),
            });
        } else {
            c.operations.push(ontology::Operation::Put(r));
        }
    }
    let b = Record::new(
        ExactRef::new("fault", "baseline:B", "1"),
        type_ref("ModelBaseline"),
        RecordData::Baseline(next.baseline_data().unwrap()),
        vec![specimens::anchor()],
    )
    .unwrap();
    c.operations.push(ontology::Operation::Put(b));
    c
}
fn worker(s: &Sandbox, stage: &str) -> Child {
    Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "shared_fault_tests::process_worker",
            "--nocapture",
        ])
        .env("KH_CURRENT_STORE", &s.store)
        .env("KH_CURRENT_STAGE", stage)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}
fn ready(s: &Sandbox) -> std::path::PathBuf {
    s.parent.join("current-ready")
}
fn wait_ready(s: &Sandbox, child: &mut Child) {
    let start = Instant::now();
    while !ready(s).exists() {
        assert!(child.try_wait().unwrap().is_none());
        if start.elapsed() > Duration::from_secs(30) {
            child.kill().unwrap();
            panic!("current process checkpoint timed out");
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}
#[test]
fn process_worker() {
    let Ok(path) = std::env::var("KH_CURRENT_STORE") else {
        return;
    };
    let stage = std::env::var("KH_CURRENT_STAGE").unwrap();
    let marker = Path::new(&path).parent().unwrap().join("current-ready");
    rt().block_on(async {
        let b = FileBackend::new(path, FileOptions::local_ext4()).unwrap();
        let h = KnowledgeHub::open(b.clone(), seed::context(), specimens::scope())
            .await
            .unwrap();
        let (head, state) = h.snapshot(seed::context()).await.unwrap();
        let c = request(&state, head, "interrupted-B");
        b.set_hook(move |point| {
            if point == stage {
                std::fs::write(&marker, point).unwrap();
                loop {
                    std::thread::park();
                }
            }
            Ok(())
        });
        if std::env::var_os("KH_CURRENT_MAINTENANCE").is_some() {
            b.compact(seed::context(), specimens::scope())
                .await
                .unwrap();
        } else {
            h.commit(seed::context(), c).await.unwrap();
        }
    });
}

const MAINTENANCE_STAGES: [&str; 7] = [
    "pack-file",
    "pack-installed",
    "pack-index",
    "pack-catalog",
    "cleanup-ready",
    "cleanup-before-remove",
    "cleanup-after-remove",
];
#[test]
fn sigkill_pack_install_and_cleanup_at_every_boundary() {
    for stage in MAINTENANCE_STAGES {
        let s = Sandbox::new();
        let runtime = rt();
        let before = runtime.block_on(async {
            let h = create(&s).await;
            let state = h.snapshot(seed::context()).await.unwrap();
            h.drain().await;
            state
        });
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "shared_fault_tests::process_worker",
                "--nocapture",
            ])
            .env("KH_CURRENT_STORE", &s.store)
            .env("KH_CURRENT_STAGE", stage)
            .env("KH_CURRENT_MAINTENANCE", "1")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        wait_ready(&s, &mut child);
        child.kill().unwrap();
        assert!(!child.wait_with_output().unwrap().status.success());
        runtime.block_on(async {
            let b = backend(&s);
            let h = KnowledgeHub::open(b.clone(), seed::context(), specimens::scope())
                .await
                .unwrap();
            assert_eq!(h.snapshot(seed::context()).await.unwrap(), before);
            assert!(matches!(
                h.operation_result(seed::context(), "catalogue".into())
                    .await
                    .unwrap(),
                OperationOutcome::Committed(_)
            ));
            b.scrub(seed::context(), specimens::scope()).await.unwrap();
            h.drain().await;
        });
        println!("current SIGKILL {stage}: exact state and lifetime receipt recoverable");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn injected_maintenance_errors_preserve_original_state() {
    for stage in MAINTENANCE_STAGES {
        let s = Sandbox::new();
        let h = create(&s).await;
        let before = h.snapshot(seed::context()).await.unwrap();
        h.drain().await;
        let b = backend(&s);
        let h = KnowledgeHub::open(b.clone(), seed::context(), specimens::scope())
            .await
            .unwrap();
        b.set_hook(move |p| {
            if p == stage {
                Err(Error::Storage {
                    stage: format!("injected {p}"),
                })
            } else {
                Ok(())
            }
        });
        assert!(
            b.compact(seed::context(), specimens::scope())
                .await
                .is_err()
        );
        b.set_hook(|_| Ok(()));
        assert_eq!(h.snapshot(seed::context()).await.unwrap(), before);
        b.compact(seed::context(), specimens::scope())
            .await
            .unwrap();
        assert_eq!(h.snapshot(seed::context()).await.unwrap(), before);
        h.drain().await;
    }
}
#[test]
fn sigkill_typed_artifact_and_baseline_at_every_boundary() {
    for stage in STAGES {
        let s = Sandbox::new();
        let runtime = rt();
        let (before, c) = runtime.block_on(async {
            let h = create(&s).await;
            let (head, state) = h.snapshot(seed::context()).await.unwrap();
            let c = request(&state, head.clone(), "interrupted-B");
            h.drain().await;
            (head, c)
        });
        let mut child = worker(&s, stage);
        wait_ready(&s, &mut child);
        runtime.block_on(async {
            assert!(matches!(
                KnowledgeHub::open(backend(&s), seed::context(), specimens::scope()).await,
                Err(Error::Busy)
            ));
        });
        child.kill().unwrap();
        assert!(!child.wait_with_output().unwrap().status.success());
        runtime.block_on(async {
            let h = KnowledgeHub::open(backend(&s), seed::context(), specimens::scope())
                .await
                .unwrap();
            let outcome = h
                .operation_result(seed::context(), "interrupted-B".into())
                .await
                .unwrap();
            if ["head-replaced", "head-synced", "acknowledgement"].contains(&stage) {
                let OperationOutcome::Committed(r) = outcome else {
                    panic!("lost B at {stage}")
                };
                let state = h
                    .read_state(
                        seed::context(),
                        Selection::Baseline(ExactRef::new("fault", "baseline:B", "1")),
                    )
                    .await
                    .unwrap();
                assert_eq!(
                    state
                        .artifact(&ExactRef::new("fault", "binary:capture", "1"))
                        .unwrap()
                        .bytes,
                    vec![0, 255, 1, 2, 128]
                );
                assert_eq!(h.commit(seed::context(), c.clone()).await.unwrap(), *r);
            } else {
                assert_eq!(h.head(seed::context()).await.unwrap(), before);
                assert!(matches!(outcome, OperationOutcome::Absent { .. }));
            }
            h.drain().await;
        });
        println!(
            "current SIGKILL {stage}: exact prior or complete B, binary bytes and original receipt"
        );
    }
}
async fn injected_publication() {
    for stage in STAGES {
        let s = Sandbox::new();
        let original = create(&s).await;
        let (head, state) = original.snapshot(seed::context()).await.unwrap();
        original.drain().await;
        let b = backend(&s);
        let h = KnowledgeHub::open(b.clone(), seed::context(), specimens::scope())
            .await
            .unwrap();
        let c = request(&state, head.clone(), "failure-B");
        b.set_hook(move |point| {
            if point == stage {
                Err(Error::Storage {
                    stage: format!("injected {stage}"),
                })
            } else {
                Ok(())
            }
        });
        let result = h.commit(seed::context(), c.clone()).await;
        assert!(result.is_err());
        if ["head-replaced", "head-synced", "acknowledgement"].contains(&stage) {
            assert!(matches!(result, Err(Error::Uncertain { .. })));
        } else {
            assert!(!matches!(result, Err(Error::Uncertain { .. })));
        }
        b.set_hook(|_| Ok(()));
        let outcome = h
            .operation_result(seed::context(), "failure-B".into())
            .await
            .unwrap();
        if ["head-replaced", "head-synced", "acknowledgement"].contains(&stage) {
            let OperationOutcome::Committed(r) = outcome else {
                panic!("post-replacement result absent")
            };
            assert_eq!(h.commit(seed::context(), c).await.unwrap(), *r);
        } else {
            assert!(matches!(outcome, OperationOutcome::Absent { .. }));
            assert_eq!(h.head(seed::context()).await.unwrap(), head);
        }
        h.drain().await;
    }
}
async fn cancelled_publication() {
    let s = Sandbox::new();
    let first = create(&s).await;
    let (head, state) = first.snapshot(seed::context()).await.unwrap();
    first.drain().await;
    let b = backend(&s);
    let h = KnowledgeHub::open(b.clone(), seed::context(), specimens::scope())
        .await
        .unwrap();
    let (started, rx) = std::sync::mpsc::channel();
    let (release, wait) = std::sync::mpsc::channel();
    let wait = std::sync::Mutex::new(wait);
    b.set_hook(move |point| {
        if point == "head-replaced" {
            started.send(()).unwrap();
            wait.lock().unwrap().recv().unwrap();
        }
        Ok(())
    });
    let c = request(&state, head, "cancelled-B");
    let job = h.clone();
    let task = tokio::spawn(async move { job.commit(seed::context(), c).await });
    while rx.try_recv().is_err() {
        tokio::task::yield_now().await;
    }
    task.abort();
    assert!(matches!(
        KnowledgeHub::open(backend(&s), seed::context(), specimens::scope()).await,
        Err(Error::Busy)
    ));
    release.send(()).unwrap();
    h.drain().await;
    let reopened = KnowledgeHub::open(backend(&s), seed::context(), specimens::scope())
        .await
        .unwrap();
    assert!(matches!(
        reopened
            .operation_result(seed::context(), "cancelled-B".into())
            .await
            .unwrap(),
        OperationOutcome::Committed(_)
    ));
}

#[test]
fn injected_typed_publication_and_cancellation_on_both_runtimes() {
    for multi in [false, true] {
        let mut builder = if multi {
            tokio::runtime::Builder::new_multi_thread()
        } else {
            tokio::runtime::Builder::new_current_thread()
        };
        builder.worker_threads(2).build().unwrap().block_on(async {
            injected_publication().await;
            cancelled_publication().await;
        });
    }
}

// A real two-process race exercises the same guarded current-format publication path.
#[test]
fn process_race_worker() {
    let Ok(path) = std::env::var("KH_CURRENT_RACE_STORE") else {
        return;
    };
    let id = std::env::var("KH_CURRENT_RACE_ID").unwrap();
    let parent = Path::new(&path).parent().unwrap().to_path_buf();
    rt().block_on(async {
        let h = KnowledgeHub::open(
            FileBackend::new(path, FileOptions::local_ext4()).unwrap(),
            seed::context(),
            specimens::scope(),
        )
        .await
        .unwrap();
        let (head, state) = h.snapshot(seed::context()).await.unwrap();
        let change = request(&state, head, &format!("race:{id}"));
        std::fs::write(parent.join(format!("ready-{id}")), b"ready").unwrap();
        let until = Instant::now() + Duration::from_secs(30);
        while !parent.join("publish-race").exists() {
            assert!(Instant::now() < until);
            std::thread::sleep(Duration::from_millis(5));
        }
        let outcome = loop {
            match h.commit(seed::context(), change.clone()).await {
                Err(Error::Busy) => {
                    assert!(Instant::now() < until);
                    std::thread::sleep(Duration::from_millis(5));
                }
                Ok(_) => break "committed",
                Err(Error::Conflict { .. }) => break "conflict",
                other => panic!("unexpected current race outcome: {other:?}"),
            }
        };
        std::fs::write(parent.join(format!("result-{id}")), outcome).unwrap();
        h.drain().await;
    });
}
#[test]
fn two_process_same_base_race_has_one_winner() {
    let s = Sandbox::new();
    rt().block_on(async {
        let h = create(&s).await;
        h.drain().await;
    });
    let mut children = Vec::new();
    for id in ["a", "b"] {
        children.push(
            Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "shared_fault_tests::process_race_worker",
                    "--nocapture",
                ])
                .env("KH_CURRENT_RACE_STORE", &s.store)
                .env("KH_CURRENT_RACE_ID", id)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap(),
        );
    }
    let until = Instant::now() + Duration::from_secs(30);
    while !["a", "b"]
        .iter()
        .all(|id| s.parent.join(format!("ready-{id}")).exists())
    {
        assert!(Instant::now() < until);
        std::thread::sleep(Duration::from_millis(5));
    }
    std::fs::write(s.parent.join("publish-race"), b"go").unwrap();
    for child in children {
        let result = child.wait_with_output().unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    let outcomes = ["a", "b"]
        .map(|id| std::fs::read_to_string(s.parent.join(format!("result-{id}"))).unwrap());
    assert_eq!(
        outcomes
            .iter()
            .filter(|s| s.as_str() == "committed")
            .count(),
        1
    );
    assert_eq!(
        outcomes.iter().filter(|s| s.as_str() == "conflict").count(),
        1
    );
    rt().block_on(async {
        let h = KnowledgeHub::open(backend(&s), seed::context(), specimens::scope())
            .await
            .unwrap();
        for (id, outcome) in ["a", "b"].into_iter().zip(outcomes) {
            let actual = h
                .operation_result(seed::context(), format!("race:{id}"))
                .await
                .unwrap();
            assert_eq!(
                matches!(actual, OperationOutcome::Committed(_)),
                outcome == "committed"
            );
        }
    });
}
