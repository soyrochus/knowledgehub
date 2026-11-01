# Rebuild the canonical demonstration

Status: the canonical rebuild is postponed by user direction. Its unfinished attempt remains at 7/11 under openspec/deferred/rebuild-canonical-hub-and-verify/. The cache-storage prerequisite pack-derived-index-storage is implemented and technically verified; see [packed-cache evidence](../delivery/evidence/packed-cache/README.md). It performed no reset or recapture.

KH-SL-012 host workflow · 1 November 2026. Applying rebuild-canonical-hub-and-verify authorises the explicit reset described here. This lifecycle creates a fresh instance; ordinary library revisions, baselines and generation history remain immutable.

Run with the existing Rust 1.99.1, Python and installed Poppler on the declared local ext4 host:

~~~sh
python3 scripts/rebuild_canonical_hub.py --reset
~~~

The only reset target is the exact ordinary repository-root hub/ directory, with expected kh:hub:self-description / urn:knowledgehub:library / main / host:maintainer scope. The command rejects root/parent/unrelated paths, links, unexpected scope and unmarked partial directories. Stop host admission and drain users before invoking reset. The orchestrator coordinates concurrent rebuilds and acquires the adapter's existing creation and store locks for deletion; an active operation refuses reset. The host must keep other applications quiescent through population. It does not add a destructive library API or retain a migration/rollback copy. Existing unrelated directories, including an externally retained orig_hub/, are outside its target.

Each attempt uses a new delivery/evidence/hub-rebuild/<run-id>/ directory. Explicit --run-dir and --target arguments allow the same canonical path and a new direct run directory only. No second approval checkpoint or silent reset is performed. Normal create/open preserve their refusal/recovery rules.

The command performs these steps in order:

1. Validate target/path/scope and create an external preflight status.
2. Run cargo fmt --all -- --check, cargo test --offline --all-targets, cargo test --offline --doc and cargo clippy --offline --all-targets -- -D warnings, plus disposable Python host lifecycle/checkpoint tests. Retain original outputs, toolchain/filesystem observations, exact source inventory and executable hashes; build release example mains. Failure stops before reset.
3. Verify unchanged checked inputs, capture current sources, the observed native log/checkpoint, original PDF/Markdown and fresh source-bound PDF extraction. Reject new or stale origins. Validate the complete source-derived model in memory before deletion.
4. Recheck/coordinate the canonical target, mark reset-started and delete all prior hub/ content. An absent target starts fresh. Create/register/populate files/3 through ordinary public APIs; create this instance's original receipts and exact A/B explanation revisions.
5. Drain/reopen, recover/retry original outcomes, compare exact A/B narratives, compact retained immutable objects and run a full scrub. Execute actual self-model neighbour/literal/composite reads with measured cold/prepared/warm work. The indexed read follows Requirement ← UseCase → FunctionDefinition → AggregateDefinition; it introduces no fictional business data.
6. Run full catalogue conformance, late-neighbour and rent mapping examples in isolated target/ fixtures, reconcile catalogue/source coverage, and report actual new records, sources, gaps, replay domain and logical/encoded/allocated/index bytes.
7. Publish result.json and generate hub/README.md after every required dataset check succeeds, then mark verified. Final task/status/report/guide edits are explicitly after the captured checkpoint; there is no recursive attempt to capture the capture itself.

Status is external to deletion: preflight, checks-passed, reset-started, populated, verified or failed. The latest local pointer is .hub-rebuild-state.json; run-specific status.json retains attempt diagnostics. A failure before reset preserves old data. After reset, failed or interrupted population can leave absent/incomplete new data and supplies no rollback promise. A later explicit --reset attempt reruns checks and clears only that marked partial canonical target. If a process was killed before it could record failed, its last non-verified phase remains visibly incomplete. Never treat that directory as a verified demonstration.

Inspect a successful instance through the public API:

~~~sh
cargo run --offline --release --example inspect_self_model -- --format files3
cargo run --offline --release --example inspect_self_model -- --format files3 --verify
~~~

The second command may prepare disposable derived indexes and performs full retained scrub. --compact additionally requests explicit host maintenance; it creates no semantic generation. Derived caches are excluded from Git and source capture. All native checks run once for the final checkpoint; post-population checks exercise the generated dataset. Source changes invalidate the check/capture and require a new batch.

Legacy positional catalogue_conformance, full_self_description and inspect_self_model invocations retain files/2; --format files2 is also explicit. New files/3 stores must use --format files3. The loader accepts --result OUTPUT to keep result artifacts in the attempt directory. Example capture/verification commands are shown in the project README.

The explicit inventory in examples/self_model_support/origins.json owns stable semantic names shared by capture, authoring and coverage audit. Moving origins retains their declared name. Public contracts keep their public namespace; newly modelled private helpers have their responsible module's internal namespace, and unnamed parameters are distinguished by signature position. Unknown origins fail before reset. Python host functions are sourced native descriptions, not instructions executed by the Hub.

Technical verification remains distinct from semantic adequacy, human acceptance, authority handover, power-loss guarantees and remote adapter parity. No old snapshot/replay identities or receipts are imported. The older 116 MB instance has different inputs; only controlled equal-input benchmarks establish format-efficiency comparisons. Actual authoritative and disposable cache sizes are reported separately.
