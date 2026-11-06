# Design

## Context

See [proposal.md](proposal.md) for TC-SL-02's outcome, exclusions and source traceability, and [the capability specification](specs/terminal-kitty-graph-view/spec.md) for observable behaviour. This is the design baseline for the user-authorized 5 November 2026 apply; tasks and dated evidence own implementation/verification status. Human acceptance remains separate.

The inspected implementation already has `Explorer` in `clients/terminal/src/explorer.ts`, exact graph extraction in `visual.ts`, browser keyboard/state handling in `tui.ts`, and CLI dispatch in `kh.ts`. `extract` supports relationships, journey and lifecycle, produces `knowledgehub-view/1`, and preserves references/origins/source bindings/coverage. `renderer.ts` uses Mermaid's browser DOM to render SVG in self-contained HTML; it is not an in-process terminal rasterizer. The browser's existing `v` action exports a diagram and must retain that meaning.

`package.ts` currently compiles one Bun entry point plus the matching Rust host and prepared stores. Its checksum, input and license inventory must grow to cover a rendering worker and assets. The current toolchain is installed Rust 1.99.1/Bun 1.4.0, local macOS arm64/APFS. No Rust/executor, file-durability, ontology/profile or host protocol decision changes.

The fixture builder and retained verification establish sufficient structure for three projections: R8 has satisfaction and scenario assertions; the StoryMap orders six activities; the offer lifecycle declares five states/four transitions. The three EntityType definitions have empty `fields`; context membership is sparse. Existing 27-gap and authority-conflict qualifications remain owned by the fixture, not resolved by rendering.

## Goals / Non-Goals

**Goals:** Add a lazy optional presentation consumer with a reversible entry/return path, a bounded renderer that stays responsive to cancellation, and one input/output owner coordinating browser text and images. Preserve the graph DTO and existing native authority checks.

**Non-Goals:** No browser DOM emulation, external renderer subprocess, shared-filesystem image transport, second graph/query engine, persisted coordinates, fixture enrichment or standalone graph-file import. The terminal view does not replace JSON/Mermaid/HTML outputs or expand their semantic contract. Other proposal exclusions also apply.

## Decisions

### 1. Reuse the graph DTO; add presentation modules only

Proposed responsibilities under `clients/terminal/src/`:

| Area | Responsibility |
| --- | --- |
| `graph-view.ts` | Viewer session/state, shared extraction, authority gates, inspector and return result |
| `graph-layout.ts` | Convert a compact exact graph to stable layout geometry and generated SVG |
| `graph-render-worker.ts` | Run layout and WASM rasterization outside the UI event loop |
| `kitty-graphics.ts` | Bounded reply parsing, probing, image IDs/placement, writes/acks and owned cleanup |
| Existing `kh.ts` / `tui.ts` | Dedicated command/action, retained browser state and coordinated terminal lifecycle |

Keep the ordinary `knowledgehub-view/1` object unchanged. Build a separate in-memory presentation record keyed by exact graph node/edge IDs containing coordinates, routes, focus and viewport transform. Layout cannot add a semantic edge. Projection rules and native origins stay in the original DTO. Full property/source reads continue through the existing explorer/host, never native files.

Reuse the existing extractor rather than interpreting Mermaid text or building a second graph from labels. Add no new host methods, privileges or public library exports. Introducing a new visual DTO version or extending model semantics is outside this design.

### 2. Make the optional entry explicit

Browser: expose **View graph (Kitty)** with `V` in the footer/help; retain lowercase `v` for export. A projection prompt selects relationships (default), journey or lifecycle and finite bounds. Capture a clone of browser model/selection/anchor/intent/page/filter/focus/scroll without adding a navigation entry. Suspend browser key/render ownership during the viewer and restore it on close. A projection mismatch produces an explanatory state, not inferred flow.

Direct command:

```sh
kh view REF --projection relationships --depth 1 --vertices 60 --edges 120
kh view model:journey:change --projection journey
kh view model:lifecycle:offer --projection lifecycle
```

