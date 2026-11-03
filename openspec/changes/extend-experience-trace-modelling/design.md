# Design

## Context

See proposal.md and specs/experience-trace-modelling/spec.md. The current catalogue stores StoryMap.journey and UserActivity.journey as one exact reference; OutcomeCut.stories names stories directly; Scenario has no requirement binding. Generic storage, provenance, bounded property/assertion reads and atomic Changes already exist. Native conformance fixtures currently assert fixed catalogue counts and must distinguish source coverage from added refinements.

This is KnowledgeHub-owned C1 of MM-SL-01. The planning origin is the sibling feature plan, but the contract below is self-contained and has no Modelarium runtime/build/test dependency.

## Goals / Non-Goals

**Goals:** one faithful product map across existing journeys; explicit task/story placement and outcome cuts; direct scenario traces; source-bound prerequisites; deterministic native validation and reopen.

**Non-Goals:** source interpretation/inference (C4), a general source compiler (C3), UI implementation, Modelarium changes, runtime dependency evaluation, legacy schema support, canonical self-model rebuild, new storage adapters or publication.

## Decisions

### 1. Use explicit scope and placement records

Replace StoryMap.journey with required journeys, a nonempty canonical Set of exact Journey references. Replace UserActivity.journey with the same shape. An activity's journeys must be a subset of its map's journeys. Retain StoryMap.activities and UserActivity.tasks as explicit ordered lists; UserTask.activity remains its single owner.

Add the generic Experience-family type StoryPlacement with graph-owned map: StoryMap, task: UserTask, story: UserStory and a nonnegative integer order. It also uses the ordinary name/description/provenance kernel. StoryMap.placements is an ordered list of exact placements. Each placement's map must select it, and its task's activity must be selected by that map. Reject duplicate exact members and ambiguous duplicate order within the same task.

Replace OutcomeCut.stories with a nonempty canonical Set of StoryPlacement references. Its map remains singular. Derived story membership follows placement.story and never becomes another authoritative list. Cut prerequisites stay within one map and remain acyclic.

Alternatives rejected: a fabricated umbrella journey misattributes source structure; per-map story copies destroy shared identity; keeping both authoritative stories and placements creates competing selections. A single-journey map is an ordinary one-member journeys set, not a legacy payload mode.

### 2. Declare two qualified trace signatures

Add Scenario illustrates Requirement with no qualifiers and no implied cardinality beyond zero-or-many. Its assertion carries ordinary exact provenance/review bindings. Existing formalises, supportsStep and satisfies links remain available; a test is not created just to connect a scenario.

Add UserStory dependsOn UserStory with dependency_kind in {required, conditional} and nullable condition Narrative. Required edges have null condition; conditional edges require a resolving nonblank condition. Reject self-links and cycles of required edges. Conditional edges are informational and not evaluated; they do not create runtime execution order.

Use distinct qualified signature identities even if another family uses a similar display name. The required-edge cycle rule is a governed structural check in the existing validator, not an evaluator for user-supplied Body code. The signature's universal acyclic flag remains false because conditional edges do not participate in that specific prohibition.

### 3. Keep ownership and provenance exact

Names/ordering/references are graph-owned; descriptions and condition narrative resolve to exact source fragments. Original story IDs survive reuse and source moves. Assertion and placement identities are independent; parallel claims are not collapsed by endpoint pair.

Schema/refinement provenance cites ontology proposal §§5–8/14/17 and a maintained general experience-contract decision document produced during implementation. Memento provides fixture evidence, not new native business types or attributed source requirements.

### 4. Keep implementation within existing native boundaries

Update catalogue.json and catalogue registration/coverage, add structural checks in validation.rs, and use existing query/neighbour property and assertion projections. Update ontology_support specimens, ontology_contract and applicable public/native examples. No new runtime, service or database is introduced.

Use the existing Rust 1.99.1 and host-owned Tokio scheduler. Register through the separately authorized action; publish application model data through normal Validate/Commit. This change does not revise native filesystem guarantees. Tests run on supported disposable native storage and distinguish assumed platforms from observed runs.

### 5. Make a clean catalogue cutover

Allocate a new catalogue/profile revision for the revised shapes. Regenerate current specimens and reference plans; update maintained examples/current-contract documentation and structural coverage totals without replacing source draft 0.2's original coverage inventory.

Do not add old journey/stories field readers, coercions, aliases, dual profile branches or migrations. Existing hub and old_hub remain untouched; their regeneration is not a task. Within newly created Hubs, accepted revisions/receipts/baselines stay immutable and exact.

C2 is the next cumulative refinement and may revise shared selector/registration encoding. C1's generic experience definitions carry forward by semantic identity in a fresh final-contract fixture; intermediate stores are not a migration requirement.

## Risks / Trade-offs

- New placement records increase graph size -> retain separate identity and bounded reads; measure the full reference in C3/C4 rather than dropping structure.
- Scope and reciprocal map membership can drift -> assess the complete candidate selection atomically and use exact guards.
- Conditional prerequisites can be mistaken for execution -> retain a typed distinction and explicit non-execution contract.
- Existing count-based tests can hide missing source families -> report original source coverage and new refinements separately.
- A future consumer needs different UI projection -> document the native shape; do not create a Modelarium change in this slice.

## Migration Plan

No migration or backward compatibility. Adopt the new catalogue identity in fresh disposable/current fixtures, reject obsolete fields, and preserve unrelated stored data unchanged. Rollback uses the previous code plus its separate untouched stores; it does not down-convert a new store.

## Acceptance and sequencing

The native proof is: register -> capture sourced map/placements/cuts/traces -> assess -> commit -> query forward/reverse -> baseline -> revise -> reopen and recover the original receipt. Include wrong owner, required-cycle, stale-head, changed-intent and denied-registration cases plus an unrelated domain.

All tasks stay in KnowledgeHub. If delivering this native boundary would require editing a Modelarium-owned file or creating its change, stop and report the scope conflict. Human interpretation review and executed technical evidence are separate.
