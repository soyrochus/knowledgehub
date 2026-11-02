//! Fixed synthetic inputs; run each case in a separate process for meaningful peak RSS.
mod resource_usage;
#[path = "support/mod.rs"]
mod seed;
#[path = "ontology_support/mod.rs"]
mod specimens;
use knowledgehub::{Content, backends::files::*, ontology::*};
use sha2::{Digest, Sha256};
use std::{os::unix::fs::MetadataExt, path::Path, time::Instant};
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn source(id: &str, origin: &str) -> Record {
    Record::new(
        ExactRef::new("measurement", id, "1"),
        type_ref("SourceArtifact"),
        RecordData::SourceArtifact(SourceArtifactData {
            origin: origin.into(),
            media_type: "application/octet-stream".into(),
            owner: "measurement:host".into(),
            status: "observation".into(),
        }),
        vec![],
    )
    .unwrap()
}
fn fixture(count: usize, size: usize) -> Vec<Operation> {
    let mut operations = vec![Operation::Put(source("edit-target", "initial"))];
    for i in 0..count {
        let r = source(&format!("source:{i:04}"), "deterministic fixture");
        // Every pair shares original content; different pairs use independent fixed pseudorandom bytes.
        let mut x = (i / 2 + 1) as u64;
        let bytes = (0..size)
            .map(|_| {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                x as u8
            })
            .collect::<Vec<_>>();
        let content = Content::capture(bytes);
        let capture = Record::new(
            ExactRef::new("measurement", format!("capture:{i:04}"), "1"),
            type_ref("SourceRevision"),
            RecordData::SourceRevision(SourceRevisionData {
                source: r.reference.clone(),
                original: content.reference.clone(),
                media_type: "application/octet-stream".into(),
                derived_from: None,
                derivation: None,
            }),
            vec![],
        )
        .unwrap();
        operations.push(Operation::Put(r));
        operations.push(Operation::Capture {
            record: capture,
            content,
        });
    }
    operations
}
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

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    let path = std::path::PathBuf::from(
        args.get(1)
            .ok_or("provide unused absolute store path, small|large|history")?,
    );
    let case = args.get(2).ok_or("provide case")?;
    if args.len() != 3 {
        return Err("provide only store path and fixed case".into());
    }
    let format = knowledgehub::FILE_FORMAT;
    let (count, size, edits) = match case.as_str() {
        "small" => (4, 2048, 8),
        "large" => (96, 16384, 8),
        "history" => (8, 2048, 64),
        _ => return Err("unknown fixed case".into()),
    };
    let backend = FileBackend::new(&path, FileOptions::native_local()?)?;
    let h = KnowledgeHub::create(backend.clone(), seed::context(), specimens::scope()).await?;
    let initial = Instant::now();
    let operations = fixture(count, size);
    let input_hash = digest(&serde_json::to_vec(&operations)?);
    for (id, ops) in [
        ("catalogue", builtin_catalogue()?.operations()),
        ("fixture", operations),
    ] {
        let (head, state) = h.snapshot(seed::context()).await?;
        let mut c = Change::at(&state, head, id);
        c.operations = ops;
        h.commit(seed::context(), c).await?;
    }
    let fixture_time = initial.elapsed().as_millis();
    let mut writes = Vec::new();
    let mut points = Vec::new();
    for n in 0..=edits {
        if n > 0 {
            let (head, state) = h.snapshot(seed::context()).await?;
            let old = state.selected[&ExactRef::new("measurement", "edit-target", "1").identity()]
                .clone();
            let mut next = state.get(&old).unwrap().clone();
            next.reference.revision = (n + 1).to_string();
            next.predecessor = Some(old);
            if let RecordData::SourceArtifact(s) = &mut next.data {
                s.origin = format!("tiny edit {n:04}");
            }
            next.seal()?;
            let mut c = Change::at(&state, head, format!("edit:{n:04}"));
            c.operations.push(Operation::Put(next));
            let before = backend.file_work();
            let bytes = disk(&path);
            let start = Instant::now();
            h.commit(seed::context(), c).await?;
            let after = backend.file_work();
            let disk_after = disk(&path);
            writes.push(serde_json::json!({"edit":n,"ms":start.elapsed().as_millis(),"new_objects":after.objects_written-before.objects_written,"encoded_written":after.encoded_bytes_written-before.encoded_bytes_written,"payload_written":after.payload_bytes_written-before.payload_bytes_written,"structural_written":after.structural_bytes_written-before.structural_bytes_written,"file_bytes_growth":disk_after.0-bytes.0,"allocated_growth":disk_after.1-bytes.1}));
        }
        if n == 0 || n == edits {
            let head = h.head(seed::context()).await?;
            let before = backend.file_work();
            let start = Instant::now();
            h.read(seed::context(), Selection::Snapshot(head), ontology_ref())
                .await?;
            let after = backend.file_work();
            points.push(serde_json::json!({"after_edits":n,"micros":start.elapsed().as_micros(),"object_reads":after.object_reads-before.object_reads,"encoded_bytes_read":after.encoded_bytes_read-before.encoded_bytes_read,"index_bytes_read":after.index_bytes_read-before.index_bytes_read}));
        }
    }
    let (head, state) = h.snapshot(seed::context()).await?;
    let before = disk(&path);
    let start = Instant::now();
    let scrub = backend.scrub(seed::context(), specimens::scope()).await?;
    let storage = serde_json::json!({"scrub_ms":start.elapsed().as_millis(),"report":scrub});
    let start = Instant::now();
    let packed = backend.compact(seed::context(), specimens::scope()).await?;
    let packing = serde_json::json!({"pack_ms":start.elapsed().as_millis(),"report":packed});
    assert_eq!(
        h.snapshot(seed::context()).await?,
        (head.clone(), state.clone())
    );
    let logical_state = digest(&serde_json::to_vec(&state)?);
    h.drain().await;
    let backend = FileBackend::new(&path, FileOptions::native_local()?)?;
    let start = Instant::now();
    let reopened = KnowledgeHub::open(backend.clone(), seed::context(), specimens::scope()).await?;
    let open_us = start.elapsed().as_micros();
    assert_eq!(
        reopened.snapshot(seed::context()).await?,
        (head, state.clone())
    );
    let post = disk(&path);
    let peak = resource_usage::peak_rss_bytes();
    println!(
        "{}",
        serde_json::json!({"case":case,"format":format,"input_sha256":input_hash,"logical_state_sha256":logical_state,"fixtures":{"sources":count,"bytes_each":size,"edits":edits,"content_policy":"each adjacent pair has identical bytes"},"fixture_ms":fixture_time,"selected":state.selected.len(),"retained":state.records.len(),"before_pack":{"encoded":before.0,"allocated":before.1,"files":before.2},"after_pack":{"encoded":post.0,"allocated":post.1,"files":post.2},"open_us":open_us,"peak_process_rss_bytes":peak,"point_reads":points,"writes":writes,"scrub":storage,"compaction":packing,"cache":"new backend instance for open; OS page cache uncontrolled; per-operation decoded cache; location cache shared by backend"})
    );
    reopened.drain().await;
    Ok(())
}
