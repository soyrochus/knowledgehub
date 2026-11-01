# KH-SL-013 — sealed derived index packs

Observed on 1 November 2026 under the existing Rust 1.99.1 and declared local ext4 profile. [Change](../../../openspec/changes/archive/2026-11-01-pack-derived-index-storage/proposal.md), [format and bounds](../../../design/packed-cache-storage.md). This run changes disposable index storage against an existing historical model; it does not reset or recapture the canonical Hub.

## Actual canonical data

The [first inspector output](canonical-inspect.json) runs the public neighbour, literal text and composite APIs, original-outcome lookup, A/B comparison and full scrub. [Reopen output](canonical-reopen.json) repeats the semantic checks from a new backend. Head remains `578baab09fc19ca06c7e35ea0486c19269b8be90fb580531d9a8cbeacb6aead6`, with 9,064 selected records, 9,070 retained revisions, eight generations and 3,738 explicit draft gaps. Structural validity is true; purpose completeness is false.

[Immutability comparison](immutability.json) checks all 22,842 accepted/non-cache regular files against the [before inventory](accepted-before.json): identical paths, bytes and hashes, totalling 16,012,885 encoded bytes. Both inspections preserve A/B explanations, changed identities, load/revise receipts and replay domain. No accepted objects were written. The original 426 files under `orig_hub/` also match the earlier relocation inventory. Source/model authority is unchanged.

## Allocated and encoded space

[Before](space-before.json) and [after](space-after.json) count regular files, allocated unique inodes (`st_blocks * 512`) and encoded file bytes. Directories and Git are excluded. Values below are bytes; the old incomplete cache and new completed cache have different coverage, so this is an operational before/after comparison rather than a same-object compression benchmark.

| Cache | Files | Encoded bytes | Allocated bytes | Peak reserved bytes | Quota bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| Previous neighbour attempt, incomplete | 65,536 | 34,782,560 | 268,435,456 | quota exhausted | 268,435,456 |
| Packed neighbours, complete | 306 | 25,728,424 | 26,177,536 | 26,177,536 | 268,435,456 |
| Packed text, complete | 22 | 1,227,581 | 1,269,760 | 1,269,760 | 134,217,728 |

Neighbours use 152 variable-size sealed packs for 71,283 distinct immutable nodes, indexing 9,063 baseline owners, 65,081 facts and 130,162 directional postings. Pack data occupies 8,129,593 bytes; JSON location sidecars occupy 17,587,689 bytes. Text uses ten packs for 2,675 nodes: 561,681 pack bytes and 664,286 sidecar bytes. Sidecars are now the larger component; they remain bounded, disposable and reused across healthy batches.

The first neighbour preparation took 14.17 seconds; a warm direct lookup took 13.79 ms. A complete three-hop Requirement → UseCase → FunctionDefinition → AggregateDefinition candidate-dependency request took 141.45 ms, examined three associations through three local batches and used zero full-State materialisations. These are actual host observations, not latency guarantees. The phrase entry verifies exact narrative bytes, and repeated dependencies, hits and coverage agree.

On reopening, the first neighbour batch loaded 17,597,923 metadata bytes once. Its following warm batch loaded zero metadata bytes. Text loaded 665,006 metadata bytes once; the composite request and its repeat both loaded zero metadata bytes. Warm composite work still reads and verifies bounded blocks (5,630,275 encoded index bytes for this route); metadata reuse does not eliminate block decompression. The existing JSON indexes and one-block cache are retained.

## Verification

The focused packed tests exercise 66,000 nodes, mixed loose/packed reads, writer-local pending reads, object hashes, normal block bounds, a dedicated oversized block, unchanged closed files after later writes, cold/warm sidecar work, independent backend publication/eviction, sidecar reconstruction and catalog-capacity failure. Neighbour fault cases additionally interrupt all four new pack publication stages after durable source commits and verify original receipts. Existing public independent-oracle tests exercise generic facts, ordering, exact scopes, legacy equivalence, late links, source successors, corruption, cursors, snippets and origin witnesses.

The [final native batch](native-final/checked.json) passed 100 Rust tests across all targets, one doctest and 11 Python host tests. Formatting, Clippy with warnings denied and release example builds passed. [native.log](native-final/native.log) retains execution output; the checked manifest binds exact inputs, logs and executable hashes. The actual inspector binary in [canonical-run.json](canonical-run.json) matches that final checked release build. Final task/status prose is recorded separately in [post-verification-sources.json](post-verification-sources.json); implementation and test bytes remain the checked inputs.

Reproduce the read-only canonical verification after building the inspector:

~~~sh
cargo build --offline --release --example inspect_self_model
target/release/examples/inspect_self_model --format files3 --verify
~~~

Do not add `--compact` or run a loader against `hub/` for this check. The [old failed rebuild run](../hub-rebuild/20261101T123801Z-d7b406f3/status.json) remains failed/incomplete and its [deferred tasks](../../../openspec/deferred/rebuild-canonical-hub-and-verify/tasks.md) remain 7/11. This evidence does not complete that old plan, create a human AcceptanceRecord, transfer document authority or establish power-loss/other-filesystem behaviour.
