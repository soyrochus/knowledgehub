# KnowledgeHub outcome slices and specification sequence

Draft 0.1 · 30 October 2026 · All delivery work is planned; no implementation or acceptance is asserted.

This plan applies the [Practical Design Manual](../source/practical-design-manual.md) to the [domain analysis](../design/domain-analysis.md). It owns journey/story organisation, outcome cuts, dependencies and specification order. The [ontology proposal](../source/system-definition-ontology-proposal.md), especially §§19–21, supplies the target contract. The [self-hosting plan](../design/self-hosting-plan.md) owns the progressive transfer of this project's definition into its running library.

## Delivery vocabulary and method

A delivery slice is the manual's project convention: a bounded increment of working behaviour delivering one coherent, demonstrable outcome, with actor/trigger, complete path, applicable safeguards and reviewable evidence. We adopt that convention here. It is distinct from a DDD context, a feature, a runtime change set, an OpenSpec change and a technical task.

Use the progression domain understanding → journey and story map → outcome slice → behavioural specification → authorised implementation → verification and review. Revisiting earlier modelling is expected. Keep the direction visible and specify the next outcome in detail; deepen later slices when evidence makes them ready.

## Journeys and observable library interactions

The primary channel is a host calling the embedded API. A maintainer initiates that work and reviews its outputs. No product UI, CLI application or screen mockup is required. The interaction states are inspectable request/result states; a future UI must preserve the same meanings.

| Journey | Actor, trigger and objective | Progression and completion condition |
| --- | --- | --- |
| KH-J-01 Retain a sourced definition | Maintainer begins converting the ontology proposal into the library's own model. | Select exact source → declare scope/profile/ownership → submit a coherent definition → inspect receipt → close/reopen → retrieve the same definition and evidence. |
| KH-J-02 Evolve and explain it | Author discovers a changed definition or a reviewer asks what it affects. | Read one base → prepare successor → inspect affected links → validate → commit/rebase explicitly → compare exact versions and provenance. |
| KH-J-03 Review exact meaning | Reviewer receives a candidate or changed scope. | Inspect definition, rationale and source → assess exact revisions → retain decision → distinguish unchanged assessed content from new unassessed content. |
| KH-J-04 Move and resume the project model | Maintainer needs an independently reconstructable copy. | Select export profile/baseline → export → stage in a fresh Hub → validate → promote → compare identities, content, history and declared recovery records. |
| KH-J-05 Develop KnowledgeHub using KnowledgeHub | Maintainer prepares the next library outcome after the handover gate. | Inspect governed model → record need and intended change → obtain review → produce a version-bound spec representation → implement outside the Hub → attach actual evidence → review and baseline. |

### Story map

The columns preserve the actor's progression. Stories below name needs; code modules and database tables are implementation tasks.

| Map level | Establish scope | Understand meaning | Propose and commit | Review and explain | Resume and exchange |
| --- | --- | --- | --- | --- | --- |
| User tasks | Choose Hub, model and profile; establish current authority. | Read definitions and exact sources. | Prepare one coherent intended change; inspect result. | Understand consequences and assess exact revisions. | Recover after interruption; reopen or transfer work. |
| Cut A: retain the first definition | KH-US-01: create/open one permitted model with an explicit profile. | KH-US-02: capture and cite the exact proposal fragment. | KH-US-03: commit a Markdown definition and typed relationship coherently. | KH-US-04: inspect exact IDs, revisions and supporting source. | KH-US-05: reopen and resolve an uncertain commit by its original identity. |
| Cut B: evolve safely | Retain current access and branch checks. | KH-US-06: read the accepted base and prepare a successor. | KH-US-07: revise document-owned meaning and graph bindings together. | KH-US-08: compare old/new definitions and retain the earlier explanation. | Preserve the rejected candidate; retry only the original intent. |
| Cut C: inspect impact | Select exact scope/baseline. | KH-US-09: find typed definitions and traverse bounded relationships. | Propose changes informed by the impact result. | KH-US-10: retain consistent results across ordered/paginated reads. | Resume with a cursor tied to the same snapshot. |
| Cut D: govern review | Apply a host-configured review policy. | Inspect exact source and proposed meaning. | Keep commit and review permission distinct. | KH-US-11: record an assessment of exact revisions; KH-US-12: recognise a successor requiring review. | Reopen the original review and its assessed content. |
| Cut E: exchange safely | Select portable model/backup scope. | Inspect contained and qualified external references. | KH-US-13: stage and validate an import before promotion. | Compare exact preserved semantics. | KH-US-14: export and reconstruct content and declared operation history. |
| Cut F: maintain the library's own model | Select the governed self-description baseline. | KH-US-15: follow the library's own rules, journeys and slices in the Hub. | KH-US-16: author the next bounded development change through governed operations. | KH-US-17: attach real implementation/run evidence and obtain scoped review. | Recover the model independently and continue from a known baseline. |

