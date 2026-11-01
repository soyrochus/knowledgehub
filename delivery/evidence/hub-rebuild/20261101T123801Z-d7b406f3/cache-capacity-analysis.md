# Derived-cache capacity finding

Attempt: delivery/evidence/hub-rebuild/20261101T123801Z-d7b406f3
Status: failed/incomplete after accepted population; no completed rebuild report or hub guide was published.

The successful checked batch ran 98 Rust tests across all targets, 11 Python host lifecycle/checkpoint tests and one doctest, with formatting and Clippy passing. Its exact checkpoint is checked.json.

The loader committed sequences 1–7 (catalogue, self-definition, checked-definition baseline, observed evidence/profile, A, explanation successor, B). The independent public partial inspection passed: 9,064 selected records, 9,070 retained revisions, 3,738 explicit draft gaps, distinct A/B narratives, both original operation receipts, valid catalogue and structural assessment. The fresh replay domain is 0b449a8e73e8b6380b8b265c478ace341845d2f67cc117338c7175a642077009.

The subsequent legacy indexed trace could not finish preparing its derived neighbour index:
- Cache nodes: 65,536 files, 34,782,560 encoded bytes, 268,435,456 allocated bytes.
- Completed cache manifests: zero.
- Accepted object files: 22,839, 16,011,956 encoded bytes, 95,576,064 allocated bytes before planned compaction.
- Error: Capacity at the existing 256 MiB allocated-space quota.
The source model was not rejected for integrity. Small-file allocation exhausted the disposable cache budget.

Recommended additional scope for review: store derived neighbour/text nodes in bounded packs using the existing files/3 object/block codec, retaining the current 256 MiB neighbour quota, stable object hashes, exact view bindings, bounded peak space and all current result/work semantics. Reuse bounded packing mechanisms; retain pack-location metadata across ready batches so packing does not introduce repeated full-index scans per hop. Verify corruption/eviction/reopen, capacity and failure handling, exact witness equivalence, and large canonical-model lookup work. This changes library cache storage beyond this host-workflow change's stated impact; user scope choice is pending.

Alternative: raise the cache quota, accepting larger allocated disk usage. This does not address the observed small-file overhead.

No cache-storage implementation or quota change has been made. The failed attempt remains visible, and the partial Hub is not presented as a completed rebuild. orig_hub/ is outside the reset target and remains preserved. A source-changing fix requires a new complete checked batch, fresh capture and explicit reset/retry under the specified workflow.
