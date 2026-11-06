# KnowledgeHub outcome slices and specification sequence

Draft 0.9 · 3 November 2026 · Earlier ontology/storage/query/packed-index/rebuild and KH-SL-014–017 outcomes retain their recorded technical evidence. The authorised MM-SL-01 sequence adds current modelling contracts, standalone tools and portable skills. Technical checks and human meaning review remain distinct.

This plan applies the [Practical Design Manual](../source/practical-design-manual.md) to the [domain analysis](../design/domain-analysis.md). It owns journey/story organisation, outcome cuts, dependencies and specification order. The [ontology proposal](../source/system-definition-ontology-proposal.md), especially §§19–21, supplies the target contract. The [self-hosting plan](../design/self-hosting-plan.md) owns the progressive transfer of this project's definition into its running library.

## Delivery vocabulary and method

A delivery slice is the manual's project convention: a bounded increment of working behaviour delivering one coherent, demonstrable outcome, with actor/trigger, complete path, applicable safeguards and reviewable evidence. We adopt that convention here. It is distinct from a DDD context, a feature, a runtime change set, an OpenSpec change and a technical task.

Use the progression domain understanding → journey and story map → outcome slice → behavioural specification → authorised implementation → verification and review. Revisiting earlier modelling is expected. Keep the direction visible and specify the next outcome in detail; deepen later slices when evidence makes them ready.

## Revised priority after the user's full-ontology clarification

The user explicitly wants the whole source ontology expressible, then KnowledgeHub
itself expressed through it. Next priority is **KH-SL-001 → KH-SL-007 → KH-SL-008**.
The new IDs preserve earlier slice identities; numeric order is not priority.
The [full-ontology change](../openspec/changes/archive/2026-11-01-express-full-ontology-and-self-model/proposal.md)
specifies these two ordered outcomes. Its inventory enumerates all source types,
relationships and rules; its self-model map names actual library records and reads.
Planning is not implementation or human acceptance.

## Journeys and observable library interactions

The primary channel is a host calling the embedded API. A maintainer initiates that work and reviews its outputs. No product UI, CLI application or screen mockup is required. The interaction states are inspectable request/result states; a future UI must preserve the same meanings.

| Journey | Actor, trigger and objective | Progression and completion condition |
| --- | --- | --- |
| KH-J-01 Retain a sourced definition | Maintainer begins converting the ontology proposal into the library's own model. | Select exact source → declare scope/profile/ownership → submit a coherent definition → inspect receipt → close/reopen → retrieve the same definition and evidence. |
| KH-J-02 Evolve and explain it | Author discovers a changed definition or a reviewer asks what it affects. | Read one base → prepare successor → inspect affected links → validate → commit/rebase explicitly → compare exact versions and provenance. |
| KH-J-03 Review exact meaning | Reviewer receives a candidate or changed scope. | Inspect definition, rationale and source → assess exact revisions → retain decision → distinguish unchanged assessed content from new unassessed content. |
| KH-J-04 Move and resume the project model | Maintainer needs an independently reconstructable copy. | Select export profile/baseline → export → stage in a fresh Hub → validate → promote → compare identities, content, history and declared recovery records. |
| KH-J-05 Develop KnowledgeHub using KnowledgeHub | Maintainer prepares the next library outcome after the handover gate. | Inspect governed model → record need and intended change → obtain review → produce a version-bound spec representation → implement outside the Hub → attach actual evidence → review and baseline. |
| KH-J-06 Express the source ontology | Model author needs the vocabulary defined by the supplied PDF. | Discover exact definitions → choose profile → submit meaningful records from the required families → inspect conformance/gaps → commit → reopen the same exact graph. Completion requires full catalogue support, not a list of accepted labels. |
| KH-J-07 Describe the library itself | Maintainer needs to inspect KnowledgeHub's own definition and evidence. | Capture exact project sources → register full vocabulary → load sourced model → follow requirement/behaviour/implementation/evidence → revise one explanation → reopen A/B. This precedes routine-authority handover. |

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
| Cut G: express the full ontology | KH-US-18: discover/register the complete exact ontology. | KH-US-19: author every source modelling family with meaningful properties. | KH-US-20: inspect schema/graph violations and completeness independently. | Inspect kernel subjects and exact source/refinement meanings. | Reopen the complete catalogue and conformance graphs without losing original artifacts. |
| Cut H: describe KnowledgeHub | KH-US-23: load the sourced library model through ordinary APIs. | KH-US-22: follow actual requirement/behaviour/code/evidence connections. | KH-US-21: preserve two exact explanations under named baselines. | KH-US-24: inspect actual observed evidence separately from intended definitions and acceptance. | Reopen and compare A/B with original receipts and explicit applicability/gaps. |

