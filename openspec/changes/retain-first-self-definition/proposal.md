# Proposal

KH-SL-001 · Proposed 30 October 2026; implementation authorised 31 October 2026. Technical evidence recorded; human acceptance pending.

## Why

The repository contains an ontology proposal and a Cargo starter, but no working KnowledgeHub API. The first useful outcome is to retain a small sourced definition of the library itself and reopen exactly that definition after restart, establishing the complete consistency and recovery boundary before expanding the model.

## What Changes

- Introduce the first embedded `knowledgehub` library path: create/open one permitted Hub/model branch under an explicit, versioned bootstrap profile.
- Retain a small Markdown definition, an exact ontology source capture and anchors, a `Requirement`, a `UseCase`, and one identified `UseCase satisfies Requirement` assertion through ordinary public operations.
- Validate definition ownership and the seed profile's record/graph constraints; read exact definitions and provenance against one snapshot.
- Publish one guarded atomic change set with current authority, expected-head checks, immutable revisions, canonical operation identity and a durable recoverable receipt.
- Supply the default single-host file adapter with coordinated writers, immutable snapshots, interruption recovery and original-operation lookup.

Source basis: [ontology proposal](../../../source/system-definition-ontology-proposal.md), §§5, 8, 17, 21.1–21.7 and 21.10. Method and ownership: [domain analysis](../../../design/domain-analysis.md); actor, stories and outcome: [KH-SL-001](../../../delivery/slice-plan.md#kh-sl-001--retain-and-reopen-one-sourced-knowledgehub-definition).

Excluded: successor-authoring/baseline comparison, broad query/traversal, human-review workflow, portable import/export, the complete self-model, database adapters, HTTP/SDK/FFI and publication. The first write still includes its full authority, concurrency, retry and recovery safeguards.

## Capabilities

### New Capabilities

- `source-bound-definitions`: Identified model scope, pinned seed profile, coherent document/graph ownership, exact source capture, validation and snapshot-bound exact reads.
- `branch-commits`: Current scoped admission, atomic branch changes, conflict guards, operation identity and durable original-result lookup.
- `file-snapshot-recovery`: Coordinated file publication, immutable reader snapshots and complete recovery of content, head and operation receipt within an explicitly tested single-host profile.

### Modified Capabilities

None. The main-spec inventory is empty.

## Impact

Implementation converts the binary starter to one library crate with owned asynchronous public contracts and internal model, use-case, storage-port and file-adapter responsibilities. See [implementation evidence](../../../delivery/evidence/kh-sl-001.md) for the executed scope and limitations. KH-D-01–06 of the [self-hosting decision register](../../../design/self-hosting-plan.md#decisions-to-resolve) record the adopted seed/authority contract, Rust 2024 with Rust 1.85 minimum, host-owned Tokio, initial Linux x86_64/local ext4 support target, current host policy and versioned canonical fingerprints with lifetime committed-receipt retention. The user directs existing Rust 1.99.1, no compiler installation and assumed platform compliance. Executed checks and that assumption are distinguished in the evidence; later-slice choices remain open.

Acceptance requires a public-contract walkthrough of seed creation, exact-source reads and reopen, plus denied/invalid changes, same-base writer races, exact retries, changed-fingerprint rejection, interruption at publication phases and recovery after a lost acknowledgement. Record build/toolchain and tested host/filesystem boundaries. The linked technical evidence records these runs independently of specification validation. Human acceptance remains pending.
