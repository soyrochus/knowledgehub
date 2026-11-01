# Rebuild the canonical demonstration

The current workflow creates an independent 0.10.0 files/4 demonstration. The
user's renamed old_hub is retained byte-for-byte, excluded from capture and never
used as an import, recovery or reset source. The first cutover requires hub absent;
an occupied older destination is refused before checks or store mutation.

Run `python3 scripts/rebuild_canonical_hub.py --reset`. Later explicit retries may
reset only the marked current canonical target. Exact path, scope, ordinary-file,
symlink and active-writer guards remain. No old metadata/lock layout is supported.

The host performs these steps:

1. Inventory old_hub as host files and verify unpublished Cargo/lock identity 0.10.0.
2. Run offline all-target, documentation, format, Clippy and disposable host checks,
   recording actual Rust/platform observations and exact source/log/executable hashes.
   The native harness uses four concurrent test cases; individual runtime/race cases
   exercise their own schedulers/processes. Build the release examples.
3. Freeze the checked source set; capture original PDF/Markdown and source-bound
   extraction plus actual native evidence. Validate the authored model in memory.
   Unknown, duplicate or stale semantic origins and changed checked inputs fail.
4. Recheck the target under creation/store coordination. Create, register and populate
   through ordinary full APIs with a fresh replay domain, history, A/B and receipts.
5. Drain/reopen, resolve/retry originals, compare A/B, compact and full-scrub retained
   closure. Verify actual model neighbour, literal and composite reads and equivalent
   results from a second backend. Count cold metadata, warm reuse and full-state loads.
6. Run catalogue/coverage, late-neighbour and rent examples in isolated fixtures.
   Compare old_hub path/length/SHA-256 inventories and report actual logical,
   encoded, allocated, peak and cache work under unchanged quotas.
7. Only after required checks pass, write result.json and hub/README.md and mark
   verified. Final task/report/guide edits are explicitly after the captured checkpoint.

Inspect through the public API:

~~~sh
cargo run --offline --release --example inspect_self_model
cargo run --offline --release --example inspect_self_model -- --verify
~~~

`--compact` requests explicit current maintenance. Catalogue/loader/inspector use the
current backend with no format argument; obsolete selectors are rejected. The loader
retains `--result OUTPUT` and `--validate-input INPUT` for current useful operations.

Run-specific evidence under delivery/evidence/hub-rebuild survives a reset. External
status progresses through preflight, checks-passed, reset-started, populated and
verified or failed. A failed attempt never establishes dataset verification. A stale
checked source/log/binary fails; implementation changes require fresh native checks.
Capture and its own generated outputs are never recursively captured.

The source ontology and practical manual retain their original drafts. Technical
verification does not establish human acceptance, authority handover, power-loss
coverage, remote-adapter parity or migration continuity. Older reports retain their
original versions; only equal-input benchmarks support storage-efficiency comparisons.
