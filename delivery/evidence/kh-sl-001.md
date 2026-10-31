# KH-SL-001 technical evidence and review

31 October 2026 · `retain-first-self-definition` · Technical checks passed;
human acceptance pending. This is evidence for the first bounded slice, not
completion of the ontology proposal's full implementation programme.

The user authorised this implementation, then explicitly required existing
Rust 1.99.1, no compiler installation and assumed platform compliance. No
compiler was installed. Rust 1.85 remains the declared target; execution on it
is assumed and is not claimed below. Original files under `source/` are unchanged.

## Reproducible run

[Raw output and SHA-256 inventory](kh-sl-001-run.txt) retain the compiler build,
Cargo version, kernel, ext4 mount/options, commands, observed results and hashes
of every checked Rust source, fixture, test, manifest, lockfile and source input.
All commands exited zero:

- `cargo fmt --all -- --check`
- `cargo clippy --offline --all-targets -- -D warnings`
- `cargo test --offline --all-targets -- --nocapture`: 8 unit/harness tests
  and 12 public-contract tests passed.
- `cargo test --offline --doc`: the README public consumer compiled.
- `cargo run --offline --example seed -- <new absolute ext4 path>`: ordinary
  public create → validate → commit → exact reads → drain → reopen → original
  lookup → exact retry completed.
- `openspec validate retain-first-self-definition --strict`: planning format
  validation passed, independently of the library checks.

Observed host: Linux x86_64, kernel `7.0.0-38-generic`, local ext4 on
`/dev/nvme0n1p2`, `rw,nosuid,nodev,relatime`; existing rustc 1.99.1
`58cf8f9c3`, Cargo 1.99.1. All file tests use isolated directories under
ignored `target/kh-tests` on that ext4 mount, not tmpfs `/tmp`.

The public walkthrough's retained store is
`target/kh-walkthrough-20261031-3`. Its original accepted seed snapshot is
`34fcb0872326608f313b334f9c54d8a38e6200789686b55961bff3b2be320aae`.
Reconstruct independently by rerunning [the example](../../examples/seed.rs)
against another new absolute ext4 path; commit IDs are intentionally fresh.

The captured ontology is 74,064 original bytes, SHA-256
`30cc6f283e2b3f5cea375ff3990c97e4716c918d239ceee1ad331cc57ac06620`.
The Markdown seed is 533 bytes, SHA-256
`906c83982cc7d969d7dadbcfe98309bc962da85262d35c045f8ec34be7237ae3`.
The raw output records both exact element revisions, the assertion and endpoint
revisions, document revision and original operation fingerprint. Assertions
compare complete returned snapshots and receipts before and after reopen;
the checks do not rely only on printed summaries.

## Coverage against ontology §21.10

| Area / capability | Executed evidence and observed outcome |
| --- | --- |
| Normal use; source-bound-definitions | Public walkthrough on both current-thread and two-worker Tokio runtimes. Empty genesis remains explicitly incomplete. Seed validation leaves genesis unchanged; one commit resolves exact Markdown, original ontology/§21.4 anchor, typed definitions, assertion, authored interpretation and exact vocabulary. Close/reopen preserves complete values. |
| Denied changes; branch-commits | Default deny prevents creation/disclosure; policy evaluator error fails closed. Revocation after validation prevents commit; a second admission check runs under the lock. Revoked read/retry/result access is denied. No model record confers permission. |
| Invalid changes; source-bound-definitions | Twelve independent invalid candidates: wrong byte digest/range, missing anchors/source revision, unsupported type, wrong endpoint type, unresolved endpoint revision, missing actor owner, competing owners, missing interpretation, unknown normative property, invalid fragment. All preserve the accepted head and establish no committed operation key. |
| Concurrency; branch-commits | Exclusive concurrent scope creation. Separate same-process instances and two processes race from one base: exactly one valid new head. Explicit Busy while a process holds publication; later loser admission returns stale Conflict. Exact revision and absence guards checked. |
| Replay; branch-commits | Original receipt survives five later commits and reopen; exact retry returns that receipt with no new head. Equivalent differently ordered/formatted JSON matches canonical intent. Changed Markdown byte, source byte, operation order and actor refuse reuse. Other actor receives no receipt. Strict input rejects duplicate/unknown keys and unsupported encodings; canonical vectors include UTF-16 key ordering and safe integer limits. |
| Recovery; file-snapshot-recovery | SIGKILL a real child at staging, an individual content file, complete content sync, generation install, temporary-reference sync, head replacement, head-directory sync and acknowledgement. Before replacement: complete prior genesis and authoritative absence. After replacement: complete new seed and original receipt, with exact retry producing no second commit. No staging/orphan promotion. |
| Persistence errors; file-snapshot-recovery | Test-only injected errors at the same eight checkpoints: pre-replacement Storage preserves prior acceptance; post-replacement Uncertain resolves the original committed result after recovery. This includes the boundary immediately before reference-directory sync. Hooks are absent from production builds. |
| Cancellation/capacity; Rust boundary | Full active/waiting capacities refuse submission before mutation. Aborting a caller preserves admission/data/branch lock while publication continues; draining waits for it, then original-result lookup succeeds. Closed admission refuses more work. Calling without a host runtime reports Unavailable and creates none. |
| Integrity/compatibility; file-snapshot-recovery | Unsupported format/profile/writer declaration and detected tmpfs are refused. Existing corrupted/missing accepted graph, raw source or old history produces Integrity, never fresh empty content, fallback or authoritative absence. |
| Snapshot consistency | A pinned genesis stays empty after seed publication and later commits; the seed snapshot retains its exact original values after subsequent head movement and reopening. Every exact read selects accepted immutable history under current policy. |
| Deferred §21.10 areas | Named baseline creation/comparison, broader filters/order/cursors/traversal, malformed bundle import and portable export/import are later slices. This change provides exact accepted snapshot selection and fixed-profile admission only; it claims no completed interchange programme or database parity. |

## Findings and limits

The selected complete path works through owned public contracts. The examples
support keeping definition meaning, source interpretation, current admission
authority and original operation outcome separate. No contradiction required
changing the owning domain rules. Scope remains one branch in one store with
the fixed seed profile; additional commits add immutable records, while successor
authoring remains deferred.

Recovery and exact reads currently scan complete accepted history under the
branch lock. Generation preparation is also serialized. This is a correctness
choice for the small model with explicit performance limits; large-model
performance has not been characterised. Receipt/history retention has no expiry,
pruning or automatic reclamation. Unaccepted staging can remain for inspection.

The mechanism evidence uses cooperative library writers, normal local ext4
behaviour, process SIGKILL and injected failures at named boundaries. It does
not demonstrate power-loss, kernel-crash, hardware/media durability, arbitrary
external modification, other hosts/filesystems, multi-host writes or actual
MongoDB/PostgreSQL parity. Mount/format observations cannot prove hardware
durability. See [the implemented support contract](../../design/file-backend.md).

Human acceptance has not been recorded. The technical review here assesses
the first slice against its behavioural deltas; it does not approve the broader
ontology, every domain hypothesis or a self-model authority transfer. The
OpenSpec change remains unarchived for project review. KH-SL-002 remains next
for progressive specification of successor authoring and exact named baselines;
later implementation has not been authorised. Repository documents retain their
existing definition ownership, with KH-SL-006 handover still pending.