Existing global workspace/model/snapshot/baseline/history-anchor and native work bounds apply. The command requires exactly one resolvable root, stdin/stdout TTY and human output. Validate options and reject `--json` or redirected streams before raw mode/probing/host admission; JSON usage diagnostics use the existing envelope without escape codes. `--help` remains model-free. No `--output` is needed; this viewer writes no derived files automatically.

Both entry points lazily load graphics code and start its worker only after invocation. Never probe during ordinary browser launch or scripting commands. Runtime absence of graphics assets cannot initialize/fail ordinary browsing. Build-time packaging must still supply all declared assets.

For direct command completion retain existing exit meanings: 0 on normal close of a complete extraction; 3 for partial/exhausted graph or render bounds; 2 for usage; 4 for denial; 5 for missing/ambiguous roots; 6 for graphics unavailable, protocol/asset/render/output failures; 130 for Ctrl-C cancellation. In-browser failures are visible messages/states with the same reason, returning to a currently authorized browser where possible. Escape/`q` close the viewer; they quit the application only when invoked directly. Missing/ambiguous roots use the existing resolver, never name guessing.

### 3. Use directed layout and a bundled WASM rasterizer

Select `@dagrejs/dagre` for directed multigraph layout, generated restricted SVG for the drawing, and `@resvg/resvg-wasm` for PNG rendering in one Bun worker. Use a bundled unmodified DejaVu Sans font through explicit font bytes; do not discover system fonts. Declare direct pinned dependencies even if Mermaid brings related layout code transitively. Record exact package versions, integrity, font/WASM source revisions, hashes and complete notices during apply.

Dagre receives sorted exact IDs and one edge key per native/projection occurrence, with multigraph support, so cycles/self-links and parallel assertions remain addressable. Node cards show label and type; edge labels show relationship/property meaning. Highlight selected nodes/edges; use explicit direction arrows and textual selected/supporting/unresolved/proposed qualifications. A deterministic presentation transform can omit long label tails visibly, while the text inspector retains full safe text and exact IDs. Missing glyphs receive a visible escaped code-point fallback; do not claim font coverage for every source script.

Only program-generated SVG primitives/text are allowed. Escape model strings; do not embed source SVG/HTML, scripts, `foreignObject`, external images, URLs or filesystem locators. Rasterize a clipped viewport from the complete bounded geometry so zoom/pan does not require a huge full-graph bitmap. Viewport clipping is labelled separately from partial extraction. SVG size, geometry and label limits are checked before raster allocation.

Use one worker so synchronous layout/WASM work cannot block Escape, signal handling or resize. Compile it as an explicit additional Bun entry point and embed WASM/font bytes as declared file assets, using module-relative asset imports rather than cwd paths. The worker receives only compact IDs/display text/geometry inputs, not source authority or native storage paths. Terminate it on timeout/cancel/close; discard any late result by viewer epoch and frame generation. Reinitialization is bounded and explicit.

**First implementation checkpoint:** compile a one-frame worker using the installed Bun 1.4.0, pinned dependencies and embedded font/WASM, relocate it, deny network/checkouts/tool executables and confirm both rendering and cancellation. Upstream documentation establishes a plausible mechanism, not evidence that this installed build works. A failed checkpoint must be resolved within these package/behaviour constraints before full integration; it cannot silently introduce a browser, system tool, compiler installation or end-user download.

Alternatives considered: existing Mermaid rendering requires a browser DOM; headless-browser/Graphviz/ImageMagick tooling would add runtime setup; a native N-API rasterizer adds target-specific linkage. The proposed JS-layout/WASM route confines dependencies to the optional client while retaining one compiled distribution.

### 4. Negotiate a small, broadly implemented Kitty subset

Use direct base64 PNG transport (`t=d`, `f=100`), a query action with an application-owned image ID, and primary device attributes to distinguish unsupported terminals. Do not trust `TERM`/`TERM_PROGRAM`, require a Kitty executable or rely on filesystem/shared-memory paths. Use the stable image/placement/query/delete subset; animation, relative placements, Unicode placeholders, recent feature-query extensions and Kitty keyboard enhancements are unnecessary.

