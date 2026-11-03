# Spec Delta

## Purpose

Prove the external-agent authoring outcome with the complete supplied Memento design and working UI, using application-specific fixture data while preserving a reusable ontology and generic tools. Trace: MM-SL-01/C4, O-01–O-06 and native/standalone G6.1–G6.4 obligations.

## ADDED Requirements

### Requirement: Self-contained original reference package
The reference SHALL retain the supplied five-document design hierarchy and working UI sources with exact provenance, media types, authority and dependency inventory. Maintained fixture inputs SHALL be usable from KnowledgeHub independently of a Modelarium checkout. Copying explicitly selected source bytes SHALL NOT require editing that source application or its OpenSpec changes.

#### Scenario: Generate from a KnowledgeHub-only checkout
- **WHEN** the maintained local fixture is selected with a fresh destination
- **THEN** the tool can capture its original documents, PDFs, HTML, CSS, scripts, images and supported extraction artifacts using the recorded source manifest
- **AND** unavailable dependencies and explicitly excluded metadata remain reported

### Requirement: Full design obligation inventory
Reference expectations SHALL identify all five contexts, 13 journeys, 36 US/R pairs, 72 S identifiers, 26 DR, 11 FA, 30 AC, six templates and five working UI surfaces from the supplied package. Every in-scope identifier and consequential interaction SHALL have exact source-to-model accounting. These application-specific counts and identifiers SHALL remain fixture data, not ontology or compiler constants.

#### Scenario: Counts match but meaning is missing
- **WHEN** a generated model has the expected totals but omits an identifier's required semantics, source support or consequential UI behavior
- **THEN** independent coverage fails for that exact obligation instead of accepting the totals as proof

#### Scenario: Reference source revision changes
- **WHEN** the maintained input digest changes
- **THEN** its expected inventory and mapping decisions require explicit regeneration/review rather than silently reusing previous coverage evidence

### Requirement: Faithful product experience and design traces
The model SHALL preserve shared stories and the actual multi-journey P1–P5 map without inventing another source journey. It SHALL support exact forward/reverse requirement, scenario, rule, command, interaction and artifact traces. Source-defined dependencies, cuts and conditional behavior SHALL retain their declared meaning without artificial TestCases used solely as missing relationship bridges.

#### Scenario: Inspect the representative approval trace
- **WHEN** verification queries US-32/R-32, S-32a/b and their declared rule, command, interaction, binding and artifact support
- **THEN** the selected native baseline resolves the actual source-defined trace and exact originals in both applicable directions
- **AND** the presence of those definitions does not imply a successful application run

### Requirement: Working UI behavior is design content
The reference SHALL account for the five UI surfaces, six templates and consequential states, controls, confirmations, failures and bindings in the supplied working UI. Exact artifacts SHALL preserve visual/behavioral details beyond structured ontology coverage. An executable graph-only renderer or a running source application SHALL NOT be required to represent that design.

#### Scenario: A script-defined behavior exceeds a generic binding
- **WHEN** a consequential UI detail cannot be completely formalized in the bounded ontology
- **THEN** its exact artifact/locator and supported structured binding remain inspectable, with the formalization limit explicitly reported rather than losing the behavior

### Requirement: Generic rich subject and extension semantics
The reference and a small unrelated-domain fixture SHALL exercise direct lifecycle subjects, logical application targets, planned realization, valid omission gaps and subtype/extension contracts through the same production ontology and tools. Concrete application entities, fields, states, limits and business rules SHALL remain model/fixture data. Obsolete schemas SHALL be rejected, without backward compatibility.

#### Scenario: A different domain uses the same capability
- **WHEN** the unrelated fixture declares different entities, attributes, enum/state values and identifiers
- **THEN** it creates and verifies a valid model through the same schemas, compiler and native checks without application-specific code branches

#### Scenario: Described code is retained
- **WHEN** a fixture includes rule/code artifacts, time limits and cancellation descriptions
- **THEN** source/metadata/traces survive reopen while no business entrypoint, timer, external action or target workflow is invoked

### Requirement: Real native persistence and honest evidence
Reference acceptance SHALL include fresh creation, named baseline, exact-byte verification, bounded trace readback, original-operation recovery and drain/reopen using the actual native API/tools. A coherent native successor SHALL preserve the original baseline and sources. Reports SHALL distinguish automated checks, executed host runs, unresolved source/interpretation gaps and human acceptance; absent evidence SHALL NOT be labelled passed.

#### Scenario: Reopen after a later native edit
- **WHEN** a coherent definition/binding successor is published after the imported baseline and the backend reopens
- **THEN** current traces resolve coherently while the original baseline retains its exact definitions, provenance and known generation receipts

#### Scenario: Agent host has not been exercised
- **WHEN** a supported host's actual import/verify run or human interpretation review has not occurred
- **THEN** the evidence records that gate as unexecuted or pending and does not infer acceptance from fixture generation or static skill inspection
