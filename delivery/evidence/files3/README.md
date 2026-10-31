# KH-SL-009 technical evidence

31 October 2026. `share-immutable-file-storage` was explicitly authorised by “Implement the first open change”. This records implemented behaviour and automated verification, not human acceptance, authority transfer or completion of the later indexing/rebuild changes. The canonical `hub/` remains files/2.

## Implemented boundary

The explicit files/3 constructor provides immutable kind-separated objects, lossless record/baseline/anchor/receipt projections, persistent typed roots, chunked originals, pinned exact/range/history/outcome reads, guarded durable publication, immutable block packs, full retained-history scrub and coordinated cleanup. Existing files/1–2 constructors, codecs, golden hashes and original outcomes are preserved. See the [adapter guide](../../../design/files3-guide.md).

The public synthetic A/B example retained 998 logical objects, six generations and three baseline records. Its baseline data occupied 1,827 unique logical bytes; anchor data occupied 556 bytes. Packing preserved every object and generation, reducing allocated regular-file bytes from 4,550,656 to 909,312. This fixture includes the complete catalogue and every family/kernel specimen. It is not the canonical library self-model and implies no acceptance of its synthetic observations.

## Executed checks

| Command / run | Observed result |
| --- | --- |
| `cargo test --offline --all-targets -- --nocapture` | 68 passed: 30 library/internal, 25 ontology contract, 12 original public contract and one files/3 public integration test. Zero failed/ignored. Example targets compiled. [Log](tests.log). |
| `cargo test --offline --doc` | One doctest passed. [Log](doctests.log). |
| `cargo fmt --all -- --check` | Exit 0. [Log](format.log). |
| `cargo clippy --offline --all-targets -- -D warnings` | Exit 0. [Log](clippy.log). |
| `cargo build --offline --release --example shared_storage --example storage_benchmark` | Exit 0 using existing Rust/Cargo 1.99.1. No compiler installation. [Log](release-build.log). |
| Public `shared_storage create`, `scrub`, `pack`, `export` | All four exited 0 on an isolated fresh store; A/B, successor, historical genesis, exact source bytes, reopen and original stale retry succeeded. [Commands/output](public-example.log). |
| `openspec validate share-immutable-file-storage --strict --json` | Valid; zero issues. [Result](openspec-validation.json). |
| Canonical inventory and readable export comparison | All 426 previously inventoried canonical files retained exact lengths/hashes. Exported ontology Markdown equals the original source bytes. [Result](canonical-and-export.json). |

[Build/input hashes](build-inputs.json) bind Cargo inputs, implementation, tests, fixtures, examples, original ontology source and release binaries to the actual toolchain/kernel/mount observations. [Dependency review](dependency-review.md) and [package checksums](dependencies.json) identify the new lossless codec dependency and its enabled features. No prior evidence was relabelled.

The tests cover strict bytes, Unicode/duplicate keys, kind separation, raw/zstd equivalence, unknown formats, finite decoding, dictionary refusal, tree sharing/count/key checks, sequence order/duplicates, all kernel projections, semantic cycles and exact/null spans; genesis, full states, historical membership, orphan exclusion, original actor/intent checks, policy rechecks, expected guards and same-base races; partial/full chunk reads, append reuse, missing required nodes/receipts, unrelated-path reads, unselected historical originals, index deletion and syntactically valid incorrect/missing index entries.

Actual SIGKILL workers cover eight publication and seven pack/cleanup checkpoints. Injected failures distinguish definite pre-publication failure from post-replacement `Uncertain`, and cancellation retains coordination until drain. Both current-thread and two-worker Tokio hosts are exercised. Reader/maintenance contention returns `Busy` without deleting the reader's dependencies. These are process faults and injected errors on the declared local ext4 profile, not power-loss, kernel-crash, hardware or other-filesystem durability evidence.

## Fixed-input storage measurements

