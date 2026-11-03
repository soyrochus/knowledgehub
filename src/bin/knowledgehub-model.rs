//! Deterministic local authoring host. Source interpretation remains external.
#[path = "../model_authoring.rs"]
mod authoring;

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("host runtime");
    match runtime.block_on(authoring::run(std::env::args().skip(1).collect())) {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).unwrap()),
        Err(error) => {
            eprintln!(
                "{}",
                serde_json::json!({"status":"failed","diagnostic":format!("{error:?}")})
            );
            std::process::exit(1);
        }
    }
}
