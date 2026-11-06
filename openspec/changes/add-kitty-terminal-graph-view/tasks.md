# Tasks

The user authorized implementation on 5 November 2026. Checkboxes record the stated implementation/evidence; actual-terminal gates remain separate. Follow [the capability specification](specs/terminal-kitty-graph-view/spec.md) and [design decisions/bounds](design.md); TC-SL-01's historical evidence does not verify this viewer.

## 1. Prove the packaged renderer foundation

- [x] 1.1 Pin direct Dagre/resvg-WASM dependencies and an unmodified DejaVu Sans font; record versions, upstream revisions, integrity/hashes and notices. Verify frozen-lockfile installation and a complete asset/license inventory without changing Rust/toolchain or fixture definitions.
- [x] 1.2 Add a lazy render-worker entry and explicit embedded WASM/font inputs to a minimal compiled renderer checkpoint. Verify one labelled PNG and worker cancellation on installed Bun 1.4.0 from a relocated path with spaces, empty tool PATH and denied checkout/network access before full viewer integration.
- [x] 1.3 Document the checkpoint, selected inputs and lazy asset loading in the terminal maintainer guide; verify missing/incompatible graphics initialization has a confined typed failure and ordinary help/text/JSON/HTML paths still work without renderer startup.

## 2. Render the existing exact graph faithfully

- [x] 2.1 Add a compact presentation adapter over unchanged `knowledgehub-view/1` data and sorted-ID directed multigraph layout. Verify cycles, self-links, duplicate labels, parallel assertions and property occurrences keep exact node/edge IDs/origins and no invented semantic links.
- [x] 2.2 Generate restricted SVG with node types, direction arrows, selection markers, visible label truncation and qualified statuses; rasterize its viewport with the bundled font. Verify long/hostile text, XML/terminal escapes, absent glyph fallback and disabled external resource resolution with focused rendered samples/tests.
- [x] 2.3 Enforce graph/display/SVG/pixel/PNG/frame limits and one-active/one-latest worker scheduling with generation cancellation. Verify oversize inputs fail before raster allocation, worker deadlines remain cancellable and obsolete frames cannot replace the current view.
- [x] 2.4 Document viewer budgets, viewport clipping versus extraction coverage, and the narrative-only entity/context limitations; verify examples match the unchanged FruitTrucks plan and all render tests preserve partial/unresolved/proposed qualifications.

## 3. Negotiate graphics and own the terminal lifecycle

- [x] 3.1 Add the bounded APC/CSI response demultiplexer and direct-terminal graphics/geometry probes. Verify fragmented/coalesced/late/malformed replies, unsupported/device-only replies, timeout, cancellation, bounded buffers and ordinary key preservation through synthetic input tests.
- [x] 3.2 Implement serialized direct PNG transmission, 4096-byte base64 chunks, application-owned IDs, acknowledgements and stream backpressure. Verify byte/frame bounds, write ordering, negative/missing acknowledgements, output stalls and no graphics interleaving with text or cleanup.
- [x] 3.3 Implement owned-placement/data cleanup and terminal/listener/worker restoration across close, partial initialization, denial, render/transport/output failure, suspend/resume and handled signals. Verify the protocol/PTY failure matrix deletes only owned IDs and never reports unwritten cleanup as successful.
- [x] 3.4 Document the minimal Kitty subset, geometry/deadline requirements and explicit tmux/screen refusal; verify normal browser/CLI commands perform no probes and simulated unsupported terminals return usable text/HTML alternatives without launching apps or creating files.

## 4. Deliver the reversible interactive viewer

- [x] 4.1 Add `kh view REF` with projection/depth/vertex/edge options, existing exact selection/work bounds and documented exit outcomes. Verify invalid options, missing/ambiguous roots, JSON mode and non-TTY input/output fail correctly before graphics bytes, including model-free help.
- [x] 4.2 Add the separately labelled `V` browser action/projection prompt and coordinated input/render ownership while preserving lowercase `v` export. Verify close/error/cancel restores exact browser model, selection, anchor, history, page, filter, focus and scroll through PTY interaction tests.
- [x] 4.3 Implement keyboard node/edge selection, inspector scrolling, pan/zoom/fit/help and the small-window state. Verify every action without a mouse, readable text controls/qualifications and selection retention across resize with synthetic PTY tests and rendered samples.
- [x] 4.4 Connect the inspector and explicit source excerpt action to exact graph data/native origin owners and existing bounded source reads. Verify node types, edge endpoints/selectors/origins, source revisions/anchors, lifecycle guards and proposed status without implicit publication disclosure or stored script execution.
- [x] 4.5 Gate every reveal/interaction/redraw with current host authority and viewer/frame epochs; hide/delete stale graphics and drop details on observed denial/disconnection. Verify Read revocation during render, pan, zoom, help, prompts and resize; explicit pinned recovery; and ordinary graph use with Open/Read but no ReadHistory.
- [x] 4.6 Update README/terminal guide/packaged quick start with the separate action/command, controls, unavailable/partial/failure paths and return behaviour. Verify the documented commands and keys against automated viewer walkthroughs while retaining draft/authority limitations and human acceptance as separate facts.

## 5. Verify native data, package and actual terminals

- [x] 5.1 Use fresh quiescent disposable copies of the existing fixtures to exercise FruitTrucks relationships/journey/lifecycle and Equipment relationships through one generic viewer. Compare exact graph references/origins/selections/coverage with ordinary JSON diagrams, and verify semantic records/refs, source bytes and original receipts remain unchanged. Do not open or capture protected `hub/` or `old_hub/`.
- [x] 5.2 Complete package integration for explicit worker entries, embedded asset inputs, hashes and license/source notices. Verify relocated compiled graphics operation with empty tool PATH and denied checkout/network access, including graphics-asset failure isolation and unchanged text/JSON/Mermaid/HTML/package regression paths.
- [x] 5.3 Run TypeScript checks and the focused/full terminal suites, compiled synthetic PTY checks and appropriate existing host/package regressions. Verify source formatting/whitespace and strict OpenSpec validation; retain logs and distinguish synthetic terminal evidence from actual image display.
- [ ] 5.4 Observe the compiled package in a directly connected Kitty terminal on the supported macOS target. Record package/terminal versions, exact fixture selections, readable three-view display, node/edge/source inspection, pan/zoom, resize and exit cleanup with evidence captures; leave this task open if execution is unavailable.
- [ ] 5.5 Observe the same bounded instructor path in directly connected Ghostty on the supported macOS target. Record its own terminal/version/visual/cleanup evidence and actual unsupported-terminal fallback; leave this task open if execution is unavailable rather than substituting PTY results.
- [x] 5.6 Publish dated TC-SL-02 implementation evidence with an acceptance-criterion-to-check mapping, exact inputs, remaining terminal/platform gates and protected-store handling. Verify it separates automated checks, actual terminal execution, unchanged fixture/source coverage and pending human meaning/usability acceptance, then review the proposed delivery sequence in the owning plan without transferring model authority.

## Workflow follow-up

5 November native attempt: the user reported a Ghostty placement rejection in
`final-04`. The [placement correction](../../../delivery/evidence/terminal-kitty-graph/20261105-placement-fix/README.md)
reproduces the cause and replaces it with `final-05`; task 5.5 still requires a
successful native run of the corrected package and its stated observations.

- Obtain separate human meaning/usability review of the bounded viewing task; preserve any limitations and FruitTrucks policy conflict independently of technical verification.
- Archive or sync only when explicitly requested through the corresponding workflow. Planning completion does not authorize implementation, archive, publication or authority handover.
