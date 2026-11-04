//! Run with an unused absolute directory; creates only disposable synthetic fixtures.
mod history_support;
mod support;
use history_support as f;
use knowledgehub::{backends::files::*, *};
use serde_json::{Value as Json, json};
use std::path::PathBuf;

async fn generated(parent: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir(&parent)?;
    for domain in ["library-loans", "garden-irrigation"] {
        let path = parent.join(domain);
        let backend = || FileBackend::new(&path, FileOptions::native_local().unwrap()).unwrap();
        let h = KnowledgeHub::create(backend(), support::context(), support::scope()).await?;
        let fixture = f::populate(&h, support::context(), domain, 3).await?;
        let anchor = h.head(support::context()).await?;
        let b = f::reader(h.scope());
        let mut q = Publications::new(anchor.clone());
        q.limits.items = 2;
        let page = h.publications(b.clone(), q.clone()).await?;
        let lost = page.continuation.clone().unwrap();
        let first_work = page.work.clone();
        let first_limits = q.limits.clone();
        f::append(&h, support::context(), "history:concurrent-later").await?;
        h.drain().await;
        let h = KnowledgeHub::open(backend(), b.clone(), support::scope()).await?;
        assert_eq!(
            h.continue_history(b.clone(), HistoryRequest::Publications(q.clone()), lost)
                .await,
            Err(Error::InvalidCursor)
        );
        let headers = f::headers(&h, b.clone(), q.clone()).await?;
        for (header, original) in headers.iter().zip(fixture.receipts.iter().rev()) {
            let lookup = h
                .publication(
                    b.clone(),
                    Publication::new(anchor.clone(), &original.operation_id),
                )
                .await?;
            assert_eq!(
                lookup.outcome,
                PublicationLookup::Published(Box::new(header.clone()))
            );
            let refs = f::affected(
                &h,
                b.clone(),
                PublicationAffected::new(anchor.clone(), &original.operation_id),
            )
            .await?;
            assert_eq!(header.clone().with_affected(refs)?, *original);
        }
        f::meaning(&h, b.clone(), &fixture).await?;
        assert_eq!(
            h.commit(support::context(), fixture.retry.clone()).await?,
            fixture.receipts[1]
        );
        let actor_b = Context::new(b.actor.clone(), std::sync::Arc::new(support::Allow));
        assert_eq!(
            h.operation_result(actor_b.clone(), fixture.retry.operation_id.clone())
                .await,
            Err(Error::OperationReuse)
        );
        assert_eq!(
            h.commit(actor_b, fixture.retry.clone()).await,
            Err(Error::OperationReuse)
        );
        let missing = h
            .publication(
                b.clone(),
                Publication::new(anchor.clone(), "history:concurrent-later"),
            )
            .await?;
        assert_eq!(missing.outcome, PublicationLookup::NotPublishedAt);
        let denied = h
            .publications(Context::new("host:denied", std::sync::Arc::new(DenyAll)), q)
            .await
            .unwrap_err();
        h.drain().await;
        let maintenance = backend();
        maintenance
            .compact(support::context(), support::scope())
            .await?;
        let h = KnowledgeHub::open(backend(), b.clone(), support::scope()).await?;
        let packed = f::headers(&h, b.clone(), Publications::new(anchor.clone())).await?;
        assert_eq!(headers, packed);
        f::meaning(&h, b, &fixture).await?;
        h.drain().await;
        println!(
            "{}",
            serde_json::to_string(
                &json!({"fixture":domain,"anchor":anchor,"initial_boundary":fixture.initial,"headers":headers,"baseline_A":fixture.baselines[0],"baseline_B":fixture.baselines[1],"sources":fixture.sources,"source_hashes":fixture.bytes.iter().map(|b| ContentRef::of(b)).collect::<Vec<_>>(),"limits":first_limits,"first_page_work":first_work,"reopen_packing":"exact","lost_token":"InvalidCursor","later_than_anchor":missing.outcome,"denied":format!("{denied:?}"),"human_acceptance":"pending"})
            )?
        );
    }
    Ok(())
}

