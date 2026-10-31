# Design

## Context

See [proposal](proposal.md). `FileStore::prepare_generation` currently clones a whole snapshot into graph JSON and copies every artifact; `load` reads every manifest object and `recover` follows the accepted parent history. `OntologyHub::create/open` explicitly require files/2, and public `State` includes retained records/artifacts. Existing hash/retry and fault tests are compatibility evidence to preserve, not tests to relabel.

The normative proposed wire format is [file-storage draft 0.3](../../../source/file-storage-proposal.md) §§3–6/8–13. Adopt its proposed parameters for this implementation; later calibration may change them only before freezing fixtures and updating that source coherently. KH-D-16 records the proposal status. Use existing Rust 1.99.1, host-owned Tokio/finite blocking admission/drain and the declared single-host local ext4 profile; no compiler installation or broadened durability claim.

## Goals / Non-Goals

**Goals:** an explicit files/3 backend with immutable shared roots, lossless old logical contracts, guarded publication/replay, bounded object reads and safe packing/scrub.

**Non-Goals:** regenerating or converting the canonical Hub, new query semantics, minimal-supporting-set authoring, review policy or authority transfer. Full public snapshots remain full; optimising point reads must not trim their promised retained state.

## Decisions

### Explicit backend and private root-read boundary

Add an explicit host constructor, working name `FileBackend::new_shared_ontology`, plus a distinct files/3 constant. Keep the files/1 and files/2 constructors/codecs exact. Extend the private storage port with pinned root selection, exact record/content-range reads, accepted-snapshot lookup and original-outcome lookup. Route typed Hub operations by explicitly supported format; do not redefine the existing `FILE_FORMAT` constant from 2 to 3. The public application still returns owned semantic values. Putting files/3 behind the old constructor would make migration implicit and is rejected.

### Content-addressed objects and persistent complete roots

Implement the source's type/length-separated logical OIDs, canonical JSON, exact blobs, 64-entry radix map leaves and 128-item sequences. Keep selected, retained-record, retained-reference-set, artifact, gap, operation and ancestor roots distinct. The companion reference set has exactly the registry's keys and allows baseline reuse without physical hash cycles. Public snapshot IDs stay opaque; a generation's OID is computed after its receipt and roots are sealed. Receipts use public snapshot IDs, so neither operations nor ancestor maps need self-referential hashes.

Factored records reconstruct `knowledgehub-record/2` and verify its original semantic digest. Baselines retain exact array order/multiplicity where accepted, including larger original supporting sets; anchors reconstruct exact source spans. Model cycles use opaque exact references; only physical OID dependencies must be acyclic. Reusing all old flat graphs with a compressed extension would retain unnecessary full-copy writes and is rejected.

### Durable publication and layered reads

Use a stable store-wide nonblocking exclusive lock for publication/maintenance and shared coordination for reads. Under final admission, check current policy, original operation intent before stale-base rejection, guards and coherent candidate constraints. Install/sync missing immutable objects and the generation before the single synced atomic ref replacement. Pre-ref failure leaves the old head; post-replacement uncertainty never reports definite absence. Unreachable objects are not accepted.

Point reads verify needed object paths; original-outcome absence requires a complete verified operation-map search at a recovered head. Full scrub checks every retained root, semantic/source digest, ancestor and receipt continuity. This explicitly changes when unrelated-history corruption is detected compared with files/2's repeated full scans; required-path integrity and complete recovery verification remain mandatory. Validation cannot skip cross-record invariants merely because storage reuses nodes.

### Compression, chunking and packs

Use fixed 1 MiB content chunks above that threshold; preserve complete original SHA-256/length. Add a reviewed zstd dependency during implementation, with raw/zstd decoding and level-3/default savings rule, no external dictionary. Pack independently verified objects in the source's bounded block grammar, not chains of binary deltas. A regenerated location index is never acceptance authority. Decoder limits are finite; malformed length/hash differs from a valid request exceeding host capacity.

### Retention and maintenance

Follow complete accepted history and logical exact-reference closure, resolving cycles with visited sets. Reclaim only preparation debris or redundant encodings after a durable verified replacement exists. Repacking changes no logical OID or public receipt and adds no generation. Read coordination prevents deletion while objects are materialised. No history pruning, hard-link correctness dependency or opaque external fetch is introduced.

## Risks / Trade-offs

- [Baseline/anchor projection changes hashes] → compare reconstructed original canonical bytes/digests, ordered legacy cases and cyclic references.
- [Shared-object corruption affects several snapshots] → verify required paths, full scrub/restore evidence and retained backups; never silently replace accepted knowledge.
- [Packing saves file blocks but needs temporary space] → report allocated and peak compaction bytes, enforce quotas, retain readable old representations until installation is durable.
- [Small model pays tree/index overhead] → measure empty/small/large models and changed-path writes; do not promise a final MB count.
- [Broad storage rewrite risks legacy replay] → retain original codec paths and run existing files/1–2 public/fault suites alongside files/3 scenarios.

## Migration Plan

Introduce files/3 only for fresh explicitly named stores and disposable fixtures. Keep root `hub/` files/2. Complete create → commit → baseline → reopen → retry, then compaction/interruption/scrub, before later neighbour or rebuild changes depend on it. `rebuild-canonical-hub-and-verify` owns explicit canonical deletion, fresh population and the complete test/walkthrough run; general conversion remains deferred. Rollback of this additive feature leaves existing stores untouched; a files/3 store must never be reopened through an older-format constructor.
