# Spec Delta

## Purpose

Let application authors describe precise logical values and model-level reference targets without mistaking metamodel definitions for domain definitions or claiming runtime type execution. Trace: MM-SL-01/C2, O-03, G6.2/G6.3/G6.4, ontology §§5/9/11/21.4 and KH-DR-01–08/12/15–18.

## ADDED Requirements

### Requirement: Explicit logical constructors
A DataType SHALL distinguish primitive, enumeration, record, list, set and reference constructors. Enumeration members, record schema and collection item type SHALL be explicit exact model data where applicable. Missing required constructor information SHALL require a permitted sourced gap; incompatible or malformed supplied information SHALL be invalid.

#### Scenario: Describe a collection of structured references
- **WHEN** an author declares an enum, a record schema and a collection whose item type describes domain references
- **THEN** native reads identify the exact members, record schema, item type and reference targets separately
- **AND** the model does not rely on a free-text representation alone to establish those distinctions

#### Scenario: Contradictory constructor
- **WHEN** a primitive supplies record-only properties, a complete enum has no members, or a collection has an unresolved item type
- **THEN** assessment reports its exact constructor/property violation rather than interpreting arbitrary Body text as a replacement schema

### Requirement: Logical references target application definitions
Logical reference types SHALL identify exact permitted EntityType, ValueObjectType, DataSchema or AggregateDefinition application definitions, including valid subtypes. FieldDefinition target restrictions SHALL narrow rather than widen its reference DataType's declared targets. Ontology ElementTypeDefinitions SHALL NOT stand in for those application targets.

#### Scenario: A field narrows a reusable reference type
- **WHEN** a reusable reference DataType permits two application definitions and a field selects one
- **THEN** its effective target contract is that declared subset under the shared assignability semantics
- **AND** references to a metatype or an unrelated application definition are rejected

#### Scenario: Widening restriction
- **WHEN** a field adds a target not assignable to any target allowed by its DataType
- **THEN** validation rejects that field contract without changing accepted state

### Requirement: Temporal and representation metadata
Logical date, timestamp and duration types SHALL be distinguishable from undifferentiated text. Authors SHALL be able to retain supplied format, unit, timezone and representation policies as source-bound definitions. Model validation SHALL check their declared structural consistency without evaluating the described application's runtime values or business predicates.

#### Scenario: Preserve expiry and waiting definitions
- **WHEN** a source describes a duration limit, timestamp policy and cancellation condition
- **THEN** their exact meanings and source anchors remain inspectable after reopen without starting timers, evaluating expiry or executing cancellation

### Requirement: Reusable logical vocabulary
Logical constructors and validators SHALL be application-independent. Concrete domain entities, fields, enum members, limits and policies SHALL be model/fixture content. Adding a richer design SHALL NOT require new built-in application types, copied business fields or code branches keyed by source identifiers.

#### Scenario: Reuse constructors with different business data
- **WHEN** the reference model and an unrelated-domain fixture declare different entity fields, enum members and time limits
- **THEN** both compile and validate through the same generic logical contracts without production-code changes
