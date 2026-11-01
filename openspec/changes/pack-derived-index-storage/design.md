# Design

## Context

See proposal.md. Objects and Tree already share the files/3 logical object grammar. Authoritative maintenance creates immutable KHPACK3 packs with independently checked blocks and rebuildable location sidecars. Disposable neighbours/text currently create a separate FileStore and one compressed loose file per node. Their quota reserves 4 KiB per node; the canonical attempt exhausted exactly 256 MiB before its manifest could be installed.

The host rebuild implementation and partial store are retained. This authorised change supersedes that active execution plan and defers its remaining regeneration/completion work. Existing source ownership, host Tokio, Rust 1.99.1 and local ext4 decisions remain. No new dependency, compiler installation or library reset operation.

## Goals / Non-Goals

**Goals:** use the existing pack grammar for derived objects, bounded transient memory and allocated space, unchanged semantic hashes, selective warm reads and actual existing-Hub verification.

**Non-Goals:** change accepted storage grammar, rewrite model history, perform canonical regeneration, implement general garbage collection/conversion, introduce preallocated mutable blocks or alter queries/authority.

## Decisions

### Write sealed blocks directly

Configure only derived FileStores for packed writes. Objects buffers a sorted set of new objects up to one decoded MiB / 512 entries; an oversized valid object gets its own bounded pack. One block per normal pack is the initial policy: each file has the existing magic, block count, canonical block header and entry stream; encoded length varies. A completed buffer is sealed, synced and installed under SHA-256 of its complete bytes, then independently verified. Per-pack derived location sidecars identify block and entry offsets. No published file is patched or appended, and no large preallocation is required.

Pending objects remain readable to that writer. Flush automatically at buffer bounds and explicitly finish before every neighbour/text completed-manifest installation, including incremental paths. Authoritative Objects retains its existing loose writes/explicit compaction. This keeps the physical change inside disposable namespaces and preserves legacy stores.

### Quota and failure boundary

Reserve rounded allocated space for each staged pack, location sidecar and catalog/manifest publication before writing. A staging hard link and final name are one physical allocation. Mutable derived catalog replacement accounts overlapping old/new files and refunds replaced allocation. Existing cache eviction/view limits remain. A new pack can fail safely before any complete view is advertised; failed after-commit cache work never replaces the original durable receipt. Interrupted debris stays disposable and participates in quota/eviction accounting.

### Reuse location metadata across batches

Share a bounded immutable location-map cache per backend and derived namespace. Bind it to the ordinary catalog file's inode/length/modified-time stamp; healthy read operations stat that catalog instead of inventorying all packs per hop. Cold, missing or changed catalogs reconstruct from verified pack indexes/blocks once, within the existing one-million-location limit. A writer retains added locations locally and merges/publishes them once at finish, avoiding whole-map copying at every block. Cache eviction invalidates the shared state; independently opened backends detect catalog replacement. Native authoritative location discovery keeps its existing protocol.

Sidecar loading bytes must appear in derived-work instrumentation. Cache validation still checks exact source origins and selected/root/version binding; a physical lookup never establishes an association by itself.

### Compatibility and maintenance

Support existing loose cache nodes alongside packs. Existing manifests remain semantically compatible; incomplete caches may be discarded under current readiness rules. Small incremental updates can produce small packs; existing quota/view eviction bounds their retention. More elaborate pack consolidation/live-node collection is deferred. The decoder checks block SHA, object OID/kind/length and finite counts/sizes; no deltas or replay chains are added.

### Verification scope

Retain the failed 20261101T123801Z-d7b406f3 run as historical evidence. Preserve all accepted hub/ files and orig_hub/. Add isolated large-node, failure, quota, packed/loose, stale-location, reopen and incremental immutability tests; measure healthy repeated reads to exclude repeated sidecar scans. Run the full existing native batch and examples, then prepare actual canonical neighbour/text indexes and compare its original head/receipts/A/B through public APIs. New evidence proves a cache-storage fix against an older captured model; it does not re-label the old rebuild as completed or capture new source authority.

## Risks / Trade-offs

- [Reading one small object decompresses a block] → bound normal blocks to 1 MiB/512 objects and retain the existing one-block read cache.
- [Packing metadata is repeatedly loaded by transient adapters] → share stamp-bound namespace location state and verify cold/warm byte counters.
- [Pending nodes are referenced before sealing] → writer-local reads plus mandatory finish before completed-manifest publication.
- [Failure consumes temporary space] → reserve staging/metadata allocation, keep quotas and test failure/eviction paths.
- [Small updates create small closed files] → bounded view/quota retention; consolidation is separate future tuning.
