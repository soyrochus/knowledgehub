//! Isolated public late-link walkthrough; all model observations are synthetic.
#[path = "neighbour_support/mod.rs"]
mod fixture;
use knowledgehub::{OperationOutcome, ontology::*};
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::PathBuf::from(
        std::env::args()
            .nth(1)
            .ok_or("provide an unused absolute store path with an existing parent")?,
    );
    let (backend, h) = fixture::create(&path).await?;
    eprintln!("neighbours: registered vocabulary and endpoints");
    let old = h.head(fixture::context()).await?;
    let initial = fixture::query(Selection::Snapshot(old), "b");
    let first = h.neighbours(fixture::context(), initial).await?;
    assert!(first.work.index_rebuilt);
    let a = fixture::baseline(&h, "A").await?;
    let (change, receipt) = fixture::commit(
        &h,
        "later-associations",
        vec![
            Operation::Put(fixture::edge("late:ab", "a", "b")),
            Operation::Put(fixture::edge("late:parallel", "a", "b")),
            Operation::Put(fixture::edge("late:self", "b", "b")),
        ],
    )
    .await?;
    let before_work = backend.file_work();
    let new = h
        .neighbours(
            fixture::context(),
            fixture::query(Selection::Snapshot(receipt.head.clone()), "b"),
        )
        .await?;
    assert!(!new.work.index_rebuilt);
    assert_eq!(
        before_work.state_materialisations,
        backend.file_work().state_materialisations
    );
    assert_eq!(
        new.neighbours
            .iter()
            .filter(|n| n.category == NeighbourCategory::Association)
            .count(),
        3
    );
    let old = h
        .neighbours(
            fixture::context(),
            fixture::query(Selection::Baseline(a.clone()), "b"),
        )
        .await?;
    assert!(
        !old.neighbours
            .iter()
            .any(|n| n.category == NeighbourCategory::Association)
    );
    eprintln!("neighbours: late incoming/outgoing links and unchanged baseline verified");
    let (head, state) = h.snapshot(fixture::context()).await?;
    let mut successor = state.get(&fixture::reference("b")).unwrap().clone();
    successor.reference.revision = "2".into();
    successor.predecessor = Some(fixture::reference("b"));
    if let RecordData::Element(e) = &mut successor.data {
        e.properties.insert(
            "name".into(),
            Value::text("A successor with the same semantic identity"),
        );
    }
    successor.seal()?;
    let mut c = Change::at(&state, head, "successor");
    c.operations.push(Operation::Put(successor.clone()));
    for r in state.selected_records() {
        if let RecordData::Assertion(a) = &r.data {
            if a.from == fixture::reference("b") || a.to == fixture::reference("b") {
                let mut edge = r.clone();
                edge.reference.revision = "2".into();
                edge.predecessor = Some(r.reference.clone());
                if let RecordData::Assertion(a) = &mut edge.data {
                    if a.from == fixture::reference("b") {
                        a.from = successor.reference.clone();
                    }
                    if a.to == fixture::reference("b") {
                        a.to = successor.reference.clone();
                    }
                }
                edge.seal()?;
                c.operations.push(Operation::Put(edge));
            }
        }
    }
    h.commit(fixture::context(), c).await?;
    let b = fixture::baseline(&h, "B").await?;
    let exact = h
        .neighbours(
            fixture::context(),
            fixture::query(Selection::Baseline(b.clone()), "b"),
        )
        .await?;
    assert!(
        !exact
            .vertices
            .iter()
            .find(|v| v.reference == fixture::reference("b"))
            .unwrap()
            .selected
    );
    assert!(
        exact
            .vertices
            .iter()
            .find(|v| v.reference == fixture::reference("b"))
            .unwrap()
            .available
    );
    // Explicitly restoring an earlier selection keeps its exact assertion pins; no automatic latest lookup.
    let (head, state) = h.snapshot(fixture::context()).await?;
    let mut restore = Change::at(&state, head, "restore-exact-selection");
    restore
        .operations
        .push(Operation::Select(fixture::reference("b")));
    for r in state.selected_records() {
        if let RecordData::Assertion(a) = &r.data {
            if a.from.identity() == fixture::reference("b").identity()
                || a.to.identity() == fixture::reference("b").identity()
            {
                let mut edge = r.clone();
                edge.reference.revision = "3".into();
                edge.predecessor = Some(r.reference.clone());
                if let RecordData::Assertion(a) = &mut edge.data {
                    if a.from.identity() == fixture::reference("b").identity() {
                        a.from = fixture::reference("b");
                    }
                    if a.to.identity() == fixture::reference("b").identity() {
                        a.to = fixture::reference("b");
                    }
                }
                edge.seal()?;
                restore.operations.push(Operation::Put(edge));
            }
        }
    }
    let restored = h.commit(fixture::context(), restore).await?;
    let restored = h
        .neighbours(
            fixture::context(),
            fixture::query(Selection::Snapshot(restored.head), "b"),
        )
        .await?;
    assert_eq!(
        restored
            .neighbours
            .iter()
            .filter(|n| n.category == NeighbourCategory::Association)
            .count(),
        3
    );
    assert!(
        restored
            .vertices
            .iter()
            .find(|v| v.reference == fixture::reference("b"))
            .unwrap()
            .selected
    );
    assert_eq!(h.commit(fixture::context(), change.clone()).await?, receipt);
    h.drain().await;
    let reopened = OntologyHub::open(
        fixture::backend(&path),
        fixture::context(),
        fixture::scope(),
    )
    .await?;
    let after = reopened
        .neighbours(
            fixture::context(),
            fixture::query(Selection::Baseline(b.clone()), "b"),
        )
        .await?;
    assert_eq!(after.neighbours, exact.neighbours);
    assert_eq!(
        reopened
            .operation_result(fixture::context(), "later-associations".into())
            .await?,
        OperationOutcome::Committed(Box::new(receipt.clone()))
    );
    assert_eq!(reopened.commit(fixture::context(), change).await?, receipt);
    println!(
        "{}",
        serde_json::json!({"scope":"synthetic registered-neighbour fixture; no human acceptance","store":path,"baseline_a":a,"baseline_b":b,"initial":first,"after_late_links":new,"historical_endpoint":after,"explicit_restored_selection":restored,"reopen":"exact","retry":"original receipt"})
    );
    reopened.drain().await;
    Ok(())
}
