# C8 native publication history — implementation and handoff

Date: 4 November 2026. User-authorised change: `expose-native-model-history`.
Base commit: `c2b8258bf71a61b0625c9721707f27511402fa09`, plus the recorded working-tree implementation.
[Implementation inputs](implementation-inputs.json) bind source files by SHA-256;
[toolchain observations](toolchain.txt) record existing Rust 1.99.1, macOS 26.6.2
build 25G83 and aarch64-apple-darwin. No compiler or dependency was installed.

The native extension supplies original publication lookup, bounded descending discovery,
affected-vector paging, current authority and finite shared continuations through the
sole `knowledgehub::KnowledgeHub` API. It preserves catalogue-3,
`knowledgehub-record/3`, `knowledgehub-request/3`, `knowledgehub-files/4`, original
receipt contents/fingerprints and actor-bound mutation recovery. The new owned response
envelope is `knowledgehub-history/1`, independently of stored semantic codecs and any
future consumer transport.

## Delivered public contract

The [consumer guide](../../../../design/native-model-history.md) contains all exact
exported signatures, request/result fields, runnable commands and consumer boundaries.

```rust,ignore
pub async fn publication(&self, context: Context, request: Publication)
    -> Result<PublicationResult, Error>;
pub async fn publications(&self, context: Context, request: Publications)
    -> Result<HistoryPage, Error>;
pub async fn publication_affected(&self, context: Context, request: PublicationAffected)
    -> Result<HistoryPage, Error>;
pub async fn continue_history(&self, context: Context, request: HistoryRequest,
    continuation: String) -> Result<HistoryPage, Error>;
pub async fn release_history(&self, context: Context, continuation: String)
    -> Result<(), Error>;
```

Every call requires current scoped **Read AND ReadHistory**. Opening separately requires
Open. Default policies deny; exhaustive Action matches must add ReadHistory; trusted
all-allow policies already grant it. Historical actor A is output attribution. Reader B
remains the caller. The example reader has only Open/Read/ReadHistory in its exact scope.
Tests separately grant B mutation/recovery permission to establish that actor mismatch
still returns OperationReuse, while A's exact stale-base retry returns the original receipt.

PublicationHeader retains scope, operation_id, actor, fingerprint_version, fingerprint,
previous_head, head and sequence unchanged, plus affected_count. Complete affected pages
reconstruct the original vector, preserving order and repetitions. NotPublishedAt proves
only non-membership in the verified anchor ancestry, including later-than-anchor operations.
Unknown/foreign/staged anchors, invalid requests, denial, unavailable policy/storage,
integrity/recovery failure and resource exhaustion have separate typed failure paths.

| Per-page/request field | Default | Hard maximum |
| --- | ---: | ---: |
| items | 100 | 1,000 |
| nodes | 256 | 8,192 |
| storage_bytes | 16 MiB | 128 MiB |
| output_bytes | 256 KiB | 4 MiB |

Checked counters report logical visited_nodes/object_reads, actual encoded_bytes_read,
decoded_bytes, their storage_bytes sum, and exact compact-JSON output_bytes including
metadata/counter/token overhead. Cache hits consume logical visits. Request accounting
starts before lease/metadata reads and includes anchor, operation, receipt, parent,
sequence, pack-index and decompression work. Failed minimum steps/indivisible values
return HistoryLimit instead of an empty endless continuation or shortened identifier.

Tokens bind actor, scope, exact anchor, kind/operation and every limit through an intent
fingerprint. They retain one logical snapshot/offset and no result vectors. All cursor
families share 64 entries and a 30-minute idle expiry. Expired, released, wrong-actor/
scope/intent and unknown tokens fail; new queries at the same retained anchor reproduce
its sequence after restart. Generic cursor release also checks history authority for
history tokens. Concurrent append and packing do not retarget an anchor.

Cold packed indexes/blocks remain existing indivisible metadata. Requests count their
read/decompression costs, may fail within finite budgets, and never trigger hidden
full-pack reconstruction. Missing/invalid derived offsets return IndexNotReady for
explicit maintenance. Existing open/recovery/maintenance retain separate costs.

## Public fixtures and retained handover

