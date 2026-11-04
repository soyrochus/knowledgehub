# Tasks

C8 `expose-native-model-history` of MM-SL-02, applied in the KnowledgeHub workspace under the user's explicit instruction on 4 November 2026. Checked tasks identify implemented and verified work; [evidence](../../../delivery/evidence/native-history/20261104-apply/README.md) retains actual outcomes. Do not resume `add-terminal-model-client` without its own apply instruction. Modelarium transport/UI changes remain in its later C9 scope.

## 1. Define and authorise the distinct history reader

- [x] 1.1 Add the owned history requests, publication-header/page/work results and typed anchor-relative non-membership/failure contracts at the sole public API boundary; verify external-consumer compilation, preserved original receipt fields and serialisation/output accounting tests without changing semantic record/request codecs.
- [x] 1.2 Add explicit `Action::ReadHistory` and require current scoped `Read` plus `ReadHistory` on lookup, affected pages and continuations; verify default denial, ordinary-read-only denial, distinct publisher/reader success, failed policy and no disclosure before authority checks.
- [x] 1.3 Implement validated defaults/hard maxima and checked counters from design decision 4; verify invalid/overflowing limits, oversized indivisible values, encoded response overhead and explicit resource failures without silent clamping/truncation.
- [x] 1.4 Document the new permission and proposed read contracts beside the implementation; verify Rustdoc examples compile and explain source-breaking exhaustive action matches, all-allow policies, original attribution and unchanged actor-bound mutation recovery.

## 2. Resolve a known original publication selectively

- [x] 2.1 Add internal selective receipt-header and accepted-anchor lookup operations using existing generation/operation/ancestry structures; verify original scope/ID/fingerprint/sequence/head/parent binding against known fixture receipts and that header reads do not expand the entire affected sequence.
- [x] 2.2 Add affected-reference count/range reads over the retained sequence; verify exact ordered reconstruction, repeated references where present, zero/last pages and a collection larger than one page without full-state, artifact or full-affected materialisation.
- [x] 2.3 Expose asynchronous known-operation history lookup through bounded jobs with original actor A visible to permitted B; verify later-than-anchor non-membership, unknown/wrong-scope anchors, malformed IDs and missing/corrupt/unavailable data remain distinct from current operation absence.
- [x] 2.4 Add a runnable known-handover lookup example and API notes; verify it resolves A's original heads/fingerprint/affected references as B after later edits, while B's actor-bound `operation_result` and operation-key reuse still fail and A's exact retry returns the unchanged original receipt.

## 3. Discover native publications in finite stable pages

- [x] 3.1 Add bounded descending accepted-generation publication walking to the storage port/public API; verify a complete multi-page traversal matches the fixture's accepted receipts, excludes rejected/staged operations, identifies the initial no-receipt boundary and needs no application audit.
- [x] 3.2 Add small actor/scope/anchor/intent-bound history cursors and release handling using finite cursor admission/lifetime; verify changed limits/operation kind, token tampering, wrong actor/scope, revocation, policy failure, release, expiry and capacity produce explicit outcomes without additional disclosure.
- [x] 3.3 Enforce actual visited-node, storage/decompression-byte and output bounds through selective reads; verify large histories and affected sets on loose and cold-packed stores with independent work counters, no hidden full scan/preparation and no endless non-advancing continuation.
- [x] 3.4 Document and demonstrate pinned paging, explicit partial/complete results and restart at a retained anchor; verify concurrent new commits do not alter a traversal and a lost process-local token fails explicitly while a new query at the same anchor reproduces the original sequence.

## 4. Preserve historical meaning and recovery

- [x] 4.1 Compose the new history reader with existing exact record/predecessor/source reads and baseline A/B comparison on two small unrelated current-contract fixtures; verify both original and successor meaning, source hashes/pins and baseline membership without adding another diff or lineage engine.
- [x] 4.2 Verify header, affected-page and full publication traversals before/after close/reopen and supported compaction/index maintenance; assert unchanged original receipts, order, baseline/source content and semantic heads, with no fabricated maintenance publication.
- [x] 4.3 Extend disposable-store fault tests for inconsistent receipt/ancestry/parent bindings, unavailable storage and interrupted history reads; verify failures never become authoritative absence and no history read creates a publication or changes existing original-outcome recovery.
- [x] 4.4 Complete the public-API walkthrough/documentation with both fixtures, known handover IDs and discovered history; execute its commands and verify explicit limits, missing/denied outcomes, no inferred timestamps/review/execution and the stated boundary on baseline discovery/comparison.

## 5. Verify and hand off the integrated native capability

- [x] 5.1 Run affected public-contract, ontology/baseline, shared-storage/fault and authoring regressions plus formatting, warnings-denied Clippy, Rust target tests and doctests using the existing toolchain; retain actual commands/results and keep unavailable native-platform/external-host checks separate.
- [x] 5.2 Exercise an available MM-SL-01 retained handover only through a disposable copy, or explicitly record its unavailability alongside the reproducible small-fixture evidence; verify a different currently authorised reader sees pre-consumer publications and preserves exact original/edited baselines and sources, without modifying canonical `hub/` or `old_hub/`.
- [x] 5.3 Deliver an implementation evidence/handoff report with exact exported signatures, permission mapping, supported contracts, anchors/operation IDs, defaults/maxima/counters and native/reopen/maintenance outcomes; verify it supplies enough information for later Modelarium C9 and terminal integration without claiming their implementation or human acceptance.
- [x] 5.4 Run strict OpenSpec validation and review the final diff against this proposal; verify all three capability deltas are represented, implementation stays within the native-history boundary, current files/4 and /3 contracts are preserved, protected/unrelated stores and Modelarium remain unchanged, and terminal implementation has not started.

## Workflow follow-up

- The planning handoff originated in Modelarium. The user subsequently authorised this native change in KnowledgeHub; Modelarium integration remains in its own later session and scope.
- Resume `add-terminal-model-client` only after this change has actual implementation/verification evidence. Reconcile its proposed host permission and history command contracts with the delivered native API before applying that consumer change.
- Modelarium C9 separately owns host/client transport, application activity separation, history/comparison presentation, browser continuity and its acceptance checks. Existing known predecessor/baseline primitives need not be reimplemented in KnowledgeHub.
- No archive, main-spec synchronisation, crate publication, canonical-store refresh or authority handover is part of this request. Record human acceptance and unsupported/unexecuted platform gates independently of technical test results.
