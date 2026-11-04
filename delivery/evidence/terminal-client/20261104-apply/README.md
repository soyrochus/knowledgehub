# TC-SL-01 · terminal client implementation evidence

4 November 2026. `add-terminal-model-client` consumes the separately delivered C8
native-history API. This report records the new host/client/fixture/distribution
checks; it does not replace C8 evidence or establish human acceptance.

The delivered boundary is the binary-local read-only Rust host, shared TypeScript
explorer, `kh` CLI/TUI, exact source/lineage/publication/baseline inspection,
derived JSON/Mermaid/offline HTML, two prepared native drafts and a local macOS
arm64/APFS package. Public library exports, codecs and Modelarium are unchanged.

## Build and fixture identities

[summary.json](summary.json) records the exact archive and executable digests.
[build-manifest.json](build-manifest.json) records source input hashes, installed
Rust 1.99.1/Bun 1.4.0, target, protocol, codecs, catalogue, native-history projection,
supported methods, permission mapping and bounds. [package-checksums.json](package-checksums.json)
covers the 316 packaged files. The retained tar archive is the quiescent master;
the tests use extracted writable copies. Native read/index caches may change in
those copies; semantic objects, refs, sources and original publications do not.

| Model | Exact final publication anchor | Original publisher / reader | Native A status |
| --- | --- | --- | --- |
| FruitTrucks | `697245f70c80cf0a1deed87d103c71c445c2264645d8c9565af1ed41af341463` | `host:publisher-A` / `host:reader-B` | Valid draft; 27 native gaps; authority/source verification incomplete. |
| Equipment | `89cebaf6618d59df958dbbf4aee68f1ce34d06ec3b4d4cf490c4168aef6bee3c` | `host:publisher-A` / `host:reader-B` | Valid draft; 10 native gaps; source verification verified. |

The [FruitTrucks handover](fruittrucks-handover.json) and [equipment handover](equipment-handover.json)
retain all original fingerprints, previous/resulting heads, sequences, exact
affected vectors, source identities and three exact baseline references. Each has
`authoring:register`, `authoring:model`, `fixture:ordinary-native-successor` and
`terminal:baseline-definition-successor`. A's baseline definition revision 1 is
retained after revision 2 supersedes it; B's parent remains exact A revision 1.
Native compaction and reopen precede B's receipt/source equality checks.

The [FruitTrucks verification](fruittrucks-verification.json) deliberately reports
`status: incomplete`, `source_coverage: false`, `native_validity: true` and
`native_completeness: false`. Its only verification failure is the retained
manual/interview authority conflict. R8 acknowledgement, Q1 instruction delivery,
proposed five-minute expiry and open production policy remain distinct. Exact
bytes and both expected native queries pass. This is not complete source-accounting
or accepted business meaning. The [equipment verification](equipment-verification.json)
has complete source accounting but still has explicit structured omissions.
Neither model executes deliveries, timers, stored rules or an application.

## Executed checks

| Check | Evidence / outcome |
| --- | --- |
| Rust regressions | `cargo test --lib --tests`: 121 target tests passed, including the five host tests. [Log](rust-regressions.log). |
| Final host checks | Five focused tests passed after final framing/permission/work-accounting changes. [Log](host-tests-final.log). |
| Documentation examples | Four doctests passed. [Log](rust-doctests.log). |
| Static checks | TypeScript no-emit, Prettier, Rust formatting, all-target Clippy with warnings denied, whitespace and strict OpenSpec passed. [Clippy](clippy-final.log), [OpenSpec](openspec-final.json). |
| Bun unit/native integration | Ten tests passed: precision/framing, finite pending work, cancellation/deadline/EOF, cursor lifetime, 64-call/no-progress cleanup, ambiguity, schemas/subtypes, native occurrences/cycles, literal narrative spans, sources, original receipts, retained baselines, live revocation and exports. [Log](bun-acceptance-final.log). |
| Compiled package | 57 commands passed from a relocated path containing spaces, another cwd and an empty tool PATH. [Summary](package.json), [full results](commands.json). All package checksums matched before use; semantic file hashes matched after exploration. |
| Strong isolation | Source checkouts, Bun/Node/Rust/Cargo execution and network access were denied to the end-user processes by macOS sandbox policy. Help, both models, exact history/affected reads, denial and offline export still passed. [Record](isolation.json). |
| Real compiled PTY | Source/history/export path, empty and failed admissions, native paging, original-snapshot read, back/forward/refresh, loss/reconnect, unknown operation versus unavailable anchor, cancellation, no-colour and resize/exit passed. [Summary](pty.json), [transcript](compiled-browse.ansi). |
| Current authority | Live ReadHistory removal cleared cached rows, including j/k navigation, while ordinary type browsing remained available. [Transcript](live-history-revocation.ansi). Authorization also gates filtering, help, prompt return and resize redraws. |
| Same CLI/TUI scope | Keyboard export and the equivalent exact-snapshot/anchor CLI export produced the same graph digest. The PTY record identifies the original A snapshot separately from the later publication anchor. |
| Offline browser | Actual Chromium rendered with zero network attempts; node and edge inspectors matched exact identities/origins; zoom worked. [Record](browser.json), [visual review](preview.png). |
| Workbench example | Executable graph consumption retained selection, anchor and coverage, then rendered the seven-node/five-edge journey offline. Composed edges retain both native property occurrences and their projection rule. [Record](workbench.json), [browser](workbench-browser.json), [visual review](workbench-preview.png). |

Native history work and full wire/output byte counts remain separate. Native
NotPublishedAt, HistoryAnchorUnavailable, zero-progress HistoryLimit, partial
discovery, denial and operational failure have distinct outcomes. One-shot
responses contain reproducible pinned intent instead of dead process tokens.

The observed instructor-style browse/source/history/export path is an automated
agent-operated technical walkthrough. It is not an independent class rehearsal
or human usability/meaning acceptance. The 15–20-minute instructor quick start is
supplied; participants need no setup for an instructor demonstration.

## Scope review and open gates

Both protected stores were compared byte-for-byte with the pre-apply inventory:
380 retained `hub/` files and the empty `old_hub/` inventory are unchanged. No old
store was decoded, imported or reset. No canonical refresh, library/API/codec
change, Modelarium change, archive or publication was performed.

Human meaning/usability acceptance, resolution of the FruitTrucks authority
conflict, Linux native execution, signing/notarization/publication, external
authoring-host gates and Modelarium C9 remain open under their own scopes. The
local Mac package and document checks do not close them.
