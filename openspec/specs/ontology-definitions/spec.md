# ontology-definitions Specification

## Purpose

Let authors inspect and use a complete versioned System Definition Ontology,
with explicit schemas and source traceability rather than a fixed two-type seed.
Trace: KH-DR-03/04/15/16; KH-US-18/19; ontology §§4–8, 17, 21.3.

## Requirements

### Requirement: Complete source catalogue
The library SHALL supply exact definitions covering the 17 kernel concepts, 128 profile types in all 14 families of source draft 0.2, and 32 typed relationship declarations in section 8. Coverage SHALL also account for specialised contracts in sections 9–14 and validation categories V-01–17. Counts alone SHALL NOT establish semantic coverage.

#### Scenario: Discover the full catalogue
- **WHEN** a permitted author discovers the registered source ontology
- **THEN** every entry in `ontology-coverage.md` resolves to its exact definition, schema, constraints, source binding and implementation coverage status
- **AND** names shared by different relationship signatures remain distinguishable

#### Scenario: A family is omitted
- **WHEN** catalogue assessment cannot resolve the UI, AI or any other declared family
- **THEN** it reports incomplete ontology support, even if the KnowledgeHub self-model does not use that family

### Requirement: Versioned ontology definitions are data
Ontology, element-type, relationship-type, constraint and profile definitions SHALL have stable semantic IDs, immutable revisions and exact source/provenance bindings. They SHALL be retrievable through public contracts. Changing a definition SHALL require an explicit successor and SHALL NOT reinterpret snapshots pinned to its earlier revision.

#### Scenario: Read the Requirement definition
- **WHEN** a reader requests the exact registered Requirement type revision
- **THEN** the result identifies its declared properties, property types, owners, cardinalities and applicable constraints independently of any Requirement instance

#### Scenario: Existing type meaning changes
- **WHEN** an author submits changed constraints under an already published definition revision
- **THEN** the library refuses the replacement and preserves earlier model interpretation

### Requirement: Explicit property and relationship schemas
Each supported type SHALL declare permitted properties, typed values, requiredness, ownership and applicable constraints. Each relationship declaration SHALL identify allowed endpoint kinds or roles, direction, cardinalities and qualifier schema. Unknown normative fields and predicates SHALL be rejected; a type label with an unchecked property bag SHALL NOT qualify as support.

#### Scenario: Distinct invokes signatures
- **WHEN** an author inspects UseCase invokes FunctionDefinition and ProcessStep invokes UseCase
- **THEN** the catalogue returns distinct qualified declarations with their respective endpoint constraints

#### Scenario: Unsupported property or qualifier
- **WHEN** a candidate supplies an undeclared normative property or relationship qualifier
- **THEN** validation identifies that field and its pinned schema without admitting the candidate

### Requirement: Preserve source and refinement status
Definitions SHALL distinguish source statements from implementation schema refinements. Source omissions SHALL be recorded explicitly with the chosen refinement and decision provenance; missing source detail SHALL NOT silently become an attributed source requirement. Draft source status SHALL remain inspectable.

#### Scenario: Source lists a type without exhaustive fields
- **WHEN** the supplied catalogue gives UserTask a concrete property schema beyond the source's type list
- **THEN** the added field choices identify their proposed/adopted decision provenance while retaining the original source binding and draft status

### Requirement: Guarded registration and exact profile selection
Registration SHALL use current scoped host authority and an atomic guarded change. Schema references and constraints SHALL resolve before admission. A model SHALL pin exact ontology/profile revisions. Modelled roles and submitted constraint descriptions SHALL NOT grant permission or execute arbitrary host code.

#### Scenario: Denied registration
- **WHEN** current host policy denies ontology registration
- **THEN** no ontology, profile, model-head change or successful original receipt becomes accepted

#### Scenario: Malformed registration
- **WHEN** a type refers to an absent parent, an invalid cardinality or an unresolved constraint definition
- **THEN** the whole registration is rejected with structured violations and the earlier catalogue remains usable

### Requirement: Catalogue availability differs from profile completeness
The complete ontology SHALL remain available independently of which families apply to a selected model. Profiles SHALL explicitly select permitted vocabulary and completeness obligations. An admitted complete profile SHALL NOT be inferred merely from the existence of some typed records.

#### Scenario: Library has no product screens
- **WHEN** the KnowledgeHub library profile declares product UI inapplicable with a reason
- **THEN** the self-model requires no invented screen, while the ontology still permits correctly constrained ScreenDefinition instances in other models
