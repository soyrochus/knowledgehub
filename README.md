# KnowledgeHub

[![Version](https://img.shields.io/badge/version-0.10.0-blue)](Cargo.toml)
[![Rust edition](https://img.shields.io/badge/Rust-2024_edition-orange?logo=rust)](Cargo.toml)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-in_development-yellow)](https://github.com/soyrochus/knowledgehub)

**A Rust library for structured, versioned, and traceable system knowledge.**

KnowledgeHub helps applications keep the knowledge behind a software system connected:
business goals, requirements, domain concepts, behaviour, interfaces, implementation
references and verification evidence. Each definition has a stable identity, an immutable
revision history and explicit relationships to the sources and decisions behind it.

For example, you can connect a requirement to the command that fulfils it, the domain rule
it must preserve and the tests that check it. When the requirement changes, earlier
baselines still resolve to the exact definitions and source bytes they originally used.

KnowledgeHub is an embedded library with a local file backend; no database server is
required. The repository is [soyrochus/knowledgehub](https://github.com/soyrochus/knowledgehub).
[Modelarium](https://github.com/soyrochus/modelarium) is a project using the library.

## What you can do

- **Build connected system models.** Use the built-in System Definition Ontology to
  describe business intent, journeys, domain models, application behaviour, information,
  visual design, architecture, operations and delivery. Start with the parts you need.
- **Keep definitions tied to evidence.** Retain original artifact bytes, content hashes
  and precise source anchors. Record interpretations, ownership and reviews separately.
- **Preserve exact history.** Create successor revisions and named baselines, compare
  versions and read earlier definitions without silently substituting newer content.
- **Make coherent changes.** Commit related records, narrative and provenance together,
  with current authority checks, expected-head guards and recoverable operation receipts.
  Retrying the same request can recover its original result.
- **Explore relationships and change impact.** Use typed selection, batch neighbour
  queries, deterministic bounded traversal, literal text search and composite impact queries.

The built-in catalogue contains 17 kernel kinds, 128 family types, 32 catalogue
relationships and six explicit prose refinements. These share the same public
`knowledgehub::KnowledgeHub` API and validation rules.

## Getting started

Version **0.10.0** is in development and has not been published to crates.io. Use the
Git repository or a local checkout as a dependency. The recorded verification runs use
Rust **1.99.1**; the minimum compiler declared in `Cargo.toml` has not been separately verified.

The file backend currently requires **macOS arm64 on writable local APFS** or
**Linux x86_64 on local ext4**, with coordinated access on a single host. Choose a store
outside cloud-sync folders. See [storage and platform support](#storage-and-platform-support)
for the verification boundary.

### Run the smallest example

```sh
git clone https://github.com/soyrochus/knowledgehub.git
cd knowledgehub
mkdir -p target
cargo run --release --example minimal_model -- "$PWD/target/first-hub"
```

Use an unused absolute store path whose parent directory already exists. The example
creates a Hub, registers the built-in ontology through an atomic change and prints the
commit receipt. To run it again, choose another unused path.

### Add it to an application

```toml
[dependencies]
knowledgehub = { git = "https://github.com/soyrochus/knowledgehub", version = "0.10.0" }
tokio = { version = "1", features = ["rt", "macros"] }
```

For development alongside a checkout, replace the Git dependency with
`knowledgehub = { path = "../knowledgehub" }`.

Your host application supplies a Tokio runtime, a `Context` containing the authenticated
actor and current `Policy`, and a `ModelScope` identifying the Hub, model, branch and
owner. Actor IDs must be qualified, such as `host:maintainer`. A policy checks the
requested action in that scope; storing a Role or Permission record does not grant access.

The following function shows the creation and registration flow once those host values
are available. Call it inside your runtime with a fresh store path:

```rust,no_run
use std::path::PathBuf;
use knowledgehub::{builtin_catalogue, Change, Context, Error, KnowledgeHub, ModelScope};
use knowledgehub::backends::files::{FileBackend, FileOptions};

async fn create_model(
    path: PathBuf,
    context: Context,
    scope: ModelScope,
) -> Result<(), Error> {
    let backend = FileBackend::new(path, FileOptions::native_local()?)?;
    let hub = KnowledgeHub::create(backend, context.clone(), scope).await?;

    let (head, state) = hub.snapshot(context.clone()).await?;
    let mut change = Change::at(&state, head, "register-catalogue");
    change.operations = builtin_catalogue()?.operations();
    let receipt = hub.commit(context, change).await?;

    println!("Accepted head: {:?}", receipt.head);
    hub.drain().await;
    Ok(())
}
```

See [minimal_model.rs](examples/minimal_model.rs) and its
[host setup](examples/support/mod.rs) for a complete executable example. The example
policy allows every action for demonstration; applications should supply their own policy.
Reopen an existing instance with `KnowledgeHub::open`. Keep the runtime alive until
`hub.drain().await` completes.

## A few core concepts

| Concept | What it means |
| --- | --- |
| Identity and revision | An identity survives renames. A published revision is immutable; an edit creates an exact successor. |
| Typed relationship | A versioned assertion with declared direction, endpoint types, qualifiers and provenance. |
| Source and anchor | Exact retained source content and the location that supports a definition or assertion. |
| Baseline | An explicit selection of exact revisions and their supporting closure, suitable for later inspection and comparison. |
| Change and receipt | A guarded atomic request and its retained outcome. Preserve the original request and operation ID to resolve an uncertain commit. |

Validation checks structure, references, ownership and applicable constraints. A partial
model can record explicit draft gaps. Passing validation establishes structural validity;
review, working implementation, observed test results and human acceptance remain
separate facts with their own scope.

## Examples and self-description

| Example | What it demonstrates |
| --- | --- |
| [minimal_model](examples/minimal_model.rs) | Create a store and register the ontology. |
| [shared_storage](examples/shared_storage.rs) | Commit sourced records, create baselines A/B, read historical content, retry an operation and reopen the store. |
| [neighbours](examples/neighbours.rs) | Read typed connections through the batch neighbour API. |
| [rent_impact](examples/rent_impact.rs) | Explore a fictional model through text and composite impact queries. |
| [inspect_self_model](examples/inspect_self_model.rs) | Inspect KnowledgeHub's persisted self-description through the ordinary public API. |

The [included self-description](hub/README.md) captures the library's own definitions,
delivery slices, specifications and evidence. Its recorded checkpoint contains 9,578
selected records, 9,584 retained revisions, eight generations and three baselines.
Inspect it from the repository root:

```sh
cargo run --release --example inspect_self_model
cargo run --release --example inspect_self_model -- --verify
```

The second command also checks actual neighbour, text and impact queries and performs
a full storage scrub. This is a historical capture: editing repository files does not
refresh its sources or transfer definition ownership from the repository to the Hub.

## Storage and platform support

The default `FileBackend` uses **`knowledgehub-files/4`**. It retains Markdown narrative,
graph records and original artifacts as shared immutable objects, with sealed packs and
rebuildable derived indexes. Use public changes to edit accepted knowledge; direct edits
to backend files bypass the supported protocol. Earlier store formats are refused;
no compatibility reader or converter is provided.

`FileOptions::native_local()` selects the native platform declaration; create/open also
check the actual storage location. Network filesystems, external/cloud-sync writers and
simultaneous writers on different hosts are outside the supported profile.

The [macOS verification report](delivery/evidence/macos-file-backend/README.md) records
native APFS checks, runtime journeys, process interruption and injected persistence
failures. A fresh Linux regression run and transfers in both directions between native
Linux and macOS fixtures remain open for that change. Earlier
[0.10.0 cutover evidence](delivery/evidence/v010/README.md) records the Linux/ext4 batch.
These results do not establish power-loss or hardware durability guarantees.

Public model contracts are independent of storage layout. The current implementation
provides one file adapter; MongoDB/PostgreSQL adapters, HTTP services, remote SDKs and
FFI remain deferred.

## Documentation and development

- [Current API contract](design/typed-ontology-api.md): exact revisions, changes,
  ownership, queries, budgets and policy checks.
- [File storage guide](design/files3-guide.md) and [native profiles](design/macos-file-backend.md):
  storage, maintenance, persistence and supported environments.
- [Ontology proposal](source/system-definition-ontology-proposal.md): the source model
  and rationale, retaining its draft status.
- [Practical Design Manual](source/practical-design-manual.md): the domain, journey and
  specification method. Its Memento examples are illustrative, not library requirements.
- [Domain analysis](design/domain-analysis.md), [delivery plan](delivery/slice-plan.md)
  and [self-hosting plan](design/self-hosting-plan.md): project responsibilities,
  implementation sequence and definition ownership.
- [Verification evidence](delivery/evidence/v010/README.md) and
  [native macOS results](delivery/evidence/macos-file-backend/README.md): scoped,
  inspectable execution results and remaining verification work.

To build the local API documentation, run `cargo doc --no-deps --open`. Before submitting
a code change, run the following on a supported local filesystem:

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
```

## Principles of Participation

Everyone is invited and welcome to contribute: open issues, propose pull requests, share ideas, or help improve documentation. Participation is open to all, regardless of background or viewpoint.

This project follows the [FOSS Pluralism Manifesto](./FOSS_PLURALISM_MANIFESTO.md), which affirms respect for people, freedom to critique ideas, and space for diverse perspectives.

## License and Copyright

Copyright (c) 2026, Iwan van der Kleijn

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
