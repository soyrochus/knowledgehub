//! Adapter fault harness. Hooks exist only in unit-test builds, never in the public product.
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
fn runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap()
}
fn start(s: &Sandbox, mode: &str, stage: &str, id: &str) -> Child {
    Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "fault_tests::process_worker", "--nocapture"])
        .env("KH_TEST_STORE", &s.store)
        .env("KH_TEST_MODE", mode)
        .env("KH_TEST_STAGE", stage)
        .env("KH_TEST_ID", id)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}
fn ready(s: &Sandbox, id: &str) -> std::path::PathBuf {
    s.parent.join(format!("ready-{id}"))
}
fn result(s: &Sandbox, id: &str) -> std::path::PathBuf {
    s.parent.join(format!("result-{id}"))
}
fn wait_file(path: &Path, child: &mut Child) {
    let start = Instant::now();
    while !path.exists() {
        assert!(
            child.try_wait().unwrap().is_none(),
            "worker exited before checkpoint: {}",
            path.display()
        );
        if start.elapsed() > Duration::from_secs(15) {
            let _ = child.kill();
            panic!("worker checkpoint timed out");
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn process_worker() {
    let Ok(path) = std::env::var("KH_TEST_STORE") else {
        return;
    };
    let mode = std::env::var("KH_TEST_MODE").unwrap();
    let stage = std::env::var("KH_TEST_STAGE").unwrap();
    let id = std::env::var("KH_TEST_ID").unwrap();
    let parent = Path::new(&path).parent().unwrap();
    let marker = parent.join(format!("ready-{id}"));
    let gate = parent.join("go");
    let output = parent.join(format!("result-{id}"));
    runtime().block_on(async {
        let backend = FileBackend::new(&path, FileOptions::local_ext4()).unwrap();
        let hub = loop {
            match KnowledgeHub::open(backend.clone(), seed::context(), seed::scope()).await {
                Ok(h) => break h,
                Err(Error::Busy) => tokio::task::yield_now().await,
                Err(e) => panic!("worker open: {e:?}"),
            }
        };
        let head = loop {
            match hub.head(seed::context()).await {
                Ok(h) => break h,
                Err(Error::Busy) => tokio::task::yield_now().await,
                Err(e) => panic!("worker head: {e:?}"),
            }
        };
        let request = seed::seed(seed::scope(), head, &id);
        if mode == "kill" {
            backend.set_hook(move |point| {
                if point == stage {
                    std::fs::write(&marker, point).unwrap();
                    loop {
                        std::thread::park();
                    }
                }
                Ok(())
            });
        } else {
            std::fs::write(marker, b"ready").unwrap();
            let started = Instant::now();
            while !gate.exists() {
                assert!(started.elapsed() < Duration::from_secs(15));
                tokio::task::yield_now().await;
            }
        }
        let outcome = hub.commit(seed::context(), request).await;
        let text = match outcome {
            Ok(r) => format!("committed:{}", r.head.0),
            Err(Error::Busy) => "busy".into(),
            Err(Error::Conflict { .. }) => "conflict".into(),
            other => panic!("unexpected worker outcome: {other:?}"),
        };
        std::fs::write(output, text).unwrap();
        hub.drain().await;
    });
}

#[test]
fn sigkill_at_each_publication_boundary_recovers_only_complete_states() {
    for stage in STAGES {
        let s = Sandbox::new();
        let rt = runtime();
        let a = rt.block_on(async {
            let h = s.create().await;
            let a = h.head(seed::context()).await.unwrap();
            h.drain().await;
            a
        });
        let request = seed::seed(seed::scope(), a.clone(), "worker");
        let mut child = start(&s, "kill", stage, "worker");
        wait_file(&ready(&s, "worker"), &mut child);
        // A separate process cannot enter the branch while the worker owns publication.
        rt.block_on(async {
            let backend = s.backend();
            assert!(matches!(
                KnowledgeHub::open(backend, seed::context(), seed::scope()).await,
                Err(Error::Busy)
            ));
        });
        child.kill().unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(!output.status.success());
        rt.block_on(async {
            let h = s.open().await.unwrap();
            let current = h.head(seed::context()).await.unwrap();
            let outcome = h
                .operation_result(seed::context(), "worker".into())
                .await
                .unwrap();
            let published = ["head-replaced", "head-synced", "acknowledgement"].contains(&stage);
            match outcome {
                OperationOutcome::Committed(r) => {
                    assert!(published, "unexpected commitment at {stage}");
                    assert_eq!(current, r.head);
                    let snapshot = h
                        .read_snapshot(seed::context(), r.head.clone())
                        .await
                        .unwrap();
                    assert_eq!(snapshot.sequence, 1);
                    assert!(snapshot.seed_complete());
                    assert_eq!(snapshot.sources[0].content.bytes, seed::ONTOLOGY);
                    assert_eq!(
                        h.commit(seed::context(), request.clone()).await.unwrap(),
                        *r
                    );
                }
                OperationOutcome::Absent { recovered_head } => {
                    assert!(!published, "missing commitment at {stage}");
                    assert_eq!(recovered_head, a);
                    assert_eq!(current, a);
                }
            }
            assert_eq!(
                h.read_snapshot(seed::context(), a.clone())
                    .await
                    .unwrap()
                    .sequence,
                0
            );
            h.drain().await;
        });
        println!(
            "SIGKILL {stage}: complete {} snapshot and authoritative original outcome",
            if ["head-replaced", "head-synced", "acknowledgement"].contains(&stage) {
                "new"
            } else {
                "prior"
            }
        );
    }
}

#[test]
fn two_process_same_base_race_has_one_winner() {
    let s = Sandbox::new();
    let rt = runtime();
    let a = rt.block_on(async {
        let h = s.create().await;
        h.head(seed::context()).await.unwrap()
    });
    let mut x = start(&s, "race", "", "x");
    let mut y = start(&s, "race", "", "y");
    wait_file(&ready(&s, "x"), &mut x);
    wait_file(&ready(&s, "y"), &mut y);
    std::fs::write(s.parent.join("go"), b"go").unwrap();
    let xout = x.wait_with_output().unwrap();
    let yout = y.wait_with_output().unwrap();
    assert!(
        xout.status.success(),
        "{}",
        String::from_utf8_lossy(&xout.stderr)
    );
    assert!(
        yout.status.success(),
        "{}",
        String::from_utf8_lossy(&yout.stderr)
    );
    let rx = std::fs::read_to_string(result(&s, "x")).unwrap();
    let ry = std::fs::read_to_string(result(&s, "y")).unwrap();
    assert_ne!(rx.starts_with("committed:"), ry.starts_with("committed:"));
    let (loser, outcome) = if rx.starts_with("committed:") {
        ("y", &ry)
    } else {
        ("x", &rx)
    };
    assert!(outcome == "busy" || outcome == "conflict");
    rt.block_on(async {
        let h = s.open().await.unwrap();
        assert!(matches!(
            h.commit(seed::context(), seed::seed(seed::scope(), a, loser))
                .await,
            Err(Error::Conflict { .. })
        ));
        let snapshot = h
            .read_snapshot(seed::context(), h.head(seed::context()).await.unwrap())
            .await
            .unwrap();
        assert_eq!(snapshot.sequence, 1);
        assert_eq!(snapshot.assertions.len(), 1);
    });
    println!("process race: {rx}; {ry}; later loser admission gives stale conflict");
}

#[tokio::test]
async fn injected_persistence_failures_distinguish_before_and_after_replacement() {
    for stage in STAGES {
        let s = Sandbox::new();
        let created = s.create().await;
        let a = created.head(seed::context()).await.unwrap();
        let backend = s.backend();
        let h = KnowledgeHub::open(backend.clone(), seed::context(), seed::scope())
            .await
            .unwrap();
        backend.set_hook(move |point| {
            if point == stage {
                Err(Error::Storage {
                    stage: format!("injected {stage}"),
                })
            } else {
                Ok(())
            }
        });
        let request = seed::seed(seed::scope(), a.clone(), "fault");
        let error = h
            .commit(seed::context(), request.clone())
            .await
            .unwrap_err();
        let after = ["head-replaced", "head-synced", "acknowledgement"].contains(&stage);
        if after {
            assert!(matches!(error, Error::Uncertain { .. }));
        } else {
            assert!(matches!(error, Error::Storage { .. }));
        }
        let recovered = s.open().await.unwrap();
        let outcome = recovered
            .operation_result(seed::context(), "fault".into())
            .await
            .unwrap();
        if after {
            let OperationOutcome::Committed(receipt) = outcome else {
                panic!("lost original receipt")
            };
            assert_eq!(
                recovered.commit(seed::context(), request).await.unwrap(),
                *receipt
            );
        } else {
            assert_eq!(outcome, OperationOutcome::Absent { recovered_head: a });
        }
    }
}

#[tokio::test(flavor = "current_thread")]
async fn cancelled_publication_keeps_lock_and_data_until_drain() {
    let s = Sandbox::new();
    let created = s.create().await;
    let a = created.head(seed::context()).await.unwrap();
    let backend = s.backend();
    let h = KnowledgeHub::open(backend.clone(), seed::context(), seed::scope())
        .await
        .unwrap();
    let (started, rx) = std::sync::mpsc::channel();
    let (release, wait) = std::sync::mpsc::channel();
    let wait = std::sync::Mutex::new(wait);
    backend.set_hook(move |stage| {
        if stage == "head-replaced" {
            started.send(()).unwrap();
            wait.lock().unwrap().recv().unwrap();
        }
        Ok(())
    });
    let request = seed::seed(seed::scope(), a, "cancelled");
    let retry = request.clone();
    let handle = h.clone();
    let waiting = tokio::spawn(async move { handle.commit(seed::context(), request).await });
    while rx.try_recv().is_err() {
        tokio::task::yield_now().await;
    }
    waiting.abort();
    assert!(matches!(s.open().await, Err(Error::Busy)));
    let handle = h.clone();
    let draining = tokio::spawn(async move { handle.drain().await });
    tokio::task::yield_now().await;
    assert!(!draining.is_finished());
    release.send(()).unwrap();
    draining.await.unwrap();
    let h = s.open().await.unwrap();
    let OperationOutcome::Committed(receipt) = h
        .operation_result(seed::context(), "cancelled".into())
        .await
        .unwrap()
    else {
        panic!("publication lost")
    };
    assert_eq!(h.commit(seed::context(), retry).await.unwrap(), *receipt);
}

#[tokio::test]
async fn missing_old_history_and_accepted_raw_content_prevent_absence_claims() {
    for old_history in [true, false] {
        let s = Sandbox::new();
        let h = s.create().await;
        let a = h.head(seed::context()).await.unwrap();
        let b = h
            .commit(
                seed::context(),
                seed::seed(seed::scope(), a.clone(), "seed"),
            )
            .await
            .unwrap();
        if old_history {
            std::fs::remove_file(s.store.join("generations").join(&a.0).join("manifest.json"))
                .unwrap();
        } else {
            let dir = s.store.join("generations").join(&b.head.0).join("sources");
            let file = std::fs::read_dir(dir)
                .unwrap()
                .next()
                .unwrap()
                .unwrap()
                .path();
            std::fs::remove_file(file).unwrap();
        }
        assert!(matches!(s.open().await, Err(Error::Integrity(_))));
        assert!(matches!(
            h.operation_result(seed::context(), "never".into()).await,
            Err(Error::Integrity(_))
        ));
    }
}
