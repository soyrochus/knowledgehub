//! Read-only local consumer. No terminal types are exported by the library.
#[path = "../terminal_client_host/mod.rs"]
mod host;
fn main() {
    if let Err(e) = host::run() {
        eprintln!("knowledgehub-client-host: {e}");
        std::process::exit(1);
    }
}
