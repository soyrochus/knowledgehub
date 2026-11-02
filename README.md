# KnowledgeHub

`knowledgehub` 0.10.0 is an unpublished embedded Rust library for governed, source-bound system definitions. Its ordinary `KnowledgeHub` API supports the complete System Definition Ontology: 17 kernel kinds, 128 family types, 32 catalogue relationships and six explicit prose refinements.

The single current file backend uses `knowledgehub-files/4`, shared immutable objects, sealed packs and bounded derived indexes. Earlier APIs and stores have no conversion or compatibility facade. Existing current-store revisions, baselines, exact source bytes and original operation outcomes remain immutable.

Use the existing Rust 1.99.1 toolchain; install no compiler. The trusted host supplies current scoped policy, qualified actor identity and its own Tokio runtime. Declared profiles are coordinated single-host Linux x86_64/local ext4 and macOS arm64/local APFS; see [native profile checks and persistence](design/macos-file-backend.md). Observed checks and the user's platform-compliance assumption remain separate.

```rust,no_run
use knowledgehub::{KnowledgeHub, Context, ModelScope, State, Error};
use knowledgehub::backends::files::FileBackend;

async fn inspect(backend: FileBackend, context: Context, scope: ModelScope) -> Result<State, Error> {
    let hub = KnowledgeHub::open(backend, context.clone(), scope).await?;
    let (_, state) = hub.snapshot(context).await?;
    hub.drain().await;
    Ok(state)
}
```

Create a backend with `FileBackend::new(path, FileOptions::native_local()?)`. Register `builtin_catalogue().operations()` through a guarded `Change`; ontology/profile selection remains explicit model data. The API includes exact/range reads, immutable successors, baseline comparison, typed selection, batch neighbours, deterministic traversal, literal text entry and composite impact queries. Reads use owned values, finite budgets, current authority and exact selection; modelled roles or agents do not confer runtime capabilities.

The four [0.10.0 changes](delivery/slice-plan.md#single-full-api-sequence--0100-cutover)
are implemented in order (22/22 tasks). [Exact verification evidence](delivery/evidence/hub-rebuild/20261101T230838Z-9b8133d6/README.md)
records 95 Rust tests, 3 documentation tests, 17 host tests and successful format,
Clippy, release builds and canonical dataset checks.

The fresh persisted [hub](hub/README.md) has 9,578 selected records,
9,584 retained revisions, eight generations and three baselines.
It uses 42.02 MB encoded regular-file storage, including derived
indexes and location metadata; exact encoded/allocated/peak figures are in the evidence.
The user's previous old_hub remains byte-for-byte unchanged and outside current capture,
mutation and import. Repository edits do not automatically refresh either captured model.

Inspect the current store through the public API:

~~~sh
cargo run --offline --release --example inspect_self_model
cargo run --offline --release --example inspect_self_model -- --verify
~~~

The [ontology proposal](source/system-definition-ontology-proposal.md) and [Practical Design Manual](source/practical-design-manual.md) retain their original draft/source status. The manual's Memento examples are not library requirements. Definition ownership remains in repository documents; structural validity, explicit draft gaps, observed technical checks, human acceptance and authority handover are distinct.
