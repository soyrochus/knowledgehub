# TC-SL-02 · Optional Kitty graph view · 5 November 2026

**Superseded package:** the user reported a Ghostty placement rejection in
`final-04`. Its synthetic terminal did not model image eviction by full-screen
erase. See the [placement correction](../20261105-placement-fix/README.md) for
the reproduced failure, corrected redraw and replacement package. The original
logs/hashes below remain historical evidence, not successful native display.

The user authorized `apply add-kitty-terminal-graph-view` on 5 November 2026.
Implementation adds `kh view REF` and the separately labelled browser `V` action.
It consumes the existing Explorer and unchanged `knowledgehub-view/1` extraction;
no Rust library, native host RPC, ontology/codec, fixture or model authority changes.
The terminal remains read only. This is technical delivery evidence, not human
meaning/usability acceptance, model completeness or authority handover.

## Delivered behavior

Relationships, declared StoryMap activity order and declared lifecycle transitions
use one generic presentation/layout/worker path. Exact nodes, endpoints, assertion
and property occurrences, native owners, parallel assertions and source bindings
remain in the original DTO/inspector. No narrative fields or missing memberships
become graph edges. Labels are inert, escaped, visibly shortened, and unsupported
font glyphs have explicit code-point fallbacks. Original source excerpts are
explicit bounded reads. No stored script or model-supplied resource is executed.

Dagre 3.1.3/graphlib 4.0.6, resvg-WASM 2.6.2 and unmodified DejaVu Sans 2.37 are
pinned. [graphics-inputs.json](graphics-inputs.json) records package integrity,
upstream identities and all asset hashes. Full notices/source locations are in
`clients/terminal/assets/graphics/`, copied to package `graphics-notices/`.
Bun 1.4.0 and existing Rust 1.99.1 were used; no compiler was installed.

The renderer checkpoint verified labelled PNG production and cancellation from
a path with spaces, empty tool PATH, denied checkout reads and denied network.
Bun 1.4.0 requires the fixed `--root clients/terminal` and explicit worker compile
entry. Font/WASM file imports resolve inside the executable. Missing worker
initialization is a typed confined GraphicsAsset failure; help, ordinary native
JSON/text reads and existing HTML export still worked in the negative build.

Viewer writes are serialized with backpressure and deadlines, 4096-byte base64
chunks, owned IDs/placements and matched acknowledgements. One worker request
and one latest viewport intent are bounded; obsolete epochs cannot reveal data.
Every action hides prior images/details and checks current scoped authority;
text/image reveal follows current checks together. Observed denial/disconnection
drops caches. Explicit retry retains root, exact content selection and independent
publication anchor. Close/error/cancel/suspend/signal paths restore resources
where output remains writable; unwritten cleanup is never reported successful.

## Verification mapping

| Capability requirement / acceptance concern | Evidence/check |
| --- | --- |
| Separate optional entry; ordinary text/export behavior | CLI usage checks; missing-worker negative build; existing browser PTY and 57-command archive suite |
| Usage failures before probes/host admission | Model-free help; JSON/non-TTY/missing argument/invalid bounds and projection checks with nonexistent workspace |
| Runtime negotiation and late reply isolation | Parser/encoder tests; fragmented/coalesced/malformed/late APC/CSI; negative, missing and device-only support replies; geometry deadline |
| Exact root, content selection and independent anchor | Four native JSON comparisons; browser return-state PTY; no-history workspace; explicit pinned retry |
| Node/edge identity, cycles, repeated labels and parallel assertions | Directed multigraph tests; exact edge IDs/origins; self-links; native two-satisfaction-assertion checks |
| Recorded journey/lifecycle semantics | Six activities / five derived list-adjacency edges; five states / four declared transitions; exact endpoint origins and proposed-expiry warning |
| Narrative-only structure stays absent | Presentation uses original DTO only; fixture plans/sources untouched; no field or context enrichment |
| Coverage and qualifications | Native JSON coverage retained; unresolved/partial/proposed tests; one-vertex partial PTY; viewport clipping labelled separately |
| Inspector and explicit original excerpts | Keyboard source/preview PTY; exact native origin-owner reads; source revision/anchor metadata; native source/hash regression |
| Keyboard controls, resize, return state | n/p/e/Enter/s, Tab, inspector scroll, pan/zoom/fit/help; size notice and resize recovery; exact browser header/page/filter retained |
| Current authority on every reveal/action | Unit revocation during worker rendering; compiled pan/zoom/help/source/resize revocation; pinned explicit recovery; no ReadHistory requirement |
| Bounded graph/display/raster/pending work | Count/label/SVG/pixel/PNG/frame bounds; worker deadline/cancellation/coalescing; clipped wide-graph pan/zoom regression |
| Lifecycle, signals and owned cleanup | Protocol/output-stall tests and compiled close/cancel/suspend/resume/handled-signal matrix; no delete-all commands |
| Model text is inert | Hostile terminal/XML/markup and absent-glyph tests; generated primitives only; no URL/image/script/resource resolution |
| Self-contained optional graphics package | Relocated compiled archive; embedded worker/font/WASM; empty PATH; OS sandbox denies checkout/network/development tools; asset-failure isolation |
| Native fixture integrity | Same four extracted graphs/render path; semantic objects/refs/header, reference/source bytes and original receipt hashes compared before/after |
| Evidence boundaries | Synthetic PTY images are not native terminal display; each actual-terminal gate and human review remain separate below |

