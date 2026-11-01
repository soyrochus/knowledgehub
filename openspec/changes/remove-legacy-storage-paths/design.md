# Design

## Context

Hard prerequisite: make-full-model-api-default. The old storage.rs contains full-generation files/1–2 persistence, seed metadata/leases and runtime dispatch to shared/. BranchStorage defaults reconstruct whole states or return unsupported errors for the older adapters. Current full publication also branches between recovered history and selective shared roots. Current typed Query has a special old assertion ordering path.

## Goals / Non-Goals

**Goals:** only current full storage executes; default construction supplies every current capability; stored identity/recovery and selective read guarantees stay explicit.

**Non-Goals:** legacy readers, adapters, migrations, selector flags, physical codec redesign or deleting immutability/version checks. Loose accepted writes, pack compaction and raw/zstd are current mechanisms.

## Decisions

### One adapter and constructor

Make FileBackend::new select knowledgehub-files/4. Remove new_ontology and new_shared_ontology rather than forwarding them. Remove the legacy Lease/Metadata/Manifest persistence path, full-history mode branches and old Profile::seed validation. Retain common ordinary-file checks, syncing, coordination, JSON codec and errors where used by the current implementation.

Make the storage port mandatory and typed. Remove defaults that only support an old adapter through full recover/materialization or unsupported newer methods. Preserve selective current operations, admitted-job lifetime and source validation. Keep future adapter semantics storage-independent but add no placeholder backend or feature.

### Current physical identity

Use the files/4 scope/receipt payload from step 1. Update current metadata/projection schemas where representation changed. Keep unchanged current logical record/request hashes, tree/object grammars, bounded KHPACK3 blocks and raw/zstd encodings according to actual meaning. A number embedded in a current codec name is not a second supported store release.

Remove legacy-refs receipt sequences/affected_root dual projection, old reconstruction branches and seed-history validation. Scrub derives receipt dependencies from the sole qualified affected set. Repacking remains lossless within a current store; accepted revisions/history/operation keys are not pruned.

Older store metadata fails before decode/publication and stays unchanged. No old store is opened to import history, including old_hub.

### Query consolidation

Preserve existing useful typed selection and pagination, using current deterministic indexed associations for relationship expansion. Remove legacy_rows/assertion-order adaptation and four-hop compatibility. Use the current traversal engine/finite limits and authority-bound shared cursor machinery. A current Query remains a useful operation, not an alias preserving old ordering. Update independent semantic oracles to current specified semantics; do not replace them with the implementation itself.

### Demonstrable cutover

Update directly broken call sites and tests in this change, even if complete host simplification is step 3. Older-format test branches become current safety tests or negative unsupported-format checks using minimal metadata, not dormant old decoders. Files/1–3 data is not a runnable verification matrix.

No live store is changed here. Keep current authority/executor/filesystem decisions, deny-by-default host policy, conflict/idempotency/recovery and explicit source/gap semantics. Existing Rust 1.99.1 only.

## Risks / Trade-offs

- [Deleting a default loses a required read capability] → current storage methods must be implemented and selective read/fault checks must pass.
- [Old query ordering has hidden callers] → update current examples/tests and preserve functionality with new deterministic expectations, without an alias path.
- [Pack decoding is mistaken for old-format support] → retain current pack/chunk encodings required for writes and compaction, remove store-version dispatch.
- [Old receipt projection remains reachable] → object grammar/dependency checks and current scrub prove the new sole receipt shape.

## Migration Plan

No conversion and no automatic store repair. Finish focused current-store checks, then apply align-current-api-tooling. Do not open or reset old_hub. Canonical creation waits for final exact-source verification in step 4.

