#[path = "../examples/support/mod.rs"]
mod seed;
mod support;
use serde_json::{Value, json};
use std::{fs, path::Path, process::Command};
fn write(p: &Path, v: Value) {
    fs::write(p, serde_json::to_vec_pretty(&v).unwrap()).unwrap();
}
fn command(args: &[&Path], verb: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_knowledgehub-model"))
        .arg(verb)
        .args(args)
        .output()
        .unwrap()
}
fn pass(out: std::process::Output) -> Value {
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).unwrap()
}
fn setup(
    s: &support::Sandbox,
) -> (
    std::path::PathBuf,
    std::path::PathBuf,
    std::path::PathBuf,
    std::path::PathBuf,
    std::path::PathBuf,
) {
    setup_with_limits(s, None)
}
fn setup_with_limits(
    s: &support::Sandbox,
    limits: Option<Value>,
) -> (
    std::path::PathBuf,
    std::path::PathBuf,
    std::path::PathBuf,
    std::path::PathBuf,
    std::path::PathBuf,
) {
    let source = s.parent.join("source");
    fs::create_dir(&source).unwrap();
    fs::write(source.join("design.md"),"A borrower reserves an available asset. Interrupted requests preserve the original booking intent. This is a synthetic design, not a recorded execution.").unwrap();
    let work = s.parent.join("run");
    let config = s.parent.join("config.json");
    let host = s.parent.join("host.json");
    let plan = s.parent.join("plan.json");
    let coverage = s.parent.join("coverage.json");
    let grants = json!([
        "Create",
        "Open",
        "Read",
        "Validate",
        "Commit",
        "RegisterOntology",
        "OperationResult"
    ]);
    write(&host, json!({"actor":"host:analyst","grants":grants}));
    write(
        &config,
        json!({"format":"knowledgehub-authoring/1","source":source,"destination":s.store,"work":work,"actor":"host:analyst","grants":grants,"scope":{"hub":"fixture:hub","model":"fixture:model","branch":"main","owner":"fixture:owner"},"authority":{"field_owners":{"requirements":"design"},"precedence":["design"],"unresolved_conflicts":[],"review_status":"agent-authored; human pending"},"inputs":[{"key":"design","path":"design.md","media_type":"text/markdown","authority":"normative-design","status":"authored-requirement","rendition_group":null,"dependencies":[],"exclude_reason":null,"expected":null}]}),
    );
    write(
        &plan,
        json!({"format":"knowledgehub-authoring/1","catalogue":knowledgehub::ontology::ontology_ref(),"profile":knowledgehub::ontology::profile_ref(),"baseline":"A","selection":null,"gaps":[],"records":[{"key":"requirement","type_ref":knowledgehub::ontology::type_ref("Requirement"),"capture":null,"provenance":["anchor:design"],"data":{"kind":"Element","data":{"properties":{"name":{"kind":"Text","value":"Reserve equipment"},"description":{"kind":"Narrative","value":{"document":{"$ref":"capture:design"},"fragment":{"$ref":"anchor:design"}}},"statement":{"kind":"Narrative","value":{"document":{"$ref":"capture:design"},"fragment":{"$ref":"anchor:design"}}},"rationale":{"kind":"List","value":[]},"claim_kind":{"kind":"Enum","value":"authored-requirement"}},"owners":[{"path":"/name","owner":"Graph"},{"path":"/description","owner":"Document"},{"path":"/statement","owner":"Document"},{"path":"/rationale","owner":"Graph"},{"path":"/claim_kind","owner":"Graph"}]}}}]}),
    );
    write(
        &coverage,
        json!({"format":"knowledgehub-authoring/1","interpretation_review":"agent-authored; human pending","source_gaps":[],"queries":[],"items":[{"id":"reserve-equipment","source":"design","disposition":"mapped","reason":null,"targets":[{"key":"requirement","path":"/data/data/properties/statement","expected":null,"anchor":"anchor:design"}]}]}),
    );
    if let Some(limits) = limits {
        let mut c: Value = serde_json::from_slice(&fs::read(&config).unwrap()).unwrap();
        c["limits"] = limits;
        write(&config, c);
    }
    pass(command(&[&config], "capture"));
    pass(command(&[&work, &plan, &coverage], "prepare"));
    (work, host, plan, coverage, config)
}

