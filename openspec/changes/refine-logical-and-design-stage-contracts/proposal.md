# Proposal

## Why

The current catalogue confuses ontology metatypes with particular application subjects, accepts reference roles by exact name rather than declared inheritance, and makes some planned architecture depend on implementation artifacts. MM-SL-01 needs generic native contracts that represent rich designs accurately without converting KnowledgeHub into their runtime.

## What Changes

- **BREAKING** Give LifecycleDefinition one exact, directly queryable application-definition subject and one authoritative binding; remove the inverse-only completion path.
- **BREAKING** Use qualified type/role selectors and declared direct/transitive assignability consistently for property references, relationship endpoints and subtype-enabled reads.
- Enforce inherited schemas and structural/semantic constraints, deterministic unambiguous multiple inheritance, and native-kind/registration boundaries.
- Make logical enum/record/collection/date/time/reference definitions explicit and distinguish planned architecture from actual realization claims.
- Preserve sourced permitted gaps as incomplete definitions; reject malformed supplied values regardless of attached gaps. Stored rule descriptions/artifacts never execute.
- Use fresh contract/catalogue/serialization identifiers and fixtures where affected. Backward compatibility is deliberately excluded.
- This is C2 of MM-SL-01. Actor: model author registering and using rich definition schemas. Completion: direct subjects, logical types, extensions and descendants validate and remain exact after native publish/reopen. C1's selected experience schemas are the cumulative catalogue predecessor; its feature is not reimplemented.

## Capabilities

### New Capabilities

- `definition-subject-bindings`: Exact lifecycle-to-application-definition ownership and native forward/reverse inspection.
- `logical-design-types`: Structured logical type constructors, model-level reference targets and bounded date/time semantics.

### Modified Capabilities

- `ontology-definitions`: Qualified selectors, declared inheritance/registration and one current rich-design contract.
- `ontology-conformance`: Inherited value/endpoint/structural checks, explicit gap behavior and design-stage realization.
- `model-navigation`: Explicit exact-type versus descendant-inclusive selection against the selected parent graph.

## Impact

KnowledgeHub only: public model/type contracts, catalogue data/bootstrap, validation and inheritance resolution, query/neighbour/impact matching, codecs/version metadata where changed, native examples/tests and current guides. Trace: ontology proposal §§4–5, 7–13, 16–17, 20–21; KH-DR-01–13/15–19; O-03/O-04 and native/standalone obligations G6.1–G6.4 in the MM feature plan.

All application-specific concepts and attributes remain model/fixture data. Modelarium controls/host adaptation and its acceptance portion of G6.1-04/G6.4-05 are MM-SL-02, not prerequisites or tasks here. No Modelarium-owned change is needed for this native outcome; if one becomes necessary to deliver it, stop rather than expanding scope. No rule/lifecycle execution engine, compatibility path, remote adapter, store conversion, canonical-hub rewrite or publication.
