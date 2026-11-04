use super::*;
#[path = "../../examples/history_support/mod.rs"]
mod fixture;
#[path = "../../examples/support/mod.rs"]
mod seed;
use std::{
    fs,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Test {
    dir: PathBuf,
    file: PathBuf,
    scope: ModelScope,
    fixture: fixture::Fixture,
}
impl Test {
    async fn new() -> Self {
        let dir = std::env::temp_dir().canonicalize().unwrap().join(format!(
            "kh-client-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        let scope = seed::scope();
        let h = KnowledgeHub::create(
            FileBackend::new(dir.join("hub"), FileOptions::native_local().unwrap()).unwrap(),
            seed::context(),
            scope.clone(),
        )
        .await
        .unwrap();
        let fixture = fixture::populate(&h, seed::context(), "terminal-test", 2)
            .await
            .unwrap();
        h.drain().await;
        let file = dir.join("workspace.json");
        let result = Self {
            dir,
            file,
            scope,
            fixture,
        };
        result.grants(&["Open", "Read", "ReadHistory"]);
        result
    }
    fn grants(&self, grants: &[&str]) {
        fs::write(&self.file,serde_json::to_vec(&json!({"format":"knowledgehub-client-workspace/1","default":"test","models":[{"alias":"test","name":"Disposable test","directory":"hub","scope":self.scope,"actor":"host:reader-B","grants":grants,"known_baselines":self.fixture.baselines}]})).unwrap()).unwrap();
    }
    async fn call(
        &self,
        s: &mut BTreeMap<String, Session>,
        method: &str,
        args: Json,
    ) -> Result<Json, Error> {
        dispatch(
            &self.file,
            s,
            &wire::Request {
                id: "t".into(),
                method: method.into(),
                model: None,
                args,
                result_bytes: None,
            },
        )
        .await
    }
    fn anchor(&self) -> SnapshotId {
        self.fixture.receipts.last().unwrap().head.clone()
    }
}
impl Drop for Test {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.dir).unwrap();
    }
}
async fn drain(s: BTreeMap<String, Session>) {
    for (_, s) in s {
        s.hub.drain().await;
    }
}
#[tokio::test]
async fn open_exact_scope_history_and_original_sources_without_mutation() {
    let t = Test::new().await;
    let mut s = BTreeMap::new();
    let info = t.call(&mut s, "info", json!({})).await.unwrap();
    assert_eq!(info["head"], t.anchor().0);
    assert_eq!(info["reader"], "host:reader-B");
    let hs = t
        .call(&mut s, "handshake", json!({"protocol":wire::PROTOCOL}))
        .await
        .unwrap();
    assert_eq!(hs["build"]["history_projection"], HISTORY_FORMAT);
    for m in [
        "publication",
        "publications",
        "publication_affected",
        "continue_history",
        "release_history",
    ] {
        assert!(hs["methods"].as_array().unwrap().contains(&json!(m)));
    }
    for receipt in &t.fixture.receipts {
        let q = Publication::new(t.anchor(), receipt.operation_id.clone());
        let got = t
            .call(
                &mut s,
                "publication",
                json!({"request":wire::owned(&q).unwrap()}),
            )
            .await
            .unwrap();
        assert_eq!(got["outcome"]["Published"]["actor"], receipt.actor);
        assert_eq!(
            got["outcome"]["Published"]["sequence"],
            receipt.sequence.to_string()
        );
        assert_eq!(
            got["outcome"]["Published"]["fingerprint"],
            receipt.fingerprint
        );
    }
    let result=t.call(&mut s,"artifact_range",json!({"selection":Selection::Baseline(t.fixture.baselines[0].clone()),"reference":t.fixture.sources[0],"offset":"0","length":t.fixture.bytes[0].len().to_string()})).await.unwrap();
    assert_eq!(result["bytes_base64"], wire::base64(&t.fixture.bytes[0]));
    assert_eq!(
        result["original"]["length"],
        t.fixture.bytes[0].len().to_string()
    );
    for method in [
        "commit",
        "validate",
        "operation_result",
        "create",
        "register",
        "snapshot",
        "read_state",
    ] {
        assert!(matches!(
            t.call(&mut s, method, json!({})).await,
            Err(Error::InvalidInput(_))
        ));
    }
    assert_eq!(
        t.call(&mut s, "head", json!({})).await.unwrap()["head"],
        info["head"]
    );
    drain(s).await;
}
#[tokio::test]
async fn scoped_grants_revocation_and_authority_overrides_fail_closed() {
    let t = Test::new().await;
    let mut s = BTreeMap::new();
    t.grants(&["Open", "Read"]);
    t.call(&mut s, "info", json!({})).await.unwrap();
    let q = wire::owned(&Publications::new(t.anchor())).unwrap();
    assert!(matches!(
        t.call(&mut s, "publications", json!({"request":q})).await,
        Err(Error::Denied {
            action: Action::ReadHistory
        })
    ));
    for override_ in ["actor", "directory", "scope", "grants"] {
        assert!(matches!(
            t.call(&mut s, "read", json!({override_:"override"})).await,
            Err(Error::InvalidInput(_))
        ));
    }
    t.grants(&["Open", "Read", "ReadHistory"]);
    t.call(
        &mut s,
        "publications",
        json!({"request":wire::owned(&Publications::new(t.anchor())).unwrap()}),
    )
    .await
    .unwrap();
    t.grants(&["Open", "Read"]);
    assert!(matches!(
        t.call(&mut s, "authorize", json!({"history":true})).await,
        Err(Error::Denied {
            action: Action::ReadHistory
        })
    ));
    t.call(&mut s, "head", json!({})).await.unwrap();
    t.grants(&["Open"]);
    assert!(matches!(
        t.call(&mut s, "head", json!({})).await,
        Err(Error::Denied {
            action: Action::Read
        })
    ));
    t.grants(&["Open", "Read", "Commit"]);
    assert!(workspace::load(&t.file).is_err());
    drain(s).await;
}
#[tokio::test]
async fn history_absence_unavailable_limits_intent_and_cursor_capacity() {
    let t = Test::new().await;
    let mut s = BTreeMap::new();
    let q = Publication::new(
        t.fixture.initial.clone(),
        t.fixture.receipts[1].operation_id.clone(),
    );
    assert_eq!(
        t.call(
            &mut s,
            "publication",
            json!({"request":wire::owned(&q).unwrap()})
        )
        .await
        .unwrap()["outcome"],
        "NotPublishedAt"
    );
    assert!(matches!(t.call(&mut s,"publication",json!({"request":wire::owned(&Publication::new(SnapshotId("a".repeat(64)),"unknown")).unwrap()})).await,Err(Error::HistoryAnchorUnavailable{..})));
    let mut q = Publications::new(t.anchor());
    q.limits.nodes = 1;
    assert!(matches!(
        t.call(
            &mut s,
            "publications",
            json!({"request":wire::owned(&q).unwrap()})
        )
        .await,
        Err(Error::HistoryLimit { .. })
    ));
    q.limits = HistoryLimits::default();
    q.limits.items = 1;
    let mut tokens = vec![];
    for _ in 0..64 {
        let p = t
            .call(
                &mut s,
                "publications",
                json!({"request":wire::owned(&q).unwrap()}),
            )
            .await
            .unwrap();
        tokens.push(p["continuation"].as_str().unwrap().to_owned());
    }
    assert!(matches!(
        t.call(
            &mut s,
            "publications",
            json!({"request":wire::owned(&q).unwrap()})
        )
        .await,
        Err(Error::Capacity)
    ));
    let mut changed = q.clone();
    changed.limits.items = 2;
    assert!(matches!(t.call(&mut s,"continue_history",json!({"request":wire::owned(&HistoryRequest::Publications(changed)).unwrap(),"continuation":tokens[0]})).await,Err(Error::InvalidCursor)));
    for token in tokens {
        t.call(&mut s, "release_history", json!({"continuation":token}))
            .await
            .unwrap();
    }
    let small = wire::Request {
        id: "small".into(),
        method: "publications".into(),
        model: None,
        args: json!({"request":wire::owned(&q).unwrap()}),
        result_bytes: Some("256".into()),
    };
    let oversized = dispatch(&t.file, &mut s, &small).await.unwrap();
    assert!(!wire::fits(&small, &oversized).unwrap());
    assert!(matches!(t.call(&mut s,"continue_history",json!({"request":wire::owned(&HistoryRequest::Publications(q.clone())).unwrap(),"continuation":oversized["continuation"]})).await,Err(Error::InvalidCursor)));
    let fresh = t
        .call(
            &mut s,
            "publications",
            json!({"request":wire::owned(&q).unwrap()}),
        )
        .await
        .unwrap();
    assert_eq!(
        fresh["items"]["Publications"][0]["operation_id"],
        t.fixture.receipts.last().unwrap().operation_id
    );
    drain(s).await;
}
#[tokio::test]
async fn workspace_paths_wrong_scope_baselines_and_wire_overhead() {
    let t = Test::new().await;
    let original: Json = serde_json::from_slice(&fs::read(&t.file).unwrap()).unwrap();
    for (field, value) in [
        ("directory", json!("../hub")),
        (
            "scope",
            json!({"hub":"wrong","model":"wrong","branch":"main","owner":"wrong"}),
        ),
        (
            "known_baselines",
            json!([t.fixture.definitions[0].reference]),
        ),
    ] {
        let mut w = original.clone();
        w["models"][0][field] = value;
        fs::write(&t.file, serde_json::to_vec(&w).unwrap()).unwrap();
        let mut s = BTreeMap::new();
        assert!(t.call(&mut s, "info", json!({})).await.is_err());
        drain(s).await;
    }
    let mut w = original.clone();
    w["models"][0]["directory"] = json!("missing");
    fs::write(&t.file, serde_json::to_vec(&w).unwrap()).unwrap();
    assert!(
        t.call(&mut BTreeMap::new(), "info", json!({}))
            .await
            .is_err()
    );
    fs::write(&t.file, serde_json::to_vec(&original).unwrap()).unwrap();
    let mut bytes = vec![];
    let r = wire::Request {
        id: "test".into(),
        method: "read".into(),
        model: None,
        args: json!({"reference":t.fixture.definitions[0].reference}),
        result_bytes: Some("256".into()),
    };
    wire::write(&mut bytes, &r, Ok(json!({"text":"x".repeat(300)}))).unwrap();
    let response: Json = serde_json::from_slice(&bytes[4..]).unwrap();
    assert_eq!(response["error"]["code"], "Capacity");
    assert_eq!(
        u32::from_be_bytes(bytes[..4].try_into().unwrap()) as usize,
        bytes.len() - 4
    );
}
