//! Retain a complete synthetic conformance model through the ordinary public API.
mod file_format;
#[path = "support/mod.rs"]
mod seed;
#[path = "ontology_support/mod.rs"]
mod specimens;
use knowledgehub::ontology::*;
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = file_format::Arguments::parse()?;
    let path = std::path::PathBuf::from(
        args.positional
            .first()
            .cloned()
            .ok_or("provide a new absolute store path with an existing parent")?,
    );
    let h = KnowledgeHub::create(
        knowledgehub::backends::files::FileBackend::new(
            &path,
            knowledgehub::backends::files::FileOptions::native_local()?,
        )?,
        seed::context(),
        specimens::scope(),
    )
    .await?;
    for (id, operations) in [
        ("catalogue", builtin_catalogue()?.operations()),
        ("specimens", specimens::specimens()?),
    ] {
        let (head, state) = h.snapshot(seed::context()).await?;
        let mut c = Change::at(&state, head, id);
        c.operations = operations;
        let report = h.validate(seed::context(), c.clone()).await?;
        assert!(report.is_complete(), "{report:?}");
        h.commit(seed::context(), c).await?;
    }
    let (head, before) = h.snapshot(seed::context()).await?;
    h.drain().await;
    let reopened = KnowledgeHub::open(
        knowledgehub::backends::files::FileBackend::new(
            &path,
            knowledgehub::backends::files::FileOptions::native_local()?,
        )?,
        seed::context(),
        specimens::scope(),
    )
    .await?;
    let after = reopened
        .read_state(seed::context(), Selection::Snapshot(head))
        .await?;
    assert_eq!(before, after);
    assert!(catalogue_coverage(&after).is_empty());
    let requirement = specimens::reference("Requirement");
    let kernel = KERNEL_KINDS
        .iter()
        .map(|kind| {
            let materialisation = match *kind {
                "ElementIdentity" => after
                    .resolve(
                        &after
                            .identity_reference(&requirement)
                            .expect("actual identity"),
                    )
                    .expect("identity projection"),
                "ElementRevision" => after.resolve(&requirement).expect("actual revision"),
                _ => after
                    .selected_records()
                    .find(|r| r.data.native_kind() == Some(*kind))
                    .expect("native kernel materialisation")
                    .clone(),
            };
            ((*kind).to_owned(), materialisation)
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    println!(
        "{}",
        serde_json::to_string(
            &serde_json::json!({"store":path,"scope":"synthetic structural conformance; no project acceptance","types":145,"family_types":128,"signatures":38,"selected_records":after.selected.len(),"definition_digests":after.records.values().filter(|r|matches!(r.data,RecordData::Type(_)|RecordData::Relationship(_)|RecordData::Constraint(_))).map(|r|(&r.reference,&r.sha256)).collect::<Vec<_>>(),"kernel":kernel,"specimens":after.selected_records().filter(|r|r.reference.model=="specimen").collect::<Vec<_>>(),"reopen":"exact"})
        )?
    );
    reopened.drain().await;
    Ok(())
}