Acquire viewport cell/pixel geometry using bounded terminal queries. A terminal that cannot supply usable geometry is explicitly unavailable for this viewer; there is no guessed cell-size rendering. Detect `TMUX`/`STY` as an out-of-scope multiplexer path and report that direct terminal access is required; do not add passthrough configuration or change terminal settings.

A single input demultiplexer must intercept relevant APC graphics replies and expected CSI geometry/device replies before `emitKeypressEvents`. It handles fragmented/coalesced frames, bounded buffers/deadlines and stale query IDs. Browser key dispatch is suspended during graphics. The demultiplexer continues filtering recognized late replies when the browser resumes; arbitrary user input is not discarded or reinterpreted as a terminal response. Cancellation ends the pending query and invalidates its epoch. Typed input during negotiation is retained for the owning UI, not replayed as a graphics approval.

One serialized output writer owns text and graphics, respects stream backpressure and never interleaves another graphics command inside image chunks. Send base64 chunks of at most 4096 bytes. Use explicit application-owned image and placement IDs, match acknowledgements and bound response time. Retain at most current and replacement image resources; a replacement becomes visible only while its epoch, selection and authority gate are valid. Delete only those IDs, with data-freeing variants on final cleanup, never a global delete-all command.

All viewer/browser text redraws use home plus ED0. ED2/ED3 also erase graphics,
including unplaced uploaded data in Ghostty, and therefore cannot appear in the
upload/reveal path or substitute for owned-ID deletion. The terminal responder
must model this eviction and reject missing-image placements. Negative placement
results retain a bounded sanitized terminal reason. See the user-reported
[placement correction](../../../delivery/evidence/terminal-kitty-graph/20261105-placement-fix/README.md).

### 5. Keep controls and exact inspection in text

The top status identifies model, content selection and independent publication anchor, projection, extraction coverage and derived/read-only status. The bottom status keeps controls and warnings outside the bitmap. Split drawing and a scrollable safe text inspector when dimensions allow. Proposed bindings within the viewer:

| Key | Effect |
| --- | --- |
| `Tab` | Switch graph/inspector focus |
| `n` / `p` | Select next/previous node or edge in stable exact-ID order |
| `e` | Switch node/edge selection mode |
| `Enter` | Inspect the selected node or edge |
| `s` | List exact source bindings; explicitly select an excerpt to read |
| Arrows | Pan the graph viewport |
| `+` / `-` / `0` | Zoom in/out or fit/reset |
| `j` / `k`, PageUp/PageDown | Scroll the focused inspector |
| `?` | Show keyboard help |
| Escape / `q` | Close to browser or finish direct command |
| Ctrl-C | Cancel pending work or terminate direct viewing with exit 130 |

The inspector exposes original graph records, exact relationship endpoints, native selectors/origins, transition guards/effects, projection rules and source bindings. For generic edges whose source metadata belongs to their native origin owner, resolve that exact owner through existing show/source methods. Source excerpts use existing bounded source preview; publication metadata is not fetched implicitly. The viewer never conflates baseline parents, predecessors or publication order with business flow.

Missing/unsupported projections and no-edge results have distinct text. The three EntityTypes can appear as nodes with empty fields; no line is synthesized from narrative `route_id`/`offer_id`. The existing model's full gap catalogue remains available in text browsing; do not claim graph extraction itself exhaustively reviewed every draft gap. Known proposal/interpretation markers and source qualifications are shown in the inspector and applicable view warnings, with absent review status labelled unrecorded.

### 6. Gate existing images as carefully as new data

The current browser hides rows while checking authority and rechecks through host `authorize` before redraw. Reuse this policy for all viewer actions, including pan, zoom, focus, help, source prompts and resize: hide owned placements/details first, check current scoped Open/Read through the host, then reveal only the current valid frame. No graphics cache can bypass that check. Ordinary viewing uses `history: false`; a publication anchor hash is orientation, not permission to disclose publication records.