## Ordered slice catalogue

All six slices are proposals. The ordering below distinguishes hard capability prerequisites from sequencing preferences. Each slice inherits applicable guarantees from earlier slices; identity, current authority, conflicts, idempotency and recovery are required from the first write.

### KH-SL-001 — Retain and reopen one sourced KnowledgeHub definition

- **Actor/trigger:** a maintainer begins the library's self-description from the exact ontology proposal.
- **Outcome:** a host can create/open one model, retain a small Markdown definition, its source/anchor and two typed elements connected by an identified assertion; after restart, it reads the same exact content and original commit receipt.
- **Complete path:** public asynchronous operations → current authority → pinned bootstrap profile → model/provenance validation → one guarded branch publication → snapshot-bound exact reads and restart recovery. The seed uses a `UseCase satisfies Requirement` assertion from the source catalogue.
- **Safeguards:** invalid types/anchors/endpoints and denied requests preserve the accepted head; same-base writer races yield an explicit conflict; exact retries recover the receipt; reuse for different intent is rejected; staged/incomplete content cannot become the accepted model.
- **Evidence target:** normal, denied, malformed, concurrent, replay, changed-fingerprint, interrupted-publication and lost-response examples through public contracts on a declared host/filesystem profile. Capture the build/toolchain and inspected content hashes. These are future checks.
- **Source/rules:** ontology §§5, 8, 17, 21.2–21.7, 21.10; KH-DR-01–12 and KH-DR-15 as applicable to the fixed seed profile.
- **Dependencies:** no prior product slice. KH-D-01–06 have adopted selections in the self-hosting plan; explicit implementation authorisation and subsequent verification remain required.
- **Deferred:** arbitrary profile expansion, broad search/traversal, review workflow, portable bundle exchange and the complete self-model. Low-level constraints needed for this seed are included; graph-wide constraints for later profiles arrive with those profiles.
- **Stories/specification:** KH-US-01–05; first bounded OpenSpec change `retain-first-self-definition`.

### KH-SL-002 — Revise a definition and recover both explanations

- **Actor/trigger:** the author corrects a definition after reading its current revision.
- **Outcome:** a coherent successor updates document-owned content, corresponding graph bindings and provenance, while the earlier definition remains retrievable under an exact named baseline.
- **Complete path:** base read → recoverable authoring candidate → validation → guarded commit → baseline creation/comparison → close/reopen → inspect both explanations.
- **Safeguards/evidence:** source edits and file renames preserve identity and pinned citations; invalid ownership and stale-base commits are rejected without losing the caller's draft; concurrent graph changes cannot invalidate a cross-record invariant; old and new baselines survive interruption and retry.
- **Hard prerequisite:** KH-SL-001 supplies exact reads, immutable records and the complete publication/recovery boundary. Adds authoring and explicit baseline/lineage operations.
- **Stories/rules:** KH-US-06–08; KH-DR-01–11. Define the document authoring round-trip and baseline semantics before implementation.

### KH-SL-003 — Explain the impact of a proposed definition change

- **Actor/trigger:** a maintainer or reviewer asks which promises, use cases and definitions depend on a selected revision.
- **Outcome:** a bounded typed query/traversal reports its selected snapshot, relationship direction, order, continuation and unresolved external references; the reader can reproduce the result.
- **Complete path:** scoped query → explicit filter/order/traversal semantics → permitted snapshot-bound result → pagination or follow-up inspection.
- **Safeguards/evidence:** denied scopes remain undisclosed; concurrent commits do not mix query pages; missing and null are distinguishable; unsupported queries fail explicitly; derived index lag is disclosed if an index is used.
- **Hard prerequisite:** KH-SL-001 provides a queryable snapshot. KH-SL-002 is a preferred predecessor for meaningful before/after examples; it is required only for a baseline comparison variant.
- **Stories/rules:** KH-US-09–10; KH-DR-03/04/07/12/15. No full-text ranking or vector retrieval is promised.

### KH-SL-004 — Record and enforce review of an exact scope

- **Actor/trigger:** a reviewer assesses a proposed or committed definition scope under the selected host policy.
- **Outcome:** the assessment, rationale and reviewed revisions remain inspectable; the policy can require that exact assessment for a later change without equating it to runtime verification.
- **Complete path:** permitted review view → exact target selection → assessment → guarded policy enforcement where configured → reopen the reviewed scope and its successors.
- **Safeguards/evidence:** changed targets do not inherit a review; revoked reviewer/caller authority is checked; a concurrent definition change cannot substitute an unreviewed revision; conflicting assessments remain explicit.
- **Hard prerequisites:** KH-SL-001 for guarded records; KH-SL-002 for successor/baseline examples. KH-SL-003 improves inspection but is a scheduling preference.
- **Stories/rules:** KH-US-11–12; KH-DR-07/09/13. Human acceptance, implementation and test outcomes retain their separate records.

