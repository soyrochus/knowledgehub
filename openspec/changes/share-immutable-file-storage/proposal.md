# Proposal

Planning draft · KH-SL-009 · 31 October 2026. Implementation is not authorised by these artifacts.

## Why

The canonical files/2 self-model occupies about 116 MB because every commit repeats a complete graph and artifacts; even baseline membership and source quotations are repeatedly expanded. The maintainer needs exact historical reads and original retries with storage growth proportional to changed content.

## What Changes

- Add explicit opt-in `knowledgehub-files/3`: shared content-addressed immutable records, typed map/sequence roots, exact original artifacts and compact generation descriptors.
- Factor baseline membership/gaps and source quotations losslessly, preserving public record/request codecs, semantic digests, opaque revisions, snapshot IDs and receipt values. Baselines select exact knowledge; generations record accepted transitions.
- Add chunk sharing, raw/zstd physical encodings, immutable block packs, complete scrub and crash-safe debris/redundant-encoding cleanup. Accepted history and committed operation IDs never expire.
- Add root-bound point/range reads and authoritative operation/ancestry maps so bounded reads need not materialise all history. Full owned snapshots still fulfil their existing contract.
- **BREAKING physical format, explicit constructor only:** files/3 cannot be opened as files/1 or files/2. Preserve both existing APIs/codecs/stores; ordinary open never converts them.

Outcome: KH-US-25–26; maintainer creates, changes, reopens, scrubs and compacts a small sourced model and obtains the same exact A/B explanations and original outcomes with shared unchanged content. Hard prerequisite: the implemented full files/2 ontology/record/request contracts from the completed, unarchived `express-full-ontology-and-self-model`; no prerequisite on future query shapes.

Source trace: ontology §§3–5, 21.4–21.7/21.9–21.10; [file-storage proposal](../../../source/file-storage-proposal.md) §§3–6/8–13; KH-DR-01–15/17/20 and KH-D-16 in the owning domain/self-hosting documents.

## Capabilities

### New Capabilities

- `shared-file-storage`: exact root-based immutable history, coherent publication/retry and bounded reads over shared objects.
- `immutable-storage-maintenance`: compression, packs, complete integrity scrub and safe reclamation without loss of accepted content.

### Modified Capabilities

None in the empty main-spec inventory. Extend the implemented storage path while preserving the unarchived seed/ontology contracts; do not archive or copy their entire specifications into this change.

## Impact

Implementation affects `src/storage.rs`, the private `Storage` port, `src/ontology/api.rs`'s current files/2 gate and snapshot selection, codecs and host backend composition. Keep model/application semantics behind owned asynchronous contracts. Internal responsibility modules may be added inside the same crate; no public adapter handles. A lossless zstd dependency and its lockfile/licensing review are proposed only for authorised implementation. Retain host-owned Tokio, finite blocking work/drain, current policy and the declared local ext4 profile; use existing Rust 1.99.1 without installation.

Acceptance requires digest/byte/receipt equality; cyclic logical-reference and original A/B fixtures; denied/invalid/concurrent/retry/interruption checks; independently restored pack content; explicit corruption failures; changed-path and allocated-space measurements. Indexes, phrase/traversal features, canonical reset/rebuild or general files/2 conversion, managed authoring, broad model exchange and authority handover are excluded. Later changes supply neighbour queries and canonical regeneration; general conversion remains deferred. Proposed tree/chunk/block and compression defaults remain reviewable choices in the source proposal; no measured size target or power-loss claim is invented.
