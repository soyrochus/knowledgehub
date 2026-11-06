# Native terminal model client · TC-SL-01

The terminal client is a read-only consumer of `knowledgehub::KnowledgeHub`.
`clients/terminal/` owns the pinned Bun/TypeScript explorer, CLI, TUI and derived
visualization. `src/terminal_client_host/` is private to the
`knowledgehub-client-host` binary. The library's API, codecs and persistence
semantics are unchanged. The native-history C8 prerequisite is documented in
[native-model-history.md](native-model-history.md); this consumer does not
implement another journal, retained catalogue or diff engine.

## Build and run

Maintainers use the installed Rust 1.99.1 and Bun 1.4.0:

```sh
cargo build --bin knowledgehub-client-host --bin knowledgehub-model --example model_successor --example terminal_fixture_handover
cd clients/terminal
bun install --frozen-lockfile
bun run assets
bun run check
cd ../..
python3 scripts/prepare-terminal-models.py target/terminal-client/my-fresh-models
bun clients/terminal/src/kh.ts --workspace target/terminal-client/my-fresh-models/workspace.json info --json
bun clients/terminal/src/kh.ts --workspace target/terminal-client/my-fresh-models/workspace-no-history.json types --json
bun clients/terminal/src/kh.ts --workspace target/terminal-client/my-fresh-models/workspace-no-history.json publications --json
```

The last command exits 4, `Denied / ReadHistory`. Ordinary browsing works with
Open/Read alone. `--help` documents every command and option without starting a
host. `knowledgehub-client-host --workspace FILE` is a private framed pipe, not
an interactive JSON-line shell. `KNOWLEDGEHUB_CLIENT_HOST` is an explicit local
launcher override for maintainers; requests cannot change it or model authority.

`bun clients/terminal/scripts/package.ts NEW-DESTINATION` builds the macOS arm64
pair, embedded renderer, two newly authored quiescent Hubs, source/coverage
references, handovers, licenses, exact build inputs, checksums and tar archive.
It refuses occupied destinations. End users run the compiled `kh`, which finds
its sibling native host and relative `models/workspace.json`. They need no
development tools or source checkout. The [Session 4 quick start](../examples/fixtures/terminal-client/QUICKSTART.md)
ships as the package README.

## Workspace admission and authority

`knowledgehub-client-workspace/1` contains a default alias and 1–64 entries:

```json
{
  "format": "knowledgehub-client-workspace/1",
  "default": "teaching",
  "models": [{
    "alias": "teaching", "name": "Disposable model",
    "directory": "models/teaching",
    "scope": {"hub":"example:hub","model":"example:model","branch":"main","owner":"example:owner"},
    "actor": "host:reader-B",
    "grants": ["Open", "Read", "ReadHistory"],
    "known_baselines": [{"model":"example:model","id":"baseline:A","revision":"1"}]
  }]
}
```

Paths resolve from the workspace file, must exist and be canonical, and reject
symlinks and dot/parent traversal. Directory device/inode, scope and actor are
bound at admission and rechecked on every request and policy call. A changed
admission requires an explicit reopen. Grants reload for native permission
checks; current authority also gates the returned result. Invalid or unavailable
policy fails closed. Unknown grants and authority/path RPC overrides are refused.
Open and Read are mandatory to open a model. ReadHistory is optional and separate;
the actor never becomes the original publisher. All other `Action` variants are
exhaustively denied. Exact handover references are validated as native Baselines.
The protected repository `hub/` and `old_hub/` are rejected before native open.

The host rejects Create, Validate, Commit, RegisterOntology, operation_result and
full-state dump RPCs. It exposes only declared reads, preparation, cursor release
and drain. Native file opening can perform ordinary journal recovery; native
queries may build derived navigation indexes within declared preparation bounds.
Those ordinary backend writes are distinct from semantic publication. There is
no hidden history repair: IndexNotReady, HistoryLimit and integrity failures stay
typed. Head, exact records, original artifacts and original receipts remain the
semantic invariants, rather than a claim that every cache file remains unchanged.

