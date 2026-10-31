# KH-SL-010 technical evidence

31 October 2026. The user committed KH-SL-009 and explicitly authorised “Continue with index-ontology-neighbours”. This records implemented behaviour and automated verification, not human acceptance, new impact/search operations or authority handover.

## Delivered boundary

The [API guide](../../../design/neighbour-api.md) describes the additive owned batch neighbour operations, exact selectors/origins, categories, selected/closure fact scopes, projections, finite preparation/read/cursor bounds and explicit files/3 capability. The model extractor handles registered/inherited/nested schemas, tagged Any values, lists/sets/unions, native references, qualifiers, provenance and representation links without guessing identifiers from strings. Parallel facts and repeated occurrences remain distinct; directional self-loops are coalesced in results.

The file adapter retains endpoint → selector → ordered posting maps in both directions under a separate disposable namespace. Complete manifests bind exact scope/membership/ontology/profile and versioned extraction/category rules. Accepted changes update prepared indexes from owner-fact removals/additions after durable graph publication; failed cache installation cannot replace the original receipt. Batched trie changes and sequence appends write final changed nodes rather than intermediate roots.

Ready reads use pinned paths and bounded exact hydration. They verify needed source records, return exact historical pins with their selection status and never call full State/history materialisation. New neighbours are explicitly unsupported on legacy formats; files/3 legacy assertion traversal preserves its existing ordering, filters, depth and pagination behaviour.

Existing model admission is preserved: selected assertions require selected exact endpoints. The public example revises assertions coherently, demonstrates unselected historical property targets and explicitly restores an older endpoint selection while keeping a newer revision retained. A profile/schema switch likewise revises selected owners' type pins coherently; old baselines retain their original schemas and facts.

## Executed verification

| Command / run | Observed result |
| --- | --- |
| cargo test --offline --lib -- --nocapture | 37 passed, zero failed/ignored. Includes old files/1–3 publication/maintenance faults, six new cache/cursor/selective-read tests and batched tree/append tests. [Log](internal-tests.log). |
| cargo test --offline --test neighbours -- --nocapture | Six public tests passed, zero failed/ignored. [Log](public-tests.log). |
| cargo test --offline --test ontology_contract --test public_contract --test shared_storage -- --nocapture | 25 + 12 + 1 passed, zero failed/ignored. Existing codecs, hashes, original outcomes, all ontology contracts and packed storage remain compatible. [Log](compatibility-tests.log). |
| cargo test --offline --doc | One doctest passed. [Log](doctests.log). |
| cargo fmt --all -- --check | Exit 0. [Log](format.log). |
| cargo clippy --offline --all-targets -- -D warnings | Exit 0; all targets/examples compiled. [Log](clippy.log). |
| cargo build --offline --release --example neighbours --example neighbour_benchmark | Exit 0 with existing Rust/Cargo 1.99.1; no compiler or new dependencies installed. [Log](release-build.log). |
| openspec validate index-ontology-neighbours --strict --json | Valid, zero issues. [Result](openspec-validation.json). |
| Public release neighbours example and neighbour_benchmark | Both exited 0 on fresh target/ stores. [Walkthrough](public-example.log), [measurements](benchmark.json), [progress](benchmark.log). |

Total: **81 native tests and one doctest passed**. [Build/input hashes](build-inputs.json) bind the implementation, tests, fixtures, examples, original sources, dependency lockfile and release binaries to observed toolchain/kernel/mount details. [Verification manifest](verification.json) records exact commands and result hashes. Earlier evidence remains historical and was not relabelled.

The independent oracle flattens direct public record/tagged/native values and resolves schema paths without importing the production extractor or private index. It compares all eligible recorded facts across every stored endpoint plus an actual identity projection, as well as directional/type/category variants. It explicitly treats positional role-name arrays as declarations, not reference objects. Other checks cover late links, successors, select/unselect, clean rebuild equivalence, schema/profile changes, exact old views, empty/incomplete budgets, projection bytes, full cursor intent, head movement, cache eviction, current-policy denial, expiry/capacity and legacy traversal equivalence.