#[test]
fn verification_budgets_fail_explicitly_and_missing_dependency_stays_visible() {
    let s = support::Sandbox::new();
    let (work, host, _, _, _) = setup_with_limits(&s, Some(json!({"visited":4})));
    let out = command(&[&work, &host], "create");
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("visited limit"));
    // Publication is retained, rather than duplicated/reset after a bounded read fails.
    let journal: Value =
        serde_json::from_slice(&fs::read(work.join("journal.json")).unwrap()).unwrap();
    assert_eq!(journal["dispatches"].as_array().unwrap().len(), 2);
    pass(command(&[&work, &host], "resolve"));
    assert!(!command(&[&work, &host], "resume").status.success());

    let s = support::Sandbox::new();
    let (work, host, plan, coverage, config) = setup(&s);
    let mut c: Value = serde_json::from_slice(&fs::read(&config).unwrap()).unwrap();
    c["work"] = json!(s.parent.join("dependency-run"));
    c["inputs"][0]["dependencies"] = json!(["unavailable.css"]);
    write(&config, c);
    pass(command(&[&config], "capture"));
    let next_work = s.parent.join("dependency-run");
    pass(command(&[&next_work, &plan, &coverage], "prepare"));
    let report = pass(command(&[&next_work, &host], "create"));
    assert_eq!(report["native_validity"], true);
    assert_eq!(report["source_coverage"], false);
    assert!(
        report["failures"]
            .as_array()
            .unwrap()
            .iter()
            .any(|v| v.as_str().unwrap().contains("missing dependencies"))
    );
    assert!(work.exists());
}
#[test]
fn fresh_authoring_native_assessment_independent_verification_and_coverage_failure() {
    let s = support::Sandbox::new();
    let (work, host, _, _, _) = setup(&s);
    let assessed = pass(command(&[&work, &host], "assess"));
    assert_eq!(assessed["report"]["native_validity"], true);
    let report = pass(command(&[&work, &host], "create"));
    assert_eq!(report["status"], "verified");
    assert!(!command(&[&work, &host], "create").status.success());
    pass(command(&[&work, &host], "resolve"));
    assert_eq!(
        pass(command(&[&work, &host], "verify"))["status"],
        "verified"
    );
    // Verification is source independent: the original checkout is no longer needed.
    fs::rename(s.parent.join("source"), s.parent.join("source-unavailable")).unwrap();
    assert_eq!(
        pass(command(&[&work, &host], "verify"))["status"],
        "verified"
    );
    let s2 = support::Sandbox::new();
    let (work2, host2, _, coverage2, _) = setup(&s2);
    let mut coverage: Value = serde_json::from_slice(&fs::read(&coverage2).unwrap()).unwrap();
    coverage["items"][0]["targets"][0]["key"] = "missing-requirement".into();
    write(&coverage2, coverage);
    let plan2 = s2.parent.join("plan.json");
    pass(command(&[&work2, &plan2, &coverage2], "prepare"));
    let report = pass(command(&[&work2, &host2], "create"));
    assert_eq!(report["native_completeness"], true);
    assert_eq!(report["source_coverage"], false);
}
#[test]
fn interrupted_dispatch_reconciles_original_intent_and_current_authority() {
    let s = support::Sandbox::new();
    let (work, host, _, _, _) = setup(&s);
    let out = Command::new(env!("CARGO_BIN_EXE_knowledgehub-model"))
        .arg("create")
        .arg(&work)
        .arg(&host)
        .env("KNOWLEDGEHUB_MODEL_FAULT", "after-commit")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(86));
    write(
        &host,
        json!({"actor":"host:analyst","grants":["Open","Read"]}),
    );
    assert!(!command(&[&work, &host], "resolve").status.success());
    write(
        &host,
        json!({"actor":"host:analyst","grants":["Create","Open","Read","Validate","Commit","RegisterOntology","OperationResult"]}),
    );
    pass(command(&[&work, &host], "resolve"));
    let report = pass(command(&[&work, &host], "resume"));
    assert_eq!(report["status"], "verified");
    let journal: Value =
        serde_json::from_slice(&fs::read(work.join("journal.json")).unwrap()).unwrap();
    assert_eq!(journal["dispatches"].as_array().unwrap().len(), 2);
    fs::write(s.parent.join("source/design.md"), "Changed source").unwrap();
    assert!(!command(&[&work, &host], "resume").status.success());
}

