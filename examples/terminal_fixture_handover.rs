//! Maintainer-only: supersede a baseline and hand over original native publications.
use knowledgehub::{backends::files::*, *};
use std::{collections::BTreeSet, path::PathBuf, sync::Arc};
#[derive(serde::Deserialize)]
struct Host {
    actor: String,
    grants: BTreeSet<String>,
}
struct Scoped {
    actor: String,
    scope: ModelScope,
    grants: BTreeSet<String>,
}
impl Policy for Scoped {
    fn allows(&self, a: &str, s: &ModelScope, x: Action) -> Result<bool, PolicyFailure> {
        Ok(a == self.actor && s == &self.scope && self.grants.contains(&format!("{x:?}")))
    }
}
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 3 {
        return Err("usage: terminal_fixture_handover GENERATION HOST OUTPUT".into());
    }
    let m: serde_json::Value = serde_json::from_slice(&std::fs::read(&args[0])?)?;
    let host: Host = serde_json::from_slice(&std::fs::read(&args[1])?)?;
    let scope: ModelScope = serde_json::from_value(m["scope"].clone())?;
    let ctx = Context::new(
        host.actor.clone(),
        Arc::new(Scoped {
            actor: host.actor,
            grants: host.grants,
            scope: scope.clone(),
        }),
    );
    let dir = PathBuf::from(m["destination"].as_str().ok_or("destination")?);
    let backend = || FileBackend::new(&dir, FileOptions::native_local().unwrap()).unwrap();
    let h = KnowledgeHub::open(backend(), ctx.clone(), scope.clone()).await?;
    let (head, state) = h.snapshot(ctx.clone()).await?;
    let a: ExactRef = serde_json::from_value(m["baseline"].clone())?;
    let b = ExactRef::new(scope.model.clone(), "model:baseline:native-B", "1");
    let mut next = h
        .read(ctx.clone(), Selection::Snapshot(head.clone()), a.clone())
        .await?;
    next.reference.revision = "2".into();
    next.predecessor = Some(a.clone());
    next.seal()?;
    // Same exact membership; definition succession is separate from Baseline.parents.
    let a2 = next.reference.clone();
    let mut change = Change::at(&state, head, "terminal:baseline-definition-successor");
    change.operations.push(Operation::Put(next));
    let receipt = h.commit(ctx.clone(), change).await?;
    let anchor = receipt.head.clone();
    let mut q = Publications::new(anchor.clone());
    q.limits.items = 2;
    let mut page = h.publications(ctx.clone(), q.clone()).await?;
    let mut headers = vec![];
    loop {
        if let HistoryItems::Publications(p) = page.items {
            headers.extend(p);
        }
        if let Some(t) = page.continuation {
            page = h
                .continue_history(ctx.clone(), HistoryRequest::Publications(q.clone()), t)
                .await?;
        } else {
            assert!(page.complete);
            break;
        }
    }
    let mut publications = vec![];
    for header in headers {
        let q = PublicationAffected::new(anchor.clone(), header.operation_id.clone());
        let mut page = h.publication_affected(ctx.clone(), q.clone()).await?;
        let mut affected = vec![];
        loop {
            if let HistoryItems::Affected {
                references, offset, ..
            } = page.items
            {
                assert_eq!(offset, affected.len() as u64);
                affected.extend(references);
            }
            if let Some(t) = page.continuation {
                page = h
                    .continue_history(ctx.clone(), HistoryRequest::Affected(q.clone()), t)
                    .await?;
            } else {
                assert!(page.complete);
                break;
            }
        }
        let original = header.clone().with_affected(affected)?;
        assert_eq!(
            h.operation_result(ctx.clone(), header.operation_id.clone())
                .await?,
            OperationOutcome::Committed(Box::new(original.clone()))
        );
        publications.push(original);
    }
    h.drain().await;
    backend().compact(ctx.clone(), scope.clone()).await?;
    let reader = Context::new(
        "host:reader-B",
        Arc::new(Scoped {
            actor: "host:reader-B".into(),
            scope: scope.clone(),
            grants: BTreeSet::from(["Open".into(), "Read".into(), "ReadHistory".into()]),
        }),
    );
    let h = KnowledgeHub::open(backend(), reader.clone(), scope.clone()).await?;
    for r in &publications {
        let got = h
            .publication(
                reader.clone(),
                Publication::new(anchor.clone(), r.operation_id.clone()),
            )
            .await?;
        assert!(
            matches!(got.outcome,PublicationLookup::Published(p)if p.clone().with_affected(r.affected.clone()).unwrap()==*r)
        );
    }
    assert!(matches!(
        h.operation_result(reader.clone(), publications[0].operation_id.clone())
            .await,
        Err(Error::Denied {
            action: Action::OperationResult
        })
    ));
    let diff = h.compare(reader.clone(), a.clone(), b.clone()).await?;
    assert!(!diff.is_empty());
    for dispatch in m["known_operations"].as_array().ok_or("operations")? {
        let original: Receipt = serde_json::from_value(dispatch["receipt"].clone())?;
        assert!(publications.contains(&original));
    }
    let mut sources = vec![];
    for (key, v) in m["key_map"].as_object().ok_or("key map")? {
        if key.starts_with("capture:") {
            let r: ExactRef = serde_json::from_value(v.clone())?;
            let x = h
                .read_artifact(reader.clone(), Selection::Baseline(a.clone()), r.clone())
                .await?;
            assert_eq!(
                x,
                h.read_artifact(reader.clone(), Selection::Baseline(b.clone()), r.clone())
                    .await?
            );
            sources.push(serde_json::json!({"reference":r,"original":x.reference}));
        }
    }
    h.drain().await;
    std::fs::write(
        &args[2],
        serde_json::to_vec_pretty(
            &serde_json::json!({"format":"knowledgehub-terminal-handover/1","scope":scope,"publisher":"host:publisher-A","reader":"host:reader-B","anchor":anchor,"baselines":[a,b,a2],"publications":publications,"sources":sources,"status":"valid draft; no process execution or human acceptance","reopened_original_receipts_equal":true}),
        )?,
    )?;
    Ok(())
}
