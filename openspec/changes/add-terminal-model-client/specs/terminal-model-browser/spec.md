# Terminal model browser

## Purpose

Give a reader an interactive terminal route from hierarchical discovery to exact definitions, relationships and evidence, while preserving the graph's meaning. Trace: KH-DR-01–07/12/15/17–22; KH-US-09/10/22/24/27; ontology §§3/5/7–8/12/17/21.3/21.7.

## ADDED Requirements

### Requirement: Guided opening and visible orientation
The browser SHALL open a configured model and display scope, exact selected snapshot or baseline, connection state and read-only mode. It SHALL provide model selection and help without requiring raw protocol requests or Rust knowledge.

#### Scenario: Instructor opens the prepared example
- **WHEN** the instructor starts browse mode and selects FruitTrucks
- **THEN** the interface shows the identified model, pinned selection and available navigation actions

### Requirement: Alternative hierarchical views
The browser SHALL provide by-type, by-context, by-journey and by-source views from actual registered records and references. Virtual groups SHALL be labelled separately from modelled containment. A record SHALL retain one exact identity when it appears in several views; missing membership SHALL appear unassigned rather than be inferred.

#### Scenario: One definition appears in two views
- **WHEN** a reader opens the same use case through its type group and context view
- **THEN** both routes select the same exact record and neither grouping creates a model relationship

#### Scenario: Context membership is absent
- **WHEN** the model does not declare the expected context reference
- **THEN** the record remains discoverable by type or unassigned grouping without guessing membership from its name

### Requirement: Bounded expandable graph navigation
Expanding a group or relationship SHALL load finite results with explicit more/limit controls. Cycles and shared targets SHALL use identifiable references rather than recursive duplication. Actual containment, ordered progression and general association SHALL have distinguishable presentation.

#### Scenario: A journey contains a legitimate loop
- **WHEN** the reader expands its recorded references repeatedly
- **THEN** navigation remains finite, identifies a revisit and does not present alphabetical ordering as the journey's sequence

### Requirement: Definition and relationship inspection
The browser SHALL expose the selected record's native properties, type, exact revision, predecessor, selection status and evidence. A relationships pane SHALL show incoming/outgoing associations and property references with their origins. Readers SHALL be able to inspect assertions and follow exact targets.

#### Scenario: Follow an incoming satisfaction link
- **WHEN** the reader opens the acknowledgement requirement and activates its incoming satisfies relationship
- **THEN** the browser opens the exact linked use case and retains the assertion and return path

### Requirement: Keyboard-complete interaction
All opening, selection, expansion, paging, pane focus, search, filtering, relationship following, source inspection, history, export and exit actions SHALL be usable by keyboard. Help SHALL show bindings; focus and statuses SHALL be expressed in text as well as colour. Resize SHALL retain selection and usable controls.

#### Scenario: Browse on a smaller terminal
- **WHEN** the user resizes while inspecting a source and switches panes using the keyboard
- **THEN** the layout adapts or displays a usable size notice while retaining selection and navigation state

### Requirement: Search and disambiguation within the browser
Search SHALL show matching exact candidates, field/source context and coverage, and require selection when several candidates match. Search/filter changes SHALL be explicit and reversible. Cancelling search SHALL return to the previous selection without altering the model.

#### Scenario: Two definitions share a search phrase
- **WHEN** both match the literal query
- **THEN** the reader can inspect and choose either result with exact identity and context visible

### Requirement: Exact navigation history and refresh
Back/forward history SHALL retain model, selection, exact subject and view position. Snapshot/baseline switches and refresh-to-head SHALL be explicit. A changed head SHALL NOT silently retarget an open definition, source, diagram or history entry.

#### Scenario: Refresh then return to an earlier inspection
- **WHEN** the user refreshes from A to B and returns to an A history entry
- **THEN** it opens A's exact subject and identifies that historical selection rather than substituting B

### Requirement: Evidence and uncertainty remain visible
The browser SHALL show exact source excerpts and stored interpretation/review states separately from implementation or acceptance claims. It SHALL make recorded gaps accessible and mark absent status as unrecorded. Unsupported media SHALL remain inspectable as metadata with explicit original export.

#### Scenario: A valid draft has open questions
- **WHEN** the instructor opens its gaps and a proposed design definition
- **THEN** the interface retains those qualifications and does not label the design accepted because it can be read

### Requirement: Recoverable interface states
Loading, empty, denied, unavailable, partial, incompatible, disconnected and failed reads SHALL have distinct text and available next actions. Cancellation SHALL stop waiting or loading without implying a changed model. Host restart SHALL preserve exact subject intent and replace invalid cursors with fresh bounded queries.

#### Scenario: A lookup fails while another pane is open
- **WHEN** a relationship request reaches capacity
- **THEN** the inspected definition remains available, the failed pane explains the limit and the user can narrow or retry the pinned request

### Requirement: Model text is displayed as data
Names, narratives and sources SHALL be displayed without executing scripts, terminal control sequences or embedded instructions. Text sanitisation SHALL preserve access to exact original bytes through explicit export and SHALL NOT alter stored definitions.

#### Scenario: Source text includes terminal escape sequences
- **WHEN** a reader previews that source
- **THEN** the terminal displays a safe textual representation while original-content export retains the exact bytes

### Requirement: Visual export from current exploration
The browser SHALL let the reader select a subject/view and finite diagram scope, preview its exact inputs and coverage, and export through the shared visualization capability. Exporting SHALL NOT create or modify native model records or stored representation bindings.

#### Scenario: Instructor exports the current neighbourhood
- **WHEN** the reader requests a two-hop diagram from the requirement
- **THEN** the browser exports the pinned native relationships with source references and any visible coverage limitation
