# Proposal

## Why

KnowledgeHub retains exact revisions, baselines and original publication receipts, but another currently authorised reader cannot inspect an earlier publisher's receipts through the public API. Existing `operation_result` is actor-bound mutation recovery, not a history reader; adopting a preinitialised Hub exposes this public-access gap without requiring another model or storage refactoring.

## What Changes

- Add current-authority, read-only access to original publication metadata and exact affected references, including publications by another actor. Keep historical attribution separate from the requesting actor's authority.
- Resolve known operation IDs first, and supply bounded, snapshot-pinned publication discovery for consumers that do not possess a complete handover. Provide opaque continuations, finite work/output limits and explicit completeness/failure outcomes.
- Reuse retained receipt, generation-parent and ancestry structures through the storage port. Do not materialise the whole model or parse backend files in consumers to obtain history.
- Preserve existing exact predecessor reads, named baselines and baseline comparison. Verify their composition with publication history; do not replace them with another lineage or diff engine.
- Keep `commit`, actor-bound `operation_result`, canonical request fingerprints and original receipt contents unchanged. Introduce separate history-read request/result contracts instead of weakening retry recovery.
- **BREAKING (Rust source integration):** propose an explicit `Action::ReadHistory` permission in addition to ordinary scoped read authority. Hosts with exhaustive `Action` matches must handle the new action; existing read permission alone does not implicitly grant publication-history disclosure.

### Outcome slice and prerequisites

**Proposed native-history slice C8 of MM-SL-02.** A currently permitted reader opens a current-contract Hub published by actor A, resolves A's known original publications, discovers additional accepted publications in bounded pages, follows their exact revisions/baselines and compares known baseline A/B after reopen. Completion requires public-API evidence with a different reader actor B, explicit denied/failure results, original attribution and measured finite reads. No Modelarium or terminal implementation is part of this slice.

Hard prerequisites are the delivered full `knowledgehub::KnowledgeHub` API, catalogue-3, record/request codecs /3 and files/4, retained receipts/ancestry, current scoped policy, exact reads and existing baseline comparison. These are present in the inspected implementation. This proposal is a focused public history-access extension, not a redo of the earlier API/storage refactorings or MM-SL-01 authoring work.

**Implementation order:** implement and verify `expose-native-model-history` before resuming `add-terminal-model-client`. The user has postponed that terminal change. Modelarium C9 consumes the accepted public contract later in its own repository/session; this change neither implements its transport/UI nor asserts its acceptance.

### Boundary, acceptance and unresolved choices

In scope: read-only native publication lookup/discovery, bounded affected-reference access, current permission, exact snapshot/receipt identity, existing lineage/baseline composition, reopen/maintenance preservation, consumer documentation and an executable public-API example. Out of scope: new ontology families, changing publication/retry semantics, full audit/event sourcing, inferred commit timestamps, arbitrary retained-record search, a new comparison engine, SQL activity, Modelarium changes, terminal code, remote adapters, migrations, source reimport, canonical-store refresh, archive and publication.

Acceptance requires an unrelated small native fixture and an MM-SL-01-shaped generated-model handover with earlier/current baselines and publications absent from any application audit. Actor B can inspect original A outcomes under explicit current permission while B's actor-bound recovery/reuse still fails. Concurrent append, policy revocation, reopen, packing, corruption/unavailability and large histories have distinct tested outcomes. Protected `hub/` and `old_hub/` remain untouched; tests use fresh fixtures or disposable copies, not canonical-store mutation.

The proposed design expects no semantic-codec or files/4 format change: existing stored fields are sufficient. If implementation disproves this expectation, stop and revise this proposal rather than silently change formats, regenerate handovers or add compatibility readers. Public Rust names and precise default/hard limits are proposed in design.md and must be published with the implementation; resource instrumentation and format feasibility must be verified, not assumed. Planning readiness does not establish implementation, runtime evidence, human review or platform parity.

## Capabilities

### New Capabilities

- `native-model-history`: Current-authority original-publication reads, bounded accepted-history discovery, exact affected-reference paging and truthful scope/completeness outcomes.

### Modified Capabilities

- `full-model-public-api`: Include the distinct native history reader and owned contracts at the existing sole public entry point while preserving actor-bound recovery and existing model reads.
- `full-model-storage-port`: Require bounded original-publication reads/enumeration without full-state or full-history materialisation and without exposing physical layout.

Existing `model-baselines` and `branch-commits` requirements are preserved and receive regression evidence rather than semantic changes.

## Impact

Proposed implementation areas: `src/contracts.rs` (history authority), a small `src/ontology/history/` contract/use-case module exposed through `src/ontology/mod.rs`, the existing `KnowledgeHub` handle and bounded jobs/cursor infrastructure, `src/storage.rs`, and selective reads in `src/storage/shared/generation.rs`, `projection.rs` and tree helpers. Tests should extend public-contract, ontology/baseline, shared-storage and fault checks plus focused history tests. Documentation/example updates belong in this repository. No extra runtime, audit database or dependency is required by this proposal.

Trace: source ontology proposal §§3/5/13/14/17/21.2–21.4/21.6–21.7/21.10; domain rules KH-DR-02/04/05/07–13/15/17/19/20; journeys KH-J-02/03/07/08 and stories KH-US-06–10/21/22/24/26. The practical manual supplies the actor → outcome → complete path → safeguards → evidence method. Modelarium's `design/memento-model-improvement-plan.md` C8 supplies the consuming need, not a replacement source of ontology meaning.
