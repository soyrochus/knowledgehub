# MM-SL-01 implementation and verification

3 November 2026. The user authorised applying the open changes. C1 revises
experience contracts; C2 supplies exact/inherited logical and draft contracts;
C3 supplies the standalone authoring host; C4 supplies portable skills and local
source-bound reference models. OpenSpec task files are the completion ledger.
Nothing is archived or published. Modelarium and the protected canonical stores
are outside this implementation.

The final exact-source crate batch passed: 115 Rust target tests, three doctests,
19 Python host checks, formatting, warnings-denied Clippy and release examples.
native-final/checked.json binds its sources, commands, logs and executables. Earlier failed all-targets, rent and fixture logs are
retained as failures, not counted as passes. The stricter inherited checks found
an old rent fixture borrowing another schema's fields and an arbitrary gap path;
the fixture now retains its DDL without borrowed field ownership and uses a real
omitted requirement statement for its draft-gap exercise.

## Native contracts and tools

Current cumulative identity: catalogue-3, record/3, request/3, files/4 layout;
unpublished crate 0.10.0. There is no old-codec reader, alias, converter or migration.
The catalogue has 146 types (17 kernel, 129 family), 40 signatures, 183 original
source rows and separately bound E01–E03 refinements. The [19-criterion matrix](criteria.md)
separates native/standalone proof from future Modelarium work.

Focused contract results are in contracts-final.log, final-neighbours.log and
inherited-schema-current.log. authoring-complete.log records seven successful
subprocess integration cases: actual capture/publication/readback, complete-native
but missing source coverage, current-authority recovery, all model-dispatch
checkpoints, actual process cancellation, changed intent/input, configured limits
and explicit nonexhausted query failures. authoring-paths-current.log adds the
final output-parent traversal/symlink cases. Required checks use existing Rust
1.99.1. platform.json records actual macOS arm64/local writable APFS observations;
this is separate from the user's platform-compliance assumption. Peak RSS was
not measured for these replay commands. Current Linux and both cross-host copy
gates remain unchecked in support-macos-file-backend.

## Source review and actual models

The final maintained inputs are examples/fixtures/memento and unrelated-design.
source-identity-review.json independently checks all 5 contexts, 13 journeys,
36 US/R pairs, 72 scenarios, 26 DR, 11 FA, 30 AC, 6 templates, P1–P5 cuts and
5 working surfaces against exact source identities/types and baseline obligations.
The reference additionally accounts for 530 UI source constructs. Exact originals
and locators retain callbacks, conditions, confirmations, permissions, failures
and styling beyond the bounded typed bindings. That is not an executable graph
or a claim of complete structured formalization. Missing linked context remains
explicit in coverage.source_gaps. Research, historical illustrations and scripted
prototype sources do not establish actual research, implementation or acceptance.

Native run directories:

- target/rich-design/apply-memento-acceptance
- target/rich-design/apply-equipment-acceptance

Each directory retains frozen executable/digest, config/current host inputs,
commands' stdout/stderr, private run, native Hub, named imported-A baseline,
original model/registration requests and receipts. The maintained run.py command
was executed with the release binary. Read each generation-manifest.json for
exact directory/scope/head/baseline/catalogue/profile/key map and original receipts.
The *-baseline-A.json reports here preserve the first readback; successor.json
in each run records coherent ordinary API evolution to native-B and exact A
sources/subjects/states/bindings/receipts after reopening. The post-successor
*-verified.json reports and run-summary.json record final readback and sizes.

The reference is valid and incomplete, with 1,504 explicit native structured
omissions. Equipment has 10 omissions. Passing source checks means the declared
obligation inventory is accounted for; it does not prove adequate business meaning.
Source-bound narrative/Body/artifacts preserve detail beyond that formalization.
Human meaning acceptance remains pending. No imported script/rule entrypoint,
application timer, provider action or target workflow is executed; no application
TestRun, implementation success or human approval is inferred.

Deliberate negative runs negative-requirement-02 and negative-ui-02 retain the
same model record totals and valid native assessment while respectively replacing
R-32 and interaction:execute. Their unchanged independent expectations fail on
those exact obligations/queries. Their 1,504 draft gaps are the same completeness
boundary as the corresponding positive 1,616-record snapshot; the small CLI test
also proves coverage failure with a complete, zero-gap native model. Retained
negative-*-current.json reports use the final verifier and are intentionally
incomplete. They are earlier reference snapshots, not the later expanded UI
locator inventory.

## Agent hosts and handover

agent-hosts/ retains exact commands, raw output, version/result and native
verification/generation artifacts. Codex CLI 0.166.0 and GitHub Copilot CLI
1.0.105 each performed actual skill-driven import and a separate verify process.
These runs reviewed/replayed a maintained equipment plan; they did not author it
from scratch. Their original native workspaces are recorded in host-result.json.
Initial sandbox initialization failures are separate from the permitted retries.
No broad permission-bypass mode was used. Claude Code 2.1.316 reported 'Not logged
in'; its import/verify gate is unexecuted, not passed. Task C4 4.1 remains open.

forward-test/ retains an independent Codex tool-session experiment that received
only the unrelated raw source and maintained skills, authored its own plan and
coverage, corrected a native Set-ordering failure, then created, reconciled and
independently verified 10 obligations/12 queries across 24 pages. It is newly
authored evidence, distinct from the CLI replay modes and from human review.

A recipient can inspect the native generation manifest and run the documented
verify command with current host authority, without a Modelarium checkout or the
original agent conversation. The supplied business design's human adequacy and
application acceptance remain pending gates. Later Modelarium registration is
MM-SL-02, not delivered here.

preservation.json compares all 380 hub files to the hash-only observation made
during this implementation before final checks; old_hub was absent. No protected
store decoder was invoked. Source originals/Cargo metadata are unchanged. The
33 copied input files total 3,412,806 bytes and match source-origin.json exactly;
excluded Finder metadata has explicit dispositions. No source application was
edited. Evidence source/executable/log hashes are retained by the final native
batch and run-summary.json. Later task-checkbox updates do not refresh a canonical
model or transfer definition authority.

Final OpenSpec status: C1 9/9, C2 15/15, C3 15/15, C4 13/14. C4 4.1 remains open for the unauthenticated Claude host. KH-SL-018 remains 11/14 for its deferred Linux/cross-host/two-platform gates. Human meaning acceptance is pending separately. After the exact-source batch, only task checkboxes and the typed API guide's stale depth limit were corrected (32, matching the already-tested implementation); no compiled source changed. See post-check-updates.json.
