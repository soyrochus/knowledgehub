# Tasks

Order 3/4 · proposed KH-SL-016. Hard prerequisite: implemented remove-legacy-storage-paths. Old_hub is user-retained data outside the current source/model.

## 1. Maintained commands and source bindings

- [x] 1.1 Remove format-selection enums/flags and obsolete old-versus-current constructors/benchmarks from maintained examples/scripts; retain useful current command options and small examples through the full API, verifying documented defaults and explicit rejection of obsolete --format arguments.
- [x] 1.2 Update actual source/module/public-operation origins and version capture after restructuring; retain archive-aware source identity resolution, verify capture plus in-memory author/coverage validation on current sources and ensure unknown/duplicate/stale origins are refused rather than aliased.

## 2. Retained directory and current guidance

- [x] 2.1 Exclude old_hub explicitly from capture/checkpoints/generated-source enumeration and appropriate ignore rules; add disposable root/nested exclusion and wrong-target cases, verifying its bytes are untouched and never appear in captured input.
- [x] 2.2 Make canonical orchestration current-only and require an absent hub for the first cutover; remove older metadata/lock-layout handling, preserve active-writer/path guards and verified/failed status rules, verifying occupied old destination and old_hub/symlink/unrelated targets are refused without writes.
- [x] 2.3 Update current guides, ownership/sequence records and example/API descriptions for sole 0.10.0 use; preserve original source books/archived plans/historical logs and verify current commands/links, focused host tests and unchanged retained data.

## Workflow follow-up

- Apply verify-v010-and-rebuild-hub after this change is implemented.
- Do not populate a store while doing tooling/model dry validation; that is not final native or canonical evidence.