The executed [example output](examples.jsonl) contains exact heads, fingerprints,
operation IDs, baseline references, source hashes, limits and measured work. Both small
fixtures retain original and changed source-bound Requirements, named A/B selections,
exact predecessors and later unrelated operations. Each completed pinned traversal has
six publications, newest first. The concurrently appended seventh publication is outside
that anchor. The initial no-receipt boundary is identified without a fabricated event.

| Fixture | Exact retained read anchor | Publications |
| --- | --- | ---: |
| library-loans | `5f71d2f7b833739b88d39e556ff6782b59311c6901f88c27068b489e40ba2dcc` | 6 |
| garden-irrigation | `d8e74836effd70cf665f6cd677cee0f2773e86eabb12d38f5b52a2b5ab65825a` | 6 |
| retained-MM-SL-01-copy | `64c57fb97530cea24795aeab477874c50b5fb3116508a2246da45762c2a6035c` | 3 |

Known small-fixture IDs: `history:catalogue` (sequence 1, 217 affected),
`history:original-A` (2, 5), `history:successor-B` (3, 4), and
`history:later:0` through `history:later:2` (4–6, one each).
`history:concurrent-later` is accepted after the captured anchor and is correctly
NotPublishedAt there. Baselines are `<fixture>/baseline:A@1` and
`<fixture>/baseline:B@1`; exact source captures are `<fixture>/policy:source@1` and `@2`.
The JSON outputs carry the full owned references rather than this display notation.

| First page (two headers) | Visits | Encoded bytes | Decoded bytes | Storage bytes | Output bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| library-loans | 23 | 7479 | 7013 | 14492 | 1536 |
| garden-irrigation | 23 | 7476 | 7013 | 14489 | 1536 |

These measurements are observations for the recorded fixtures, not general latency or
constant-cost claims. Independent FileWork assertions verify request storage counts,
no full-State materialisation and no hidden index rebuild. The cold-pack test explicitly
clears the location cache after opening, so request-time discovery is included.
A 25-publication fixture exercises multi-page ancestry walking; a 400-reference storage
specimen checks selective header/count/range reads, repetitions and a last empty range.
A header/count read for that specimen needs two physical object reads, without reading
its affected leaves. Public catalogue reconstruction exceeds a page and checks offsets,
last pages and zero-affected accepted publications.

The [retained output](retained.jsonl) comes from a disposable copy of
`target/rich-design/apply-equipment-acceptance/hub`, with its original generation manifest
and successor handoff. Reader `host:history-reader` resolves both earlier original
`host:analyst` receipts exactly, discovers all three accepted publications, reads original
and edited baseline definitions, follows exact predecessors, compares A/B, and verifies
two unchanged source captures (655 original bytes total). There are no application audit
rows. Native-valid draft status and pending meaning review remain unchanged.

The documented standalone `knowledgehub-model verify` workflow was also executed with
the retained run and current host inputs on disposable copies. Only the copied journal's
physical `config.work` and `config.destination` were relocated; exact plan, coverage,
semantic requests, original receipts and captured sources were retained. The
[verification report](retained-verification.json) says verified/source-accounted,
native-valid, native-incomplete, 60 exact records and 655 original bytes, with human
acceptance pending. This existing workflow deliberately retains actor A for its
actor-bound recovery checks; the separate history reader demonstration uses B and
does not weaken those checks.

## Reopen, maintenance and failures

Tests compare full reconstructed receipts and descending order before/after close/reopen,
compaction and cold packed reopen. Exact source bytes, baseline membership, original
fingerprints/actors/heads and current semantic head remain equal. Maintenance creates no
publication. Existing scrub, index rebuild, canonical retry and process-fault regressions
are exercised separately from read-only history checks.

Injected receipt scope/key/fingerprint/version/actor/sequence/head/parent and ancestry
binding faults fail explicitly. Missing receipts, unavailable reads and interrupted reads
never become absence. Orphan/staged publications and foreign-scope anchors cannot join
accepted ancestry. Default denial and Read-only denial perform no storage disclosure;
revocation/policy failure, tampered tokens, changed kind/limits/anchor, wrong actor/scope,
release, expiry and capacity have explicit outcomes. Large identifiers and response
envelope overhead produce resource errors without truncation. Checked counter overflow
is exercised independently.

