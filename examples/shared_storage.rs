//! Files/3 walkthrough and explicit host maintenance. All fixtures are synthetic.
#[path = "support/mod.rs"]
pub(crate) mod seed;
#[path = "ontology_support/mod.rs"]
pub(crate) mod specimens;
use knowledgehub::{OperationOutcome, backends::files::*, ontology::*};
use std::path::Path;

async fn baseline(h: &OntologyHub, name: &str) -> Result<ExactRef, knowledgehub::Error> {
    let (head, state) = h.snapshot(seed::context()).await?;
    let r = ExactRef::new("demo", format!("baseline:{name}"), "1");
    let b = Record::new(
        r.clone(),
        type_ref("ModelBaseline"),
        RecordData::Baseline(state.baseline_data()?),
        vec![specimens::anchor()],
    )?;
    let mut c = Change::at(&state, head, format!("baseline:{name}"));
    c.operations.push(Operation::Put(b));
    h.commit(seed::context(), c).await?;
    Ok(r)
}
pub async fn walkthrough(path: &Path) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let backend = FileBackend::new_shared_ontology(path, FileOptions::local_ext4())?;
    let h = OntologyHub::create(backend.clone(), seed::context(), specimens::scope()).await?;
    let genesis = h.head(seed::context()).await?;
    for (id, operations) in [
        ("catalogue", builtin_catalogue()?.operations()),
        ("specimens", specimens::specimens()?),
    ] {
        let (head, state) = h.snapshot(seed::context()).await?;
        let mut c = Change::at(&state, head, id);
        c.operations = operations;
        h.commit(seed::context(), c).await?;
        eprintln!("shared_storage: committed {id}");
    }
    let a = baseline(&h, "A").await?;
    let (head, state) = h.snapshot(seed::context()).await?;
    let old = specimens::reference("Requirement");
    let original = state.get(&old).unwrap().clone();
    let mut next = original.clone();
    next.reference.revision = "2".into();
    next.predecessor = Some(old.clone());
    specimens::put_property(
        &mut next,
        "name",
        Value::text("A revised synthetic requirement"),
    );
    let mut c = Change::at(&state, head, "successor");
    c.operations = specimens::successor_operations(&state, vec![next.clone()])?;
    let receipt = h.commit(seed::context(), c.clone()).await?;
    let b = baseline(&h, "B").await?;
    assert_eq!(h.commit(seed::context(), c.clone()).await?, receipt);
    let final_head = h.head(seed::context()).await?;
    eprintln!("shared_storage: baselines A/B and exact retry verified");
    h.drain().await;
    let backend = FileBackend::new_shared_ontology(path, FileOptions::local_ext4())?;
    let reopened = OntologyHub::open(backend.clone(), seed::context(), specimens::scope()).await?;
    assert!(
        reopened
            .read_state(seed::context(), Selection::Snapshot(genesis))
            .await?
            .records
            .is_empty()
    );
    assert_eq!(
        reopened
            .read(seed::context(), Selection::Baseline(a.clone()), old.clone())
            .await?,
        original
    );
    assert_eq!(
        reopened
            .read(
                seed::context(),
                Selection::Baseline(b.clone()),
                next.reference.clone()
            )
            .await?,
        next
    );
    assert_eq!(
        reopened
            .operation_result(seed::context(), "successor".into())
            .await?,
        OperationOutcome::Committed(Box::new(receipt.clone()))
    );
    assert_eq!(reopened.commit(seed::context(), c).await?, receipt);
    let source = specimens::document();
    let bytes = reopened
        .read_artifact(
            seed::context(),
            Selection::Baseline(a.clone()),
            source.clone(),
        )
        .await?;
    assert_eq!(bytes.bytes, seed::ONTOLOGY);
    let range = reopened
        .read_artifact_range(
            seed::context(),
            Selection::Snapshot(final_head.clone()),
            source,
            0,
            31,
        )
        .await?;
    assert_eq!(range.bytes, &seed::ONTOLOGY[..31]);
    eprintln!("shared_storage: exact reopen, historical reads and bytes verified; scrubbing");
    let report = backend
        .scrub_ontology(seed::context(), specimens::scope())
        .await?;
    eprintln!("shared_storage: full retained-history scrub verified");
    reopened.drain().await;
    Ok(
        serde_json::json!({"format":SHARED_FILE_FORMAT,"scope":"synthetic fixtures; no human acceptance","baseline_a":a,"baseline_b":b,"head":final_head,"reopen":"exact","retry":"original receipt","storage":report}),
    )
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    let mode = args.get(1).map(String::as_str).unwrap_or("help");
    let path = std::path::PathBuf::from(args.get(2).ok_or(
        "usage: shared_storage create|scrub|pack|export ABSOLUTE_STORE [ABSOLUTE_EXPORT_DIR]",
    )?);
    if mode == "create" {
        println!("{}", walkthrough(&path).await?);
        return Ok(());
    }
    let backend = FileBackend::new_shared_ontology(&path, FileOptions::local_ext4())?;
    let hub = OntologyHub::open(backend.clone(), seed::context(), specimens::scope()).await?;
    match mode {
        "pack" => println!(
            "{}",
            serde_json::to_string(
                &backend
                    .compact_ontology(seed::context(), specimens::scope())
                    .await?
            )?
        ),
        "scrub" => println!(
            "{}",
            serde_json::to_string(
                &backend
                    .scrub_ontology(seed::context(), specimens::scope())
                    .await?
            )?
        ),
        "export" => {
            let out = std::path::PathBuf::from(
                args.get(3)
                    .ok_or("provide an explicit new absolute export directory")?,
            );
            if !out.is_absolute() {
                return Err("absolute export directory required".into());
            }
            std::fs::create_dir(&out)?;
            let head = hub.head(seed::context()).await?;
            let state = hub
                .read_state(seed::context(), Selection::Snapshot(head.clone()))
                .await?;
            std::fs::write(out.join("graph.json"), serde_json::to_vec_pretty(&state)?)?;
            std::fs::write(
                out.join("ontology.md"),
                hub.read_artifact(
                    seed::context(),
                    Selection::Snapshot(head.clone()),
                    specimens::document(),
                )
                .await?
                .bytes,
            )?;
            std::fs::write(
                out.join("selection.json"),
                serde_json::to_vec_pretty(
                    &serde_json::json!({"scope":hub.scope(),"snapshot":head,"authority":"derived readable export"}),
                )?,
            )?;
            println!("{}", out.display());
        }
        _ => return Err("expected create, scrub, pack or export".into()),
    }
    hub.drain().await;
    Ok(())
}
