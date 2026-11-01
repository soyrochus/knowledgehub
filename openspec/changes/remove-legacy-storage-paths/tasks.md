# Tasks

Order 2/4 · proposed KH-SL-015. Hard prerequisite: implemented make-full-model-api-default. No legacy implementation or migration feature is a completion requirement.

## 1. Current-only storage

- [ ] 1.1 Make FileBackend::new select files/4; remove older constructors, old format readers/writers/leases and dispatch flags; update directly affected call sites, document the single constructor and verify fresh use plus non-mutating rejection of files/1–3 metadata.
- [ ] 1.2 Make storage operations consume actual typed scope/state/receipts directly; remove seed wrappers, synthetic transport and old-backend unsupported/full-history defaults, verifying every current required port method and zero hidden full-State materialisation for selective neighbour/impact reads.
- [ ] 1.3 Remove legacy receipt roots/sequences and reconstruct/scrub only the sole qualified affected projection; preserve current loose writes, chunk reuse, sealed raw/zstd packs and location reconstruction, verifying changed receipt projection, complete current scrub, idempotency and unchanged old baselines after compaction.

## 2. Navigation and recovery

- [ ] 2.1 Consolidate typed Query selection/paging and expansion on current full/indexed semantics; remove legacy sorting/four-hop/adapter paths while preserving enumeration/type/ID filtering, and verify current ordering, finite limits, origin witnesses and independent semantic-oracle results.
- [ ] 2.2 Port/remove old-backend test branches only after corresponding current safety coverage exists; run targeted publication/cancellation/process-fault, reader/compaction, cursor and selective-read checks, document actual results and verify no old-format feature/alias/decoder remains in maintained code.

## Workflow follow-up

- Apply align-current-api-tooling next; it verifies/removes any residual host modes already touched mechanically here.
- Do not modify hub/old_hub or import old data. Final regeneration belongs to step 4.

