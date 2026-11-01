mod support;
#[path = "../examples/shared_storage.rs"]
#[allow(dead_code)]
mod walkthrough;
use knowledgehub::{backends::files::*, ontology::*};
use walkthrough::{seed, specimens};

#[tokio::test]
async fn public_shared_walkthrough_pack_reindex_and_materialise() {
    let s = support::Sandbox::new();
    walkthrough::walkthrough(&s.store).await.unwrap();
    let b = FileBackend::new(&s.store, FileOptions::local_ext4()).unwrap();
    let h = KnowledgeHub::open(b.clone(), seed::context(), specimens::scope())
        .await
        .unwrap();
    let (head, before) = h.snapshot(seed::context()).await.unwrap();
    let original = h
        .operation_result(seed::context(), "catalogue".into())
        .await
        .unwrap();
    let loose = b.scrub(seed::context(), specimens::scope()).await.unwrap();
    eprintln!("test: packing");
    let packed = b
        .compact(seed::context(), specimens::scope())
        .await
        .unwrap();
    eprintln!("test: packed; comparing state");
    assert!(packed.allocated_file_bytes < loose.allocated_file_bytes);
    assert_eq!(packed.generations, loose.generations);
    assert_eq!(packed.logical_object_bytes, loose.logical_object_bytes);
    assert_eq!(packed.objects, loose.objects);
    assert!(packed.removed_files > 0);
    assert_eq!(
        h.snapshot(seed::context()).await.unwrap(),
        (head, before.clone())
    );
    assert_eq!(
        h.operation_result(seed::context(), "catalogue".into())
            .await
            .unwrap(),
        original
    );
    // A syntactically valid index may omit or misdirect a required object; neither proves absence.
    let reference = std::fs::read_dir(s.store.join("refs"))
        .unwrap()
        .find_map(|e| {
            let p = e.unwrap().path();
            (p.extension().is_some_and(|e| e == "json")).then_some(p)
        })
        .unwrap();
    let reference: serde_json::Value =
        serde_json::from_slice(&std::fs::read(reference).unwrap()).unwrap();
    for e in std::fs::read_dir(s.store.join("objects/packs")).unwrap() {
        let p = e.unwrap().path();
        if p.extension().is_some_and(|e| e == "idx") {
            let bytes = std::fs::read(&p).unwrap();
            let mut index: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            let root = reference["generation"].as_str().unwrap();
            index.as_object_mut().unwrap().remove(root);
            std::fs::write(&p, serde_json::to_vec(&index).unwrap()).unwrap();
            assert_eq!(h.snapshot(seed::context()).await.unwrap().1, before);
            let mut index: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
            index[root]["block_offset"] = "0".into();
            std::fs::write(&p, serde_json::to_vec(&index).unwrap()).unwrap();
            assert_eq!(h.snapshot(seed::context()).await.unwrap().1, before);
        }
    }
    std::fs::remove_file(s.store.join("objects/catalog.json")).unwrap();
    for e in std::fs::read_dir(s.store.join("objects/packs")).unwrap() {
        let p = e.unwrap().path();
        if p.extension().is_some_and(|e| e == "idx") {
            std::fs::remove_file(p).unwrap();
        }
    }
    assert_eq!(h.snapshot(seed::context()).await.unwrap().1, before);
    assert_eq!(
        h.operation_result(seed::context(), "catalogue".into())
            .await
            .unwrap(),
        original
    );
    b.scrub(seed::context(), specimens::scope()).await.unwrap();
}
