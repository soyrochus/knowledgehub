# Proposal

## Why

After the full-contract cutover, files/1–2 persistence and format dispatch would remain unnecessary implementation and verification costs. The user requested one current backend with the latest complete capabilities, without compatibility switches or conversion.

## What Changes

- **BREAKING:** make FileBackend::new select the sole current storage contract and remove new_ontology/new_shared_ontology, older readers/writers and version-selection branches.
- Use the full typed storage port directly; remove seed envelopes, synthetic-profile adapters, legacy receipt projections and unsupported-operation defaults retained for old backends.
- Identify the incompatible clean receipt/storage contract as knowledgehub-files/4. Recognize only that current contract; use current record/request/object codecs according to their actual schemas, without cosmetic renumbering or multi-version readers.
- Consolidate typed Query execution onto current selection/indexed traversal semantics, dropping old assertion-order/four-hop compatibility and preserving useful enumeration, paging and current-authority capabilities.
- Keep active loose-write/compaction, sealed packs, bounded raw/zstd, location reconstruction and exact accepted history. No data converter or legacy feature flags.
- Port/remove directly affected old-backend test branches while retaining the corresponding safety checks through the current implementation.

## Capabilities

### New Capabilities

- `full-model-storage-port`: mandatory full-model storage operations without seed wrappers or old-backend fallback methods.

### Modified Capabilities

- `shared-file-storage`: replace explicit opt-in/legacy preservation with one default current format.
- `file-snapshot-recovery`: express durability against the sole current format.
- `ontology-neighbour-lookup`: remove legacy navigation preservation; retain current query semantics.
- `bounded-model-traversal`: remove legacy-depth/additive-only framing.

## Impact

Order 2/4, proposed KH-SL-015. Hard prerequisite: [make-full-model-api-default](../make-full-model-api-default/proposal.md). Actor: host creating/opening a current full Hub. Completion: one constructor and implementation serve all current capabilities; old stores are rejected without modification; current history, receipts and packed storage pass focused fault/selective-read checks. Follow with [align-current-api-tooling](../align-current-api-tooling/proposal.md).

Trace ontology §§5/16–17/21.2/21.4–21.7/21.10 and KH-DR-02–12/15/17/19–22. src/storage.rs and shared generation/projection/maintenance plus query dispatch are affected. Old files are not migrated or recreated by open. The current hub and the user's future old_hub are not modified by this change. Fresh canonical creation belongs to step 4. No network/database adapter, hidden runtime, new filesystem claim, history pruning or compiler installation.

