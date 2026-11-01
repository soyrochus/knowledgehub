//! Read the canonical persisted self-model through the public files/2 API.
mod file_format;
#[path = "self_model_support/mod.rs"]
mod host;
#[path = "support/mod.rs"]
mod seed;

use knowledgehub::{ontology::*, *};
use serde_json::json;
use std::path::PathBuf;

fn narrative(state: &State, record: &Record) -> Result<serde_json::Value, Error> {
    let Some(Value::Narrative { document, .. }) = record.element().and_then(|p| p.get("statement"))
    else {
        return Err(Error::NotFound);
    };
    let content = state.artifact(document).ok_or(Error::NotFound)?;
    let text = std::str::from_utf8(&content.bytes)
        .map_err(|_| Error::InvalidInput("expected UTF-8 narrative".into()))?;
    Ok(json!({"document": document, "content": ContentRef::of(&content.bytes), "text": text}))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = file_format::Arguments::parse()?;
    let mut args = options.positional.iter().cloned();
    let store = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("hub"));
    if args.next().is_some() {
        return Err("usage: cargo run --release --example inspect_self_model -- [STORE]".into());
    }
    let store = if store.is_absolute() {
        store
    } else {
        std::env::current_dir()?.join(store)
    };
    eprintln!("Opening persisted self-model at {}", store.display());
    if (options.verify || options.compact) && options.format != file_format::Format::Files3 {
        return Err("--verify/--compact require --format files3".into());
    }
    let backend = options.format.backend(&store)?;
    let compacted = if options.compact {
        Some(
            backend
                .compact_ontology(seed::context(), host::scope())
                .await?,
        )
    } else {
        None
    };
    let hub = OntologyHub::open(backend.clone(), seed::context(), host::scope()).await?;
    let (head, state) = hub.snapshot(seed::context()).await?;
    let report = assess(&state);
    if !report.is_valid() {
        return Err(Error::Invalid(report.violations).into());
    }
    if !catalogue_coverage(&state).is_empty() {
        return Err(Error::InvalidInput("incomplete ontology catalogue".into()).into());
    }
    let a = host::r("kh:baseline:A");
    let b = host::r("kh:baseline:B");
    let view_a = state.view(state.baseline(&a).ok_or(Error::NotFound)?);
    let view_b = state.view(state.baseline(&b).ok_or(Error::NotFound)?);
    let requirement = "urn:requirement:atomic-change";
    let old = view_a
        .current("knowledgehub", requirement)
        .ok_or(Error::NotFound)?;
    let new = view_b
        .current("knowledgehub", requirement)
        .ok_or(Error::NotFound)?;
    let old_narrative = narrative(&view_a, old)?;
    let new_narrative = narrative(&view_b, new)?;
    if old.reference.revision != "self-1"
        || new.reference.revision != "self-2"
        || old_narrative == new_narrative
    {
        return Err(Error::InvalidInput("expected distinct exact A/B explanations".into()).into());
    }
    eprintln!("Recovered both baseline narratives; comparing A/B and original receipts");
    let differences = hub.compare(seed::context(), a.clone(), b.clone()).await?;
    let mut receipts = Vec::new();
    for operation in ["load-self-definition", "revise-explanation"] {
        let OperationOutcome::Committed(receipt) = hub
            .operation_result(seed::context(), operation.into())
            .await?
        else {
            return Err(Error::NotFound.into());
        };
        receipts.push(json!({
            "operation_id": receipt.operation_id,
            "sequence": receipt.sequence,
            "head": receipt.head,
            "fingerprint": receipt.fingerprint,
        }));
    }
    let indexed = if options.verify {
        Some(host::verify::indexed(&hub, &backend, seed::context()).await?)
    } else {
        None
    };
    let scrub = if options.verify {
        Some(
            backend
                .scrub_ontology(seed::context(), host::scope())
                .await?,
        )
    } else {
        None
    };
    hub.drain().await;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "store": store,
            "format":options.format.name(),
            "compaction":compacted,
            "scrub":scrub,
            "indexed":indexed,
            "scope": state.scope,
            "head": head,
            "profile": state.profile,
            "selected_records": state.selected_records().count(),
            "retained_revisions": state.records.len(),
            "structurally_valid": report.is_valid(),
            "explicit_gaps": report.gaps.len(),
            "purpose_complete": report.is_complete(),
            "baseline_a": {"reference": a, "selected_records": view_a.selected_records().count(), "requirement": old, "narrative": old_narrative},
            "baseline_b": {"reference": b, "selected_records": view_b.selected_records().count(), "requirement": new, "narrative": new_narrative},
            "changed_identities": differences.iter().map(|d| d.identity.replace('\0', "::")).collect::<Vec<_>>(),
            "original_receipts": receipts,
        }))?
    );
    Ok(())
}
