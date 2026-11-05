# Optional Kitty terminal graph view

## Purpose

Let a permitted reader inspect exact model objects and relationships graphically inside a compatible terminal through an explicitly optional view. This consumes the delivered terminal exploration/visualization contracts and preserves their identity, authority and coverage semantics. Trace: KH-DR-01–07/12/17–19/21–22; KH-US-09/10/22/24/27; ontology §§3–5/8/12/17/21.7/21.10.

## ADDED Requirements

### Requirement: Separate optional graphics entry
The client SHALL expose a separately labelled View graph (Kitty) browser action and a dedicated interactive `kh view REF` command. Graphics SHALL start only after explicit invocation. Ordinary text browsing, non-interactive reads and diagram exports SHALL remain usable without graphics support or renderer initialization.

#### Scenario: Reader opens the existing browser
- **WHEN** a reader runs `kh browse` on a terminal without graphics support
- **THEN** ordinary browsing works without sending graphics queries or initializing the graph renderer

#### Scenario: Reader requests graphics for a selected definition
- **WHEN** the reader invokes View graph (Kitty) on an exact selected definition
- **THEN** the client opens the optional viewer for that definition and retains the browser's return state

### Requirement: Interactive invocation has explicit failure outcomes
The graphics command SHALL require terminal input and output and human output mode. Invalid arguments or JSON mode SHALL fail as usage errors before terminal probing. Unavailable graphics SHALL produce an explicit reason and text/HTML alternatives without automatically launching another application or writing a file.

#### Scenario: Agent requests JSON from the interactive command
- **WHEN** `kh view REF --json` is invoked
- **THEN** the command exits 2 with a usage result and emits no graphics or terminal-control sequence

#### Scenario: Standard output is redirected
- **WHEN** the reader invokes `kh view REF` with non-terminal input or output
- **THEN** it exits 2 without probing the terminal or writing graphics bytes into the redirected stream

### Requirement: Runtime graphics availability is negotiated
The viewer SHALL query actual Kitty graphics support with a finite deadline before sending model images. Terminal-name environment variables SHALL NOT establish support. Negative, missing, malformed or incompatible responses SHALL leave the viewer unavailable without damaging ordinary keyboard input or hanging the browser.

#### Scenario: Advertised terminal does not answer the graphics query
- **WHEN** environment variables suggest Kitty support but the query times out
- **THEN** the browser reports graphics unavailable, keeps its prior selection usable and creates no model image

#### Scenario: Late or fragmented query response arrives
- **WHEN** a response is fragmented or arrives after cancellation or timeout
- **THEN** recognized response bytes do not become browser commands or prompt text and do not reopen the viewer

### Requirement: Every displayed graph retains one exact selection
The viewer SHALL retain the selected model, exact root, snapshot or baseline and independent publication anchor throughout extraction, inspection and viewport changes. It SHALL use the shared graph extraction contract. Refresh or model/selection changes SHALL be explicit and SHALL invalidate pending old render results.

#### Scenario: Head advances while the reader pans
- **WHEN** the accepted head advances after the graph opens
- **THEN** panning, zooming and inspection continue using the original exact selection without substituting newer revisions

#### Scenario: Pending render finishes after leaving the viewer
- **WHEN** a previous rendering operation finishes after the reader returns to the browser or changes selection
- **THEN** its image and inspector content are discarded rather than drawn into the new context

### Requirement: Relationship meaning and identity survive rendering
Displayed nodes and edges SHALL retain exact native references, direction, selector identity and assertion/property origins. Parallel assertions and repeated occurrences SHALL remain independently inspectable. Layout and grouping SHALL be presentation metadata and SHALL NOT create ownership, containment or business relationships.

#### Scenario: Two assertions have the same endpoints
- **WHEN** the FruitTrucks satisfaction assertions share source and target
- **THEN** the reader can select both edge identities and inspect their separate origins even if the drawing groups their appearance

