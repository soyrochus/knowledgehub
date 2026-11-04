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
