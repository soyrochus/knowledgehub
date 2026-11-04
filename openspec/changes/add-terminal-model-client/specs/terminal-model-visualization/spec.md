# Terminal model visualization

## Purpose

Turn an explicitly selected native model scope into a faithful, editable and locally viewable diagram with inspectable provenance and coverage. Trace: KH-DR-01–06/12/17/19/21–22; KH-US-09/10/22/24; ontology §§3/5/8/12/17/21.7/21.9.

## ADDED Requirements

### Requirement: Explicit bounded visual selection
Diagram requests SHALL identify exact model selection, starting subjects, view kind, direction/relationship filters and finite work/output bounds. All data used by one export SHALL use that selection. Missing or ambiguous starting subjects SHALL require resolution rather than generate a guessed diagram.

#### Scenario: Head changes during export
- **WHEN** the model advances after diagram extraction starts
- **THEN** every exported subject, edge and source remains bound to the originally selected snapshot or baseline

#### Scenario: Export a definition reached through an original publication
- **WHEN** the reader explicitly exports an affected definition from A's publication after successor B exists
- **THEN** the export identifies A's original resulting snapshot and exact definition/source inputs, without silently combining them with B's content selection or treating a baseline identity as a publication anchor

### Requirement: Faithful graph identity and edges
Graph export SHALL preserve exact subject references, relationship directions, selector identity, assertion/property origins and parallel occurrences. Virtual groups and layout SHALL be distinguished from semantic facts. A diagram edge SHALL have an inspectable model origin or be explicitly marked as presentation-only.

#### Scenario: Two assertions share endpoints
- **WHEN** two retained assertions connect the same definitions
- **THEN** the graph keeps both origins even if the renderer groups their labels for readability

### Requirement: Meaningful diagram views
The client SHALL support a generic relationship view and journey/lifecycle views grounded in explicit step, state, transition and ordering definitions. It SHALL report missing structure or an unsupported view rather than infer process flow, state transitions or domain ownership from unrelated links or display names.

#### Scenario: Lifecycle structure is incomplete
- **WHEN** a model lacks an explicit transition required by the requested lifecycle view
- **THEN** the export identifies the missing structure and does not invent that transition

### Requirement: Editable and structured outputs
An export SHALL provide versioned graph JSON and editable Mermaid source with a manifest identifying exact selection, included references, source bindings, filters, bounds and coverage. Equivalent inputs SHALL produce stable graph/source ordering. These outputs SHALL NOT claim native import, backup or round-trip completeness.

#### Scenario: Workbench consumes an export
- **WHEN** an agent reads the graph and manifest to produce an explanatory artifact
- **THEN** it can trace every included definition/relationship to its exact native reference and identify the export's limits

### Requirement: Offline rendered preview
The client SHALL provide a self-contained HTML preview of the exported graph, renderable locally without a running service, network access or separately installed diagram renderer. It SHALL include readable labels, a legend, exact selection and an inspector linking visual elements to included native references and evidence.

#### Scenario: Instructor presents without connectivity
- **WHEN** the exported preview is opened in a browser on an offline machine
- **THEN** the diagram renders and its included reference/source metadata remain inspectable without fetching remote assets

### Requirement: Visible partial and unresolved results
Every output format SHALL distinguish exhausted complete extraction from reached limits, missing inputs, unavailable sources and unresolved external references. A partial graph SHALL carry its qualification visibly in the rendered view as well as structured metadata. An incomplete search SHALL NOT be described as no relationships.

#### Scenario: Two-hop extraction reaches an edge bound
- **WHEN** the requested graph exceeds the configured edge budget
- **THEN** JSON, Mermaid annotations and the HTML preview identify the reached bound and do not claim a complete model view

### Requirement: Definition status is preserved
Visuals SHALL preserve stored proposal, interpretation, review and gap qualifications where relevant. Executable-looking diagrams SHALL be labelled as model definitions or derived views. Rendering SHALL NOT establish business execution, implementation, verification success or human acceptance.

#### Scenario: Show the FruitTrucks expiry choice
- **WHEN** the offer lifecycle includes a proposed five-minute validity choice
- **THEN** the preview retains its proposed teaching status rather than presenting it as established client policy

### Requirement: Publication metadata remains provenance
Publication metadata included in an inspection artifact SHALL remain distinct from modelled business relationships, record predecessors, baseline parents and navigation history. Native sequence SHALL NOT supply an inferred timestamp, process transition, review decision, execution result or acceptance claim.

#### Scenario: Publication order differs from business order
- **WHEN** definitions were published in a sequence that is not a modelled journey or lifecycle order
- **THEN** the renderer retains publication provenance without adding business edges or ordering from that sequence

### Requirement: Safe derived artifact writing
Export SHALL write only explicitly selected output locations, refuse overwriting existing files without explicit replacement, and keep artifacts outside native stores. Text embedded in Mermaid/HTML SHALL be escaped and treated as data. Generation SHALL execute no stored model scripts or source application behaviour.

#### Scenario: Model label contains HTML or diagram control syntax
- **WHEN** that definition is exported
- **THEN** its label remains text in the view and cannot introduce executable content or unrelated diagram structure

#### Scenario: Output points inside the model store
- **WHEN** the user selects a native storage path as the export destination
- **THEN** the client refuses before writing and preserves the model
