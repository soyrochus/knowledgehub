# Proposal

User-authorised proposal and implementation · KH-SL-013 · 1 November 2026.

## Why

The canonical self-model's neighbour index reached its 256 MiB allocated-space quota at 65,536 small files, although their encoded contents occupied only 34,782,560 bytes. Packing actual immutable index nodes removes that per-file allocation overhead and permits bounded deterministic reads of the existing model.

## What Changes

- Store newly generated neighbour and text index objects in sealed variable-size pack files using the existing files/3 pack grammar, bounded compression blocks and exact object hashes. No large preallocated store and no edits/appends to published packs.
- Bound staging and final allocated bytes under the unchanged 256 MiB neighbour / 128 MiB text quotas; make pending nodes readable to the writer, finish packs before view-manifest publication and preserve explicit capacity/failure outcomes.
- Share bounded pack-location metadata across ready local batches, with exact cache-epoch invalidation and accounted cold metadata reads. Ready traversal must not reload every pack index on each hop.
- Preserve query membership/order/witnesses, immutable accepted objects, baselines, receipts and policy/cursor semantics. Support existing loose cache nodes and reconstruction after corruption or eviction.
- Postpone the unfinished canonical rebuild, retaining its 7/11 tasks and failed-attempt evidence under openspec/deferred/. Its old completion plan is obsolete pending this prerequisite; do not reset or recapture the Hub in this change.
- Verify packing and incremental publication on isolated fixtures and on the existing canonical dataset, reporting encoded, allocated and peak storage plus cold/warm lookup work.

## Capabilities

### New Capabilities

- `packed-derived-index-storage`: bounded, sealed physical storage and selective lookup for disposable exact-view indexes.

### Modified Capabilities

None in the main inventory. This extends the completed unarchived files/3/neighbour/query implementation without changing its storage-independent result contracts.

## Impact

KH-SL-013 serves the host maintainer and library reader; completion means existing data can prepare and query exact derived indexes under the original quotas using sealed packs. Hard prerequisites are KH-SL-009–011. Trace ontology §§5/16/21.5–21.7/21.10, file-storage §§7/10.3/12/14 and KH-DR-20–22.

Changes are confined to the existing single Rust crate's private file/cache mechanisms, tests, host origin inventory and owning documentation. Use existing Rust 1.99.1/Tokio/ext4 support, with no new dependency or compiler installation. Authoritative storage already supports sealed packs; reuse that grammar rather than introducing a second persistent format.

Acceptance includes crash/partial publication and quota cases, packed/loose equivalence, incremental updates leaving closed packs unchanged, reopen/cache eviction, warm lookup metadata reuse, full native checks and actual canonical neighbour/text/composite reads. Canonical reset/regeneration, new model authority, new ontology families, general migration/backup, database/network adapters and human acceptance are excluded. Runtime packet-size choices are implementation decisions recorded in design; no pending user decision blocks this bounded scope.