#### Scenario: Labels collide or a relationship forms a cycle
- **WHEN** distinct definitions share a label or recorded links contain a cycle
- **THEN** their exact identities remain distinct and the finite viewer displays the recorded structure without recursive duplication

### Requirement: Existing projections determine supported semantics
The viewer SHALL support relationships, journey and lifecycle projections from explicit recorded structure. Journey order SHALL follow the declared activity list; lifecycle arrows SHALL use declared transition endpoints. Missing order, transitions or ownership SHALL remain missing or unsupported rather than be inferred from names or publication order.

#### Scenario: Reader opens the route-change journey
- **WHEN** the journey projection is selected for FruitTrucks
- **THEN** the six declared activities appear in their recorded order with their exact references

#### Scenario: Reader opens the offer lifecycle
- **WHEN** the lifecycle projection is selected for the offer definition
- **THEN** its five states and four declared transitions are available for inspection, with the activation guard and proposed expiry qualification preserved

### Requirement: Narrative-only structure is not invented
The viewer SHALL show only recorded structural relationships. Narrative field names, source examples and permitted ontology endpoint types SHALL NOT become inferred entity fields, cardinalities, operational records or executable process flows. Unsupported detailed views SHALL explain the missing structure.

#### Scenario: Reader inspects the Route entity definition
- **WHEN** its structured field list is empty although its source describes `route_id` or stop ordering
- **THEN** the viewer retains the empty structure and source references without inventing entity edges or an operational route instance

### Requirement: Coverage and definition qualifications stay visible
The viewer SHALL visibly distinguish complete requested extraction, reached bounds, unresolved endpoints and unsupported projections. Stored proposal/interpretation qualifications and graph warnings SHALL remain accessible in text. Complete extraction SHALL NOT be described as model completeness, business execution or human acceptance.

#### Scenario: Vertex budget truncates the neighbourhood
- **WHEN** extraction reaches the selected vertex bound
- **THEN** the viewer labels the result partial, identifies the bound and retains the verified subset without presenting omitted links as absent

#### Scenario: Valid draft includes the expiry proposal
- **WHEN** the offer graph is shown from the current FruitTrucks draft
- **THEN** expiry remains proposed, draft limitations remain accessible and successful drawing does not mark the model accepted or complete

### Requirement: Nodes and edges have a text inspector
The viewer SHALL let the reader select nodes and edges by keyboard and inspect their exact identities, type or relationship meaning, native properties, sources and origins. Original source excerpts SHALL be read on explicit request at the retained selection. Missing evidence or status SHALL remain distinguishable from empty or accepted evidence.

#### Scenario: Reader inspects a satisfaction edge
- **WHEN** that edge is selected
- **THEN** the inspector identifies its direction, exact endpoints, relationship selector, assertion origin and source bindings

#### Scenario: Reader inspects a source
- **WHEN** the reader requests an excerpt from a selected object's source binding
- **THEN** the exact permitted source is read through the existing source workflow and its revision/anchor remain visible

### Requirement: Keyboard navigation and return state are preserved
The viewer SHALL provide keyboard node/edge selection, inspector scrolling, pan, zoom, fit/reset, help and close. Focus, selection, controls and qualifications SHALL use text as well as colour. Closing the browser-launched viewer SHALL restore the prior browser selection, scope, navigation and scroll state without implicit refresh.

#### Scenario: Reader adjusts the viewport and returns
- **WHEN** the reader pans, zooms, inspects an edge and closes the viewer
- **THEN** the browser resumes the same exact definition, page, filter and position without changing its navigation history

#### Scenario: Terminal becomes too small
- **WHEN** resizing leaves insufficient drawing or inspector space
- **THEN** the viewer clears its image, shows a usable size notice and close controls, and retains exact selection for a later resize

### Requirement: Graphics obey current disclosure authority
Every graph reveal, inspection and redraw SHALL recheck current scoped authority through the existing host policy. Pending checks SHALL hide previous model images and details. Observed denial, disconnection or unavailable policy SHALL clear cached images/details and prohibit redraw until explicit permitted recovery. Ordinary graph reads SHALL NOT require ReadHistory.

