# Tasks

KH-SL-013 · Explicitly authorised proposal and implementation. Preserve accepted data and the deferred rebuild's separate failed status; completion is technical verification of packed derived storage.

## 1. Scope and retained planning

- [x] 1.1 Defer the obsolete active rebuild artifacts at 7/11, retain their failed-run evidence and existing Hub data, and update owning sequence/origin links; verify the active CLI inventory and current document links identify pack-derived-index-storage as the next implemented scope.

## 2. Sealed bounded writes

- [x] 2.1 Implement variable-size derived packs with bounded sorted blocks, pending-writer reads and explicit finish before neighbour/text manifests; verify stable object hashes, mixed loose/packed reads, bounded/oversized blocks, incremental writes leaving sealed bytes unchanged and document the physical format.
- [x] 2.2 Account pack, sidecar, staging and replacement-catalog allocation under unchanged quotas; verify capacity, interrupted publication, reconstruction and after-commit failure behaviour without accepted data mutation.

## 3. Selective reads and semantic compatibility

- [x] 3.1 Reuse bounded per-namespace pack locations with catalog-epoch invalidation and accounted cold metadata; verify repeated ready batches avoid whole-index reloads, independent backend changes/eviction invalidate safely, and document cold/warm semantics.
- [x] 3.2 Integrate packed neighbour/text preparation and incremental views; verify exact results, witnesses, snippets, cursor policy, reopen and corrupt/evicted-cache behaviour against existing public/internal/legacy oracles and a fixture exceeding 65,536 logical nodes.

## 4. Actual data and final evidence

- [x] 4.1 Prepare and query the existing canonical Hub through public neighbour/text/composite APIs, retain measured encoded/allocated/peak/cache work, and verify every accepted file, head, A/B and original receipt stays unchanged; do not reset or recapture it.
- [x] 4.2 Run the full native/all-target/doc/format/Clippy and host-check batch under the existing toolchain, preserve build/input-bound evidence and update KH-SL-013 status; verify the deferred rebuild remains uncompleted and human acceptance/authority handover are absent.

## Workflow follow-up

- Review scoped technical evidence before archive. A future refreshed canonical rebuild/recapture needs its own resumed scope; its obsolete attempt is not relabelled as completed here.