## Compatibility, transport and limits

| Contract | Delivered boundary |
| --- | --- |
| Client framing | `knowledgehub.client/1`; 4-byte big-endian length + UTF-8 JSON; correlated string IDs |
| Native API | unpublished 0.10.0, full `KnowledgeHub`; matching method/DTO handshake required |
| Semantic/storage | record/request `/3`, catalogue-3, files/4; no old-format reader/converter |
| History projection | `knowledgehub-history/1`; publication, publications, publication_affected, continue_history, release_history |
| History authority | current scoped Read + ReadHistory; ordinary reads need Open + Read |
| Native history defaults | 100 items, 256 nodes, 16 MiB encoded+decoded storage, 256 KiB output |
| Native history maxima | 1,000 items, 8,192 nodes, 128 MiB storage, 4 MiB output per response |
| Shared cursors | native pool 64 across query/neighbour/traversal/history; 30 idle minutes; exact intent and current authority |
| Wire | frame maximum 8 MiB; default complete result envelope 1 MiB; explicit 256..8 MiB−1024 per RPC |
| Consumer output | default 1 MiB, CLI explicit 8 KiB..8 MiB−1024 including result envelope |
| Aggregate command | default 512 data RPCs, explicit up to 1,024; at most 64 history data reads and 10,000 collected items/references |
| Preparation | default/explicit native 10,000 owners, 100,000 facts, 64 MiB; no silent unlimited rebuild |
| Original content | ranges at most 64 KiB; preview reads available remaining bytes; original export at most 8 MiB, full SHA-256 check |
| Client pending work | at most 16 admitted operations; one host process reused, native dispatch sequential |

Unsigned native counters and offsets cross the wire as canonical decimal strings.
Only schema-defined request counters are converted; authored Text, Integer and
Decimal values keep their native tags and exact text. Responses preserve error
variants, original actor/fingerprint/heads/sequence and native `HistoryWork`.
Native failures remain typed control responses within the hard frame bound, even
when a requested data budget is too small for their diagnostics. An oversized
history data response retains its original native work in the Capacity error;
an undisclosed continuation is released. Full framed response bytes and final encoded CLI bytes are measured separately
from native output/storage work; neither is substituted for native work.

Cancellation detaches the caller without claiming to stop already admitted native
work. Its slot remains occupied until a response or EOF; close waits, drains the
host, and reports an operational failure if its 30-second drain deadline is
reached. Malformed, truncated, uncorrelated or incompatible responses invalidate
the transport. No reply loss is interpreted as an empty successful result.
Cleanup calls are separate from the 64 history data-call guard; at most the
bounded native pool can need release before drain.

## Shared exploration semantics

`Explorer` and `openExplorer(workspace, options)` are the local TypeScript API,
also used by CLI and TUI. `scripts/walkthrough.ts` is an executable example;
call `close()` in `finally` without handling private framing. It
captures one content `Selection` and a separate publication snapshot anchor.
A selected snapshot defaults its anchor to itself; baseline selection captures
head once unless an explicit history anchor is supplied. Baseline parents and
definition predecessors never become publication anchors. There is no result
cache capable of bypassing current policy. TUI data is hidden while authority is
checked and rechecked before redraw; denial/disconnection clears cached rows,
including keyboard scrolling, filtering, help, prompts and resize. Exact back/forward navigation queries
the retained intent afresh; live pages retain their full request. Invalid tokens
restart visibly at the pinned first page; disconnected sessions require explicit
reconnect. Refresh alone explicitly captures latest head.

Types and inherited schemas come from exact registered definitions. Resolution
accepts exact JSON references, selected semantic IDs or unambiguous display names;
duplicates return exact candidates. Arbitrary native kinds and assertions remain
inspectable. Neighbour selectors, categories, directions, occurrences and fact
scope are native. Tree projections mark cycles, shared references and bounded
frontiers. Context/journey/source groupings use recorded references and disclose
unassigned or unexamined memberships. They do not infer ownership from names.

