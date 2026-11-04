# KnowledgeHub terminal client · Session 4

This local macOS arm64 package contains `kh`, its matching native host, an offline
Mermaid renderer and two small native draft models. It needs no Bun, Node, Rust,
Cargo, source checkout, network service or learner prework. Keep the executable
pair together. Extract the archive to an ordinary local APFS directory; paths
with spaces are supported. The local build is unsigned and not notarized.

The FruitTrucks process and proposed design are teaching material. R8 requires
acknowledgement; Q1 is instruction delivery within sixty seconds, not an
acknowledgement deadline. Five-minute expiry is a proposed teaching choice with
production policy open. Nothing here executes deliveries, timers or stored code.
Equipment is an unrelated design with a registered extension hierarchy.

## Instructor walkthrough (15–20 minutes)

Run commands in the extracted directory. In another cwd use the absolute path to
`kh`; its default workspace stays relative to the executable. `models/` contains
relative reader-B workspaces, exact source/coverage inputs and `handover.json`
with publisher A's original receipts, heads and A/B/superseded baseline references.

1. **0–3 minutes:** `./kh --help`, `./kh models`, `./kh info --json`.
   Retain `selection.reference` and `publication_anchor` from the info envelope.
   `./kh browse` opens the keyboard browser; `?` shows all controls.
2. **3–7 minutes:** press `f`, type `=Requirement`, Enter, Space. Press `f`, type
   `R8`, Enter, then Enter to inspect. `l` lists recorded links; filter
   `satisfies` and Enter follows the actual UseCase. `s` lists exact sources;
   select an Anchor and `z` previews the retained text. `h`/`H` restore exact
   navigation state. `t`, `c`, `J`, `u` open type/context/journey/source groups.
   These reference groupings do not invent containment or missing memberships.
3. **7–10 minutes:** `p` lists accepted native publications, `n` pages when
   available, Enter shows an original header and `a` opens its affected sequence.
   `o` followed by `authoring:model` inspects A's original publication.
   From its affected rows, Enter opens a definition at the original header's
   snapshot. Reader B is distinct from publisher A. `b` shows selected/handover
   baselines; `D` explicitly requests bounded retained discovery.
4. **10–13 minutes:** quit with `q` and repeat the pinned read:

   ```sh
   ./kh --snapshot SNAPSHOT --history-anchor ANCHOR show model:requirement:ack --json
   ./kh --history-anchor ANCHOR publication authoring:model --affected --json
   ./kh --history-anchor ANCHOR publications --history-items 1 --json
   ./kh baselines --retained --json
   ./kh --workspace models/workspace-no-history.json types --json
   ./kh --workspace models/workspace-no-history.json publications --json
   ```

   Replace SNAPSHOT/ANCHOR with the exact values retained in step 1. The last
   command exits 4 with `Denied / ReadHistory`; ordinary reads remain available.
   `--items 1` demonstrates a partial consumer result (exit 3). It preserves
   exact intent, not a dead process token. `--history-nodes 1` demonstrates an
   explicit native limit even when zero items fit. No hidden history repair runs.
5. **13–16 minutes:** copy exact baseline JSON objects from `baselines` or the
   handover. `./kh --baseline 'BASELINE_JSON' show model:requirement:ack --json`
   reads A; `./kh compare 'A_JSON' 'B_JSON' --json` invokes the existing unpaged
   native comparison. Baseline definition predecessors, baseline parents,
   publication anchors and operational plan revisions have different meanings.
6. **16–20 minutes:** export and open a local artifact in a browser:

   ```sh
   ./kh diagram model:requirement:ack --output acknowledgement.html --format html
   ./kh diagram model:lifecycle:offer --projection lifecycle --output offer.mmd --format mermaid
   ./kh diagram model:journey:change --projection journey --output journey.json --format json
   ./kh --model equipment type SpecialDefinition --json
   ./kh --model equipment publications --json
   ```

   The HTML is self-contained and works offline. Select a node and an edge to
   inspect exact references, property paths, original owners and source bindings.
   Zoom with buttons and drag the background to pan. A paired manifest records
   selection, projection, coverage and digests. The TUI `v` export uses the same
   extraction function. `x` exports exact original source bytes with a full hash
   check; existing files require explicit `--replace` in the CLI.

## Boundaries and recovery

`./kh COMMAND --help` lists grammar, literal search coverage, finite limits and
exit meanings. `--json` emits one `knowledgehub-exploration/1` envelope on stdout;
diagnostics never masquerade as an empty successful result. A baseline is never
converted into a generation anchor. Compare has explicit baseline operands and
no native snapshot-anchor argument. Selected and handover baseline listing is
not a retained catalogue; `--retained` needs ReadHistory and truthful completion.

`r` explicitly refreshes content and publication anchor to current head; `h`
restores the previous exact state. `R` reconnects using the retained selection
and anchor. Invalid or expired tokens visibly restart the pinned first page.
No disconnected retry silently switches to latest. Esc cancels the caller;
already admitted native work drains before close. `NO_COLOR=1 ./kh browse`
uses the same keyboard controls; resizing retains state.

Do not put exports inside a Hub. Paths through symlinks and occupied outputs
are refused. Native opening/reads may recover ordinary backend journals or build
derived navigation indexes. The host exposes no semantic writes, validation,
registration or actor-bound operation recovery. Preserve stores and original
receipts; use fresh copies for workshops. Quiescent copying occurs only after
all native writers and hosts drain. Do not copy a live writer's store.

If macOS blocks the unsigned executable, use your organization's approved local
software process. No signing, notarization or publication is claimed. Linux
native execution and other filesystem profiles remain unverified; this package
does not close existing cross-host or external analyst/host acceptance gates.
Automated PTY/browser checks are technical evidence; instructor observation is
separate from human usability or meaning acceptance, which remains pending.
