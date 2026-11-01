# KnowledgeHub

`knowledgehub` is an embedded Rust library for governed, source-bound system
definitions. The explicit files/3 backend shares immutable storage and implements the System Definition Ontology:
17 kernel kinds, 128 family types, 32 catalogue relationships and 6 explicit
prose refinements. It supports typed values and constraints, exact binary and
Markdown artifacts, immutable revisions, named baselines and bounded navigation.
A public consumer loads KnowledgeHub's own sourced model, records observed native
evidence, revises an explanation and recovers both exact baselines after reopening.

Implementation was authorised on 31 October 2026. The self-model retains explicit
draft gaps and source status; human acceptance and authority handover remain pending.
The [ontology proposal](source/system-definition-ontology-proposal.md) remains the
source draft. The [Practical Design Manual](source/practical-design-manual.md)
supplies the method; its Memento examples are not KnowledgeHub requirements.

## Inspect the canonical self-model

The repository's actual persisted KnowledgeHub is in [`hub/`](hub/).
It contains the ontology catalogue and KnowledgeHub's own sourced model, exact
artifacts, immutable revision history, A/B baselines and original operation receipts.
It is repository data, preserved independently of Cargo's `target/` directory.

Open it through the public library API:

```sh
cargo run --offline --release --example inspect_self_model -- --format files3
```

This read-only example reports the model, draft gaps and original receipts, and
reads both versions of the atomic-change explanation. The stored model captures
the verified source snapshot described in its evidence; it is not automatically
updated when repository files change. See the [files/3 guide](design/files3-guide.md)
for its layout and update rules.

Neighbour and text indexes now use [sealed variable-size packs](design/packed-cache-storage.md)
with bounded compression blocks. The existing canonical data passed public indexed
queries, reopen and full scrub without changing accepted files; its completed neighbour
cache uses about 25 MiB. [Evidence](delivery/evidence/packed-cache/README.md) records
the measurements and full checks. The unfinished canonical rebuild remains
[deferred at 7/11](openspec/deferred/rebuild-canonical-hub-and-verify/tasks.md);
this storage change did not refresh the captured model.

## Reproduce the full public walkthrough

Use the existing Rust compiler, Python 3 and installed Poppler `pdftotext`.
Choose fresh absolute store paths with an existing parent on the supported local
ext4 platform. These commands compile and run with the existing toolchain:

```sh
python3 scripts/verify_full_ontology.py --run-dir delivery/evidence/hub-rebuild/manual-run
python3 examples/capture_self_model.py --log delivery/evidence/hub-rebuild/manual-run/native.log --checked-manifest delivery/evidence/hub-rebuild/manual-run/checked.json
mkdir -p target/ontology-demo
cargo run --offline --release --example catalogue_conformance -- --format files3 "$PWD/target/ontology-demo/catalogue"
cargo run --offline --release --example full_self_description -- --format files3 target/self-model-input.json "$PWD/target/ontology-demo/library"
```

The first consumer retains complete synthetic specimens for every family and
signature. The second captures exact project sources, original PDF/Markdown and
PDF extraction manifests, loads the applicable self-model, follows its real
requirement/case/run path, creates A/B, closes/reopens, compares them and recovers
original receipts. Its result JSON contains group bindings, Q01–Q08 answers and
explicit gaps. Structural conformance does not manufacture execution or acceptance.
These commands create separate disposable demonstrations. The canonical `hub/`
already exists; do not pass it as the new-store destination to either loader.

Read the [typed API and version contract](design/typed-ontology-api.md) and
[self-model walkthrough](design/self-model-consumer.md) for exact addresses,
authoring, source capture, query budgets and review boundaries.

## Use the typed library

Host composition creates `FileBackend::new_shared_ontology` for files/3;
`FileBackend::new_ontology` retains files/2 compatibility. The host supplies
qualified actor identity, a current scoped `Policy` and its own Tokio runtime.
Stored roles never grant runtime access. Ontology registration has a separate
`RegisterOntology` admission action.