Final review also added an exact-output-budget regression: complete publication and
affected pages succeed when their limit equals their actual encoded response length,
and fail one byte below it. The initial [reproduction](exact-output-before-fix.log)
exposed unnecessary provisional continuation overhead. The reader now sizes the actual
complete/partial envelope before testing the limit; the final focused suite records
the passing fix. No stored format, attribution or mutation behavior changed.

## Scope and handoff

[Preservation evidence](preservation.json) compares pre-verification and final byte-hash
inventories: root hub has 380 files unchanged; old_hub has no files at this checkout;
the postponed terminal proposal has nine files unchanged; the retained source equipment
Hub has 744 files unchanged. All generated/mutated stores are disposable target fixtures.
Neither protected historical store was opened by the current decoder.

No source requirements were edited. No codec conversion, new adapter, crate release,
main-spec sync, archive, canonical refresh, source reimport or authority transfer occurred.
Modelarium C9 and terminal transport/UI/permission mapping remain their own future work.
Known baseline comparison uses the existing compare engine; publication discovery is not
a retained-baseline catalogue or generic snapshot diff. No timestamps, review decisions,
execution evidence or human acceptance are inferred.

Native execution evidence is local macOS arm64/APFS. The user's platform-compliance
assumption remains distinct from run evidence. Linux, cross-host runtime checks,
power-loss durability and the pre-existing external Claude host gate were not executed
by this change. Human meaning review and acceptance remain pending.

## Verification commands and results

All final checks passed. There are **126 distinct Rust target test cases**: 124 in the
full batch, plus the foreign/staged-anchor and exact-budget cases added during final
review. The final changed storage/history areas were rerun as shown below; repeated
focused executions are not added to the distinct total.

| Command | Actual result | Evidence |
| --- | --- | --- |
| `cargo test --all-targets` | 124 passed | [Full batch](targets.log) |
| `cargo test --lib storage::shared` | 26 passed after bounded file-read tightening | [Storage](final-storage.log) |
| `cargo test --lib storage::shared::history` | 7 passed after exact-envelope fix | [Final history storage](final-history-storage.log) |
| `cargo test --test native_history` | 4 passed after exact-envelope fix | [Final public history](final-history.log) |
| `cargo test --doc` | 4 passed, including 2 compile-fail contracts | [Doctests](doctests.log) |
| `cargo fmt --all -- --check` | passed | [Formatting](fmt.log) |
| `cargo clippy --all-targets -- -D warnings` | passed | [Clippy](clippy.log) |
| `/opt/homebrew/bin/python3 tests/host_workflows.py` | 19 passed | [Host checks](host-tests.log) |
| `openspec validate expose-native-model-history --strict --json` | valid, no issues | [OpenSpec](openspec.json) |
| `git diff --check` | passed | Final working-tree check |

The initial system Python 3.9 invocation lacked `tomllib`; its [diagnostic](host-tests-python39.log)
is retained. Repeating with the already-installed Homebrew Python 3.14 passed all 19
checks. No interpreter or package was installed. The intentionally failing exact-budget
reproduction is retained separately from the passing final logs.

Executed final public walkthrough commands, from this repository:

```sh
cargo run --release --example native_history -- "$PWD/target/native-history-20261104-final-budget"
cargo run --release --example native_history -- retained \
  target/rich-design/apply-equipment-acceptance/run/generation-manifest.json \
  target/rich-design/apply-equipment-acceptance/successor.json \
  "$PWD/target/native-history-equipment-20261104-copy"
cargo run --release --bin knowledgehub-model -- verify \
  "$PWD/target/native-history-equipment-20261104-run-copy" \
  "$PWD/target/native-history-equipment-current-host.json"
```

The first command used a fresh unused parent. The second used a quiescent 744-file
copy; the third used retained run-file copies with only their physical destination/work
fields relocated. [Example execution](examples.log), [retained reader execution](retained.log)
and [standalone verification execution](retained-verification.log) all exited successfully.
The source handover, original stored model and protected stores retain their byte inventories.

Review covered all three capability deltas (`native-model-history`,
`full-model-public-api`, `full-model-storage-port`). Implementation, tests, guide and
repo-owned delivery vocabulary agree. Main specs were not synchronised and the change
was not archived. Technical completion does not establish human acceptance or unblock
automatic implementation in either consumer repository.
