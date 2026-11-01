# Tasks

Technical implementation and scoped verification are complete. The user authorised
this change through an explicit apply request; planning artifacts alone did not
supply that authorisation. Each checked item binds implemented behaviour to the
native/public-consumer evidence indexed in `delivery/evidence/full-ontology.md`.
The sourced self-model retains explicit draft gaps. Human adequacy, acceptance,
archive/sync and repository-authority handover remain separate follow-up work.
Existing Rust 1.99.1 was used; no compiler was installed. Platform compliance is
the user's assumption, recorded independently of actual native observations.

## 1. Preserve the existing contract and exact inputs

- [x] 1.1 Record implementation authorisation and the reviewed KH-D-11–15 choices against this exact proposal/design/inventory; verify source requirements, proposed refinements, inherited KH-D-01–06 and explicit exclusions remain distinguishable before code changes.
- [x] 1.2 Retain immutable files/1 seed fixtures, canonical request/record vectors and original receipts; run the existing public/race/replay/fault/executor checks and record the actual starting Rust/build/source hashes so later compatibility checks can compare exact values.
- [x] 1.3 Capture source PDF/Markdown and the selected repository working-tree or real Git snapshot with independent original hashes; verify every K/T/R/P/V/S inventory denominator and all source/self-model bindings resolve without rewriting original files or labelling modified bytes as HEAD.

## 2. Establish typed ontology contracts and guarded v2 storage

- [x] 2.1 Add owned value/property/schema/reference/violation contracts with separate v2 revision tokens and canonical record digests; verify nested typed values, safe/lossless numeric encodings, lists versus sets, absent/null, unknown fields and independent reuse of a revision token for different content.
- [x] 2.2 Implement the trusted meta-schema, atomic ontology/profile registration and exact public discovery; verify mutually referring/recursive type references resolve as one candidate, inheritance cycles and bad schemas fail, and default denial/revocation/policy errors cannot admit registration; document the bootstrap/definition distinction.
- [x] 2.3 Implement all K01–K17 native/public materialisations and one typed exact resolver; verify represents targets an actual element revision, supportsAssertion targets a native assertion, contradicts links two native assertions, and wrong kernel roles fail without shadow records; document exact addressing.
- [x] 2.4 Implement the closed deterministic constraint primitives and profile assessment layers; verify type/cardinality/ownership/context/root/cycle/membership/schema/contract/review-scope checks and that explicit draft gaps never waive integrity; document which checks establish structure versus human adequacy or actual execution.
- [x] 2.5 Implement exact binary/UTF-8 artifact capture and tagged native/derived locator manifests; verify original PDF/Markdown bytes and digests remain independent, every declared locator kind resolves a meaningful fixture, bad ranges/quotes/source-manifest bindings fail, and exact binary content survives reopen; document extraction evidence limits.
- [x] 2.6 Implement opt-in files/2 manifests, selected revision maps and versioned request/record codecs under the existing publication/receipt protocol; verify complete typed graph/artifact admission, current policy, same-base Busy/conflict, changed-intent refusal, cancellation, lost response and injected pre/post-replacement failures, while original files/1 hashes/receipts remain unchanged; document explicit versions and rollback limits.
- [x] 2.7 Document the new public registration/typed-record consumer and runtime boundary; compile and exercise it on existing Rust 1.99.1 using both Tokio schedulers, finite admission and draining, with no driver/private file handles or hidden runtime.

## 3. Deliver the complete catalogue with family-level evidence

Every family task includes meaningful complete/draft examples and independent
invalid examples for every type's declared obligations. Update exact row-to-
schema/specimen/evidence bindings as that family lands. Shared fixtures may
exercise several rows, but a name-only instance is not completion.

