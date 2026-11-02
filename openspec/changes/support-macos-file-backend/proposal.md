# Proposal

Application authorised on 2 November 2026. The Mac implementation and complete native
verification passed; see [evidence](../../../delivery/evidence/macos-file-backend/README.md).
Linux verification is assumed at the user's direction pending a separate machine;
native copy directions and final cross-platform closure remain open.

## Why

KnowledgeHub 0.10.0 compiles on macOS with Rust 1.99.1, but ordinary Hub creation/opening is refused by its Linux-only filesystem profile. A maintainer needs the complete persistence journey to run locally on the current Apple Silicon Mac, with the same history, atomicity and recovery guarantees as the existing Linux implementation.

## What Changes

- Extend the existing `FileBackend::new` with an explicitly checked macOS arm64/local APFS profile; retain Linux x86_64/local ext4 and the storage-independent full `KnowledgeHub` API. No separate public Mac backend or store-format selector.
- Define native file/directory persistence and locking under the existing guarded commit, original-outcome and uncertainty contracts. Compile-only success cannot establish runtime support.
- Keep files/4 and its record/request/object codecs. Recognise Linux and Mac creation-profile metadata separately from the currently observed host profile. Verify complete, quiescent synthetic fixture copies in both directions without conversion, identity changes or metadata rewriting.
- Make maintained examples, fixtures, source bindings, diagnostics and native-verification scripts operate on the declared native profile. Remove runtime assumptions about `/proc` and `findmnt` on the Mac path while preserving Linux observations.
- Require native Mac/APFS and Linux/ext4 full safety evidence, including publication and maintenance process faults. Preserve current canonical/retained stores and historical evidence.

## Capabilities

### New Capabilities

None. Platform support extends existing file persistence and host-tooling behaviour.

### Modified Capabilities

- `file-snapshot-recovery`: checked APFS/ext4 support, native persistence/coordination and platform-specific runtime evidence under the existing recovery contract.
- `shared-file-storage`: one current files/4 backend across supported hosts, explicit creation-profile metadata and lossless offline fixture continuity.
- `current-api-host-tooling`: native profile selection and truthful platform-aware verification/diagnostics in maintained commands.

## Impact

Proposed [KH-SL-018](../../../delivery/slice-plan.md#kh-sl-018--run-the-current-file-backend-natively-on-macos), KH-J-11 and KH-US-31–33. Actor: Mac maintainer/host application. Hard prerequisites: implemented KH-SL-014–017, current full-model safety coverage and access to both native verification profiles. Complete only when the full Mac create/commit/query/reopen/retry/compact/scrub journey and native recovery cases pass, Linux remains verified, and exact-source evidence identifies each actual platform. Relevant domain rules are KH-DR-02/04/07–13/15/17/20.

Trace: source ontology §§5/14/17/20.4/21.2/21.4–21.7/21.10. The practical manual §§3/8–10/12–16 supplies the learning, outcome-slice and scenario method. Mac support is a newly proposed implementation profile, not a requirement attributed to the source books. Proposed KH-D-23–25 live in the [decision register](../../../design/self-hosting-plan.md#proposed-macos-file-support--2-november-2026).

Affected areas: `src/storage.rs`, shared generation/object/pack/maintenance/cache publication, native-profile composition in examples and tests, `/proc`-based benchmark metrics, `scripts/verify_full_ontology.py`, host evidence/source-binding helpers and current guides. No new driver dependency, compiler installation, API facade or format migration is planned. Crate publication/version bump is outside scope.

First Mac target: arm64/local APFS, using the available macOS 26.6.2 host for initial evidence; older OS versions, Intel Macs and other filesystems are deferred. The exact APFS directory-sync/flush sequence and volume configuration are implementation feasibility gates recorded in design/tasks; failure to establish them blocks the support claim. Power-loss, kernel-crash and hardware guarantees, network/cloud-synchronised folders, concurrent multi-host access, general interchange and canonical regeneration are excluded.

**Original proposal-preparation boundary (superseded by the later Apply request):** the user initially requested creation without application. Every implementation task remains unchecked. Preserve the pre-existing compilation repair in `src/storage.rs` and unrelated `hub.zip`; do not change Rust code, `hub/`, `old_hub/`, captured sources or historical reports while preparing this change. No human acceptance or definition-authority handover is implied.