/// The caller supplies a quiescent disposable copy. Never open the retained source store here.
async fn retained(
    manifest: PathBuf,
    successor: PathBuf,
    copy: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let manifest: Json = serde_json::from_slice(&std::fs::read(manifest)?)?;
    let successor: Json = serde_json::from_slice(&std::fs::read(successor)?)?;
    let original = PathBuf::from(manifest["destination"].as_str().ok_or("destination")?);
    if std::fs::canonicalize(&copy)? == std::fs::canonicalize(original)? {
        return Err("a disposable copy is required".into());
    }
    let scope: ModelScope = serde_json::from_value(manifest["scope"].clone())?;
    let b = f::reader(scope.clone());
    let backend = FileBackend::new(copy, FileOptions::native_local()?)?;
    let h = KnowledgeHub::open(backend.clone(), b.clone(), scope).await?;
    let anchor = h.head(b.clone()).await?;
    let before = backend.file_work();
    let headers = f::headers(&h, b.clone(), Publications::new(anchor.clone())).await?;
    let history_work = backend.file_work();
    assert_eq!(
        before.state_materialisations,
        history_work.state_materialisations
    );
    let mut known = 0;
    for op in manifest["known_operations"]
        .as_array()
        .ok_or("known operations")?
    {
        let expected: Receipt = serde_json::from_value(op["receipt"].clone())?;
        let mut q = Publication::new(anchor.clone(), &expected.operation_id);
        q.limits = HistoryLimits::MAX;
        let result = h.publication(b.clone(), q).await?;
        let PublicationLookup::Published(header) = result.outcome else {
            return Err("known original publication missing".into());
        };
        let mut refs = PublicationAffected::new(anchor.clone(), &expected.operation_id);
        refs.limits = HistoryLimits::MAX;
        assert_eq!(
            header.with_affected(f::affected(&h, b.clone(), refs).await?)?,
            expected
        );
        known += 1;
    }
    let a: ExactRef = serde_json::from_value(successor["baseline_A"].clone())?;
    let z: ExactRef = serde_json::from_value(successor["baseline_B"].clone())?;
    let mut source_hashes = vec![];
    for (key, reference) in manifest["key_map"].as_object().ok_or("key map")? {
        if !key.starts_with("capture:") {
            continue;
        }
        let r: ExactRef = serde_json::from_value(reference.clone())?;
        let original = h
            .read_artifact(b.clone(), Selection::Baseline(a.clone()), r.clone())
            .await?;
        assert_eq!(
            h.read_artifact(b.clone(), Selection::Baseline(z.clone()), r.clone())
                .await?,
            original
        );
        source_hashes.push(json!({"reference":r,"content":original.reference}));
    }
    for changed in successor["changed"].as_array().ok_or("changed")? {
        let old: ExactRef = serde_json::from_value(changed[0].clone())?;
        let new: ExactRef = serde_json::from_value(changed[1].clone())?;
        let original = h
            .read(b.clone(), Selection::Baseline(a.clone()), old.clone())
            .await?;
        let edited = h
            .read(b.clone(), Selection::Baseline(z.clone()), new)
            .await?;
        assert_eq!(edited.predecessor, Some(original.reference));
        assert_ne!(edited.sha256, original.sha256);
    }
    let diff = h.compare(b.clone(), a.clone(), z.clone()).await?;
    assert!(!diff.is_empty());
    assert_eq!(h.head(b).await?, anchor);
    println!(
        "{}",
        serde_json::to_string(
            &json!({"fixture":"retained-MM-SL-01-copy","anchor":anchor,"reader":"host:history-reader","known_original_receipts":known,"headers":headers,"baseline_A":a,"baseline_B":z,"source_hashes":source_hashes,"differences":diff.len(),"history_physical_work_before":before,"history_physical_work_after":history_work,"human_acceptance":"pending"})
        )?
    );
    h.drain().await;
    Ok(())
}
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [directory] => generated(PathBuf::from(directory)).await,
        [mode, manifest, successor, copy] if mode == "retained" => retained(manifest.into(), successor.into(), copy.into()).await,
        _ => Err("usage: native_history /absolute/unused/parent OR native_history retained GENERATION-MANIFEST SUCCESSOR.json /disposable/copy".into()),
    }
}