Literal search requires explicit exact type/field coverage and returns native
spans. It is not semantic or whole-artifact search. Source discovery follows
exact narrative/provenance bindings and preserves statuses; unsupported binary
previews offer original-byte export. Gaps use the public bounded native coverage
projection from an explicit type-parent traversal; omitted coverage is visible,
and this is not a new complete gap catalogue.

Publication lookup preserves complete anchor-relative `NotPublishedAt` separately
from unknown/unavailable anchors, corruption, denial and zero-progress limits.
Discovery is descending accepted sequence; affected references retain original
vector order and repetitions. Opening an affected definition in the TUI uses the
original publication header's snapshot. Baselines have selected, validated
handover and opt-in retained origins. Retained discovery composes publications,
affected pages and exact reads at each original header.head, and declares partial
coverage until every required range and candidate read finishes. Native compare
is the existing unpaged exact A/B operation with no snapshot-anchor parameter;
the consumer bounds output without advertising another comparison engine.

`knowledgehub-exploration/1` carries model, reader, content selection, independent
anchor, reproducible intent, data, coverage, warnings and typed failure. Exit 0
means complete requested read, including NotPublishedAt; 2 usage, 3 partial/limit/
readiness, 4 denial, 5 missing/ambiguous, 6 operational/integrity/unavailable, and
130 cancellation. One-shot results never advertise process-local tokens as
reusable. Partial discovery retains verified items and an exact restart intent.

## Derived visualizations and Workbench boundary

`knowledgehub-view/1` records exact vertices, membership/availability, edge owners,
selectors/occurrences, native direction and source bindings. Relationship
projections use native neighbours; parallel assertion identities remain separate.
Journey order uses declared StoryMap activity lists; lifecycle transitions use
declared from/to, guards and effects. Missing order, transitions or ownership stay
missing or explicitly unsupported. Proposed expiry remains proposed descriptive
design. Node/edge bounds, unresolved endpoints and native partial coverage remain
visible in JSON, Mermaid, HTML and paired digest manifests.

The pinned Mermaid 12.3.0 browser bundle is embedded in self-contained HTML, with
strict rendering, escaped data/text, a network-denying CSP, legend, coverage,
zoom/pan and exact node/edge/source inspectors. Composed lifecycle/journey edges retain both
actual native property occurrences and their explicit projection rule. Output requires an explicit ordinary
path outside any configured store; symlink traversal and occupied files are refused
unless replacement is explicitly requested. Original-source exports preserve bytes.

Workbench consumers can retain `knowledgehub-view/1` identities and provenance:

```sh
bun clients/terminal/src/kh.ts --workspace WORKSPACE diagram model:requirement:ack --format json --output graph.json
bun clients/terminal/scripts/consume-view.ts graph.json new-local-preview.html
```

The example validates the view format and retains exact input identities; it
renders offline with the same bundle and emits Mermaid. It publishes no native
model and automatically imports no learner artifact. These are editable derived
inspection artifacts, not backup/import, implementation or acceptance evidence.

## Verification and remaining gates

Host tests, native reader-B integration, transport simulations, CLI checks,
compiled real-PTY keyboard/recovery checks, actual browser rendering with network
blocked, relocation/empty-PATH checks and package evidence belong to TC-SL-01.
The earlier C8 native-history evidence is a prerequisite, not proof of this UI.
See the dated [terminal evidence](../delivery/evidence/terminal-client/20261104-apply/README.md).
Observed instructor-style operation is technical observation; human meaning and
usability acceptance remain pending. Verified distribution is local macOS arm64
on APFS. Linux native execution, signing/notarization/publication, Windows and new
filesystem support are separate and unclaimed. Modelarium, crate publication,
canonical Hub refresh, archive and authority transfer remain outside this scope.