- [x] 3.1 Materialise all K/T/R/P definitions, shared structured values and profile obligations from `ontology-coverage.md` as typed catalogue data; verify registration integrity, exact count/identity/source/refinement coverage and public definition discovery, with no unresolved schema dependency or unlisted normative field.
- [x] 3.2 Complete the 7 Purpose/business schemas and their fixtures; verify capability/goal/function/measure distinctions, realises/contributesTo/decomposesInto endpoints, prohibited decomposition cycles and exact readback; document their registered row/evidence bindings.
- [x] 3.3 Complete the 8 Requirements/reasoning schemas and fixtures; verify requirement/rule/policy/assumption/question/decision/rationale/constraint meanings, proposed versus adopted status and claim-kind provenance without inferred approval; document row/evidence bindings.
- [x] 3.4 Complete the 10 Experience schemas and fixtures; verify explicit step identity/order/alternatives/handoffs, story/outcome/cut membership and missing actor/trigger/completion obligations; document row/evidence bindings.
- [x] 3.5 Complete the 6 Strategic DDD schemas and fixtures; verify context-qualified language, ownership/cross-context references and distinct same-label meanings without conflating deployment ownership; document row/evidence bindings.
- [x] 3.6 Complete the 7 Tactical DDD schemas and fixtures; verify one root and aggregate membership/invariants, value equality/contracts and lifecycle references, including valid incomplete-root draft and invalid two-root cases; document row/evidence bindings.
- [x] 3.7 Complete the 9 Application behaviour schemas and fixtures; verify full S01 input/output/guard/effect/error/determinism/limit/body contracts, exact logical handlers and a rejected name-only complete function; document row/evidence bindings.
- [x] 3.8 Complete the 11 Events/processes schemas and fixtures; verify definitions versus occurrences/executions, exact schemas/delivery/correlation, state/process membership, valid declared loops, invalid cross-process states and timer/compensation contracts; document row/evidence bindings.
- [x] 3.9 Complete the 10 Information/transformation schemas and fixtures; verify units/nullability/cardinality/selectors, all five transformation kinds, loss/inverse claims and exact run trace mappings without assumed reversibility; document row/evidence bindings.
- [x] 3.10 Complete the 12 Interaction/visual schemas and fixtures; verify component versus placement, P01–P06 composition/display/edit/use/issue semantics, data/action/schema bindings and normative layout versus diagram presentation, including invalid UI bindings; document row/evidence bindings.
- [x] 3.11 Complete the 11 Architecture/integration schemas and fixtures; verify component/module/interface/port/adapter/deployment distinctions, implements/translates signatures and exact native artifact/error mappings; document row/evidence bindings.
- [x] 3.12 Complete the 9 Security/trust schemas and fixtures; verify explicit permission/policy/trust/classification/retention/audit meaning, configuration secret references and that stored roles/policies never grant runtime admission; document row/evidence bindings.
- [x] 3.13 Complete the 9 Quality/operations schemas and fixtures; verify measurable scenario/objective/resource/configuration/recovery contracts and exact observed deployment/build/environment references without invented successful measurements; document row/evidence bindings.
- [x] 3.14 Complete the 9 AI participation schemas and fixtures; verify model/prompt/tool/context/retrieval/limit/evaluation/human-boundary contracts and absence of implied runtime execution or privilege; document row/evidence bindings.
- [x] 3.15 Complete the 10 Delivery/assurance schemas and fixtures; verify slice/change/task/release distinctions, exact cases/runs/evidence/acceptance scope and no promotion of planned work to observed success; document row/evidence bindings.
- [x] 3.16 Assess KH-SL-007 using the complete public catalogue; verify all 17 kernel kinds, 128 family types, 32 source and 6 prose/refined signatures plus V/S obligations have meaningful conformance and reopen evidence, and a missing family forces incomplete-support reporting; retain the measured catalogue/store size and scoped result.

## 4. Add coherent successors and exact named baselines

- [x] 4.1 Implement append-only revision lineage and selected normative revision maps; verify stable IDs through renames, exact predecessor guards, preservation of old bytes and refusal of changed content under a published token; document history versus selected-state semantics.
- [x] 4.2 Implement coherent document/property/assertion successor operations; verify changed narrative bindings and explicit endpoint pins commit together, incomplete bindings preserve the head and old selected provenance remains inspectable; document the authoring request/retained-candidate contract.
- [x] 4.3 Implement immutable named baseline create/read with exact ontology/profile/member/supporting-revision closure; verify valid historical support, wrong or missing exact endpoints, conflicting normative selections and no latest/timestamp substitution after head movement; document baseline membership.
- [x] 4.4 Implement stable baseline comparison; verify exact additions/removals/successors, assertion and provenance/representation changes, Unicode-ID ordering and current access checks without mutating either baseline; document comparison results.
- [x] 4.5 Exercise revision/baseline races, exact replay after later head movement and interruption around B publication; verify complete A/B and original receipts recover with no duplicate successor and no inherited review, using both scheduler configurations; retain feature evidence and support limits.

