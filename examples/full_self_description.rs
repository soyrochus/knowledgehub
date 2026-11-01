//! Public files/2 consumer. Prepare exact host input with capture_self_model.py first.
mod file_format;
#[path = "self_model_support/mod.rs"]
mod host;
#[path = "support/mod.rs"]
mod seed;
#[path = "ontology_support/mod.rs"]
mod specimens;
use knowledgehub::{ontology::*, *};
use std::path::PathBuf;
async fn submit(
    h: &OntologyHub,
    id: &str,
    operations: Vec<knowledgehub::ontology::Operation>,
) -> Result<Receipt, Error> {
    let (head, state) = h.snapshot(seed::context()).await?;
    let mut c = Change::at(&state, head, id);
    c.operations = operations;
    eprintln!("Assessing {id}: {} ordered operations", c.operations.len());
    let report = h.validate(seed::context(), c.clone()).await?;
    if !report.is_valid() {
        return Err(Error::Invalid(report.violations));
    }
    let receipt = h.commit(seed::context(), c).await?;
    eprintln!("Committed {id}: sequence {}", receipt.sequence);
    Ok(receipt)
}
async fn baseline(h: &OntologyHub, name: &str) -> Result<ExactRef, Error> {
    let (_, state) = h.snapshot(seed::context()).await?;
    let reference = host::r(&format!("kh:baseline:{name}"));
    let r = Record::new(
        reference.clone(),
        type_ref("ModelBaseline"),
        RecordData::Baseline(state.baseline_data()?),
        vec![host::file_anchor_ref("design/domain-analysis.md")],
    )?;
    submit(
        h,
        &format!("baseline:{name}"),
        vec![knowledgehub::ontology::Operation::Put(r)],
    )
    .await?;
    Ok(reference)
}
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = file_format::Arguments::parse()?;
    let mut args = options.positional.iter().cloned();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let input = PathBuf::from(args.next().unwrap_or("target/self-model-input.json".into()));
    let input_bytes = std::fs::read(input)?;
    let capture: host::Capture = serde_json::from_slice(&input_bytes)?;
    if options.validate_input {
        let catalogue = builtin_catalogue()?;
        let mut state = State::empty(host::scope());
        state.ontology = Some(ontology_ref());
        state.profile = Some(profile_ref());
        for record in catalogue.records {
            state
                .selected
                .insert(record.reference.identity(), record.reference.clone());
            state.records.insert(record.reference.key(), record);
        }
        state.artifacts = catalogue.artifacts;
        let authored = host::author(&root, &capture, state, input_bytes)?;
        let report = assess(&authored.state);
        if !report.is_valid() {
            return Err(Error::Invalid(report.violations).into());
        }
        println!(
            "{}",
            serde_json::json!({"validated_origins":capture.files.len(),"units":capture.units.len(),"records":authored.state.selected.len(),"gaps":report.gaps.len()})
        );
        return Ok(());
    }
    let store = PathBuf::from(args.next().ok_or(
        "usage: full_self_description [--format files3] INPUT_JSON NEW_STORE [--result OUTPUT]",
    )?);
    if args.next().is_some() {
        return Err("unexpected positional argument".into());
    }
    let checked=capture.checked.as_ref().ok_or("A checked source/run manifest is required; prepare it with scripts/verify_full_ontology.py and pass --checked-manifest to capture_self_model.py")?;
    let run_start = checked
        .get("started_at")
        .and_then(serde_json::Value::as_str)
        .ok_or("checked run has no actual start time")?;

    let backend = options.format.backend(&store)?;
    let hub = OntologyHub::create(backend, seed::context(), host::scope()).await?;
    submit(
        &hub,
        "register-catalogue",
        builtin_catalogue()?.operations(),
    )
    .await?;
    let (_, state) = hub.snapshot(seed::context()).await?;
    let authored = host::author(&root, &capture, state, input_bytes)?;
    let (operations, groups) = authored.finish();
    let loaded = submit(&hub, "load-self-definition", operations).await?;
    let definitions = baseline(&hub, "checked-definitions").await?;
    // An actual log remains tied to its declared build/source inventory. Historical logs are explicitly labelled.
    let (_, state) = hub.snapshot(seed::context()).await?;
    let mut b = host::Builder::new(state);
    let source_anchor = host::file_anchor_ref(&capture.log);
    let log = b
        .sources
        .get(&capture.log)
        .cloned()
        .unwrap_or_else(|| host::capture_ref(&capture.log));
    let build = host::capture_ref("host-inventory.json");
    let run = b.make(
        "verification",
        "TestRun",
        "kh:run:captured-native-conformance",
        if capture.checked.is_some() {
            "Observed native conformance run for captured checked code"
        } else {
            "Historical observed log; current-code verification is not claimed"
        },
        source_anchor.clone(),
        host::properties([
            ("baseline", Value::Ref(definitions.clone())),
            ("build", Value::Artifact(build)),
            (
                "environment",
                Value::Ref(host::r("kh:environment:verified-host")),
            ),
            ("started_at", Value::text(run_start)),
            (
                "result",
                Value::enumeration(
                    if b.state.artifact(&log).is_some_and(|c| {
                        String::from_utf8_lossy(&c.bytes).contains("test result: ok")
                    }) {
                        "passed"
                    } else {
                        "inconclusive"
                    },
                ),
            ),
            (
                "observations",
                Value::List(vec![b.narrative(&source_anchor)]),
            ),
            ("evidence", host::artifact_list([log.clone()])),
        ]),
    )?;
    b.edge(
        "kh:assertion:run-executes",
        "TestRun",
        "executes",
        run.clone(),
        host::r("kh:test:kill-after-head-replacement"),
        source_anchor.clone(),
    )?;
    let evidence = b.make(
        "verification",
        "EvidenceArtifact",
        "kh:evidence:native-conformance-log",
        "Exact original native test log",
        source_anchor.clone(),
        host::properties([
            ("artifact", Value::Artifact(log)),
            ("observed_scope", host::list([definitions.clone(), run])),
            ("observation_kind", Value::enumeration("validation-result")),
            ("captured_at", Value::text(&capture.captured_at)),
        ]),
    )?;
    b.edge(
        "kh:assertion:evidence-criterion",
        "EvidenceArtifact",
        "supportsCriterion",
        evidence,
        host::r("kh:criterion:recover-original-receipt"),
        source_anchor,
    )?;
    let (mut ops, _) = b.finish();
    ops.push(knowledgehub::ontology::Operation::SelectProfile {
        ontology: ontology_ref(),
        profile: self_profile_ref(),
    });
    submit(&hub, "load-observed-evidence", ops).await?;
    let a = baseline(&hub, "A").await?;
    let (head, state) = hub.snapshot(seed::context()).await?;
    let original = host::r("urn:requirement:atomic-change");
    let mut next = state.get(&original).ok_or(Error::NotFound)?.clone();
    next.reference.revision = "self-2".into();
    next.predecessor = Some(original.clone());
    let mut authored = host::Builder::new(state.clone());
    let text=b"# Atomic governed change\n\nThe library checks current authority and exact expected revisions before publishing the complete typed graph, original artifacts, selected baseline and original operation receipt. An uncertain caller resolves that original operation after recovery. This revised explanation is authored for this walkthrough; it does not change historical test outcomes or record human acceptance.\n".to_vec();
    let doc = authored.capture_successor("urn:document:knowledgehub-seed", text.clone())?;
    let fragment =
        authored.anchor_successor("kh:anchor:atomic-explanation", doc, text.len() as u64)?;
    next.provenance.push(fragment.clone());
    specimens::put_property(&mut next, "statement", authored.narrative(&fragment));
    let (mut operations, _) = authored.finish();
    operations.extend(specimens::successor_operations(&state, vec![next.clone()])?);
    let mut revised = Change::at(&state, head, "revise-explanation");
    revised.operations = operations;
    let report = hub.validate(seed::context(), revised.clone()).await?;
    if !report.is_valid() {
        return Err(format!("successor: {:?}", report.violations).into());
    }
    let revised_receipt = hub.commit(seed::context(), revised.clone()).await?;
    let b = baseline(&hub, "B").await?;
    let differences = hub.compare(seed::context(), a.clone(), b.clone()).await?;
    hub.drain().await;
    drop(hub);
    let reopened = OntologyHub::open(
        options.format.backend(&store)?,
        seed::context(),
        host::scope(),
    )
    .await?;
    let view = reopened
        .read_state(seed::context(), Selection::Baseline(a.clone()))
        .await?;
    assert!(catalogue_coverage(&view).is_empty());
    assert_eq!(
        view.current(&original.model, &original.id)
            .unwrap()
            .reference,
        original
    );
    assert_eq!(
        reopened.commit(seed::context(), revised).await?,
        revised_receipt
    );
    assert_eq!(
        reopened
            .operation_result(seed::context(), "load-self-definition".into())
            .await?,
        OperationOutcome::Committed(Box::new(loaded))
    );
    assert_eq!(
        reopened
            .compare(seed::context(), a.clone(), b.clone())
            .await?,
        differences
    );
    let view_b = reopened
        .read_state(seed::context(), Selection::Baseline(b.clone()))
        .await?;
    let old_doc = match view
        .get(&original)
        .unwrap()
        .element()
        .unwrap()
        .get("statement")
        .unwrap()
    {
        Value::Narrative { document, .. } => document,
        _ => panic!("old narrative"),
    };
    let new_doc = match view_b
        .get(&next.reference)
        .unwrap()
        .element()
        .unwrap()
        .get("statement")
        .unwrap()
    {
        Value::Narrative { document, .. } => document,
        _ => panic!("new narrative"),
    };
    assert_ne!(old_doc, new_doc);
    assert_ne!(
        view.artifact(old_doc).unwrap().bytes,
        view_b.artifact(new_doc).unwrap().bytes
    );
    let answers = [
        (
            "Q02",
            vec![
                "urn:knowledgehub:library",
                "kh:actor:maintainer",
                "kh:actor:host",
                "kh:actor:reader",
            ],
        ),
        (
            "Q03",
            vec![
                "kh:command:commit-change-set",
                "urn:use-case:commit-change-set",
                "kh:function:admit-change",
            ],
        ),
        (
            "Q04",
            vec![
                "kh:aggregate:model-branch",
                "kh:invariant:atomic-publication",
                "kh:decision:KH-D-04",
            ],
        ),
        (
            "Q05",
            vec![
                "kh:representation:admission-code",
                "kh:module:ontology-application",
                "kh:module:storage",
            ],
        ),
    ]
    .into_iter()
    .map(|(question, ids)| {
        Ok((
            question,
            ids.into_iter()
                .map(|id| view.resolve(&host::r(id)).ok_or(Error::NotFound))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    })
    .collect::<Result<std::collections::BTreeMap<_, _>, Error>>()?;
    let mut q = Query::select(Selection::Baseline(a.clone()));
    q.starts = vec![original.clone()];
    q.direction = Direction::Reverse;
    q.signatures = vec![
        relationship_ref("UseCase", "satisfies"),
        relationship_ref("TestCase", "checks"),
        relationship_ref("TestRun", "executes"),
    ];
    q.depth = 3;
    q.page_size = 1000;
    let path = reopened.query(seed::context(), q).await?;
    assert!(
        path.records
            .iter()
            .any(|r| r.reference == host::r("kh:run:captured-native-conformance"))
    );
    let assertions=view.selected_records().filter_map(|record|if let RecordData::Assertion(data)=&record.data {Some(serde_json::json!({"reference":record.reference,"signature":data.signature,"from":data.from,"to":data.to}))}else{None}).collect::<Vec<_>>();
    let assessment_count = view
        .selected_records()
        .filter(|record| {
            matches!(record.data, RecordData::Review(_))
                || record.type_ref == type_ref("AcceptanceRecord")
        })
        .count();
    assert_eq!(assessment_count, 0, "no human assessment is invented");
    let mut originals = std::collections::BTreeMap::new();
    for file in &capture.files {
        let reference = match file.path.as_str() {
            "source/system-definition-ontology-proposal.md" => {
                ExactRef::new("sdo", "urn:source:ontology:capture", "catalogue-1")
            }
            "source/system-definition-ontology-proposal.pdf" => {
                ExactRef::new("sdo", "sdo:source:ontology-pdf:capture", "catalogue-1")
            }
            _ => host::capture_ref(&file.path),
        };
        let actual = view.artifact(&reference).ok_or(Error::NotFound)?;
        assert_eq!(ContentRef::of(&actual.bytes), file.reference);
        originals.insert(file.path.clone(), file.reference.clone());
    }
    let summary = serde_json::json!({"store":store,"source_status":capture.source_status,"checked_current_code":capture.checked.is_some(),"groups":groups,"assertions":assertions,"source_originals":originals,"actual_human_assessments":assessment_count,"answers":answers,"structurally_valid":assess(&view).is_valid(),"purpose_complete":assess(&view).is_complete(),"Q01":{"types":145,"signatures":38,"missing":catalogue_coverage(&view)},"Q02":host::r("urn:knowledgehub:library"),"Q03":host::r("kh:command:commit-change-set"),"Q04":host::r("kh:aggregate:model-branch"),"Q05":host::r("kh:representation:admission-code"),"Q06":{"records":path.records.iter().map(|r|&r.reference).collect::<Vec<_>>(),"assertions":path.assertions.iter().map(|r|&r.reference).collect::<Vec<_>>()},"Q07":{"A":a,"B":b,"changed":differences.iter().map(|d|&d.identity).collect::<Vec<_>>()},"Q08":{"gaps":assess(&view).gaps,"inapplicable":view.profile_definition().unwrap().inapplicable,"human_acceptance":"absent; repository authority retained"},"selected_records":view.selected.len(),"retained_records":view.records.len()});
    let output = options.result.unwrap_or_else(|| {
        store
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("self-model-result.json")
    });
    std::fs::write(&output, serde_json::to_vec_pretty(&summary)?)?;
    println!(
        "{}",
        serde_json::to_string(
            &serde_json::json!({"result":output,"selected_records":view.selected.len(),"retained_records":view.records.len(),"explicit_gaps":assess(&view).gaps.len(),"Q01_Q08":"resolved through public API; acceptance absent"})
        )?
    );
    reopened.drain().await;
    Ok(())
}