Maintainer references: [Bun executable bundling](https://bun.com/docs/bundler/executables)
and [Mermaid configuration](https://mermaid.js.org/config/usage).

## Optional terminal graph view · TC-SL-02

`kh view REF` and the browser's separately labelled uppercase `V` action open
View graph (Kitty). Lowercase `v` still exports. Normal browsing and ordinary
CLI/JSON/Mermaid/HTML paths send no graphics queries and start no render worker.
The command requires terminal stdin/stdout and human output. Usage validation
(including `--json` rejection) and model-free help happen before host admission.
A missing or ambiguous exact root fails before probing. No files or apps open
automatically. Direct tmux/screen sessions are refused; use a direct terminal.

```sh
./kh view model:requirement:ack
./kh view model:journey:change --projection journey
./kh view model:lifecycle:offer --projection lifecycle
./kh --model equipment view model:entity:Booking
```

The browser asks for projection and finite bounds, suspends its own key/render
handling, then restores model, exact content selection, independent publication
anchor, navigation stacks, page, filter, row and scroll without refreshing.
The restored browser rechecks current authority. The viewer's `R` explicitly
retries its retained exact root/selection/anchor; it never substitutes latest.
The ordinary workspace's Open/Read grants suffice; ReadHistory is not requested.

Controls: `Esc`/`q` close, Ctrl-C cancels, `?` toggles help, Tab changes focus,
`n`/`p` select the next/previous exact node or edge, `e` switches node/edge mode,
Enter inspects the native origin owner, `s` lists source bindings, then `n`/`p`
choose a source and Enter explicitly reads up to 4096 original bytes. `j`/`k`
and PageUp/PageDown scroll the inspector; arrows pan, `+`/`-` zoom and `0` fits.
These controls need no mouse. The inspector retains full safe properties,
selectors, endpoints, origin occurrences and source revisions/anchors; original
byte export remains the separate existing source workflow. Model text and stored
code are inert. Missing glyphs are displayed as `[U+CODEPOINT]`; no font coverage
is inferred. Missing order/fields/ownership remains missing.

A terminal must answer the Kitty graphics query and cell/pixel geometry queries
within one second per phase. `TERM`/`TERM_PROGRAM` cannot establish support. The
minimal subset is direct PNG/base64 (`t=d`, `f=100`), query, upload, explicit
placement and owned-ID deletion. Frames use at most 4096 base64 bytes per chunk,
serialized writes, matched acknowledgements, two owned images, one visible
placement, and a two-second output-stall deadline. One input demultiplexer owns
raw bytes and filters fragmented, malformed and late recognized APC/CSI replies
even after returning to text browsing. It retains at most 64 KiB response bytes.
It preserves ordinary UTF-8 and keyboard sequences. No global image deletion is
sent. Output failures are reported; unwritten cleanup is never certified.
Text redraw uses home plus ED0 (`ESC[H ESC[0J`), preserving uploaded/unrelated
images. ED2/ED3 erase graphics too and must not occur between upload and placement
or during viewer/browser redraw. Placement failures include the sanitized terminal
reason. The [Ghostty placement correction](../delivery/evidence/terminal-kitty-graph/20261105-placement-fix/README.md)
records the original failure and stateful terminal regression.
Unterminated control fragments have a one-second deadline; expired recognized
response bytes are discarded, and Ctrl-C remains available inside a broken APC.

Every viewer interaction (including help, source selection and resize) hides the
old image/details, checks current host authority, then reveals only the current
epoch/frame after another check. Denial/disconnection drops graph and inspector
caches. There is no instantaneous out-of-band revocation claim for an idle
terminal. Close, cancellation, handled SIGINT/SIGTERM/SIGHUP and suspend/resume
release owned images, worker and listeners and restore raw/cursor state where
output remains writable. Uncatchable process termination cannot guarantee cleanup.

The default extraction is depth 1, 60 vertices and 120 edges; explicit hard maxima
are depth 8, 200 vertices and 500 edges. Existing lower native work/byte bounds
still apply. Display labels are visibly shortened (160 code points at most,
three drawing lines); the inspector preserves full safe text. Worker input and
SVG each have a 2 MiB cap; viewport dimensions are at most 4096 each and 4,194,304
pixels total (16 MiB RGBA). PNG is at most 8 MiB and the encoded frame at most
12 MiB. One worker request and one latest coalesced intent are retained, with
a five-second layout/raster deadline and epoch cancellation. Windows smaller
than 90 columns × 24 rows clear graphics and show a size notice, retaining the
exact selection. Viewport clipping is presentation, separately labelled from
partial extraction, unresolved endpoints, unsupported projections and warnings.

Complete requested extraction does not establish model completeness, execution
or acceptance. FruitTrucks retains its 27 gaps, source/authority conflict and
proposed expiry. The Route/ChangeOffer/Acknowledgement structured fields remain
empty; narrative `route_id` and similar text generate no schema edges. Context
grouping uses recorded membership only. Journey edges use declared StoryMap list
occurrences; lifecycle edges use declared transition endpoints, preserving guards
and proposed-design qualifications in the full inspector/source bindings.

Exit meanings remain 0 normal complete close, 3 partial/render bound, 2 usage,
4 denied, 5 missing/ambiguous, 6 graphics unavailable or asset/protocol/render/
output failure, and 130 cancellation. Unsupported terminals receive text/HTML
alternatives, such as `kh browse` or an explicit `kh diagram REF --format html
--output NEW.html`. Browser launches return to a currently permitted text state.

### Renderer maintenance and packaged checkpoint

Pinned inputs are Dagre 3.1.3 (graphlib 4.0.6), resvg-WASM 2.6.2 and unmodified
DejaVu Sans 2.37. `clients/terminal/assets/graphics/inventory.json` records exact
versions, upstream revisions, npm integrity and SHA-256 asset hashes. That folder
retains full licenses, font authors and corresponding source locations; the
package copies it as `graphics-notices/`. `bun.lock` fixes installation inputs.
`bun clients/terminal/scripts/graphics-assets.ts` verifies asset/package identity.
No Rust/toolchain, host method, DTO version, ontology or fixture change is needed.

The worker is an explicit additional compile entry, with file imports for WASM
and font bytes. Bun 1.4.0 requires the fixed compile root below; compiled worker
resolution uses `src/graph-render-worker.js` within the embedded filesystem.
Initialization is lazy and asset/worker failures are typed `GraphicsAsset`,
confined to explicit graphics use. No system fonts or external resource resolvers
are used. SVG contains generated primitives and escaped text only.

```sh
bun build --compile --root clients/terminal \
  clients/terminal/scripts/graphics-checkpoint.ts \
  clients/terminal/src/graph-render-worker.ts --outfile '/tmp/kh checkpoint'
'/tmp/kh checkpoint' /tmp/checkpoint.png
bun clients/terminal/scripts/package.ts NEW-DESTINATION --models QUIESCENT-MODELS-DIR
```

The labelled-PNG/cancellation checkpoint ran on installed Bun 1.4.0 from a
relocated path with spaces, empty tool PATH, denied source-checkout reads and
denied network. This proves that build mechanism, separately from terminal pixel
display. `--models` copies already quiescent current-contract teaching fixtures;
it does not reauthor plans or source definitions. The package still has two
executables, with the worker/font/WASM embedded in `kh`. Build inputs, dependency
notices, exact fixture handovers and checksums remain inspectable. Actual Kitty
and Ghostty observations and human meaning/usability review retain separate gates.
See the dated [TC-SL-02 implementation evidence](../delivery/evidence/terminal-kitty-graph/20261105-apply/README.md)
for exact inputs, automated checks and the remaining native-terminal observations.
