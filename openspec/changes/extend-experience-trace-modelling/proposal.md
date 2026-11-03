# Proposal

## Why

A product-wide design cannot be faithfully expressed when StoryMap/UserActivity are restricted to one journey and scenarios have no direct requirement trace. MM-SL-01 needs reusable experience semantics so an analyst can create and query one source-bound design model outside Modelarium.

## What Changes

- **BREAKING** Replace single-journey map/activity ownership with explicit multi-journey scope and map-specific story placements; cuts select placements without duplicating story identities.
- Add qualified scenario-illustrates-requirement and declared story-prerequisite relationships, preserving exact sources and the difference between informational dependency and execution order.
- Keep the source ontology catalogue coverage and existing generic identity/ownership/atomicity guarantees; record the new shapes as explicit design refinements, not statements attributed to draft 0.2.
- Verify the product-wide Memento example and a small unrelated domain. No Memento entity, field, state, identifier or count becomes built-in vocabulary or validator logic.
- Use new contract/catalogue identities and regenerated fixtures. No legacy readers, aliases, dual schema modes, translators or migration work.
- This is C1 of MM-SL-01. Actor: model author with a source design. Completion: meaningful map/cut/trace definitions validate, publish, reopen and answer exact bounded queries. No earlier MM-SL-01 change is a hard prerequisite; C2 coordinates subsequent catalogue evolution.

## Capabilities

### New Capabilities

- `experience-trace-modelling`: Product-wide journey scope, story/task placements, outcome cuts, scenario traces and declared prerequisites.

### Modified Capabilities

None. The existing general ontology-definition, provenance, conformance and guarded-publication contracts remain applicable.

## Impact

KnowledgeHub only: src/ontology/catalogue.json, catalogue/validation/navigation support, ontology coverage records, native specimens/tests and maintained current-contract documentation. Trace: ontology proposal §§5–8, 14, 17, 21.3–21.7/21.10; KH-DR-01–06/08–13/15–19; MM feature O-01/O-02.

No Modelarium code/configuration/OpenSpec change or application acceptance is required. No rule execution, source-app implementation, backend addition, publication or existing hub/old_hub rebuild is included. Exact schema decisions are recorded in this change's design; later changes consume them. Planning creates no working-model or human-acceptance evidence.
