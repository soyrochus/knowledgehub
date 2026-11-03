# Open source board libraries for Memento

Research date: 2026-10-15. Audience: front-end engineering and product review. Status: investigation complete; candidate spikes and technology decision pending.

## Recommendation

**Reopen the board choice and test Plait, AntV X6, and Foblex Flow against the same Memento board, with React Flow as the comparison baseline.** Plait is the first candidate for a broad whiteboard; X6 is the first alternative for an application built around rich cards and relationships; Foblex supplies an Angular-native comparison. Give Excalidraw a short feasibility test for interactive cards. Keep BlockSuite conditional on proving compatibility with Memento's collaboration contract. Include a direct HTML/SVG implementation as a credible fallback with an explicit engineering cost.

This is an engineering assessment from upstream documentation and source licences. It does not establish a winner through measured prototypes. None of the reviewed options establishes all of Memento's present and future needs out of the box. Drawing tools, interactive application cards, multiplayer delivery, and domain history are different capabilities; a candidate must demonstrate how they work together.

The [Platform Specification](../memento-platform-spec.md) currently selects React and React Flow. This investigation broadens the comparison to Angular, Svelte, Web Components, framework-independent JavaScript, and native browser rendering. It leaves the selected baseline in place until the spike review. Choosing Angular or another application framework would require an explicit platform amendment after that review. The [spike briefs](../../delivery/releases/v1.0/board-library-spikes.md) are proposed assignments, not implementation authorization.

## Board elements and interactions to assess

The mandatory product baseline comes from [functional sections 1 and 3](../memento-spec.md), [technical sections 2 and 3](../memento-technical-spec.md), and [platform sections 5 and 6](../memento-platform-spec.md). The wider inventory below interprets the requested Miro/Mural-like direction as an extensibility test. It does not promote future elements into v1.0 scope.

| ID | Element or interaction | Required demonstration | Scope |
| --- | --- | --- | --- |
| R01 | Sticky Notes | Edit text, resize its presentation, retain author and content revision independently of position. | MVP; DR-02/03/12. |
| R02 | Email and task Source Cards | Structured fields, source links, observed status, freshness, annotation, expandable detail. Source content is read-only. | MVP; DR-04/05. |
| R03 | Proposal, Decision, and Action cards | Different visual states, evidence links, editable proposal, preserved Decision meaning, exact Action review controls. | MVP; DR-05–10. |
| R04 | Frames and grouping | Named bounds; move, group, ungroup, resize, and remove Placements while retaining represented records. | MVP; DR-03/11. |
| R05 | Typed Relationships | Directed, labelled connections, endpoint validation, inspectable meaning, reconnect through an authorized command. Decorative arrows remain distinct. | MVP; technical 2.1. |
| R06 | Memory Chart | Navigate the same records and evidence with view-local graph/tree layout; unplaced records remain discoverable. | MVP; AC-20. |
| R07 | Working surface | Pan/zoom, selection, inspector, moving cards, presence, keyboard and pointer interaction. | MVP; FA-01/09. |
| R08 | Text and general shapes | Headings, formatted text, rectangles, ellipses, diamonds, lines, arrows, colours and borders. | Broader whiteboard assessment. |
| R09 | Freehand and annotation | Pen strokes, erase, select, move and scale a stroke beside cards; optional pressure support. | Future extension. |
| R10 | Images and documents | Images with resize/crop; an attachment/PDF preview card with original reference. File storage and preview are separate work. | Future extension. |
| R11 | Links and media | Link previews, permitted embedded pages/video, and useful fallback when embedding is unavailable. | Future extension. |
| R12 | Structured widgets | Table, checklist, Kanban area, chart, and a mind map. Test real editable content, rather than a screenshot. | Future assessment; task cards already in MVP. |
| R13 | Composition tools | Multi-select, alignment, snapping, ordering, duplication, clipboard, and reusable templates. | Interaction quality / future scope as applicable. |
| R14 | Board output | Export/reload a portable board representation; assess PNG/SVG/PDF export including custom cards and assets. | Recovery needed for MVP; participant exports are future scope. |
| R15 | Facilitation | Comments, reactions/votes, timer, presentation/follow mode. | Future assessment; presence already in MVP. |
| R16 | Collaboration and domain control | Expected revisions, explicit conflicts, ordered accepted updates, reconnect, attribution, and revision-aware undo. | Mandatory architecture; AC-02–04/13–16/19–20. |

