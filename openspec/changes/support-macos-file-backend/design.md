# Design

## Context

See [proposal](proposal.md) for the outcome and exclusions, and [KH-SL-018](../../../delivery/slice-plan.md#kh-sl-018--run-the-current-file-backend-natively-on-macos) for actor/journey/stories. This design was subsequently authorised and implemented with complete native Mac verification; [evidence](../../../delivery/evidence/macos-file-backend/README.md) records the observed scope and deferred cross-platform closure. KH-D-23–25 in the [decision register](../../../design/self-hosting-plan.md#proposed-macos-file-support--2-november-2026) own the proposed profile/metadata/persistence choices; the source books remain unchanged.

Pre-implementation inspection established:

- `src/storage.rs` admits only Linux x86_64/ext4, using Linux `statfs` magic and `/proc/self/mountinfo`. The existing uncommitted compilation repair uses compile-time target gates and returns `Incompatible` on the Mac. Preserve that repair as the starting point.
- `shared/generation.rs` separately hard-codes the Linux `filesystem_profile` during both metadata creation and validation. The same-device layout check already applies to every managed subtree. Merely removing the first platform guard cannot make a usable Mac backend.
- Ordinary persistence uses `persist`/`sync_dir`, but recovery-reference and pack-finishing paths also call `sync_all` directly. Objects/packs use hard-link installation; references/derived manifests use replacement rename. All publication and maintenance paths need the same native persistence reasoning.
- Examples, integration/unit/process-worker fixtures and one independently opened packed-cache test backend construct `local_ext4`. Derived neighbour/text stores correctly clone their parent's support declaration; preserve that propagation.
- `verify_full_ontology.py` requires `findmnt`; neighbour/storage benchmarks read `/proc/self/status`. Source capture/audit describes public contract and module identities explicitly, so adding APIs/modules also requires current semantic bindings.
- On 2 November 2026, the available host reports macOS 26.6.2 build 25G83, arm64, with Rust/Cargo 1.99.1. A normal `minimal_model` run returns the existing Linux-only incompatibility. Builds/no-run compilation pass, but no APFS protocol or recovery run has been established.

## Goals / Non-Goals

**Goals:** isolate native filesystem mechanisms inside the existing adapter; establish checked native persistence without changing model meanings; preserve existing Linux data/behaviour; make full Mac runtime evidence inspectable.

**Non-Goals:** another semantic API/backend, a format migration or compatibility facade, an automatic file repair/profile rewrite, changes to the domain aggregate boundaries, canonical regeneration or general live-store transfer. No tests may decode or import `old_hub`.

## Decisions

The delta trace is owned here so it is not lost as unmerged spec preamble. All rows implement proposed KH-SL-018; imported source statements remain definitions, not execution evidence.

| Delta / behavioural contracts | Stories and domain rules | Source / proposed decision |
| --- | --- | --- |
| [File recovery](specs/file-snapshot-recovery/spec.md): checked support, acknowledgement, full runtime journey and native recovery evidence. | KH-US-31–33; KH-DR-02/04/07–13/15/17/20. | Ontology §§20.4/21.2/21.4–21.7/21.10; KH-D-23/25. |
| [Shared storage](specs/shared-file-storage/spec.md): one format, creation/current profile distinction and exact offline fixture continuity. | KH-US-31–32; KH-DR-02/04/08–12/15/17/20. | Ontology §§5/16/21.4–21.7; KH-D-23/24. |
| [Host tooling](specs/current-api-host-tooling/spec.md): native commands/observations and isolated honest verification. | KH-US-31/33; KH-DR-02/05/07/11–13/15/18/20. | Ontology §§14/17/21.2/21.6/21.10; existing retained-store/historical-evidence rules. |

### 1. One adapter, explicit profiles and native composition

Keep `FileBackend::new` and `FileOptions::local_ext4` unchanged in role. Add `FileSupport::local_apfs` and `FileOptions::local_apfs`, declaring host `macos-aarch64`, filesystem `apfs`, coordinated single-host access. Add a fallible `FileOptions::native_local()` composition helper for maintained examples/test fixtures: select the appropriate declared profile on Linux x86_64 or macOS aarch64, and explicitly refuse elsewhere. Filesystem truth is still checked during create/open; the helper does not certify an arbitrary path.

Keep private compile-time branches for native detection and persistence. Linux retains actual ext4 confirmation from mount information; Mac uses descriptor-bound `fstatfs`, the filesystem type name and local/read-only mount flags, plus the common ordinary-directory/same-device checks. A host assertion cannot bypass observation. The host must also exclude cloud-synchronised/external-writer arrangements that filesystem type alone cannot detect. Profile construction does not expose `libc` values in ordinary model contracts.

A second public `MacBackend` would duplicate immutable object, query and guarded publication behaviour. Blind auto-detection without a host declaration would obscure the coordination assumption. The proposed helper only composes an explicit declaration and retains admission checks.

### 2. Creation metadata and current environment have different responsibilities

Preserve `knowledgehub-files/4`, its metadata shape and all current logical/physical codecs. Recognise exactly:

| Creation profile | Origin |
| --- | --- |
| `linux-x86_64/local-ext4/single-host/1` | Existing and newly created Linux stores. |
| `macos-aarch64/local-apfs/single-host/1` | Newly created Mac stores. |

The existing `filesystem_profile` field becomes explicitly documented as the checked creation profile. Store metadata parsing validates that recognised label, format, scope, replay identity, codecs and features; a separate native check validates the current environment every time. Opening never rewrites the label to match the destination host. Unknown values and mismatched current host declarations fail before store mutation.

This deliberately extends the currently Linux-only validation policy. It does not add fields or change payload meaning under an old codec. Existing Linux metadata remains exact; an older Linux binary may refuse a Mac-created profile, and no support for that binary is promised. Keeping host identity as an eternal location restriction would prevent exact current-format fixtures from being used on the other supported platform; rewriting the label would destroy its original provenance.

Use only complete, closed synthetic fixtures to verify Linux→Mac and Mac→Linux continuity. Source/destination inventories and semantic snapshots establish completeness, followed by destination reopen, original retry, append, query, compact and scrub. The source remains quiescent/unchanged. Do not add a copying utility, live backup guarantee, new replay scope or data converter. Do not copy or open user `hub/`/`old_hub/` as test fixtures.

### 3. Native persistence keeps the existing acknowledgement boundary

Introduce private, fallible regular-file sync, directory sync and publication-fence helpers. Audit every direct call in creation, recovery, loose objects, shared pack installation/cleanup and both derived cache paths. Keep Linux's existing file/directory persistence ordering and error meanings.

The proposed Mac sequence is:

1. Finish each new regular file and issue a checked full-file flush. Use the existing Rust 1.99.1 implementation where its behaviour is established, or an explicit Darwin `F_FULLFSYNC` wrapper; do not add a weaker fallback.
2. Persist namespace changes with Darwin directory `fsync`, including both source/destination directories after moves, hard-link installation and cleanup. Do not assume regular-file `sync_all` is an appropriate directory primitive.
3. Before deleting original pack encodings, finish, verify and install the replacement pack/index/catalogue; persist their directories and finish a full-file flush fence on a retained regular file on that volume. Only then admit cleanup.
4. For a commit, persist complete candidate objects and the temporary reference, replace the same-volume reference, persist its directory and finish a full-file flush fence on the published reference before the existing `head-synced`/acknowledgement checkpoints. Creation and recovery likewise finish namespace persistence before their final regular-file fence.

Apple describes full flush as draining earlier synced I/O on the same device, while a barrier alone is an ordering operation; this motivates the final fence rather than silently treating either primitive as interchangeable. [Apple XNU fcntl manual](https://raw.githubusercontent.com/apple-oss-distributions/xnu/main/bsd/man/man2/fcntl.2).

The published Rust source uses `F_FULLFSYNC` for Apple file sync, so adding another unconditional identical flush everywhere could duplicate work. Its online source is contextual evidence, not verification of the installed Rust 1.99.1 build or APFS directory behaviour. Confirm those exact mechanisms and the proposed ordering in an isolated native feasibility check before enabling APFS admission. [Rust standard-library filesystem source](https://doc.rust-lang.org/src/std/sys/fs/unix.rs.html).

This feasibility gate verifies the selected protocol, not a licence to weaken it. Required unsupported calls or failed persistence leave the profile unverified/disabled; document the blocker and revise the design before choosing another protocol. Pre-reference failure preserves the accepted head. A failure after possible replacement, including the new final fence, produces `Uncertain` with original-operation recovery. Derived-cache failures remain explicit index failures; they cannot become authoritative acceptance evidence.

### 4. Reuse stable coordination and semantic safety checks

Retain descriptor-owned nonblocking `flock`, stable lock files, shared read leases, exclusive publication/maintenance and exclusive creation. Verify separately opened same-process instances and child processes on both native filesystems. Retry only according to the existing `Busy`, expected-head conflict and actor-bound original-result contracts. Do not replace a contended lock file, poll forever or suppress failed locks.

Use existing fault workers/hooks, preserving their eight publication checkpoints and seven maintenance checkpoints; extend injected errors to native directory/full-flush boundaries. Keep current-thread/multithread Tokio, bounded jobs, cancellation and drain behaviour. Modelled roles and provenance grant no runtime authority. All I/O remains behind the adapter's bounded blocking jobs and host-owned runtime.

### 5. Port host composition and observations, not evidence claims

Replace fixed ext4 composition in maintained examples and valid-profile fixtures with the native helper; retain explicit wrong-profile/coordination negative cases. Include independently opened backends, child workers and disposable cache stores. Keep APIs storage-independent and register added public/profile/module source bindings through current audit helpers.

Choose native platform observations in `verify_full_ontology.py`: Linux keeps `findmnt`; Mac records `sw_vers`, architecture and structured local-volume/mount observations, for example `diskutil info -plist` corroborated by adapter admission. Missing required observations fail the batch. Resource measurements use documented platform-specific units, for example an audited native `getrusage` path; optional unavailable values are labelled rather than fabricated as zero. APFS allocation/compression/cloning can change disk measurements: retain logical correctness and configured capacity assertions, report actual allocated bytes, and justify any filesystem-specific performance expectation without weakening safety tests.

Run the full current all-targets tests, doctests, formatting, Clippy and host workflow checks on both native profiles, using installed Rust 1.99.1. Build release examples and execute isolated minimal/catalogue/shared-storage/query/self-description walkthroughs. Each fresh evidence directory binds one exact source checkpoint, command/environment, native profile and executable/log digests. Both platforms must test the same implementation checkpoint; metadata-only final reports must identify their relation to that checkpoint.

Updating source-origin bindings is permitted during later implementation; running the destructive canonical lifecycle is not. The verifier and documented commands must allow isolated support evidence without invoking reset. Historical v010 reports remain exact. Neither the current stored self-model nor source ownership refreshes automatically when repository files change.

## Risks / Trade-offs

- [Directory sync or full-file fences behave differently on the installed stack] → make native feasibility a prerequisite, retain failure/uncertainty mapping, and stop before admitting an unverified profile.
- [Host-profile metadata is confused with ontology profile or current location] → document separate roles and test foreign-origin/unknown-profile and wrong-location cases without metadata rewriting.
- [A Linux-only fixture or child worker remains] → inventory every composition site and execute the full native batch; a skipped test is not a substitute for its guarantee.
- [APFS performance assertions are inherited from ext4] → distinguish semantic/capacity guarantees from measured allocation and timing; retain meaningful shared-storage checks and report actual conditions.
- [A build is reported as native parity] → require both source-matched native logs, actual query/scrub results and all fault checkpoints; missing Linux access leaves completion open.
- [Verification mutates a retained demonstration] → isolate fixtures/evidence and verify host guards; keep `hub/`, `old_hub/` and historical evidence outside mutation scope.

## Migration Plan

No data migration, reset or metadata rewrite. On later implementation authorisation: establish the native protocol; implement profile/mechanism/metadata changes with focused safety checks; update maintained composition and source audit; complete both native evidence batches and closed synthetic copy checks; review the resulting support matrix. Keep the package unpublished at its current version unless separately instructed.

Rollback is a code rollback with the existing Linux behaviour retained. A pre-extension binary is not promised to open Mac-created metadata. Do not rewrite a created Mac store to masquerade as Linux for rollback, remove accepted revisions or refresh the canonical self-model. Human acceptance and authority handover remain separate decisions.

## Open Questions

- Exact observed APFS volume case-sensitivity, mount/device details and hardware assumptions are recorded during the first native run; neither an earlier-macOS matrix nor an Intel-Mac claim follows from it.
- Exact latency/allocation results are measurements, not preselected promises. They do not waive current finite budgets or shared-storage correctness.

Native feasibility and access to matching-source Linux evidence are explicit implementation/completion gates above, not deferrable support decisions.

## Application result — 2 November 2026

The feasibility gate passed before APFS admission. Installed Rust 1.99.1, explicit
Darwin regular-file F_FULLFSYNC, directory fsync, retained-file fences and stable
coordination were exercised natively. The complete Mac batch and all isolated
walkthroughs passed, including both cache namespaces, exact A/B/source/query
results, original retries and compact/scrub/reopen. No canonical/retained bytes
or historical evidence changed. Final status/report edits follow the checked
source checkpoint and do not refresh its captured self-model.

The user explicitly directed Linux verification to be accepted as given for now
and performed on a separate machine. It remains an assumption. Tasks 5.2, 5.3
and the two-platform closure portion of 5.4 remain open, including both native
copy directions. Human acceptance and authority handover remain separate.