## Recorded checks and retained inputs

- TypeScript check, focused/full terminal tests and formatter checks passed.
  [bun-final.log](bun-final.log): 23 tests passed, 530 assertions, native release
  host integration included. [focused-final.log](focused-final.log) records the
  final rendering/protocol/session checks. One initial debug-host integration run
  exceeded its existing 120-second per-test deadline; the release-host run passed.
- Five Rust host tests passed; [host-regression.txt](host-regression.txt) records
  the observed tool result. Host bytes match the retained TC-SL-01 package.
- [package-regression.json](package-regression.json): 57 compiled command checks,
  330 package checksums, relocated cwd with spaces and empty development-tool PATH.
- [html-browser.json](html-browser.json): existing offline HTML rendered five
  nodes/five edges with zero network attempts; [capture](html-preview.png).
- [graph-data.json](graph-data.json): exact native selections/anchors, IDs,
  coverage and graph digests for FruitTrucks relationships/journey/lifecycle and
  unrelated Equipment relationships. Each adapter/render input matched a fresh
  ordinary JSON extraction; original DTO bytes remained unchanged.
- [isolation.json](isolation.json): OS sandbox denies `/Users/ivanderk/src`
  reads, network and execution of Bun/Node/Cargo/rustc. Normal compiled commands
  and HTML export passed from the relocated package.

Retained full maintainer outputs are under `target/terminal-graphics/` and
`/private/tmp/knowledgehub-kitty-validated-20261105/`. The final package and
synthetic graphics/legacy PTY summaries are identified in [completion.json](completion.json).
The final archive is `target/terminal-graphics/knowledgehub-terminal-kitty-macos-arm64-final-04.tar.gz`;
its runtime bytes match the OS-isolated, relocated graphics-tested pair.
[graphics-pty.json](graphics-pty.json) records all 25 passing compiled cases and
32 unchanged semantic/source/reference/receipt files.
[legacy-pty.json](legacy-pty.json) records the existing browser regression under
the same OS isolation; [asset-failure.json](asset-failure.json) records the final
missing-worker build and available text/JSON/HTML alternatives.
The host/store inputs are fresh copies of the retained quiescent TC-SL-01 package,
not regenerated fixtures. Native opening may perform ordinary operational/cache
writes; semantic publication is prohibited. Protected repository `hub/` and
`old_hub/` were not opened, decoded, captured or targeted by writes. This task
makes no claim based on recapturing those historical stores.

## Actual terminal and human gates

See [native-terminal-gates.json](native-terminal-gates.json) and the
[remaining observation checklist](NATIVE-TERMINAL-CHECKLIST.md). Kitty is absent
from the application inventory/PATH. Ghostty CLI reports 1.4.0, but the computer-use
tool refused app access: “Computer Use is not allowed to use the app
com.mitchellh.ghostty for safety reasons.” No alternative UI automation bypass was
attempted. Actual Kitty display and actual Ghostty display/unsupported-terminal
observations therefore remain open (tasks 5.4 and 5.5). Protocol/PTY success does
not close either gate or establish broad terminal/version compatibility.

FruitTrucks retains 27 native gaps, its source/authority conflict and proposed
expiry. Successful requested extraction/rasterization establishes neither a
complete nor an accepted business model. Linux execution, signing/notarization,
publication, Modelarium C9, canonical Hub refresh, external authoring-host gates,
authority transfer and human meaning/usability acceptance are not established.
No change was archived or synchronized to main specs.