## 5. Add bounded public model navigation

- [x] 5.1 Implement exact-type/ID selection and directional qualified-signature traversal over one resolved snapshot/baseline; verify stable documented order, kernel endpoints, distinct assertion retention, cycles, absent/null preservation and explicit external references; document supported operators.
- [x] 5.2 Implement finite page/depth/visited-subject/edge budgets and explicit exhaustion/truncation; verify the documented defaults/maxima, rejected excessive requests and preservation of total budgets across pages; document narrower-request recovery after a total bound.
- [x] 5.3 Implement actor/query/selection-bound opaque continuation state with capacity, explicit release and idle expiry; verify 64-cursor capacity, 30-minute inactivity through a controlled clock, invalidation after backend close/reopen, actor/predicate misuse rejection and no head mixing between pages; document continuation lifetime.
- [x] 5.4 Verify current policy on every selection/traversal/continuation, including revocation between pages, and exercise requirement-to-case-to-run queries through public contracts; retain feature evidence and document safe errors without adapter layout.

## 6. Load KnowledgeHub's full applicable self-model

- [x] 6.1 Implement the host-side exact-source/working-tree/build capture for `self-model-map.md`; verify original PDF/Markdown/code/log artifacts, resolving anchors, actual compiler/filesystem facts and historical versus current-code evidence, with no false Git revision or fabricated observation; document input provenance.
- [x] 6.2 Implement public loading of the purpose/domain/language/rule/journey/requirement/behaviour/information groups; verify every selected owning-document item and public semantic contract/I/O operation has a typed record/source binding and preserve proposed analysis status; retain the group coverage manifest.
- [x] 6.3 Implement public loading of architecture/module/adapter/policy/decision/operation/delivery/verification groups; verify actual selected source modules, all exposed operation contracts, real case/run/build/evidence links and explicit planned/inapplicable/gap status without synthetic acceptance; retain the group coverage manifest.
- [x] 6.4 Deliver the public consumer's exact required assertion paths and Q01–Q08 reads; verify catalogue discovery, requirement → use case/function/component/source and requirement → case → actual run/evidence navigation without private file reads; document and run the example as written.
- [x] 6.5 Extend that consumer to establish A, coherently revise the sourced explanation and explicit pins into B, drain/close/reopen and compare both baselines; verify identities, original artifacts/provenance, exact old/new narratives and original operation outcomes remain exact; retain the scoped KH-SL-008 walkthrough result.

## 7. Verify the integrated outcome and record review boundaries

- [x] 7.1 Run integrated full-catalogue/self-model normal/denied/invalid/concurrent/replay/cancellation/corruption/fault/restart checks plus unchanged files/1 conformance; record checked source/build/toolchain/filesystem and all observable results against ontology §21.10 without power-loss or deferred-adapter claims.
- [x] 7.2 Check the entire K/T/R/P/V/S and self-model group/Q coverage manifests against actual registered revisions, specimens and evidence; verify no missing family, substituted generic bag, unsupported relationship or unexecuted success is hidden by a passing self-model profile.
- [x] 7.3 Review actual findings/limits at both outcome boundaries, update the owning domain/slice/self-hosting documents and keep implementation, structural conformance, observed runs and human acceptance separate; verify no automatic repository authority handover or completion of deferred interchange/review/query breadth is asserted.

## Workflow follow-up

- Obtain actual project review/acceptance independently of technical checks.
- Sync/archive only after that review and required evidence, preserving existing
  source requirement identities and the first change's independent status.
- Reassess remaining KH-SL-002–006 scope using actual delivered capabilities;
  portable interchange, broader review/query work and authority handover retain
  their explicit remaining obligations.
