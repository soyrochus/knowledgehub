# Logical design contract

Authorised implementation refinement of ontology draft 0.2 §§4–5, 7–13 and 17,
3 November 2026. The repository owns definitions; human adequacy review remains
separate from technical conformance.

The current cumulative catalogue/profile revision is catalogue-3, with record/3
and request/3 semantic encodings. C1's intermediate catalogue-2 specimens are
disposable. Files/4 continues to identify the storage layout; unsupported record
and request codecs fail explicitly. No old selectors or subject shapes are read,
converted or migrated, and existing canonical stores are not rewritten.

Reference schemas and relationship endpoints use tagged TypeSelector::Type with
an ExactRef or TypeSelector::Role with AnySubject/ElementRevision. Types match
themselves and exact direct/transitive descendants. Names are diagnostic only.
Native kinds require their actual native payload and cannot acquire application
parents or be extended. Registration uses current RegisterOntology authority;
ordinary model authority cannot register schemas. Profiles select vocabulary and
completeness obligations, and cannot inject fields.

Parents form an exact immutable DAG. Fields and constraints retain their exact
declaration origins. A diamond inherits each origin once. Distinct origins with
one field name and local shadowing are rejected, regardless of equal shapes.
All inherited integrity obligations remain active. Stored business Body/code,
time and cancellation descriptions are data and never invoked by assessment.

Query.include_subtypes defaults to false. Typed selection, traversal target
filtering, Neighbours.include_subtypes and impact target/step predicates share
the same matcher. Inclusive selection without a type is invalid. Reads retain
exact snapshot/baseline parents, stable paging, current disclosure checks and
explicit budget truncation. Exhaust continuations and check exhausted before
claiming complete coverage; lookup by semantic ID remains exact identity lookup.

LifecycleDefinition.subject is its singular authoritative exact EntityType or
AggregateDefinition application definition (including modelling descendants).
EntityType.lifecycle and LifecycleDefinition.subject_type are removed. Reverse
lookup follows the native /subject property relationship. Ontology metatypes
cannot substitute for an application subject.

DataType.value_kind distinguishes primitives, enumeration, record, list, set,
reference, date, timestamp and duration. enum_values, record_schema, item_type
and reference_targets provide constructor-specific structured information.
Incompatible supplied constructor fields fail. Logical reference targets select
application EntityType, ValueObjectType, DataSchema or AggregateDefinition
records. Optional FieldDefinition.reference_targets is a nonempty exact subset
of the direct reference DataType's targets, never a widening or a collection
shortcut. Representation Body retains source-bound units, formats, timezone
and runtime policies. This does not validate application instances.

Module and Adapter require implementation_status planned/implemented. Planned
definitions permit absent artifacts; implemented definitions require resolving
nonempty artifacts or an exact permitted sourced omission. Supplied empty,
unresolved or malformed data always fails. Gaps apply to genuinely omitted
top-level obligations, with exact subject/path, nonblank reason and resolving
source provenance. They preserve valid/incomplete status, never execution or
acceptance. Baselines retain exact historical subjects, schemas, gaps and sources.

Run `cargo test --offline --test logical_design_contract` for native contracts
and `cargo test --offline --test ontology_contract` for all catalogue families.
Future Modelarium consumption remains outside these native checks.