Track viewer epoch, exact selection and render generation together. A result returning after denial, switch, cancellation or close cannot be uploaded/revealed. On observed Read denial, unavailable policy or transport loss delete owned images, drop graph/inspector/frame caches and stop pending work. Return a denial/disconnection state; retry/reconnect must be explicit and use fresh reads at the retained exact intent. Restore the parent browser through its existing current-authority gate; never restore a raw cached screen. There is no claim of instantaneous out-of-band revocation while an idle terminal has received no interaction/check.

Graph input and parent browser state remain in the UI process; worker frame buffers are ephemeral and released on replacement or close. Layout coordinates are never committed as model facts. As before, native opening/index preparation may perform ordinary operational writes, while semantic publication remains prohibited.

### 7. Publish finite viewer bounds and recoverable states

These are proposed implementation defaults/hard limits, not measured performance claims. Existing lower caller/native budgets still take precedence.

| Work/resource | Proposed bound |
| --- | --- |
| Graph extraction | Depth 1 by default, 1..8; 60 vertices/120 edges by default; explicit maxima 200/500 |
| Native work/data | Existing Explorer call/item/byte/preparation limits; same exact extraction/partial handling |
| Graph labels | 160 displayed code points, up to three lines; visibly ellipsized, full safe text in inspector |
| Worker drawing input | At most 2 MiB generated display/geometry data; no embedded artifacts |
| Generated SVG | At most 2 MiB; checked before rasterization |
| Viewport raster | At most 4096 pixels per dimension and 4,194,304 total pixels, RGBA at most 16 MiB |
| PNG / transmitted frame | At most 8 MiB PNG and 12 MiB encoded graphics frame |
| Terminal resources | At most two owned images and one visible placement; one upload at a time |
| Pending rendering | One active worker request plus one latest coalesced viewport intent |
| Deadlines | Probe 1 second; geometry/acks 1 second per phase; layout+raster worker 5 seconds; output stall 2 seconds |
| Reply parsing | At most 64 KiB retained control-response bytes; finite stale-response tracking |
| Usable split layout | At least 90 columns × 24 rows; smaller windows show a text size notice |

Budget refusal and worker timeout clear the drawing and state the reached bound; they do not silently substitute fewer edges. Explicit extractor partial graphs can render with visible qualification. Pan/zoom/resize change only presentation and stay within raster bounds. Resize cancels/coalesces obsolete rendering and preserves selected exact IDs. While output or authorization is pending, keep keyboard cancellation responsive.

State progression: closed → probing → extracting → rendering → viewing; any active phase can enter partial, unsupported, denied, disconnected, size-limited or failed text states. Every terminal-ownership transition has a `finally` cleanup path for raw mode/cursor/listeners/worker/images. Suspend/resume and handled SIGINT/SIGTERM/SIGHUP also clear owned graphics and revalidate before a resumed display. Uncatchable process kills cannot guarantee application cleanup and are not reported as successful restoration. On stdout failure attempt cleanup where possible and report failure, never certify deletion that was not written.

### 8. Extend the existing package without changing fixture authority

Include the worker as a compile entry point, font/WASM asset bytes and their full license/source notices in build inputs, checksums and dependency inventories. The existing two-executable contract remains; no end-user renderer program is added. Loading dependencies/assets remains lazy. Extend `design/terminal-client.md`, the packaged quick start and README usage during implementation with direct-terminal requirements, commands, keys, unavailable outcomes and dataset limitations.

Use fresh disposable copies of quiescent current-contract fixtures; do not rerun or modify `build.py`, their plans/sources, retained master archives or historical evidence to add relationships. Record exact fixture IDs/selections/hashes and compare semantic objects, branch refs, source bytes and original receipts before/after exploration. Operational caches are a distinct native concern. Preserve `hub/` and `old_hub/` without opening or capturing them.

### 9. Separate automated checks from actual visual execution

Verification layers, in dependency order:

