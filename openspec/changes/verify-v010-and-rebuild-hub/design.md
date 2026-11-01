# Design

## Context

Hard prerequisite: align-current-api-tooling after the two API/storage changes. The user selected version 0.10.0 and a new canonical Hub after renaming their prior Hub to old_hub. Earlier implementation and rebuild evidence remains scoped to older source checkpoints. The existing explicit host lifecycle already distinguishes preflight/checks/reset/population/dataset verification but must now operate only on the current contract.

## Goals / Non-Goals

**Goals:** all still-required safety guarantees are checked through the only API; exact 0.10.0 evidence binds the new model; old_hub is untouched; completion is based on actual dataset results.

**Non-Goals:** old-reader/decoder coverage, migration continuity, old receipts as current outcomes, publication, automatic human assessment, definition-authority transfer or new platform support.

## Decisions

### Coverage before harness removal

Build a compact guarantee-to-current-test map from the former seed/full cases: exact canonical intent and byte sensitivity; actor/scope reuse; denied/revoked access; atomic late failure; same-base and graph-wide conflicts; process interruption/uncertainty; cancellation/drain/locks; source/digest/anchor/ownership; baselines and original outcomes; finite query/cursor limits; selective read and cache integrity.

Port an uncovered invariant before deleting its only harness. Retire redundant old-backend comparisons after independent current semantic/fault evidence exists. Preserve immutable fixture intent where useful; do not maintain a historical test count as an acceptance target.

### One final checked batch

Finish all implementation, source bindings, version metadata and checked current guides before the run. Execute cargo fmt --all -- --check, cargo test --offline --all-targets, cargo test --offline --doc, cargo clippy --offline --all-targets -- -D warnings and host tests, then build required release examples. Record actual Rust 1.99.1/platform observations, exact source/log/executable identities and observed counts. Verify Cargo.toml and the knowledgehub lock entry are 0.10.0 and publish=false.

Capture current repository/PDF/Markdown/extraction and this actual batch only after all checks pass. Freeze the checkpoint through capture/population. Final task/report/generated-guide edits occur after capture and are explicitly listed; they do not require recursively recapturing the model. Any implementation/input change invalidates the run and requires refreshed checks.

### Retain old_hub and create fresh hub

Recheck ordinary-path boundaries, expected current target policy and absent hub. Inventory old_hub by host file hashes without decoding its old implementation. If hub is occupied by the previous instance, stop and report the required user rename; do not delete or rename it. Preserve old_hub throughout. Lock current creation/publication boundaries and create the new instance through normal APIs.

Register the entire ontology, load the new sourced self-model and native evidence, create exact A/B explanations and this instance's original receipts/replay domain. No old history, receipt, baseline selection or source artifact is imported from old_hub. Explicitly selected historical evidence still identifies its actual older run.

### Actual verification and measures

Drain/reopen, resolve/retry originals and compare A/B. Compact current accepted objects and fully scrub retained closure. Verify neighbour/text/three-hop composite reads in the actual library model and exact agreement from another backend. Report cold metadata once, warm reuse, bounded block work, encoded/allocated/peak regular-file bytes, logical objects, packs/sidecars, counts and declared draft gaps under unchanged cache quotas.

Run full-catalogue/coverage and late-neighbour/rent demonstrations in isolated current fixtures, not the canonical model. Inventory old_hub again and require exact byte/path equality. Publish verified status/result.json and the current hub guide only after every required command and dataset check succeeds. Preserve old attempt evidence outside the target as historical.

## Risks / Trade-offs

- [Deleting old tests removes a guarantee] → guarantee-to-current-test map and independent positive/negative/fault cases.
- [Tests pass but generated data fails] → separate incomplete/failed dataset status; no completed report until required reopen/scrub/query checks pass.
- [The user's renamed directory changes or is captured] → pre/post host inventories, explicit exclusion and exact target guards.
- [Current source changes after checking] → exact checkpoint refusal rather than reusing old success.
- [A smaller model manufactures storage savings] → actual source/model counts and controlled comparisons only; no history pruning.

## Migration Plan

There is no recovery/migration promise for superseded stores. The user's rename preserves the previous instance outside the supported current implementation. A failed new creation may leave an absent/incomplete current hub; an explicitly authorised retry may reset only that current target after refreshed checks. Neither failure nor retry modifies old_hub.

