# Tasks

KH-SL-018 · KH-J-11 · KH-US-31–33. **Application authorised by the user on 2 November 2026.** Tasks are checked only when their implementation and corresponding evidence exist. Use installed Rust 1.99.1 on both native hosts; install no compiler. Preserve the pre-existing compilation repair and unrelated local work. Tests use isolated current-format stores, never repository `hub/` or user-retained `old_hub/`.

## 1. Establish the native persistence feasibility boundary

- [x] 1.1 Verify the design's regular-file full flush, directory `fsync`, same-volume replacement/link installation and final publication-fence sequence on an isolated native Mac/APFS path with the installed Rust 1.99.1; deliver the exact OS/build/architecture, observed volume/mount/case facts, exercised call/error behaviour and ordering record in a new evidence path. Keep APFS admission disabled if the selected protocol cannot be established. Trace: recovery / Durable current-format acknowledgement; ontology §21.6/21.10.
- [x] 1.2 Verify stable nonblocking shared/exclusive advisory locking between separately opened same-process descriptors and child processes, exclusive creation and lock release after child death on native APFS; document actual results and required host restrictions alongside 1.1, without changing lock identity or treating a mock as native evidence. Trace: recovery / Declared and checked file-backend support profile; ontology §21.6/21.10.

## 2. Admit explicit native profiles without rewriting current stores

- [x] 2.1 Add explicit APFS support/options and the fallible native-profile composition helper inside the existing adapter boundary, with compile-time native filesystem probes; verify native APFS admission, retained actual ext4 detection and refusal of mismatched declarations, nonlocal/non-APFS observations, uncoordinated hosts and cross-device/symlink layouts. Document which negative cases are native observations versus injected probe results. Update current API/profile guidance with the same change. Trace: recovery / Declared and checked file-backend support profile; shared / One current default file format.
- [x] 2.2 Replace hard-coded Linux metadata validation/creation with recognised creation-profile handling and independent current-location checks; verify unchanged Linux metadata bytes, correct Mac creation metadata, rejection of unknown profiles/codecs/scope/older formats before mutation, and retained replay/head/history identity. Document the creation/current-profile distinction and pre-extension-binary limitation. Trace: shared / Recognised creation profile and checked current location; ontology §21.5/21.9.

## 3. Preserve publication, maintenance and recovery on both native paths

- [x] 3.1 Route creation, ordinary persistence, recovery-reference sync and reference publication through the documented native helpers/final fence, auditing direct `sync_all` sites; verify injected regular-file/directory/full-flush failures before replacement leave the head unchanged, failures after possible replacement return uncertainty, and reopen/original retry recovers one exact receipt. Keep Linux ordering and errors intact and document both sequences. Trace: recovery / Durable current-format acknowledgement; ontology §21.4/21.6/21.10.
- [x] 3.2 Apply native persistence/ordering to loose-object installation, shared pack/index/catalogue replacement, cleanup and both derived cache namespaces; verify complete replacement closure before deletion, reader/compaction exclusion, index rebuild/corruption behaviour and exact pre/post-compaction state/history/receipt equality. Retain finite quotas and document actual filesystem allocation measurements separately from semantic guarantees. Trace: shared / Exact offline current-format continuity; existing shared-storage/maintenance and cache contracts.
- [x] 3.3 Extend current fault workers/hooks for native sync/fence failures and preserve all eight publication and seven maintenance process-kill checkpoints; run focused native APFS recovery, lost-response, cancellation/drain, independent-writer/read-lease and current-thread/multithread cases, recording exact prior-or-complete-new outcomes and unchanged original bytes. Maintain a guarantee-to-case map and document the process-fault boundary without a power-loss claim. Trace: recovery / Native recovery evidence per support profile; ontology §21.10.

## 4. Make maintained host paths and observations native