1. Worker/package checkpoint, pure layout identity/multigraph tests, bounded parser/encoder tests, malformed/late reply and backpressure/cleanup checks.
2. Synthetic terminal/PTY checks for negotiation success/failure, no-response fallback, geometry, keyboard/resize/close, cancellation, authority revocation and no stale-frame redraw. These prove client behaviour, not terminal pixel display.
3. Public-API integration on disposable FruitTrucks and Equipment copies. Compare the exact graph IDs/origins/selection/coverage with ordinary `diagram --format json`; test ordinary Open/Read without ReadHistory. Verify dataset drafts and semantic/source hashes are unchanged.
4. Relocated compiled package with empty tool PATH, denied checkout/network access and embedded assets; preserve existing text/CLI/HTML regressions.
5. Actual directly connected Kitty and Ghostty on the supported macOS target, each with terminal version, package hash, exact selection, observed readable labels/arrows, selection/inspector, pan/zoom, resize/exit cleanup and evidence captures. An unavailable terminal leaves its individual gate open rather than being replaced by a successful PTY simulation.
6. Separate human meaning/usability review of the bounded instructor task. Technical checks or agent-operated walkthroughs do not close this gate or resolve FruitTrucks policy.

Trace implementation evidence to ontology §21.10's normal, denied, read-consistency and API-boundary concerns. No new mutation/durability contract is introduced. Run TypeScript checks and the terminal suite; use existing Rust/host regressions proportionately for host-facing integration/packaging. Do not rerun unrelated disruptive storage tests solely for a new presentation layer.

## Risks / Trade-offs

- **WASM/worker/font bundling differs in installed Bun:** Resolve with the first compiled relocation checkpoint before UI integration; keep dependencies pinned and retain the actual result.
- **Dense graphs, long labels or unusual scripts reduce readability:** Start with bounded neighbourhoods, retain text inspection, visible label truncation and missing-glyph fallback; do not assert universal typography support.
- **Reply parsing corrupts key handling or images survive text redraw:** One demultiplexer/output owner, bounded stale-reply filtering and explicit owned-image cleanup; verify normal/error/resize/signal paths.
- **Old images reveal cached data after denied reads:** Hide before policy checks, invalidate epochs, release all image/frame/inspector caches on observed denial and test help/pan/zoom/resize paths.
- **Extra assets or renderer initialization break existing text startup:** Lazy initialization, negative asset tests and unchanged text/JSON/export regression checks in the compiled package.
- **Terminal/version differences and multiplexers:** A minimal directly negotiated protocol subset, explicit unsupported outcomes and separate actual Kitty/Ghostty evidence; no blanket console compatibility claim.
- **Rendering implies a complete business model:** Preserve sources, proposal markers and coverage; keep the 27 fixture gaps, authority conflict and narrative-only entity structure explicit.

## Migration Plan

No model, ontology, storage, host protocol or source migration is needed. After a separate apply authorization, implement/check the renderer checkpoint, integrate the optional viewer, verify disposable fixtures and package, and record scoped evidence before claiming support. Leave historical TC-SL-01 evidence and archives unchanged. Rollback is the prior terminal package; it can open the same current-contract stores because no semantic change is required.

## Open Questions

Apply pinned Dagre 3.1.3/graphlib 4.0.6, resvg-WASM 2.6.2 and unmodified DejaVu Sans 2.37; `clients/terminal/assets/graphics/inventory.json` owns exact upstream identities, integrity and hashes. Actual Kitty/Ghostty execution remains separately gated. Whether practitioners find the bounded view useful remains a separate human review outcome, not an implementation decision or inferred acceptance.

## Primary technical references

Reviewed on 5 November 2026; these describe upstream capabilities, not execution of this change:

- [Kitty graphics protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/): image formats, queries, chunked transmission and scoped deletion.
- [Ghostty features](https://ghostty.org/docs/features): documented Kitty graphics support.
- [Dagre project](https://github.com/dagrejs/dagre): directed client-side graph layout.
- [resvg WASM package](https://github.com/thx/resvg-js/tree/main/wasm) and [owned font-buffer/render API](https://github.com/thx/resvg-js/blob/main/wasm/index.d.ts).
- [Bun executable workers/assets](https://bun.com/docs/compiler/executables): explicit worker entry points and embedded file imports; verify installed 1.4.0 locally during apply.
- [DejaVu font notices](https://github.com/dejavu-fonts/dejavu-fonts/blob/master/LICENSE): retain the applicable original notices with the selected unmodified font.