Each row ran files/2 and files/3 with identical deterministic catalogue, synthetic records and exact artifact bytes. Every adjacent artifact pair intentionally shares content. Both versions produced identical fixture-input hashes and final logical-state hashes. Each run used its own process and fresh store; binary/source hashes are recorded. Fixed inputs and complete per-operation diagnostics are in [small/2](small-2.json), [small/3](small-3.json), [large/2](large-2.json), [large/3](large-3.json), [history/2](history-2.json) and [history/3](history-3.json).

| Fixture | Files/2 encoded bytes | Files/3 loose encoded bytes | Files/3 packed encoded bytes | Allocated bytes, files/2 → packed files/3 | Retained generations / logical objects |
| --- | ---: | ---: | ---: | ---: | ---: |
| Small: 4 × 2 KiB captures, 8 tiny edits | 11,569,688 | 883,106 | 735,342 | 11,812,864 → 749,568 | 11 / 648 |
| Large: 96 × 16 KiB captures, 8 tiny edits | 26,975,081 | 1,857,313 | 1,704,419 | 27,148,288 → 1,720,320 | 11 / 1,017 |
| History: 8 × 2 KiB captures, 64 tiny edits | 78,170,264 | 1,671,980 | 1,112,693 | 80,224,256 → 1,126,400 | 67 / 1,489 |

Files/3 unique logical object bytes were respectively 1,351,306; 2,407,880; 2,684,545. Packed block counts were 2/3/4. Each compacted store had six regular files, including metadata, reference, stable lock and derived location data. Derived bytes were 159,165; 250,969; 367,433 and are included in the encoded totals. These measurement models have zero baseline records; the separate full-family A/B example measures three. All old revisions, original operations and generations remained retained.

The final small/large edits wrote 14 new objects with 198 logical payload bytes and 14,477/18,637 structural bytes. The 64th history edit wrote 16 objects, 198 logical payload bytes, 17,697 structural bytes and 12,168 encoded loose bytes. This shares the old graph/content rather than writing another complete graph or original artifact. Full candidate validation and full owned snapshots still inspect/materialise the model; changed-path disk writes do not imply sublinear validation CPU.

The fixed ontology point read required **five physical object retrievals before and after edits in all three cases**. In the history fixture, encoded object bytes changed only from 6,501 to 6,637 after 64 edits; it did not load the 67 historical states or unrelated originals. Legacy object counters are not instrumented: their zero values are unavailable measurements, not zero I/O. Cold packed index repair intentionally scans verified packs and is accounted separately; normal location-cache lookups do not reread the full location map on every request.

Compaction encoded high-water observations were 1,617,552; 3,560,836; 2,783,777 bytes, with allocated high-water observations of 3,858,432; 7,127,040; 7,741,440. These observations include the new durable replacement while old encodings still exist, count temporary hard-link aliases once, and exclude directory blocks/Git storage. Per-process peak RSS was about 38.1/73.7/38.2 MB for files/3 versus 41.4/76.4/151.2 MB for files/2. RSS includes the whole fixture process and owned state/validation, not just the adapter caches.

Indicative files/3 scrub times were 405/621/2,639 ms; packing including its pre-scrub took 2,439/6,939/4,870 ms. Reopen used a new backend instance; the OS page cache was uncontrolled. Runs shared the host with verification activity, so timing is observational, not a controlled latency benchmark or a portable performance promise. Exact timings, cache conditions and per-edit growth are retained in the JSON results.

To reproduce, build the two release examples, use unused absolute paths with existing parents, and run:

```sh
target/release/examples/storage_benchmark /absolute/new/small-2 small 2
target/release/examples/storage_benchmark /absolute/new/small-3 small 3
target/release/examples/storage_benchmark /absolute/new/large-2 large 2
target/release/examples/storage_benchmark /absolute/new/large-3 large 3
target/release/examples/storage_benchmark /absolute/new/history-2 history 2
target/release/examples/storage_benchmark /absolute/new/history-3 history 3
```

The storage savings measured here apply to these fixed fixtures. They do not predict a final canonical Hub size. KH-SL-010/011 own deterministic indexed navigation; KH-SL-012 owns deleting/regenerating the canonical Hub, new source/evidence capture and its complete walkthrough. General conversion and history pruning remain outside this implementation.