#### Scenario: Read is revoked before a zoom or help action
- **WHEN** the reader requests another viewer interaction after Read is revoked
- **THEN** existing images and details are hidden, denial is shown and neither the interaction nor resize can redisplay cached model content

#### Scenario: Reader has ordinary access without publication history
- **WHEN** the workspace grants Open/Read but not ReadHistory
- **THEN** relationship, journey, lifecycle and source inspection remain usable without disclosing original publication metadata

### Requirement: Rendering and terminal resources remain bounded
The viewer SHALL enforce finite graph, label, raster, encoded-image and pending-work limits before expensive allocation or transmission. It SHALL handle cycles and parallel edges within those bounds, serialize terminal writes with backpressure and allow cancellation. Reached graph/render limits SHALL be explicit rather than silently omitting content.

#### Scenario: Graph or raster exceeds the viewer budget
- **WHEN** requested work would exceed a declared rendering bound
- **THEN** the viewer reports the reached limit without allocating an oversized raster or claiming a complete displayed view

#### Scenario: Resize events arrive while rendering
- **WHEN** several resizes occur during a pending frame
- **THEN** obsolete work is discarded or coalesced within the bounded queue and only a currently authorized frame can be displayed

### Requirement: Viewer lifecycle restores the terminal
On close, cancellation, denial, handled signal, render failure or transport failure, the viewer SHALL remove its own image placements/data, restore input/cursor state and release its listeners and tasks. It SHALL NOT clear unrelated images or claim cleanup succeeded after output failure. Browser launch SHALL recover to a usable text state when possible.

#### Scenario: Image transmission fails after partial initialization
- **WHEN** output or image acknowledgement fails after viewer resources were allocated
- **THEN** cleanup is attempted for those owned resources, terminal state is restored where possible and an operational failure is reported

#### Scenario: Reader cancels or terminates the viewer
- **WHEN** Escape, close, Ctrl-C or a handled termination signal ends the viewer
- **THEN** owned graphics and input handlers are released and no delayed render can recreate them

### Requirement: Model content cannot control rendering execution
Model labels, narratives and sources SHALL remain inert text in both raster and inspector. The viewer SHALL execute no stored script and resolve no model-supplied filesystem or network resources. Presentation sanitization SHALL preserve access to exact source bytes through the existing explicit export workflow.

#### Scenario: Label contains terminal or SVG control syntax
- **WHEN** an object label includes escape sequences or markup
- **THEN** it appears as safe text and cannot issue terminal commands, add drawing elements or fetch resources

### Requirement: Graphics assets are optional self-contained package inputs
The supported package SHALL include pinned renderer dependencies, fonts/runtime assets, hashes and notices. Explicit graphics viewing SHALL need no development tools, external renderer process or downloads. Missing or incompatible graphics assets SHALL report an operational failure confined to the optional viewer while text browsing and existing exports remain available.

#### Scenario: Package is relocated and run offline
- **WHEN** a matching macOS arm64 package is launched from another directory with spaces and no development tools on PATH
- **THEN** the three prepared graph views render using packaged assets without checkout paths, system fonts or network access

### Requirement: Verification separates protocol and native terminal evidence
Delivery evidence SHALL identify build inputs, fixture selections, target and terminal versions, synthetic checks and actual graphics observations separately. Both existing fixtures SHALL demonstrate generic rendering. Support claims for Kitty and Ghostty SHALL require actual terminal evidence; human meaning/usability acceptance SHALL remain a separate gate.

#### Scenario: Protocol simulation passes without a terminal run
- **WHEN** encoder/parser and PTY simulations pass but actual terminal rendering has not been observed
- **THEN** the report retains the native graphics gate as open and does not claim that terminal verified

#### Scenario: Existing fixtures are used for verification
- **WHEN** the three FruitTrucks projections and Equipment relationships are exercised
- **THEN** client behaviour uses the same generic path, fixture definitions remain unchanged and the report preserves their draft/source limitations