For R02/R03/R12, the decisive question is whether a card can contain usable forms, buttons, text selection, menus, scrollable content, and accessible focus while it participates in board gestures. Being able to draw a box with text does not answer that question.

## Open source eligibility

All required shipped functionality must be buildable and self-hostable from open-source code. Source visibility, a free tier, an educational licence, or a production licence key does not satisfy the requirement. Use the [Open Source Definition](https://opensource.org/osd) as the eligibility reference. Copyleft licences remain eligible; record their actual distribution obligations with the selected packages.

Each spike must inventory the exact package versions, renderer/adapters, plugins, example code, assets, fonts, and any collaboration service it uses. A core repository's licence does not establish the licence of everything in a demo. Keep required capabilities independent of proprietary services. The licence links below describe the inspected upstream trees on the research date; the pinned spike packages still require their own inventory.

Two boundaries are particularly relevant:

- React Flow and Svelte Flow have an [MIT core](https://github.com/xyflow/xyflow). React Flow's [freehand example](https://reactflow.dev/examples/whiteboard/freehand-draw) explicitly uses the xyflow Pro licence. Count freehand as work to implement independently with eligible dependencies; do not count that example as reusable MIT functionality. Other examples must be checked individually.
- The current [tldraw SDK licence](https://tldraw.dev/community/license) requires a production key and explicitly distinguishes the SDK from open-source software. Exclude it from the shortlist. MIT examples do not change the SDK's status. Historical versions are not a substitute for evaluating a maintained dependency.

## Candidate comparison

The priority and fit columns are engineering judgments. The linked upstream references support the licence and capability descriptions. No relative performance, development-speed, or maintenance-quality ranking has been measured.

| Candidate | Integration and licence | Useful starting capabilities | Main gap for Memento | Priority |
| --- | --- | --- | --- | --- |
| **Plait with selected Drawnix components** | Framework-independent core; Angular and React view layers. [Plait MIT licence](https://raw.githubusercontent.com/worktile/plait/develop/LICENSE); [Drawnix MIT project](https://github.com/plait-board/drawnix). | [Plait](https://github.com/worktile/plait) provides board primitives and plugins for drawing, rich text, images, mind maps and custom flow nodes. [Drawnix](https://github.com/plait-board/drawnix/blob/develop/README_en.md) adds an implemented whiteboard with freehand and export. | Prove arbitrary interactive cards, Frame behaviour, and accepted-command interception. Identify which capabilities are reusable packages versus Drawnix application code. | First broad-whiteboard spike. |
| **AntV X6** | JavaScript/TypeScript, SVG plus HTML; framework-independent host with framework renderers. [MIT licence](https://raw.githubusercontent.com/antvis/X6/master/LICENSE). | [Custom SVG/HTML nodes](https://x6.antv.antgroup.com/en/tutorial/basic/node), ports, edges, [embedding/grouping](https://x6.antv.antgroup.com/en/tutorial/basic/interacting), and editing plugins. | Build the sticky-note/rich-text experience, freehand and facilitation. Test HTML rendering and export carefully; upstream documents browser rendering limitations. | First rich-card and vanilla-JavaScript alternative. |
| **Foblex Flow** | Angular-native. [MIT licence](https://raw.githubusercontent.com/Foblex/f-flow/main/LICENSE). | [Angular UI in nodes](https://flow.foblex.com/examples/custom-nodes), [nested groups](https://flow.foblex.com/examples/grouping), connections and [selection/layout helpers](https://flow.foblex.com/examples/overview). | A node editor foundation: drawing tools and broader whiteboard widgets require application work. An Angular choice also changes the selected application baseline. | Principal Angular spike. |
| **React Flow** | React DOM nodes with SVG edges. [MIT core](https://github.com/xyflow/xyflow). | [Custom components](https://reactflow.dev/learn/customization/custom-nodes), ports, subflows, viewport, selection and resizers. | Broad whiteboard tools remain application work. Separate open-source core capability from separately licensed examples. | Equal-fixture baseline; current choice gets no automatic preference. |
| **Excalidraw** | Embeddable React editor. [MIT licence](https://raw.githubusercontent.com/excalidraw/excalidraw/master/LICENSE). Angular would host a React boundary. | [Drawing, images, arrows, selection, undo and export](https://github.com/excalidraw/excalidraw). [Custom data and scene callbacks](https://docs.excalidraw.com/docs/@excalidraw/excalidraw/api/props/) can associate domain IDs. | [Custom embeddable rendering](https://docs.excalidraw.com/docs/@excalidraw/excalidraw/api/props/render-props) offers a card experiment; it does not prove a general custom-element system or command interception. Hosted-app collaboration is distinct from the embedded package. | Short card/command feasibility gate. |
| **BlockSuite Edgeless** | Native Web Components; framework-independent host. [MPL-2.0 licence](https://raw.githubusercontent.com/toeverything/blocksuite/main/LICENSE). | [Rich text, shapes, brush, connectors, frames, groups and link cards](https://blocksuite.io/components/editors/edgeless-editor); [custom block specs](https://blocksuite.io/guide/block-spec). | The [store is natively Yjs-based](https://blocksuite.io/guide/store). Prove accepted commands and explicit conflicts without silent merging of authoritative fields. Broad editor adoption carries a larger model integration cost. | Conditional architecture gate. |
| **ngx-vflow** | Angular components and Signals. [MIT project](https://github.com/artem-mangilev/ngx-vflow). | Custom nodes, edges and handles, nested flows, navigation and interaction helpers. | Similar category to Foblex; broad drawing tools remain custom. Check Angular compatibility against the chosen release. | Angular reserve; short comparison if Foblex fails or has an expensive gap. |
| **maxGraph** | Framework-independent TypeScript diagram engine. [Apache-2.0 project](https://github.com/maxGraph/maxGraph). | Shapes, edges, transformations, graph operations and automatic layouts; maintained successor to mxGraph. | Prove native interactive card integration, then build richer editing and whiteboard tools. Its API/model is another adapter to maintain. | Diagramming reserve. |
| **JointJS core** | Framework-independent SVG engine with HTML integration. [MPL-2.0 project](https://github.com/clientIO/joint); [HTML shapes](https://docs.jointjs.com/learn/features/customizing-shapes/html-inside-shapes/). | Custom elements, ports, links and graph composition. | Common polished editor controls can come from JointJS+. Its [Toolbar](https://docs.jointjs.com/learn/features/toolbar/) and [selection examples](https://docs.jointjs.com/react/getting-started/selecting-elements/) show that distinction. Budget replacements using open-source code. | Reserve; compare core only. |
| **Konva** | Canvas scene graph, usable from JavaScript and several framework adapters. [MIT project](https://github.com/konvajs/konva). | Interactive shapes, events, transforms, images, strokes and export building blocks. | Build editor tools, connection routing and text integration. [DOM portals](https://konvajs.org/docs/react/DOM_Portal.html) are overlays and are absent from the canvas image export. | Custom rendering fallback. |
| **Fabric.js** | Framework-independent Canvas object model. [MIT project](https://github.com/fabricjs/fabric.js). | Object manipulation, groups, shapes, brushes, controls and SVG/JSON/image I/O. | Build semantic connectors, rich DOM card overlays, collaboration and editor shell. Text objects do not establish arbitrary HTML controls. | Custom rendering fallback; alternative to Konva. |
| **Direct HTML/CSS and SVG** | Native browser surface; optional [Moveable](https://github.com/daybrush/moveable) and [Selecto](https://github.com/daybrush/selecto), both MIT. | Ordinary HTML cards, SVG edges/shapes, application-owned state and interaction boundary. Helpers provide transforms and region selection. | Own viewport coordinates, edge routing, hit testing, group transforms, clipboard, history, drawing, export, accessibility and performance. | Credible fallback spike if higher-level options fail. |
| **Svelte Flow** | Svelte-specific, [MIT in xyflow](https://github.com/xyflow/xyflow). | Custom node/edge application pattern comparable in purpose to React Flow. | A framework change alone does not provide a broader whiteboard toolset. | Reserve if Svelte becomes an application preference. |
| **Rete.js** | Framework-independent core with React/Angular/Vue/Svelte integrations; [MIT core](https://github.com/retejs/rete). | Custom visual interfaces, workflow and graph-processing infrastructure. | Its visual-programming emphasis adds little to the whiteboard requirement; assess every extra plugin separately. | Longlist only. |
| **ProjectStorm React Diagrams** | React/TypeScript; [MIT project](https://github.com/projectstorm/react-diagrams). | Extensible diagram models and HTML nodes. | Process-editor emphasis; broader whiteboard tools and collaborative semantics remain application work. | Longlist only. |
| **diagram-js** | Framework-independent JavaScript toolbox; [MIT project](https://github.com/bpmn-io/diagram-js). | Diagram display/editing foundation, extensible into specialized editors. | Requires substantial board/card product composition; less direct than the first candidates. | Longlist only. |

Drawnix is an application built on Plait, rather than a second interchangeable SDK. Likewise, adopting BlockSuite components is a different scope from adopting AFFiNE's entire product. The spike should use the smallest reusable component boundary that delivers the required surface.

## Element coverage

These tables assess implementation paths from the documented primitives above. **N** = supplied primitive/tool; **E** = application component or supported extension must be composed; **B** = substantial custom subsystem; **?** = decisive feasibility remains unresolved. N does not mean a complete Memento workflow. All candidates require Memento-specific data, permissions, statuses, and persistence.

### Principal editor foundations

| Capability | Plait / Drawnix parts | X6 | Foblex Flow | React Flow |
| --- | --- | --- | --- | --- |
| Sticky Note with editable content | E | E | E | E |
| Interactive Source/Proposal/Decision/Action card | ? | E | E | E |
| Named Frames and grouping semantics | E | E | E | E |
| Directed/labelled connector primitive | N | N | N | N |
| Text and general shapes | N | N | E | E |
| Freehand tool | E | B | B | B |
| Images | N | N | E | E |
| PDF/document preview and media card | E | E | E | E |
| Table, checklist, Kanban or chart widget | ? | E | E | E |
| Mind map and general graph layout | N / E | E | E | E |
| Selection and viewport primitives | N | N | N | N |
| Portable output including custom DOM content | ? | ? | B | B |

Plait's freehand path includes selected MIT Drawnix code; do not assume the base package includes the complete application tool. Native mind-map layout is distinct from applying a revision-checked layout to a shared board. For Foblex/React Flow, an image is ordinary component content; that does not supply crop tooling. A preview card does not supply document conversion or blob storage.

### Broad editors and custom surfaces

| Capability | Excalidraw | BlockSuite | Konva | Fabric.js | Direct HTML/SVG |
| --- | --- | --- | --- | --- | --- |
| Sticky Note with editable content | E | E | E | E | E |
| Interactive Memento card | ? | E | B | B | E |
| Named Frames and grouping semantics | E | E | B | B | B |
| Bound drawing/diagram connector | N | N | B | B | B |
| Text and general shapes | N | N | N | N | E |
| Freehand tool | N | N | E | N | B |
| Images | N | E | N | N | E |
| PDF/document preview and media card | E / ? | E | B | B | E |
| Structured interactive widgets | ? | E | B | B | E |
| Memory Chart projection and layout | B | E | B | B | B |
| Selection and viewport | N | N | B | E / B | B |
| Export custom DOM cards with drawings | ? | ? | B | B | B |

For **every** option, Memento-specific typed Relationship rules, comments/voting/timers, meaningful templates, authorization, durable attribution, explicit conflicts, reconnect recovery, and domain-safe undo need application integration. Existing collaboration or history machinery can reduce UI work only if it preserves those contracts. No generic editor feature records a valid Decision or grants an Action Approval.

## Integration requirements that can eliminate a candidate

1. **Keep semantic records outside the editor document.** Map visual IDs to Placement IDs and separate record references. Content and arrangement revisions remain independent; removing a shape must not delete evidence. Rebuild the projection from authoritative records.
2. **Control acceptance.** A gesture produces a local preview and a proposed command. The server accepts or rejects it using expected revisions and operation identity. Rejecting a command must restore the accepted projection without corrupting the editor's local history. An `onChange` notification alone does not prove this is possible.
3. **Separate drawing from meaning.** Decorative arrows and frame membership must not invent semantic Relationships. A generic edge editor cannot freely rewrite Decision supersession. Imported/generated diagrams require explicit mapping and validation.
4. **Preserve rich-card interaction.** Test text selection, editing, scroll, menus, buttons, focus and IME input at multiple zoom levels, with card resize and parent movement. Canvas overlays and SVG `foreignObject` renderers need browser-specific evidence.
5. **Reconcile remote changes.** Preserve an active draft while applying an independent committed move. Show a same-content conflict explicitly. Restore authoritative state after missed updates. BlockSuite's Yjs foundation is an additional boundary to prove, rather than an automatic disqualification for using a CRDT internally.
6. **Own undo and portability.** A committed undo is a fresh revision-aware command. It cannot erase a recorded Decision or reverse external execution. Export must account for fonts, images, DOM overlays, evidence references, and missing assets. Library JSON is an adapter format, not the domain's only durable store.
7. **Measure representative complexity.** Use realistic email text, task metadata, evidence links, forms and connectors. Assess accessibility and pointer/touch behaviour as well as frame rate. An upstream large-node demo with simple boxes cannot establish the supported Memento board size.

## Proposed decision sequence

Use the [common fixture, individual briefs and review gates](../../delivery/releases/v1.0/board-library-spikes.md) to keep the comparison fair.

1. Run the Plait and X6 spikes first. They test the broad-whiteboard and rich-card approaches directly.
2. Run Foblex in Angular and React Flow with the same fixture. If Angular is attractive, briefly compare ngx-vflow before selecting its library.
3. Run short Excalidraw and BlockSuite gates only on their decisive risks. Extend a passing candidate to the common fixture; stop a failing one with a small reproducer.
4. Use the native HTML/SVG fallback if the first candidates cannot reconcile card interactivity, whiteboard breadth, and command control. Consider Konva/Fabric when drawing needs justify a Canvas layer; avoid adding two complete editor engines before measuring the integration cost.
5. Take the best two eligible candidates through the same collaboration, accessibility, portability, and performance exercise. Select by observed coverage and remaining engineering effort. Record the final platform decision and synchronize affected specifications/PDFs at that point.

A credible choice must cover all MVP elements, demonstrate a viable path for every wider element, and state the cost of each gap. If no candidate does this within an acceptable effort, narrow the intended whiteboard scope explicitly or fund a custom surface. A percentage of feature names checked off is insufficient when a missing capability is the interactive card or authoritative-command boundary.

## Evidence and review status

Primary upstream repositories, licence files and official documentation linked beside the claims were inspected on 2026-10-15. Several documentation pages expose newer APIs than older examples; spike owners must pair documentation with their pinned package release and record a working build. Release compatibility, issue resolution, contributor continuity and dependency security belong in the finalist review, rather than being inferred from GitHub stars.

The investigation itself introduced no runtime dependency, application code, feature-scope change, or AC verification. The separately authorized [native UI baseline](../../spikes/01-basic-html/README.md) now implements [UI-01–UI-08](../spec-ui-spike.md) with zero runtime libraries and [local browser evidence](../../spikes/01-basic-html/verification.json), dated 2026-10-20. This is partial evidence for SP-07; full comparative spike execution, measured candidate coverage, final technology selection, and human acceptance remain pending. The release plan records the investigation under [E-002](../../delivery/releases/v1.0/plan.md#32-e-002--reassess-open-source-board-libraries).