#[test]
fn inventory_limits_escape_and_changed_plan_fail_before_publication() {
    for case in [
        "path",
        "overlap",
        "bytes",
        "total",
        "files",
        "obsolete",
        "symlink",
        "pages",
        "depth",
        "visited",
        "payload",
        "output-symlink",
        "output-traversal",
    ] {
        let s = support::Sandbox::new();
        let (_, _, _, _, config) = setup(&s);
        let mut c: Value = serde_json::from_slice(&fs::read(&config).unwrap()).unwrap();
        c["work"] = json!(s.parent.join("negative-run"));
        match case {
            "path" => c["inputs"][0]["path"] = json!("../design.md"),
            "overlap" => c["work"] = json!(s.parent.join("source/overlap")),
            "bytes" => c["limits"] = json!({"file_bytes":8}),
            "total" => {
                c["limits"] = json!({"file_bytes":200,"total_bytes":200});
                fs::write(s.parent.join("source/second.md"), "x".repeat(150)).unwrap();
                let mut input = c["inputs"][0].clone();
                input["key"] = json!("second");
                input["path"] = json!("second.md");
                c["inputs"].as_array_mut().unwrap().push(input);
            }
            "pages" => c["limits"] = json!({"page_size":1001}),
            "depth" => c["limits"] = json!({"depth":33}),
            "visited" => c["limits"] = json!({"visited":100001}),
            "payload" => c["limits"] = json!({"total_bytes":268435457}),
            "output-traversal" => c["work"] = json!(s.parent.join("source/../escaped-run")),
            "output-symlink" => {
                #[cfg(unix)]
                std::os::unix::fs::symlink(&s.parent, s.parent.join("alias")).unwrap();
                c["work"] = json!(s.parent.join("alias/escaped-run"));
            }
            "files" => {
                fs::write(s.parent.join("source/unaccounted.md"), "undeclared").unwrap();
            }
            "obsolete" => c["format"] = json!("knowledgehub-authoring/0"),
            "symlink" => {
                #[cfg(unix)]
                std::os::unix::fs::symlink(
                    s.parent.join("source/design.md"),
                    s.parent.join("source/link.md"),
                )
                .unwrap();
            }
            _ => unreachable!(),
        }
        write(&config, c);
        let out = command(&[&config], "capture");
        assert!(!out.status.success(), "case {case}");
        assert!(!s.store.exists(), "case {case} published a store");
    }
    let s = support::Sandbox::new();
    let (work, host, _, _, _) = setup(&s);
    fs::write(work.join("plan.json"), "{}").unwrap();
    assert!(!command(&[&work, &host], "create").status.success());
    assert!(!s.store.exists());
}

