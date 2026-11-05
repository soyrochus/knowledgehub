# Proposal

## Why

The delivered terminal explorer can export relationship, journey and lifecycle diagrams, but readers must leave the console to see the rendered graph. An optional Kitty graphics viewer would let an instructor or permitted reader inspect these same definitions and their connections inside a compatible terminal while keeping ordinary text browsing available everywhere it already works.

## What Changes

- Add a separately labelled **View graph (Kitty)** action in the browser and a dedicated interactive `kh view REF` command. Ordinary `browse`, non-interactive reads and the existing `diagram` export retain their behaviour.
- Reuse the shared `Explorer` and `knowledgehub-view/1` extraction for `relationships`, `journey` and `lifecycle`. Preserve exact selections, typed edge origins, parallel assertions, source bindings and extraction coverage.
- Add terminal capability negotiation, bounded local graph layout/raster rendering, Kitty image transmission, keyboard node/edge selection, a text inspector, pan/zoom and resize/exit cleanup. Unsupported terminals receive an explicit unavailable result and can continue text browsing or export HTML.
- Package the renderer, its assets and notices with the existing macOS arm64 distribution so end users need no browser process, Graphviz, ImageMagick, Kitty executable, development runtime, source checkout or network download.
- Verify all three views against the existing disposable FruitTrucks fixture, generic relationships against the unrelated Equipment fixture, fallback behaviour, current-authority handling and the relocated compiled package. Record actual Kitty/Ghostty execution separately from synthetic protocol checks.

### Outcome slice, prerequisites and acceptance

**Proposed consumer slice TC-SL-02 — Inspect an exact model graph inside the terminal.** A permitted reader selects a definition, explicitly enters the optional graphics view, follows a node or edge to its exact properties and source references, adjusts the viewport, and returns to the same text-browser selection. A direct command offers the same viewer. The bounded outcome includes unavailable-terminal, denied, partial, cancelled and failed-render paths.

Hard prerequisites are the delivered TC-SL-01 explorer, trusted host admission/current permission checks, exact native reads/neighbours and the existing graph projections. The current baseline is [design/terminal-client.md](../../../design/terminal-client.md), the checked-out `clients/terminal/` implementation and its [4 November evidence](../../../delivery/evidence/terminal-client/20261104-apply/README.md). The completed but unarchived `add-terminal-model-client` change owns those existing consumer contracts; its historical planning wording does not mean that the implementation is absent. C8 history remains an existing underlying prerequisite, not new history work.

Acceptance requires a relocated compiled package displaying the R8 relationship neighbourhood, ordered journey and declared offer lifecycle; exact node/edge inspection; readable controls and coverage; return to text browsing; refusal/fallback without graphics support; and cleanup/recovery on resize, cancellation, denial and connection failure. Ordinary Open/Read permission is sufficient for ordinary graph data; no new ReadHistory requirement is introduced. Technical native-terminal execution and human meaning/usability acceptance are separate evidence gates.

### Dataset boundary and exclusions

FruitTrucks already supplies the directed assertions, ordered activity list and explicit lifecycle endpoints needed for this outcome. Its 27 native gaps, unresolved manual/interview authority conflict and proposed expiry remain unchanged. A complete requested graph extraction does not mean a complete or accepted business model.

Detailed entity-schema diagrams are excluded: `Route`, `ChangeOffer` and `Acknowledgement` currently have empty structured field lists, with field meanings and cross-references retained in narrative sources. This change does not infer those links, enrich either fixture, create operational route/offer records or implement deliveries, timers, process execution or full BPMN semantics. Context grouping remains limited to recorded membership.

Also excluded are semantic model writes, new host RPCs/library exports/codecs, stored layout authority, native import/backup, Modelarium changes, new backends, Sixel/iTerm image protocols, animation, mouse interaction, tmux/screen passthrough support, new OS/filesystem support, signing/publication, canonical Hub refresh, archive and authority transfer. Preserve repository `hub/` and `old_hub/` byte-for-byte; use disposable current-contract copies for future verification.

### Proposed choices and remaining review

The first target remains macOS arm64/APFS with a directly connected Kitty or Ghostty terminal; runtime probing, rather than terminal-name environment variables, decides availability. Exact renderer dependency versions/assets will be pinned and recorded during implementation. Bundled renderer/asset compatibility and actual terminal behaviour require new evidence; Linux execution and human usability/meaning acceptance remain separate gates. This proposal authorizes planning only.

## Capabilities

### New Capabilities

- `terminal-kitty-graph-view`: Optional interactive terminal rendering of the existing exact graph projections, including availability, faithful inspection, keyboard navigation, current-authority handling, resource cleanup and self-contained distribution.

### Modified Capabilities

None. This additive presentation capability consumes the delivered `terminal-model-browser`, `terminal-model-visualization` and `terminal-client-distribution` contracts without weakening their requirements. Those capabilities currently live in the completed, unarchived prerequisite change rather than `openspec/specs/`; this proposal neither duplicates them under new names nor modifies that change.

## Impact

Implementation would extend `clients/terminal/src/kh.ts` and `tui.ts`, reuse `explorer.ts`/`visual.ts`, and add isolated viewer, rendering and terminal-protocol modules under `clients/terminal/src/`. Renderer dependencies/assets, package/checksum/license generation, focused tests, PTY checks, documentation and dated delivery evidence would change. No Rust library/host wire or storage change is required. The supported toolchain remains installed Rust 1.99.1 and Bun 1.4.0; install no compiler.

Trace: ontology proposal §§3–5/8/12 (identity, relationships and derived layout), §§14/17 (evidence and gaps), §§21.2–21.3/21.7/21.10 (consumer boundary, bounded selected reads and verification); KH-DR-01–07/12/17–19/21–22; KH-J-02/03/09 and KH-US-09/10/22/24/27. The practical manual's journey → outcome slice → behavioural specification → authorized implementation/evidence → review loop supplies the method. TC-SL-02 is proposed here; this planning change does not revise the approved delivery sequence or transfer definition authority.
