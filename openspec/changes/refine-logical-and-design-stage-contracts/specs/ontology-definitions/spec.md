## ADDED Requirements

### Requirement: Qualified assignable type selectors
Reference properties and relationship endpoints SHALL select qualified exact type definitions or explicit governed subject roles. Assignability SHALL follow the selected exact direct/transitive parent graph. Display names SHALL NOT establish identity. Generic subject roles and native payload boundaries SHALL remain explicit and SHALL NOT grant registration or mutation privileges.

#### Scenario: Direct and transitive descendants
- **WHEN** a field requires type T and values resolve to T, its direct subtype and its transitive subtype
- **THEN** each meets the type selector, subject to its other obligations
- **AND** an unrelated type, a same-named type from another identity, and a base value where only a descendant is allowed are rejected

#### Scenario: Native record impersonation
- **WHEN** a registered application type claims parentage intended to impersonate a governed native payload kind
- **THEN** registration rejects the invalid boundary crossing and ordinary model authority gains no schema or native-record privileges

### Requirement: Deterministic inheritance resolution
Declared parents SHALL resolve exactly and form an acyclic graph. Inherited fields and constraints SHALL have deterministic identity-based resolution, including unambiguous multiple inheritance. Conflicting inherited declarations or local shadowing SHALL be rejected. Constraints inherited from ancestors SHALL remain applicable alongside descendant constraints.

#### Scenario: Diamond inheritance with one declaration origin
- **WHEN** two valid parent paths inherit the same exact field/constraint declaration
- **THEN** the descendant inherits that declaration once and the resolved schema is independent of traversal order

#### Scenario: Ambiguous or invalid hierarchy
- **WHEN** parents are unresolved or cyclic, or different inherited declarations compete for one field name, or a child shadows an inherited field
- **THEN** registration fails with the affected exact definitions rather than choosing a parent by order

### Requirement: Governed extension and profile selection
New types, parents, signatures and ontology/profile selections SHALL use authorized guarded registration. A profile SHALL select declared vocabulary/obligations, never inject undeclared fields. Prepared model meaning SHALL bind the exact schema/parent revisions and selections; intervening relevant changes SHALL NOT silently alter it.

#### Scenario: Authorized extension survives reopen
- **WHEN** a registration actor publishes qualified extension types, a signature and a profile, then model data uses them
- **THEN** reopen resolves those immutable schema tokens and exact baseline bindings under the new contract

#### Scenario: Model edit attempts schema registration
- **WHEN** an ordinary model editor submits a registration or a profile attempts to add undeclared fields
- **THEN** the change is rejected without advancing the accepted head

### Requirement: One new contract without compatibility branches
Affected catalogue/profile and public schema contracts SHALL have explicit new identities. Unsupported obsolete contracts SHALL fail clearly. Fresh generated fixtures SHALL replace earlier generated inputs; no legacy reader, alias, payload translation, dual-version execution or migration SHALL be introduced. Current-contract history SHALL retain its exact immutable bindings.

#### Scenario: Old shape is presented
- **WHEN** a request supplies an obsolete selector or lifecycle subject shape to the revised contract
- **THEN** it is rejected with the unsupported identity or schema path rather than converted
- **AND** unrelated stores remain untouched

#### Scenario: Generic extension fixture
- **WHEN** the rich reference and unrelated-domain fixtures exercise new parent/reference/signature contracts
- **THEN** their domain names, attributes, enum values and policies remain fixture data with no corresponding built-in application vocabulary or validators