#[test]
fn cancelled_model_process_resumes_original_operation_without_duplicate_model() {
    let s = support::Sandbox::new();
    let (work, host, _, _, _) = setup(&s);
    let mut child = Command::new(env!("CARGO_BIN_EXE_knowledgehub-model"))
        .arg("create")
        .arg(&work)
        .arg(&host)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let started = std::time::Instant::now();
    loop {
        let checkpoint = fs::read(work.join("journal.json"))
            .ok()
            .and_then(|b| serde_json::from_slice::<Value>(&b).ok());
        if checkpoint
            .as_ref()
            .is_some_and(|j| j["dispatches"].as_array().is_some_and(|d| d.len() == 2))
        {
            child.kill().unwrap();
            child.wait().unwrap();
            break;
        }
        assert!(
            child.try_wait().unwrap().is_none(),
            "child completed before cancellation checkpoint"
        );
        assert!(
            started.elapsed().as_secs() < 60,
            "publication checkpoint timeout"
        );
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    pass(command(&[&work, &host], "resolve"));
    let report = pass(command(&[&work, &host], "resume"));
    assert_eq!(report["status"], "verified");
    let j: Value = serde_json::from_slice(&fs::read(work.join("journal.json")).unwrap()).unwrap();
    assert_eq!(j["dispatches"].as_array().unwrap().len(), 2);
}

#[test]
fn original_model_dispatch_recovers_at_each_process_checkpoint() {
    for stage in ["before-dispatch", "after-commit", "after-receipt"] {
        let s = support::Sandbox::new();
        let (work, host, _, _, _) = setup(&s);
        let out = Command::new(env!("CARGO_BIN_EXE_knowledgehub-model"))
            .arg("create")
            .arg(&work)
            .arg(&host)
            .env(
                "KNOWLEDGEHUB_MODEL_FAULT",
                format!("{stage}:authoring:model"),
            )
            .output()
            .unwrap();
        assert_eq!(
            out.status.code(),
            Some(86),
            "{stage}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        pass(command(&[&work, &host], "resolve"));
        let report = pass(command(&[&work, &host], "resume"));
        assert_eq!(report["status"], "verified", "{stage}");
        assert!(report["head"].is_string());
        let journal: Value =
            serde_json::from_slice(&fs::read(work.join("journal.json")).unwrap()).unwrap();
        assert_eq!(journal["dispatches"].as_array().unwrap().len(), 2);
        assert_eq!(
            journal["dispatches"][1]["request"]["operation_id"],
            "authoring:model"
        );
    }
}

#[test]
fn finite_query_pages_and_truncation_never_claim_full_coverage() {
    for limited_pages in [false, true] {
        let s = support::Sandbox::new();
        let limits = if limited_pages {
            json!({"pages":1})
        } else {
            json!({})
        };
        let (work, host, plan, coverage, _) = setup_with_limits(&s, Some(limits));
        let mut p: Value = serde_json::from_slice(&fs::read(&plan).unwrap()).unwrap();
        let mut second = p["records"][0].clone();
        second["key"] = json!("second-requirement");
        p["records"].as_array_mut().unwrap().push(second);
        write(&plan, p);
        let mut c: Value = serde_json::from_slice(&fs::read(&coverage).unwrap()).unwrap();
        let mut q = knowledgehub::Query::select(knowledgehub::Selection::Snapshot(
            knowledgehub::SnapshotId("replaced".into()),
        ));
        q.type_ref = Some(knowledgehub::ontology::type_ref("Requirement"));
        q.page_size = 1;
        if !limited_pages {
            q.subject_limit = 1;
        }
        c["queries"] = json!([{"name":"finite requirements","kind":"query","query":q,"expected_ids":["model:requirement","model:second-requirement"]}]);
        write(&coverage, c);
        pass(command(&[&work, &plan, &coverage], "prepare"));
        let report = pass(command(&[&work, &host], "create"));
        assert_eq!(report["native_validity"], true);
        assert_eq!(report["native_completeness"], true);
        assert_eq!(report["source_coverage"], false);
        assert_eq!(report["queries"][0]["exhausted"], false);
        assert!(!report["failures"].as_array().unwrap().is_empty());
    }
}