### KH-SL-005 — Transfer the model without changing its meaning

- **Actor/trigger:** a maintainer needs a portable model copy and a separately reconstructable backup.
- **Outcome:** exports identify the selected profile, exact revisions, original source bytes, required artifacts and review/history scope; staged import into a fresh Hub reconstructs that scope without bypassing admission.
- **Complete path:** choose model-exchange or recovery-backup profile → export → inspect/stage → validate profile, hashes, authority and endpoints → guarded promotion → semantic comparison and reopen.
- **Safeguards/evidence:** corrupted, unresolved-required, incompatible and stale imports preserve the destination head. Recovery backup preserves the operation ledger/identity scope required to resolve old retries; ordinary model exchange must state its different replay scope. These modes must be specified before any parity claim.
- **Hard prerequisites:** KH-SL-001/002 supply revision/ownership and recovery semantics. Exporting reviews requires KH-SL-004; bounded traversal from KH-SL-003 is useful but not intrinsically required for whole-snapshot export.
- **Stories/rules:** KH-US-13–14; KH-DR-02/04–11/13–15. No database driver or native dump is introduced.

### KH-SL-006 — Use the governed self-model to develop the next outcome

- **Actor/trigger:** the maintainer moves from the bootstrap documents to routine development using the running library.
- **Outcome:** a reviewed, navigable KnowledgeHub model represents this library's selected purpose, language, contexts, rules, journeys, stories, slices, specifications and evidence obligations; the maintainer makes one later development change through the same governed operations.
- **Complete path:** register the needed self-description profile → map exact repository revisions → import/stage through ordinary APIs → review coverage and explicit gaps → record authority handover → prepare one subsequent spec representation → attach independently produced implementation evidence → baseline and reconstruct.
- **Safeguards/evidence:** authority is assigned per field/scope; Markdown and graph cannot both own the same definition; documents are version-bound representations after handover; imported specs remain unimplemented until actual evidence exists. The handover can be reversed explicitly without destroying history.
- **Hard prerequisites:** KH-SL-002/003/004/005 provide coherent authoring, navigation, review and reconstructability. Complete type coverage for this slice is a profile definition task, not an excuse to copy the whole ontology before the first useful path.
- **Stories/rules:** KH-US-15–17; the full rule catalogue. This slice can be delivered largely as ontology/model content plus a real usage walkthrough; it need not introduce another library subsystem.

## Cross-cutting release work packages

These establish supported boundaries rather than inventing additional user outcomes.

| Package | Required result | Boundary |
| --- | --- | --- |
| KH-RWP-001 File-backend conformance | Integrated normal/denied/concurrent/replay/fault/restart evidence covering ontology §21.10 and every supported filesystem profile. | Slice-specific recovery evidence starts in KH-SL-001; this package does not defer it. Process-kill checks alone do not establish power-loss durability. |
| KH-RWP-002 Public contract and interchange review | Inspectable compatibility/version policies, public API usage without storage leakage, and portable reconstruction with declared recovery semantics. | Rust API, ontology, native file format and bundle versions are reviewed separately. |
| KH-RWP-003 Self-description acceptance | A reviewer follows source → definition → slice/spec → implementation/run evidence and reconstructs the selected self-model without hidden local state. | Records actual scope, gaps and acceptance owner; a documentation check cannot complete it. |

## Specification sequence and readiness

1. Review the domain distinctions, context map and rule ownership with the user. This authored draft is available for that review; no approval is implied.
2. Keep all six outcome cuts visible and reassess their dependencies when the first path exposes a mistake.
3. Prepare the first change's proposal, behavioural deltas, design and unchecked tasks. Its scope is KH-SL-001, including its applicable safeguards.
4. Use the adopted [first-slice decisions](../design/self-hosting-plan.md#decisions-to-resolve), reconcile the contracts and obtain explicit implementation authorisation. User adoption is recorded there; complete artifacts and passing OpenSpec validation do not supply implementation authorisation or working-library evidence.
5. Implement and verify the selected change only when requested. Use the result to deepen the next ready slice and its specifications.
6. Progress from a small self-description seed to the explicit authority handover in KH-SL-006. Keep code and native run evidence in their owning tools with exact Hub references.

The [first change](../openspec/changes/retain-first-self-definition/proposal.md) specifies only KH-SL-001. The later descriptions above are a rolling plan, not an approved backlog of implementation-ready changes. Main specifications under `openspec/specs/` are populated through the normal adoption/archive workflow, not by declaring every proposed future behaviour already current.