## Ordered slice catalogue

The catalogue below retains the original slice intentions and chronological reviews. Subsequent implementation/evidence entries record their actual bounded outcomes; the latest delivered sequence is [MM-SL-01](#mm-sl-01--external-source-bound-model-authoring). KH-SL-002–006 retain the identities and remaining scope reconciled below. Each slice inherits applicable guarantees from earlier slices; identity, current authority, conflicts, idempotency and recovery are required from the first write.

### KH-SL-001 — Retain and reopen one sourced KnowledgeHub definition

- **Actor/trigger:** a maintainer begins the library's self-description from the exact ontology proposal.
- **Outcome:** a host can create/open one model, retain a small Markdown definition, its source/anchor and two typed elements connected by an identified assertion; after restart, it reads the same exact content and original commit receipt.
- **Complete path:** public asynchronous operations → current authority → pinned bootstrap profile → model/provenance validation → one guarded branch publication → snapshot-bound exact reads and restart recovery. The seed uses a `UseCase satisfies Requirement` assertion from the source catalogue.
- **Safeguards:** invalid types/anchors/endpoints and denied requests preserve the accepted head; same-base writer races yield an explicit conflict; exact retries recover the receipt; reuse for different intent is rejected; staged/incomplete content cannot become the accepted model.
- **Evidence target:** normal, denied, malformed, concurrent, replay, changed-fingerprint, interrupted-publication and lost-response examples through public contracts on a declared host/filesystem profile. Capture the build/toolchain and inspected content hashes. Actual KH-SL-001 runs and limitations are recorded in [technical evidence](evidence/kh-sl-001.md); no human acceptance is inferred.
- **Source/rules:** ontology §§5, 8, 17, 21.2–21.7, 21.10; KH-DR-01–12 and KH-DR-15 as applicable to the fixed seed profile.
- **Dependencies:** no prior product slice. KH-D-01–06 have adopted selections in the self-hosting plan; implementation was explicitly authorised on 31 October 2026 and verified using existing Rust 1.99.1, with platform compliance assumed at the user's direction. Human acceptance remains pending.
- **Deferred:** arbitrary profile expansion, broad search/traversal, review workflow, portable bundle exchange and the complete self-model. Low-level constraints needed for this seed are included; graph-wide constraints for later profiles arrive with those profiles.
- **Stories/specification:** KH-US-01–05; first bounded OpenSpec change `retain-first-self-definition`.

### KH-SL-007 — Express the complete System Definition Ontology

- **Actor/trigger:** a model author needs the vocabulary expressly defined by the supplied ontology PDF.
- **Outcome:** every kernel concept, all 128 types in the 14 §6 families, all 32 §8 signatures and detailed §§9–17 contracts can be represented, validated, committed and read back through the library. Prose relationship refinements retain explicit provenance.
- **Complete path:** exact ontology/type/property/relationship/profile registration → schema-driven typed authoring → record/graph/completeness assessment → guarded commit → public discovery/exact reads → close/reopen. Original PDF and Markdown bytes are independent captures.
- **Safeguards:** unknown fields/types/qualifiers, wrong endpoints/roles, invalid multiplicity/ownership/lineage and corrupt artifacts are rejected; native assertion/anchor/revision subjects need no duplicate nodes; all first-write safeguards remain.
- **Evidence:** an exhaustive K/T/R/P/V/S manifest binds registered revisions, meaningful valid/invalid fixtures and reopen results for every entry. A missing UI/AI family is incomplete support even if the library self-model does not use it.
- **Dependency:** KH-SL-001's admission/recovery foundation. Source details absent from the proposal use visibly proposed KH-D-11–15 refinements, not attributed source requirements.
- **Stories/rules/specification:** KH-US-18–20; KH-DR-01–18; `ontology-definitions` and `ontology-conformance` in `express-full-ontology-and-self-model`.
- **Excluded:** executing arbitrary modelled algorithms/processes/AI, a product UI, new database/service adapters or authority transfer. Their ontology definitions remain representable.

### KH-SL-008 — Express KnowledgeHub itself and recover its explanations

- **Actor/trigger:** the maintainer needs the library's actual purpose, domain, contracts, implementation and evidence inspectable through the ontology.
- **Outcome:** a complete applicable sourced model of KnowledgeHub, not just the seed, supports concrete requirement → behaviour → implementation/artifact and requirement → case → actual run/evidence navigation; after revising an explanation, A/B remain exact after reopen.
- **Complete path:** capture exact source/repository/build inputs → register KH-SL-007 vocabulary → ordinary public model load → named A → bounded typed inspection → coherent successor/document/assertion change → named B → drain/close/reopen → compare/resolve original operations.
- **Safeguards:** exact identity/lineage/source/authority, no silent repinning/rebase, bounded current-access navigation and original retry/recovery. Capture actual observations; preserve proposed analysis and historical evidence status; do not invent runtime UI/AI, test success or acceptance.
- **Evidence:** the change's concrete self-model group manifest and Q01–Q08 walkthrough, integrated conflict/replay/interruption checks, and separate catalogue-support/self-model-completeness reports.
- **Dependencies:** KH-SL-007 plus the successor, exact-baseline and bounded-navigation portions now specified in the same change. Routine document-authority handover remains KH-SL-006.
- **Stories/rules/specification:** KH-US-06–10/15–17/21–24; KH-DR-01–19; `model-baselines`, `model-navigation`, `knowledgehub-self-description` in `express-full-ontology-and-self-model`.

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

The [first change](../openspec/changes/archive/2026-11-01-retain-first-self-definition/proposal.md) specifies only KH-SL-001. The later descriptions above are a rolling plan, not an approved backlog of implementation-ready changes. Main specifications under `openspec/specs/` are populated through the normal adoption/archive workflow, not by declaring every proposed future behaviour already current.

## Review after KH-SL-001

The implemented seed validates the proposed distinction between modelling,
provenance and branch admission for this small path. Process-interruption and
replay examples support keeping the original receipt in accepted history.
No model contradiction required changing the owning domain rules. The adapter
serializes full-history recovery and candidate preparation under one branch
lock; this is a deliberate small-model performance limit, not a new domain rule.

The earlier recommendation to implement KH-SL-002 next is superseded by the
user's full-ontology clarification. Successor document/fragment authoring,
revision lineage and exact baseline/comparison operations needed by that outcome
are now specified concretely inside KH-SL-008. This does not mark any older slice
implemented or remove its unfulfilled obligations. Human acceptance and KH-SL-006
authority handover have not occurred.

## Reconcile the earlier slices without losing their scope

| Earlier slice | Relationship to the next full-ontology/self-model change |
| --- | --- |
| KH-SL-002 | Its core coherent successors and exact A/B explanations are specified in model-baselines for KH-SL-008. Reassess delivery evidence before marking the older slice complete; managed filesystem working-copy/import authoring is not silently promised. |
| KH-SL-003 | Exact type/ID filtering, bounded directional traversal and snapshot-bound continuations are pulled forward. Broader filters/search/query operators remain separately specified work. |
| KH-SL-004 | ReviewDecision/AcceptanceRecord definitions and exact assessment scope are representable. Host-configured review workflow, required approval enforcement and dispute policy remain later work. |
| KH-SL-005 | Binary artifacts and exact source records are prerequisites delivered here. Portable interchange/recovery-backup export/import and replay-domain continuity remain later work. |
| KH-SL-006 | Full applicable self-description and public inspection are pulled forward to KH-SL-008. Actual acceptance, reconstructability via interchange, routine governance of the next change and scoped authority handover remain explicit gates. |

Preparation remains progressive: the new bounded proposal is the concrete next
planning set, with unchecked implementation tasks. Existing Rust 1.99.1 and the
user-directed platform assumption apply; install no compiler. Use actual findings
to revise the residual sequence rather than automatically closing old slices.

## Review after implementation of KH-SL-007/008

The user authorised and committed the bounded full-ontology/self-model change.
The full catalogue is materialised as constrained definitions; synthetic complete,
draft and invalid specimens exercise every family. The public self-model consumer
retains exact sources, actual native evidence and two reopened explanations.
See [measured technical results](evidence/full-ontology.md) and the
[consumer instructions](../design/self-model-consumer.md). Human acceptance is
pending. A valid draft with declared gaps is not a complete accepted self-definition.

The evidence supports retaining exact branch admission, source ownership and
original receipt rules for the enlarged model. No new domain boundary follows
from an implementation module. Full-generation/history validation is a measured
small-model implementation tradeoff; release-mode runs are used for the larger
walkthrough.

The next sequence is review of the actual self-model gaps and outcome evidence,
then progressive specification of the remaining need. KH-SL-002 still has managed
working-copy authoring scope; KH-SL-003 retains broader query work; KH-SL-004 retains
mandatory review policy; KH-SL-005 retains portable exchange/backup/replay continuity;
KH-SL-006 retains actual acceptance, independent reconstruction and scoped authority
handover. These earlier slices are not automatically marked complete or archived.

## Next sequence: compact persistence and deterministic local navigation

The user requested [file-storage draft 0.3](../source/file-storage-proposal.md) and implementation planning for generic nearest-neighbour lookup, then selected canonical deletion/regeneration with the complete test suite instead of migration. Order: **KH-SL-009 → KH-SL-010 → KH-SL-011 → KH-SL-012**. The user authorised KH-SL-009, then KH-SL-010, and on 1 November continued with the next change, KH-SL-011. All three are implemented and technically verified at their recorded boundaries. KH-SL-012 remains planned; these changes did not execute the canonical reset/rebuild.

| Journey / story | Need and completion |
| --- | --- |
| KH-J-08 Preserve and inspect knowledge efficiently | Maintainer retains exact model history without complete file copies, then reopens exact records and original outcomes. KH-US-25: share unchanged content; KH-US-26: preserve baseline/generation distinctions through compaction and recovery. |
| KH-J-09 Trace a proposed change | Human or external AI finds a story, follows actual local ontology associations and receives exact candidate impacts with evidence. KH-US-27: discover incoming/outgoing neighbours, including later links; KH-US-28: traverse several directions/steps in one bounded request; KH-US-29: search narrative and receive grouped, projected paths without autonomous reasoning. |
| KH-J-10 Rebuild and verify the canonical demonstration | Maintainer checks current code, explicitly deletes old generated content, captures verified inputs and repopulates/reopens files/3. KH-US-30: demonstrate the current self-model, fresh A/B and indexed reads with actual full-suite evidence and honest new history. |

### KH-SL-009 — Retain exact history with shared immutable files

- **Actor/trigger:** maintainer observes approximately 116 MB for the current full-generation demonstration.
- **Outcome:** explicit files/3 create/commit/read/reopen retains shared records/artifacts, compact baselines, original receipts and history, including packing, full scrub and safe cleanup.
- **Hard prerequisites:** implemented files/2 ontology/record/request contracts; source file-storage §§3–6/8–13. No dependency on future query shapes or future associations.
- **Stories/rules:** KH-US-25–26; KH-DR-01–15/17/20. [Change](../openspec/changes/archive/2026-11-01-share-immutable-file-storage/proposal.md).
- **Acceptance:** byte/digest/replay equality, full-history recovery tests and structural sharing/allocation measurements. Does not convert `hub/` or change definition authority.
- **Observed delivery:** explicit files/3, exact A/B/retry/reopen, packs/scrub/cleanup, 68 tests plus one doctest, process-fault boundaries and fixed-input measurements are recorded in [KH-SL-009 evidence](evidence/files3/README.md). The 64-edit fixture retained 67 generations in 1,112,693 packed encoded bytes versus 78,170,264 with files/2; its fixed point read stayed at five objects. This is technical verification, not human acceptance or a canonical-size claim. Root `hub/` remains byte-exact files/2; proceed to KH-SL-010 planning/authorisation next.

### KH-SL-010 — Retrieve actual ontology neighbours quickly

- **Actor/trigger:** reader follows local associations whose future routes were unknown at insertion time.
- **Outcome:** generic batch incoming/outgoing lookup returns exact assertions and reference occurrences using snapshot-bound adjacency, including late insertion and new registered types.
- **Hard prerequisite:** KH-SL-009 root-bound point reads, immutable maps and read coordination; deriving all selected records through files/2 on every lookup would defeat the outcome.
- **Stories/rules:** KH-US-27; KH-DR-03/04/07/12/15/17/19/21. [Change](../openspec/changes/archive/2026-11-01-index-ontology-neighbours/proposal.md).
- **Acceptance:** scan-equivalent results, exact old/new views, rebuild/corruption behaviour and measured lookup work independent of unrelated edges except tree height. No transitive closure cache or per-question index.
- **Observed delivery:** [KH-SL-010 evidence](evidence/neighbours/README.md) records owned batch operations, generic extraction, incremental indexes, selective reads, finite cursors, legacy equivalence, 81 native tests plus one doctest and public examples. Adding 3,000 unrelated assertions left the fixed neighbourhood at seven required records/three examined candidates with zero full-state materialisations; index reads rose from eight to nine. Cache allocation and cold-build costs are explicitly reported. Human acceptance and KH-SL-011 impact/search remain separate.

### KH-SL-011 — Answer bounded trace questions in one deterministic read

- **Actor/trigger:** human or external AI asks which physical tables are linked to a story containing a phrase.
- **Outcome:** type/phrase selection and typed mixed-direction traversal execute locally over indexed hops and return grouped targets, origin paths, compact evidence and explicit ambiguity/coverage limits.
- **Hard prerequisite:** KH-SL-010. A rent example supplies explicit logical-to-physical mappings as an isolated demonstration, not a new universal ontology family or a claim about KnowledgeHub's own tables.
- **Stories/rules:** KH-US-28–29; KH-DR-03/04/07/12/15/18/19/21/22. [Change](../openspec/changes/archive/2026-11-01-query-deterministic-model-impact/proposal.md).
- **Acceptance:** one public request plus explicit pagination, deterministic results, bounded fan-out/cycles and missing-mapping/unsupported-external outcomes; no LLM, inferred edges, scripts or automatic schema-change decisions.
- **Observed delivery:** [KH-SL-011 evidence](evidence/impact/README.md) records literal UTF-8/ASCII-only entry, disposable trigram candidates with exact narrative verification, state-aware mixed traversal, one-view grouped dependencies and finite shared cursors. The isolated rent fixture returns a seven-hop physical-schema witness with DDL evidence through ordinary registration. The full native run passed 92 tests plus one doctest, with final strengthened rent scenarios, formatting, Clippy and release examples. Across 0/100/1000 unrelated records, one ready public request used eight local batches, seven associations, one text candidate and zero full-State loads. Human acceptance, long-history stress and canonical regeneration are separate.

### KH-SL-012 — Rebuild and verify the canonical self-model

- **Actor/trigger:** maintainer regenerates the root `hub/` demonstration after shared storage and indexed query implementation.
- **Outcome:** run the full native suite/checks on final code, capture current verified sources, delete all previous canonical content, populate a fresh files/3 self-model, then drain/reopen/scrub and execute public/catalogue/indexed walkthroughs.
- **Hard prerequisites:** verified KH-SL-009/010/011/013; current capture/loader/source bindings and successful exact-source native checks. No general migration or recovery-backup implementation is needed.
- **Stories/rules:** KH-US-30; KH-DR-02/04–13/15/17–23. [Change](../openspec/changes/archive/2026-11-01-rebuild-canonical-hub-and-verify/proposal.md).
- **Acceptance:** old generated content removed; new replay domain/history and actual new A/B/receipts/source/gap counts; all required tests/walkthroughs pass with new evidence. Preflight failure preserves old data; post-reset failure remains explicitly incomplete. Older evidence is not relabelled. KH-SL-005 backup/interchange and KH-SL-006 authority scope remain deferred.

- **Resumed scope:** the user authorised a fresh checked/captured reset on 1 November 2026 after KH-SL-013. Reuse three implemented tooling tasks and rerun eight execution tasks, including sealed-cache quotas/hashes and cold/warm reopen equivalence. The previous failed attempt remains historical.

- **Observed delivery:** [KH-SL-012 completed evidence](evidence/hub-rebuild/20261101T190114Z-9b3a2f55/README.md) records a fresh reset after 100 Rust tests, one doctest, 11 host tests, formatting, Clippy and release builds. Capture covers 149 artifacts/2,049 units; the new canonical instance has 9,264 selected records, 9,270 revisions and 3,815 draft gaps. Both inspectors, full scrub/compaction, packed-cache hashes/quotas/warm reuse, catalogue, late-neighbour, rent and coverage walkthroughs pass. Regular files allocate about 40.1 MiB. Human acceptance and authority handover remain separate.

### KH-SL-013 — Keep derived indexes compact with sealed packs

- **Actor/trigger:** maintainer/reader prepares indexes for the actual library self-model after small-file allocation exhausts the cache quota.
- **Outcome:** variable-size sealed packs with bounded compression blocks retain exact query semantics under unchanged allocated-space quotas, with reusable warm location metadata.
- **Dependencies:** KH-SL-009–011. [Change](../openspec/changes/archive/2026-11-01-pack-derived-index-storage/proposal.md). The user explicitly authorised proposing and implementing this outcome on 1 November 2026.
- **Acceptance:** bounded publication/failure/quota handling, old/new cache equivalence, unchanged accepted Hub data, actual canonical queries and full technical evidence.
- **Sequence revision:** KH-SL-012's old completion plan is postponed at 7/11 after a 256 MiB neighbour-cache allocation failure. [Retained artifacts](../openspec/changes/archive/2026-11-01-rebuild-canonical-hub-and-verify/proposal.md) and failed evidence remain historical. No reset/recapture is part of KH-SL-013; the user subsequently authorised the refreshed KH-SL-012 scope above.
- **Observed delivery:** [KH-SL-013 evidence](evidence/packed-cache/README.md) records 100 native tests, one doctest, 11 host tests, formatting, Clippy and release builds. The existing canonical model passes public neighbour/text/three-hop composite reads, reopen and full scrub, with all 22,842 non-cache files unchanged. The completed neighbour cache allocates 26,177,536 bytes across 306 files; text allocates 1,269,760 bytes. Cold metadata is loaded once per namespace and warm batches load none. Human acceptance and authority handover remain separate.

## Single full API sequence — 0.10.0 cutover

The user selected removal of backward API/storage support and requested four ordered changes, package version 0.10.0 and a fresh Hub after their own rename to old_hub. All four changes are implemented and technically verified; all 22 tasks are complete. [Final evidence](evidence/hub-rebuild/20261101T230838Z-9b8133d6/README.md) binds 0.10.0 native checks and the fresh current canonical dataset, including retained old_hub equality. Existing source/domain rules remain in force for the sole current implementation.

### KH-SL-014 — Ordinary consumers use the full API

[make-full-model-api-default](../openspec/changes/make-full-model-api-default/proposal.md). Actor: library consumer/host. Outcome: one full KnowledgeHub, direct typed scope/state/receipts and consistent unpublished 0.10.0 metadata. No prerequisite change; current publication/source/policy/query guarantees remain. Acceptance: ordinary imports and full-model operations work, old public facades are absent and focused checks pass. Relevant rules: KH-DR-01–13/15–19.

### KH-SL-015 — One current persistence implementation

[remove-legacy-storage-paths](../openspec/changes/remove-legacy-storage-paths/proposal.md). Hard prerequisite: KH-SL-014. Actor: host creating/opening a current model. Outcome: one default constructor/format, direct complete storage port and current indexed navigation. Acceptance: no old-format dispatcher/decoder/alias, safe refusal of older stores, current selective reads, compaction and recovery. Relevant rules: KH-DR-02–12/15/17/19–22.

### KH-SL-016 — Commands and provenance use the sole API

[align-current-api-tooling](../openspec/changes/align-current-api-tooling/proposal.md). Hard prerequisite: KH-SL-015. Actor: maintainer authoring/capturing/inspecting. Outcome: latest-only commands and actual source bindings, with old_hub excluded and untouched. Acceptance: maintained invocations, capture/model/coverage and retained-directory guard tests pass; historical evidence remains labelled. Relevant rules: KH-DR-01/02/05/07/12/13/15/18/23.

### KH-SL-017 — Verify and retain the new canonical model

[verify-v010-and-rebuild-hub](../openspec/changes/verify-v010-and-rebuild-hub/proposal.md). Hard prerequisite: KH-SL-016 and therefore all earlier changes. Actor: maintainer delivering the first sole full API demonstration. Outcome: exact 0.10.0 full-suite/capture evidence and fresh public-API Hub with new A/B/history/receipts. Acceptance: coverage map, all required checks, reopen/scrub/indexed/catalogue/fixture results and unchanged old_hub inventory; explicit gaps and no invented acceptance/authority handover. Relevant rules: KH-DR-01–13/15–23.


Necessary caller/test adjustments land with the change that breaks compilation. KH-SL-017 supplies final cross-cutting proof, not the first tests for earlier work. No older-store migration is planned. The old_hub rename was observed during planning, but absence of hub and retained-data boundaries must be rechecked immediately before new creation.

Observed KH-SL-014–017 outcome: one full root API and unpublished 0.10.0 identity;
one current files/4 constructor/required storage port and unified bounded traversal;
current-only tools/source bindings/retained-directory guards; full 95-test native,
3-test documentation and 17-test host batch plus release builds and actual fresh
Hub/catalogue/fixture verification. The new head has 9,578 selected records and
3,452 explicit draft gaps. Final task/report/status edits follow the captured
checkpoint; no human acceptance, authority handover or archive is implied.

## Proposed native macOS outcome — 2 November 2026

### KH-SL-018 — Run the current file backend natively on macOS

Status: subsequently authorised by the user's Apply request and implemented with
[complete native Mac evidence](evidence/macos-file-backend/README.md). All Mac
checks and isolated journeys passed. The user directed Linux verification to be
accepted as given for now and run separately. Native Linux regression, both native
fixture-copy directions and final cross-platform review remain open. This is
technical Mac verification, not human acceptance or authority handover.

- **Actor/trigger:** a maintainer on the current Apple Silicon Mac wants to create,
  use and reopen a full Hub through the ordinary embedded API, instead of stopping
  at a successful compilation followed by an unsupported-platform error.
- **Outcome:** the same `KnowledgeHub` and `FileBackend::new` serve macOS
  arm64/local APFS and the existing Linux x86_64/local ext4 profile, with checked
  host declarations and the same guarded publication, history and recovery rules.
- **Hard prerequisites:** the implemented KH-SL-014–017 sole full API, files/4,
  current host tooling and safety coverage. Native macOS/APFS and Linux/ext4 runs
  are evidence prerequisites; a matching Rust version alone supplies neither.
- **Journey KH-J-11:** declare a supported host/path → create/register the full
  catalogue → commit sourced definitions and baseline A → commit a successor and
  baseline B → close/reopen → resolve exact reads/queries and original retries →
  compact/scrub → recover a deliberately interrupted operation.
- **Stories:** KH-US-31: as a Mac maintainer, persist and reopen a full model so I
  can use the library locally; KH-US-32: inspect the same exact history and
  original outcomes after recovery or an offline current-format fixture copy;
  KH-US-33: run maintained commands and identify which native platform and failure
  cases their evidence actually covers.

| Story-map level | Establish scope | Write and inspect | Resume and maintain | Verify and review |
| --- | --- | --- | --- | --- |
| Tasks | Select checked local APFS/ext4 profile. | Use ordinary full API and exact source/ownership rules. | Reopen A/B, retry, query, compact and scrub. | Exercise conflicts/interruption and inspect native evidence. |
| Proposed cut | KH-US-31: supported Mac creation with safe refusal elsewhere. | KH-US-31: complete sourced definition and successor. | KH-US-32: exact history and operation identity survive. | KH-US-33: current Mac/Linux commands and separate observed results. |

- **Completion:** the ordinary Mac path actually runs; existing Linux behaviour
  passes its native regression batch; both retain denial/conflict/idempotency,
  reader/maintenance coordination, all eight publication and seven maintenance
  process-kill checkpoints, and sync-error/uncertainty checks. A closed synthetic
  files/4 fixture is checked in both copy directions without rewriting metadata
  or semantic values. Builds alone do not satisfy this slice.
- **Boundaries:** first Mac architecture is arm64; support evidence records the
  exact OS/APFS configuration, beginning with the available macOS 26.6.2 host.
  Intel Macs, other filesystems, network/cloud-synchronised directories,
  simultaneous multi-host access, general import/export, power-loss validation,
  crate publication and canonical regeneration are deferred. No implementation
  work may reset, refresh or import `hub/` or `old_hub/` as part of this slice.
- **Trace:** ontology §§5/14/17/20.4/21.2/21.4–21.7/21.10;
  KH-DR-02/04/07–13/15/17/20. The
  [change](../openspec/changes/support-macos-file-backend/proposal.md) owns the
  bounded behavioural delta. Repository definitions and historical Hub captures
  keep their existing authority and exact revisions.

## MM-SL-01 — External source-bound model authoring

The user authorised applying the open changes on 3 November 2026. Their dependency order and recorded technical status are:

| Change | Delivered boundary | Task status |
| --- | --- | --- |
| extend-experience-trace-modelling | Multi-journey maps, shared placements/cuts, scenario traces and qualified dependencies. | 9/9 |
| refine-logical-and-design-stage-contracts | Exact/inherited types, direct lifecycle subjects, logical constructors, planned realization and honest draft gaps. | 15/15 |
| add-source-bound-model-authoring-tools | Bounded exact-source capture, typed compilation, native assessment/publication, original-operation recovery and independent coverage checks. | 15/15 |
| deliver-model-import-and-verification-skills | Portable agent skills, reviewed source accounting and persisted reference/unrelated models. | 13/14; one external host execution gate remains open. |

The working journey is source documents/UI → external analyst/agent interpretation → typed plan → native assessment/publication → independent verification. Source authority and interpretation stay with that external author; the tool delegates schema and graph checks to the native library. Capture alone does not produce a complete interpretation.

The [local reference fixtures](../examples/fixtures/README.md) have actual A/B baselines, exact original sources, recovered receipts and exhausted readback queries after reopen. The Memento model is valid and incomplete: its 1,504 structured omissions remain explicit while narrative/code/artifacts retain details beyond the formalized graph. Declared source-accounting success, complete business meaning, implementation evidence and human acceptance remain separate results.

[The exact-source batch](evidence/mm-sl-01/20261103-apply/README.md) passed 115 Rust target tests, three doctests, 19 host checks, formatting, Clippy and release examples. The task files retain pending external gates; platform closure remains with KH-SL-018. The root canonical Hub was preserved at its earlier codec-/2 checkpoint; the /3 reference Hubs are separate instances under target/rich-design.

Review sequence: inspect source-to-model meaning and the explicit structured gaps, obtain human meaning review, then select further formalization as its own bounded change. MM-SL-02 registration/Modelarium editing, canonical self-model refresh, publication and authority handover remain later work.

### MM-SL-02 C8 — Inspect original native publications under current authority

The user authorised `expose-native-model-history` on 4 November 2026. A current reader
can resolve another publisher's original operations, discover accepted publications in
bounded pages at an exact anchor, reconstruct affected vectors and compose the results
with existing exact source/predecessor/baseline reads. `ReadHistory` is explicit alongside
ordinary `Read`; actor-bound operation recovery and mutation reuse remain unchanged.
The [consumer guide](../design/native-model-history.md) records the delivered signatures,
limits, continuations and failure outcomes. [Evidence](evidence/native-history/20261104-apply/README.md)
separates public/runtime/reopen/packing checks from human acceptance and unavailable
platform gates. No new storage/semantic codec, canonical refresh, Modelarium transport/UI
or terminal implementation belongs to this cut. Later C9 and terminal work require their
own bounded authorisation and must consume the delivered contract.

### TC-SL-01 — Explore a small native model from a terminal

The user authorised `add-terminal-model-client` on 4 November 2026, following C8.
The slice supplies a binary-local read-only host, shared CLI/TUI exploration,
native publication/affected-reference reads, exact source inspection, A/B baseline
comparison and offline derived diagrams. FruitTrucks and unrelated equipment
fixtures retain exact sources, draft omissions, original publisher-A receipts,
reader-B authority and superseded baseline coverage. No model execution or human
meaning acceptance is inferred from authoring validity or source accounting.

The [consumer guide](../design/terminal-client.md), [quick start](../examples/fixtures/terminal-client/QUICKSTART.md)
and [evidence](evidence/terminal-client/20261104-apply/README.md) own commands and
observed checks. Delivery is a local macOS arm64/APFS package with paired native
executables and relative workspaces. Human review/acceptance, Linux native
execution, signing/notarization/publication, Modelarium C9, canonical refresh and
authority transfer remain separate. Review the bounded consumer before selecting
another implementation slice; do not close earlier external authoring/host gates
from this terminal evidence.

### TC-SL-02 — Inspect an exact model graph inside the terminal

The user authorized `add-kitty-terminal-graph-view` on 5 November 2026, consuming
the delivered TC-SL-01 Explorer/visualization and C8 native-history foundation.
`kh view REF` and uppercase browser `V` provide an explicitly optional graph view
with bounded embedded rendering, keyboard node/edge/source inspection, current
authority checks, exact pinned recovery and a reversible return to browsing.
The existing JSON/Mermaid/HTML exports and lowercase `v` retain their contracts.

The implementation and automated verification are recorded in the change tasks
and [dated TC-SL-02 evidence](evidence/terminal-kitty-graph/20261105-apply/README.md).
Actual directly connected Kitty and Ghostty observations remain independent open
gates; synthetic PTY success does not establish either terminal's image display.
Review the bounded instructor path on available native terminals and obtain
separate human meaning/usability review before selecting another consumer slice.
No new library/host contract, fixture enrichment, Modelarium C9 work, canonical
Hub refresh, archive or definition-authority transfer follows from this delivery.

A subsequent user attempt in Ghostty exposed a placement failure: full-screen
erase reclaimed the just-uploaded image. The [placement correction](evidence/terminal-kitty-graph/20261105-placement-fix/README.md)
uses text-only erase and tests actual image-storage state in the protocol
responder. Package `final-05` supersedes `final-04`. Successful native observation
remains a separate gate; passing a responder that accepts every placement was
insufficient evidence. Retain this case in the consumer verification sequence.
