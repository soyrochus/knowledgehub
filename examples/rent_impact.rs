#[path = "neighbour_support/mod.rs"]
mod fixture;
mod rent_support;
#[path = "ontology_support/mod.rs"]
mod specimens;
use knowledgehub::Error;
#[tokio::main]
async fn main() -> Result<(), Error> {
    let path = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example rent_impact -- /absolute/absent/fixture-directory");
    let (_, hub) = rent_support::create(std::path::Path::new(&path)).await?;
    let page = hub
        .model_impact(fixture::context(), rent_support::query())
        .await?;
    println!("{}", serde_json::to_string_pretty(&page).unwrap());
    Ok(())
}