```rust,no_run
use knowledgehub::{Context, Error};
use knowledgehub::ontology::{ModelScope, OntologyHub, State};
use knowledgehub::backends::files::FileBackend;

async fn inspect(
    backend: FileBackend,
    context: Context,
    scope: ModelScope,
) -> Result<State, Error> {
    let hub = OntologyHub::open(backend, context.clone(), scope).await?;
    let (_, snapshot) = hub.snapshot(context).await?;
    hub.drain().await;
    Ok(snapshot)
}
```

Public I/O operations return owned values. A caller retains `Change::at` and its
complete ordered operations before validation/commit. Guarded publication covers
typed records, exact artifacts, selections and the original actor-bound receipt.
Exact retries recover that receipt before stale-base checks. Current access still
applies. The host drains admitted work before stopping Tokio; defaults are 4 active
and 32 waiting blocking jobs per backend. Both scheduler configurations are tested.

## Deterministic dependency reads

The additive files/3 operations `search_text`, `traverse` and `model_impact`
combine literal text entry, bounded typed traversal and exact evidence. A composite
request returns grouped candidate dependencies in one pinned view, with explicit
ambiguity, coverage gaps and work limits. The engine follows recorded facts and
does not infer whether a database migration is necessary.

Run the isolated fictional rent example with an absent absolute destination:

~~~sh
cargo run --offline --release --example rent_impact -- /absolute/unused/rent-fixture
~~~

See the [query contract and limits](design/deterministic-query-api.md),
[executable fixture/request](examples/rent_support/mod.rs) and
[technical evidence](delivery/evidence/impact/README.md).
The [canonical rebuild](design/canonical-rebuild.md) is postponed after its
derived-cache allocation failure. The active [sealed-pack change](openspec/changes/pack-derived-index-storage/proposal.md)
preserves existing data and verifies compact derived indexes; it performs no reset.

## Compatibility and supported boundary

The original `KnowledgeHub` and `FileBackend::new` preserve the files/1 seed API,
canonical hashes and original receipts. Its independent walkthrough remains:

```sh
cargo run --offline --example seed -- /absolute/new/seed/store
```

Files/2 explicitly opts into new file/request/record codecs and preallocated opaque
revision tokens with independent content digests. It does not migrate files/1 or
copy its operation identities into a new replay domain. Each API refuses the other
format. The [first-slice contract](design/file-backend.md) and its
[historical evidence](delivery/evidence/kh-sl-001.md) remain separate.

One crate and default file adapter support coordinated single-host writers on
Linux x86_64/local ext4. Stable locks, immutable generations, synced atomic head
publication and retained original receipts protect accepted changes. Query pages,
depth, visited subjects/edges and temporary cursor capacity/lifetime are bounded.
The larger sourced model is a scoped demonstration, not a scale benchmark.

Rust 2024 declares a Rust 1.85 minimum. Verification uses the existing Rust 1.99.1
at the user's direction, with no compiler installation; minimum-compiler execution
is not claimed. Platform compliance is the user's assumption. Native observations
and process-interruption checks do not establish power-loss or kernel-crash durability.

Portable interchange, broader query/review workflows, MongoDB/PostgreSQL adapters,
services, remote SDKs, FFI and publication remain deferred. Repository definitions
retain their authority until an explicit scoped handover. Technical checks and
project acceptance remain distinct.

## Development model

[Domain analysis](design/domain-analysis.md) →
[journeys and outcome slices](delivery/slice-plan.md) →
[bounded OpenSpec change](openspec/changes/retain-first-self-definition/proposal.md) →
authorised implementation and evidence → review and revised sequence.
The [self-hosting plan](design/self-hosting-plan.md) owns bootstrap authority and
decisions; [implementation record](design/implementation-record.md) records the
authorisation and contract review. Original material under `source/` remains
unchanged. Implementation, technical verification and human acceptance remain
separate facts.

## Principles of Participation

Everyone is invited and welcome to contribute: open issues, propose pull requests, share ideas, or help improve documentation. Participation is open to all, regardless of background or viewpoint. 

This project follows the [FOSS Pluralism Manifesto](./FOSS_PLURALISM_MANIFESTO.md), which affirms respect for people, freedom to critique ideas, and space for diverse perspectives. 

## License and Copyright

Copyright (c) 2026, Iwan van der Kleijn

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
