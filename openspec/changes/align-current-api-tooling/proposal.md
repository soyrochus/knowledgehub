# Proposal

## Why

Examples, capture, benchmarks and current documentation still teach format selection and older API paths. They must describe and exercise the single 0.10.0 implementation and must never recursively capture the user's retained old_hub.

## What Changes

- Remove files2/files3 CLI selection, older-constructor switches and comparisons whose purpose is keeping old releases usable. Use the sole full API in all maintained examples and scripts.
- Keep small examples as ordinary full-model instances; update source/module/API bindings to actual new public identities without aliases to removed software APIs.
- Make inspection, scrub, compaction and rebuild orchestration current-only. Explicitly exclude old_hub and other generated stores from capture/checkpoints.
- Preserve the user's old_hub directory: no rename, deletion, import, migration or decoder for it. Require the canonical destination to be absent for the first 0.10.0 regeneration.
- Update current guides, dependency sequence and current specification wording. Keep source originals, archived planning and historical execution evidence at their original scope.
- Keep archive-aware source identity resolution because it serves the current OpenSpec/provenance workflow.

## Capabilities

### New Capabilities

- `current-api-host-tooling`: unambiguous latest-only authoring/inspection, accurate source bindings and retained-store isolation.

### Modified Capabilities

- `source-bound-definitions`: update remaining source/read/executor examples to the full API after seed-only requirements are removed by step 1.

## Impact

Order 3/4, proposed KH-SL-016. Hard prerequisite: [remove-legacy-storage-paths](../remove-legacy-storage-paths/proposal.md). Actor: maintainer capturing/inspecting/authoring current models. Completion: maintained commands select only the current implementation, capture/author/coverage bindings agree and tests prove old_hub is untouched and excluded. Follow with [verify-v010-and-rebuild-hub](../verify-v010-and-rebuild-hub/proposal.md).

Trace ontology §§5.2/17/21.2–21.3/21.5–21.7/21.10 and KH-DR-01/02/05/07/12/13/15/18/23. Examples/file_format, support fixtures, capture/audit/verification/reset scripts, origins and owning guides are affected. Do not rewrite historical logs or source books to appear current; no new human acceptance or authority transfer. Version is already 0.10.0 from step 1. This change does not populate hub or rename the user's store.

