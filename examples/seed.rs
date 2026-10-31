//! cargo run --example seed -- /absolute/new/store/path
mod support;
use knowledgehub::{
    backends::files::{FileBackend, FileOptions},
    *,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("supply an absolute new store path on local ext4")?;
    let runtime = tokio::runtime::Builder::new_current_thread().build()?;
    runtime.block_on(async {
        let scope = support::scope(); let context = support::context();
        let backend = FileBackend::new(&path,FileOptions::local_ext4())?;
        let hub = KnowledgeHub::create(backend,context.clone(),scope.clone()).await?;
        let a = hub.head(context.clone()).await?;
        let request = support::seed(scope.clone(),a,"self-definition:1");
        assert!(hub.validate(context.clone(),request.clone()).await?.is_valid());
        let receipt = hub.commit(context.clone(),request.clone()).await?;
        let seed = hub.read_snapshot(context.clone(),receipt.head.clone()).await?;
        assert!(seed.seed_complete()); assert_eq!(seed.sources[0].content.bytes,support::ONTOLOGY);
        let assertion = hub.read_assertion(context.clone(),receipt.head.clone(),seed.assertions[0].reference.clone()).await?;
        assert_eq!(assertion,seed.assertions[0]);
        hub.drain().await; drop(hub);
        let backend = FileBackend::new(&path,FileOptions::local_ext4())?;
        let reopened = KnowledgeHub::open(backend,context.clone(),scope).await?;
        assert_eq!(reopened.read_snapshot(context.clone(),receipt.head.clone()).await?,seed);
        assert_eq!(reopened.operation_result(context.clone(),request.operation_id.clone()).await?,OperationOutcome::Committed(Box::new(receipt.clone())));
        assert_eq!(reopened.commit(context.clone(),request).await?,receipt);
        println!("snapshot={} sequence={} source_sha256={} source_bytes={} elements={} assertions={}",
            receipt.head.0,receipt.sequence,seed.sources[0].content.reference.sha256,seed.sources[0].content.bytes.len(),seed.elements.len(),seed.assertions.len());
        println!("Original source, Markdown, exact revisions and receipt survived reopen and exact retry.");
        println!("operation={} fingerprint={}",receipt.operation_id,receipt.fingerprint);
        for d in &seed.documents {
            println!("document={} revision={} sha256={} bytes={}",d.reference.id,d.reference.revision,d.content.reference.sha256,d.content.bytes.len());
        }
        for e in &seed.elements { println!("element={} revision={}",e.reference.id,e.reference.revision); }
        for a in &seed.assertions { println!("assertion={} revision={} from={} to={}",a.reference.id,a.reference.revision,a.from.revision,a.to.revision); }
        reopened.drain().await;
        Ok::<_,Error>(())
    })?;
    Ok(())
}
