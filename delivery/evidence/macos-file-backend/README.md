# Native macOS file-backend work

Mac implementation and runtime verification completed on 2 November 2026 using
installed Rust 1.99.1, macOS 26.6.2 build 25G83, arm64 and writable local,
case-insensitive APFS on `/dev/disk3s5` (`/System/Volumes/Data`).

The user authorised application, then explicitly directed that Linux verification
be accepted as given for now and performed separately on another machine. This
assumption is not a native Linux result. OpenSpec is **11/14 verified tasks**:
5.2 (Linux), 5.3 (both native fixture directions) and the two-platform closure
portion of 5.4 remain open. The observed Mac outcome is delivered; full technical
cross-platform closure is not claimed.

| Contract | Observed Mac result | Linux / cross-host boundary |
| --- | --- | --- |
| Native persistence/coordination feasibility | Full-file flush, directory fsync, link/rename ordering and final fence passed; independent descriptors/children, busy locks, exclusive creation and death release passed. | Native Linux regression assumed at user direction. |
| Declared profile and creation provenance | Actual APFS admission; wrong declarations/symlinks refused. Nonlocal/readonly/name/device negatives are explicitly injected observations. Both recognised origin labels retain exact metadata/history in synthetic parser tests. | Actual ext4 branch retained in code; native cross-host origin proof deferred. |
| Full safety matrix | 105 Rust test executions (including repeated example tests), 19 Python host tests, doctest command, formatting, warnings-denied Clippy and release examples passed. All 20 guarantee groups observed. | No Mac log is labelled Linux evidence. |
| Publication / maintenance recovery | All eight publication and seven maintenance SIGKILL checkpoints, injected sync/fence failures, cancellation/drain, independent writers/read leases and both Tokio runtime configurations passed. | Process faults and injected errors only; no power-loss/kernel-crash/hardware claim. |
| Runtime journeys | Minimal, full catalogue/specimens, A/B/source/range/retry, neighbours, rent-impact, benchmarks and isolated self-description author/inspect/compact/scrub/reopen passed. Obsolete format flags refused. | Both closed-fixture native copy directions remain unverified. |
| Definitions and retained data | Source-origin validation and coverage audit passed; 380 canonical files and 232 historical evidence files retain exact bytes. `old_hub` was absent before/after, never created or decoded. | Repository ownership remains; no canonical refresh or authority handover. |

The [native checked manifest](macos-20261102-01/checked.json) binds 202 source files,
28 test/release executables, command results, compiler/platform facts, logs and the
extended guarantee map. Its [native log](macos-20261102-01/native.log) and separate
build logs retain actual executions. [Feasibility evidence](feasibility/README.md)
records the protocol prerequisite, including installed standard-library source.

The [isolated walkthrough manifest](macos-20261102-01/walkthrough.json) and
[command receipts](macos-20261102-01/commands.json) bind successful runtime results
and expected obsolete-argument failures. The
[coverage audit](macos-20261102-01/coverage-audit.json) reconciles the complete
catalogue/specimens and current source origins. The isolated self-model contains
9,883 selected records, 9,889 retained revisions, eight generations and three
baselines. Its 3,557 explicit draft gaps remain; structural validity does not
establish purpose completeness or human acceptance.

[Before compaction](macos-20261102-01/inspect-before-compact.json),
[after compaction](macos-20261102-01/inspect-compacted.json) and
[fresh reopen](macos-20261102-01/inspect-reopened.json) preserve the exact head,
A/B narratives, original receipts and neighbour/text/composite query results.
Full scrub retains all 24,272 objects and eight generations. Compaction installs
one verified pack before removing 24,272 loose files. Observed allocated regular
file bytes change from 132,407,296 to 43,880,448; this is an APFS measurement, not
a cross-filesystem allocation promise. Derived cache quotas remain 256/128 MiB.

The small storage benchmark reports 57,081,856 peak resident bytes; the full
neighbour benchmark reports 184,303,616. Darwin `getrusage` reports bytes. Linux
procfs KiB parsing and unavailable/error paths have focused host tests; the Linux
platform-observation branch is tested with supplied observations, not a native
Linux execution. Required missing platform facts produce no successful manifest.

[Retained before](feasibility/retained-before.json) and
[retained after](macos-20261102-01/retained-after.json) match without using a store
decoder. Fixtures remain isolated under `target/macos-file-backend-20261102-01/`.
Original `source/` books, package identity/dependencies and unrelated `hub.zip`
were not edited. No archive, crate publication, human acceptance or authority
transfer occurred.

[Final report/source relation](macos-20261102-01/report-relation.json) records
the status/report-only edits after the checked checkpoint. Executable code and
checked historical sources remain exact; the captured self-model is not silently
refreshed after those edits. [Native profile guidance](../../../design/macos-file-backend.md)
owns the mechanism description and supported failure boundary.
