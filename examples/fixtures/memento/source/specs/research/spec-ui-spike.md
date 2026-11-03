# Memento — UI interaction spike specification

Version 1.0 · 2026-10-20 · Scope: local UI experiment; human acceptance pending.

## Purpose and authority

Demonstrate a small spatial board where a participant can inspect the evidence behind a process, rearrange its elements, navigate the canvas and extend the graph. This specification defines the observable interaction baseline for UI spikes. The [shared scene](../delivery/releases/v1.0/board-scene-proposal.md) supplies the Atlas fixture; the [spike briefs](../delivery/releases/v1.0/board-library-spikes.md) retain the broader comparative experiments.

[`01-basic-html`](../spikes/01-basic-html/README.md) is the bottom-end, native-browser baseline. **It must use no runtime libraries**, frameworks, packages, CDN resources, remote fonts, vendor APIs or hosted services. Implement its UI using repository-owned HTML, CSS, JavaScript and SVG plus built-in browser APIs. It opens directly from `index.html` without installation, compilation, a server or network access. Bun may serve an optional loopback preview; Playwright may verify it as development tooling. Neither is part of the browser runtime. Other candidate spikes must identify their own runtime dependencies.

The [Functional and Domain Specification](memento-spec.md), [Technical Specification](memento-technical-spec.md) and [Platform Specification](memento-platform-spec.md) continue to own product behaviour, collaboration contracts and application technology. This experiment changes none of those contracts or the selected application platform.

## Required behaviour

| ID | Requirement and observable result |
| --- | --- |
| UI-01 Fixture and evidence | Retain nine process shapes, ten process arrows, four Yes/No labels, two post-its, the initial highlighted constraint-review step, its two-comment thread, and two linked evidence conclusion cards. Selecting a step highlights its relevant evidence. Inspect E1's three illustrative messages, E2's three versioned document excerpts, and the two comments. Source content and conclusions remain distinguishable. |
| UI-02 Move elements | Independently move every process shape, post-it and evidence card using a labelled, focusable move handle. Preview movement continuously; retain the dropped position for the page session. Pointer capture completes a drag outside the viewport. Evidence buttons, text selection and typing do not start dragging or panning. Keyboard arrows on a move handle move by 10 board units; Shift+arrow moves by 1. Geometry changes do not change content or evidence associations. |
| UI-03 Navigate | Pan by dragging empty canvas, or with the Hand tool across non-control surfaces. Provide zoom in/out, a 100% control and Fit board, with a visible zoom percentage. Zoom ranges from 20% to 200%; wheel/trackpad zoom is anchored at the pointer. Toolbar zoom is anchored at the viewport centre. Fit computes bounds from all movable elements, including additions and negative positions, down to the 20% zoom limit; larger arrangements remain accessible by panning. Viewport resizing preserves board coordinates. On narrow screens keep the spatial board, tools and inspector usable without page horizontal overflow; zoom/pan supplies access to the whole scene. |
| UI-04 Create and edit | A small Box palette button can be dragged onto the viewport to create a rectangular process box centred at the drop position, accounting for pan and zoom. Clicking or keyboard-activating it creates a box at the viewport centre. Dropping outside creates nothing. Each new box has a stable local ID, can be selected and moved, and has a plain-text editable label in the inspector. Post-it text is also editable there. Saving changes only the selected object's text; blank labels are rejected visibly. Labels accept up to 240 characters and wrap within the fixed card width; content height may grow without changing board position. User text is rendered as text, never HTML. |
| UI-05 Connections | Existing solid process arrows and dashed evidence connections keep their original endpoint IDs and labels while either endpoint moves and while navigating. Provide a Connect tool: choose a process shape as the source and another as the target to add one solid directed arrow. Reject self-links and duplicate directed process links with an explanation. Evidence cards and notes are not valid process endpoints. New arrows are visual fixture connections, not published semantic Relationships. Connection creation exits after success; Escape or selecting another tool cancels it. |
| UI-06 Controls and cancellation | Clearly indicate Select/Hand/Connect mode and the selected object. Escape cancels an active drag, palette drag or connection and restores the pre-gesture state; pointer cancellation does likewise. Escape also closes source/comment details and returns focus to their opener. Source/comment details and the label editor remain outside the transformed world so their controls stay readable at any zoom. Provide Reset board to restore the original elements, text, connections, selection, inspector and fitted viewport. Reload has the same initial-state result. No storage is used. |
| UI-07 Readability and input | Preserve distinct shape families, arrows, evidence line styles, light/dark appearance, visible keyboard focus, meaningful control names and status feedback. Support mouse and single-pointer touch/pen movement, palette creation and panning through Pointer Events. Native buttons supply keyboard activation. At 50%, 100% and 200% zoom, evidence inspection and editing must work without unintended object movement or viewport changes. At 320, 390, 736 and 1440 pixel widths, controls remain reachable; source details wrap/scroll without page overflow. |
| UI-08 Reproducibility | Keep an executable browser check, a dated result with actual tool/browser versions and zero runtime dependencies, and current desktop/mobile/dark/inspection/interaction screenshots. Verify direct file opening with HTTP(S) requests blocked, all UI-01–UI-07 workflows, coordinate conversion at multiple zooms, cancellation, reset and optional loopback HTTP serving. Record unverified environments separately. |

## Implementation boundary

Use one board coordinate system for all movable element geometry and SVG connectors. Separate local object identity/content, placement geometry, visual connections and viewport/selection/tool state. Convert pointer coordinates through the viewport's inverse translation and scale for movement and creation. Derive connector endpoints from current board geometry; avoid grid-specific routing that detaches when elements move. The fixture's frame is a visual guide and does not constrain movement or establish semantic membership.

The local state is experimental, not a substitute for server-accepted commands. Moving a Source Card preserves its illustrative observation and inspection. Spatial proximity, arrows and frame bounds do not record a Decision, establish evidence or perform an external action (DR-03/04). See functional sections 4.7–4.10 and technical sections 2–3 for product integration requirements.

Resizing, grouping, multi-selection, freehand, automatic layout, undo history, deletion tools, persistence, authentication, multiplayer, conflict/reconnect handling, real ingestion, model inference and external writes are outside this increment. This specification covers part of the original V05 manipulation check; its resizing requirement and the full SP-07/C01–C07 experiments remain separate work. No product release AC is completed by this spike.

## Review demonstration

1. Open `index.html` offline; inspect E1, E2 and the review comments.
2. Move the constraint step, a post-it and its evidence card; observe attached arrows and unchanged evidence.
3. Pan, zoom to 50% and 200%, and operate evidence controls; return with Fit board.
4. Drag Box to open space, rename it, and connect an existing process step to it.
5. Move the new box and verify its arrow follows. Exercise keyboard movement and cancellation.
6. Repeat creation at a different zoom/pan, then Reset and reload; the original fixture returns.

Actual verification, screenshots, remaining gaps and human review are recorded in the [baseline README](../spikes/01-basic-html/README.md), [check record](../spikes/01-basic-html/verification.json) and [SP-07 brief](../delivery/releases/v1.0/board-library-spikes.md#sp-07-direct-html-and-svg-fallback). Automated checks and human acceptance remain separate facts.
