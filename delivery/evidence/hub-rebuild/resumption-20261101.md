# Rebuild resumption — 1 November 2026

The user authorised checking the old rebuild against pack-derived-index-storage, deleting hub/ and regenerating it. The previous attempt remains failed at delivery/evidence/hub-rebuild/20261101T123801Z-d7b406f3/. Its 7/11 task snapshot follows. The new attempt reuses the three implemented tooling tasks and repeats every source-bound execution task against the current checkpoint. Old evidence and orig_hub/ remain outside the reset target.

## Previous task state

# Tasks

Deferred by user direction on 1 November 2026. The unfinished 7/11 attempt and failed evidence remain historical. Its active completion plan is obsolete pending [pack-derived-index-storage](../../changes/pack-derived-index-storage/proposal.md); no completion/archive is claimed.

Planning only; KH-SL-012 / KH-US-30 follows verified KH-SL-009–011. Applying this change explicitly authorises deletion of the generated repository-root hub/ as part of a fresh rebuild; editing these artifacts does not execute it. Use existing Rust 1.99.1 and installed tooling, with no compiler installation. The preserved attempt completed 7/11 tasks; the remaining tasks are postponed, not accepted or completed.

## 1. Prepare the public rebuild workflow

- [x] 1.1 Add explicit files/3 selection to catalogue/self-description/inspector host examples while preserving legacy invocations; update stable source/module/API bindings for all new implementation and orchestration origins, verify fixture authoring accepts every captured origin and document the new commands.
- [x] 1.2 Extend verification/capture tooling with new run-specific evidence paths and exact input/log/build bindings; verify old evidence is not overwritten, stale source/logs are refused and generated hub/cache/output trees cannot be recursively captured.
- [x] 1.3 Implement the explicit-reset host orchestrator with exact canonical-path/scope guards, writer coordination and separate completion/failure status; verify disposable-fixture deletion, refused repository/parent/unrelated/symlink targets, preflight failure preservation and interrupted-population retry. Document that reset creates a new instance rather than changing immutable revisions.

## 2. Run the complete suite and capture its actual inputs

- [x] 2.1 Finalise code and checked documentation, then run cargo fmt --all -- --check, cargo test --offline --all-targets, cargo test --offline --doc and cargo clippy --offline --all-targets -- -D warnings; retain actual output, exit codes, source/build hashes, counts and platform observations, and verify any failure stops before canonical deletion.
- [x] 2.2 Capture fresh current repository/PDF/Markdown/extraction and native-run inputs against that successful batch; verify exact hashes, new source bindings and source-checkpoint status, retain the input manifest and reject changed checked inputs rather than reuse historical success.

## 3. Delete old content and regenerate the canonical Hub

- [x] 3.1 Drain canonical users, validate the repository-root hub/ target and execute the explicit reset, removing its complete previous contents without a retained migration/rollback store; verify old sentinel/history/domain data is absent before fresh creation and unrelated work and historical evidence outside hub/ remain intact.
- [x] 3.2 Populate a new files/3 Hub through ordinary create/register/commit APIs with the complete catalogue and current sourced self-model, new native evidence bindings and new A/B revisions/receipts; verify fresh replay-domain/snapshot history, required source/vocabulary coverage and actual draft gaps rather than copying old counts or importing old receipts.
- [ ] 3.3 Drain/close/reopen, run full scrub and the public inspector, and verify both new exact A/B narratives, comparisons, actual original receipts/retries for this run and authority boundaries; keep rebuild status incomplete/failed until all generated-dataset checks pass.

## 4. Public walkthroughs, measurements and completion

- [ ] 4.1 Execute the full catalogue conformance walkthrough and delivered neighbour/phrase/composite examples, including late associations and isolated rent mappings, using fresh fixture stores or disposable clones; verify actual canonical queries separately and no invented fixture concepts enter the library self-model.
- [ ] 4.2 Record active logical/encoded/allocated and derived-index bytes, object/file counts, actual selected/source/gap counts and cold/prepared/warm read work; distinguish changed source-model inputs from controlled format-efficiency benchmarks and report every required command's real result.
- [ ] 4.3 Publish the completed new run report and generate the files/3 hub guide/inspector instructions only after successful checks; preserve older-run evidence as historical, mark the fresh replay domain and captured checkpoint explicitly, update KH-SL-012 status and verify documented commands against the rebuilt hub/.

## Workflow follow-up

- Review scoped evidence before archive; technical checks do not supply human acceptance or definition-authority handover.
- General conversion, complete recovery backup/restore and replay-preserving migration remain deferred KH-SL-005 work; no implementation of them is required to rebuild this generated demonstration.