- [x] 4.1 Update valid-profile composition in maintained examples, integration/unit fixtures, independently opened packed-cache backends and child workers to the native helper; retain explicit unsupported-profile negative cases and support propagation into derived stores. Verify minimal creation/reopen and catalogue/shared-storage/query walkthroughs on isolated APFS fixtures, with old selectors still rejected; update runnable guidance alongside these paths. Trace: tooling / Unambiguous current commands; recovery / Full native runtime journey.
- [x] 4.2 Replace Linux-only assumptions in verification platform observations and benchmark RSS collection with native branches, explicit units and optional-unavailable values; retain Linux mount observations and make missing required platform facts fail verification. Verify both observation branches and missing-data/error paths with focused host tests, and execute Mac benchmarks far enough to exercise their resource reports without procfs or fabricated zeros. Document actual observed fields. Trace: tooling / Native observations without Linux-only assumptions.
- [x] 4.3 Update current source inventory/audit and self-model authoring bindings for added profile APIs/private modules; verify current origin resolution, unknown/duplicate-origin refusal and isolated self-description authoring/inspection without canonical reset. Update the coverage/consumer guidance, retaining exact historical captures and repository definition ownership. Trace: tooling / Isolated support verification and honest evidence; ontology §§5/14/21.10.

## 5. Integrate source-matched native evidence and review the outcome

User direction during application: ignore Linux verification for now, accept it as given, and perform it separately on another machine. This is an assumption, not an observed native Linux result. Tasks 5.2 and 5.3 retain their unverified status; the latter needs both native fixture directions.

- [x] 5.1 At one exact implementation checkpoint, run the full offline all-targets test suite, doctests, formatting, Clippy with warnings denied, host workflow tests and release-example build on native macOS arm64/APFS using Rust 1.99.1; execute isolated full catalogue/specimen, A/B/source, exact/range/read/query, original-retry, compact/scrub and self-description journeys. Bind source/executable/log hashes, native observations and guarantee coverage in fresh evidence; compare retained-data/historical-evidence inventories without decoding `old_hub`. Trace: recovery / Full native runtime journey and Native recovery evidence per support profile; ontology §21.10.
- [ ] 5.2 Run the same implementation checkpoint and complete regression/fault/walkthrough matrix on native Linux x86_64/ext4 with installed Rust 1.99.1, recording its own environment, results and hashes. Verify original ext4 detection, current full safety cases and no lost guard/recovery coverage; leave completion open if that native run is unavailable or source-mismatched. Trace: recovery / Native recovery evidence per support profile.
- [ ] 5.3 Close and inventory independent synthetic Mac-origin and Linux-origin files/4 fixtures, copy each offline to the other supported host, and verify destination open, exact A/B/source/query equality, original receipt/retry, one ordinary guarded successor and compact/scrub; confirm source copies and destination creation-profile metadata remain unchanged and incomplete copies fail explicitly. Record both native directions without using user stores, a converter or a live-copy claim. Trace: shared / Recognised creation profile and checked current location and Exact offline current-format continuity; ontology §§5/21.4–21.7/21.10.
- [ ] 5.4 Review the two native evidence sets, fixture continuity, guarantee map and retained-data boundaries; publish an exact support/failure matrix, remaining gaps and relation of final report edits to checked source revisions. Reconcile KH-SL-018 and proposed KH-D-23–25 with the observed results; mark implementation tasks complete only where delivered work and evidence support them, keeping human acceptance, authority handover, publication and canonical refresh separate. Verify OpenSpec strict validation and local documentation links after report edits.

## Workflow follow-up

- The artifacts are ready for review; implementation requires a new explicit apply request. Do not run the apply workflow as part of proposal creation.
- Native Linux access or a successful build must never be inferred from Mac evidence. Missing required native evidence leaves the affected tasks and change incomplete.
- No archive, crate publication, canonical rebuild, source refresh or authority transfer is included. Those actions require their own scope.

## Application evidence — 2 November 2026

[Mac results and source relation](../../../delivery/evidence/macos-file-backend/README.md):
11/14 tasks verified. The complete native Mac batch and all isolated runtime,
benchmark, self-description and maintenance journeys passed. The original
checked task snapshot precedes these completion/status edits.

Tasks 5.2 and 5.3 are deferred under the user's Linux-verification direction; no
Linux execution or two-direction native copy is claimed. The Mac report/matrix
portion of 5.4 is delivered, while its review of both native evidence sets and
cross-host continuity remains open. No archive, publication, canonical refresh,
human acceptance or authority handover was performed.