Injected cache failures occur after node persistence, staged-manifest persistence and complete manifest installation. Original commit lookup/retry remains exact. Corrupt/wrong-view manifests and missing derived nodes rebuild; missing required authoritative data fails integrity. A ready lookup succeeds while an unrelated original chunk is deliberately missing and its materialisation counter stays unchanged. Reader/eviction coordination is tested on both current-thread and two-worker Tokio hosts. These new observations are cache fault/cancellation/coordination checks, alongside the existing process-SIGKILL suites; they are not new power-loss or other-filesystem evidence.

## Measured direct-neighbour work

The deterministic fixture registers an explicit new vocabulary, retains a fixed three-assertion neighbourhood for b and adds unrelated parallel self-loop assertions at c. A separate original artifact and later clock revisions exercise unrelated content/history. The benchmark records cold preparation separately, reopens a fresh backend for each first page, then repeats lookups on that backend. The OS page cache is uncontrolled and verification shared the host; timings are observations, not portable latency guarantees.

| Unrelated assertions | Indexed facts / directional postings | Needed record reads | Examined candidates / physical postings read | Index objects read | Source objects read | Cache encoded / allocated regular-file bytes |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 3,772 / 7,544 | 7 | 3 / 6 | 8 | 26 | 836,577 / 6,311,936 |
| 100 | 4,372 / 8,744 | 7 | 3 / 6 | 8 | 26 | 1,168,771 / 8,912,896 |
| 1,000 | 9,772 / 19,544 | 7 | 3 / 6 | 9 | 37 | 4,038,052 / 32,350,208 |
| 3,000 | 21,772 / 43,544 | 7 | 3 / 6 | 9 | 38 | 10,290,386 / 83,107,840 |

Every measured fixed lookup returned the same three associations, prepared zero owners and materialised zero full states. Index growth is exactly six recorded facts/twelve directional postings per added assertion: its model association plus actual native/provenance references. It does not grow with possible future routes. Extra source/node reads reflect trie paths, not unrelated record or history scans.

The initial preparation read 222 owners and built 3,772 facts in 13,701,683 microseconds. At later checkpoints the prepared indexes were updated incrementally: explicit readiness checks took 1,075/985/1,333 microseconds and rebuilt no owners. Fresh-backend pages took 8,814/6,297/6,381/6,564 microseconds. Subsequent pages retained identical structural work; all repetitions/timings are recorded in the JSON.

Eight additional unrelated generations left fixed lookup work at seven records, nine index objects, 38 source objects and three examined candidates. The high-fan-out read returned 50 results, reported a page bound and remained unexhausted, using 50 examined candidates/physical posting reads, 53 needed records and six index objects. It materialised zero full states. The whole benchmark process peaked at 139,083,776 bytes RSS, including graph validation/commits and owned fixtures, not merely index reads.

The cache uses compressed loose immutable nodes and retains previous advertised views until bounded eviction. Its allocated footprint is materially larger than encoded bytes because of filesystem blocks; the table includes actual native/administrative facts and old shared roots. These numbers are not a canonical Hub size prediction. The cache is capped at 256 MiB allocated regular files/32 views and remains independently disposable. Authoritative packing does not automatically pack this separate cache.

Reproduce with unused absolute paths and existing parents:

~~~sh
cargo run --offline --release --example neighbours -- /absolute/unused/public-demo
cargo run --offline --release --example neighbour_benchmark -- /absolute/unused/benchmark
~~~

The actual successful direct-binary paths were target/neighbour-evidence/public-demo-current and target/neighbour-evidence/benchmark-current. [Canonical audit](canonical-inventory.json): the initial git status showed hub/, while the final readonly check found the original store under orig_hub/ with all 426 inventoried files byte-exact and hub/ absent. This implementation ran only isolated target/ stores and did not perform the canonical reset/rebuild or alter that observed layout.

KH-SL-011 remains the separate text-entry/multi-hop impact change; KH-SL-012 owns canonical regeneration and its new full-suite/model evidence. No graph inference, AI execution, new adapter, automatic archive or human acceptance is claimed.
