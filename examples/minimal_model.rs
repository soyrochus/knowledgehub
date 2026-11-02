//! Minimal model creation uses the same complete API as every other consumer.
mod support;
use knowledgehub::{backends::files::*, *};
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: seed /absolute/unused/store")?;
    let backend = FileBackend::new(path, FileOptions::native_local()?)?;
    let hub = KnowledgeHub::create(backend, support::context(), support::scope()).await?;
    let (head, state) = hub.snapshot(support::context()).await?;
    let mut change = Change::at(&state, head, "register-catalogue");
    change.operations = builtin_catalogue()?.operations();
    let receipt = hub.commit(support::context(), change).await?;
    println!("{}", serde_json::to_string_pretty(&receipt)?);
    hub.drain().await;
    Ok(())
}
