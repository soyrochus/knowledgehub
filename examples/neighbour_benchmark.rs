//! Fixed actual neighbourhood with increasing unrelated facts and retained generations.
#[path = "neighbour_support/mod.rs"]
mod fixture;
use knowledgehub::{backends::files::*, ontology::*, *};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, os::unix::fs::MetadataExt, path::Path, time::Instant};
fn disk(path: &Path) -> (u64, u64, u64) {
    let m = std::fs::metadata(path).unwrap();
    if m.is_file() {
        return (m.len(), m.blocks() * 512, 1);
    }
    std::fs::read_dir(path)
        .unwrap()
        .map(|e| disk(&e.unwrap().path()))
        .fold((0, 0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2))
}
fn rss() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .unwrap()
        .lines()
        .find_map(|l| {
            l.strip_prefix("VmHWM:")
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse::<u64>().ok())
        })
        .unwrap_or(0)
        * 1024
}
async fn measured(
    h: &OntologyHub,
    b: &FileBackend,
    q: Neighbours,
) -> Result<serde_json::Value, Error> {
    let before = b.file_work();
    let start = Instant::now();
    let p = h.neighbours(fixture::context(), q).await?;
    let elapsed = start.elapsed().as_micros();
    let after = b.file_work();
    assert!(!p.work.index_rebuilt);
    assert_eq!(before.state_materialisations, after.state_materialisations);
    assert_eq!(p.work.prepared_owners, 0);
    if let Some(c) = &p.continuation {
        h.release_cursor(fixture::context(), c.clone()).await?;
    }
    Ok(
        serde_json::json!({"micros":elapsed,"results":p.neighbours.len(),"exhausted":p.exhausted,"reached":p.reached,"work":p.work,"source_object_reads":after.object_reads-before.object_reads,"source_encoded_bytes":after.encoded_bytes_read-before.encoded_bytes_read,"state_materialisations":after.state_materialisations-before.state_materialisations}),
    )
}
fn query(head: SnapshotId) -> Neighbours {
    let mut q = fixture::query(Selection::Snapshot(head), "b");
    q.categories = BTreeSet::from([NeighbourCategory::Association]);
    q.preparation.allow_rebuild = false;
    q
}
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::PathBuf::from(
        std::env::args()
            .nth(1)
            .ok_or("provide an unused absolute store path with an existing parent")?,
    );
    let (mut backend, mut h) = fixture::create(&path).await?;
    let clock = Record::new(
        fixture::reference("clock"),
        type_ref("SourceArtifact"),
        RecordData::SourceArtifact(SourceArtifactData {
            origin: "fixed clock seed".into(),
            media_type: "application/octet-stream".into(),
            owner: "fixture:host".into(),
            status: "observation".into(),
        }),
        vec![],
    )?;
    let mut initial = fixture::binary("unrelated-original", vec![19; 2_100_000]);
    initial.push(ontology::Operation::Put(clock));
    initial.extend(
        ["fixed:ab1", "fixed:ab2"]
            .into_iter()
            .map(|id| ontology::Operation::Put(fixture::edge(id, "a", "b"))),
    );
    initial.push(ontology::Operation::Put(fixture::edge(
        "fixed:self",
        "b",
        "b",
    )));
    fixture::commit(&h, "fixed-facts", initial).await?;
    let mut checkpoints = Vec::new();
    let mut previous = 0;
    for unrelated in [0, 100, 1000, 3000] {
        if unrelated > previous {
            let operations = (previous..unrelated)
                .map(|i| {
                    ontology::Operation::Put(fixture::edge(&format!("unrelated:{i:05}"), "c", "c"))
                })
                .collect();
            fixture::commit(&h, &format!("unrelated:{unrelated}"), operations).await?;
        }
        previous = unrelated;
        let head = h.head(fixture::context()).await?;
        let mut preparing = query(head.clone());
        preparing.preparation.allow_rebuild = true;
        let start = Instant::now();
        let preparation = h.prepare_neighbours(fixture::context(), preparing).await?;
        let prep_us = start.elapsed().as_micros();
        let usage = disk(&path.join("cache/neighbours"));
        h.drain().await;
        backend = fixture::backend(&path);
        h = OntologyHub::open(backend.clone(), fixture::context(), fixture::scope()).await?;
        let cold = measured(&h, &backend, query(head.clone())).await?;
        assert_eq!(cold["results"], 3);
        let mut warm = Vec::new();
        for _ in 0..3 {
            warm.push(measured(&h, &backend, query(head.clone())).await?);
        }
        checkpoints.push(serde_json::json!({"unrelated_assertions":unrelated,"preparation_us":prep_us,"preparation":preparation,"cold_page":cold,"warm_pages":warm,"cache":{"encoded":usage.0,"allocated":usage.1,"regular_files":usage.2}}));
        eprintln!("neighbour_benchmark: {unrelated} unrelated assertions verified");
    }
    let mut generations = Vec::new();
    for i in 1..=8 {
        let (head, state) = h.snapshot(fixture::context()).await?;
        let old = state.current("neighbour-fixture", "clock").unwrap();
        let mut clock = old.clone();
        clock.reference.revision = (i + 1).to_string();
        clock.predecessor = Some(old.reference.clone());
        if let RecordData::SourceArtifact(s) = &mut clock.data {
            s.origin = format!("unrelated tiny edit {i}");
        }
        clock.seal()?;
        let mut c = Change::at(&state, head, format!("history:{i}"));
        c.operations.push(ontology::Operation::Put(clock));
        let receipt = h.commit(fixture::context(), c).await?;
        generations.push(serde_json::json!({"additional_generations":i,"lookup":measured(&h,&backend,query(receipt.head)).await?}));
    }
    let head = h.head(fixture::context()).await?;
    let mut fanout = fixture::query(Selection::Snapshot(head), "c");
    fanout.categories = BTreeSet::from([NeighbourCategory::Association]);
    fanout.selectors = BTreeSet::from([NeighbourSelector::Assertion {
        definition: fixture::relation(),
    }]);
    fanout.direction = NeighbourDirection::Outgoing;
    fanout.page_size = 50;
    fanout.examined_limit = 50;
    fanout.preparation.allow_rebuild = false;
    let high = measured(&h, &backend, fanout).await?;
    assert_eq!(high["results"], 50);
    assert_eq!(high["exhausted"], false);
    let input_hash = format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&(
            fixture::vocabulary(),
            [0, 100, 1000, 3000],
            8,
            "fixed actual associations a->b x2 and b->b; unrelated c->c"
        ))?)
    );
    println!(
        "{}",
        serde_json::json!({"scope":"synthetic registered vocabulary; exact deterministic associations; no human acceptance","input_sha256":input_hash,"checkpoints":checkpoints,"unrelated_generations":generations,"high_fanout":high,"peak_process_rss_bytes":rss(),"cache_conditions":"fresh backend for cold page; subsequent calls share OS cache; decoded-object caches are per operation; OS page cache not flushed; shared host"})
    );
    h.drain().await;
    Ok(())
}
