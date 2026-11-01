# KnowledgeHub 0.10.0 current-only verification and fresh canonical Hub

All four authorised changes were applied in order: the full API became the ordinary
KnowledgeHub API; the default backend became current-only files/4; commands and source
bindings were aligned; then a complete exact-source batch and new canonical population
were verified. No changes are archived by this implementation run.

Actual offline results: **95 Rust tests, 3 documentation tests, 17 host tests**,
formatting and all-target Clippy with -D warnings passed. Release examples built.
All 57 mapped current safety cases, covering 15 guarantees, appear as passing in
[checked.json](checked.json), which binds 190 checked source files, 28 executables,
command exit codes and the exact [native log](native.log). Existing Rust 1.99.1 and
local ext4 were observed. No compiler was installed.

The new files/4 store has replay domain `b4741f6fb5c4f945d8d7289780dcce11ab25b025b87d1dd9ad5efa1497862497`. Its current head has
**9,578 selected records, 9,584 retained revisions,
8 generations and 3 exact baselines**. Capture contains 197 artifacts,
2,757 source units and all 26 PDF pages, bound at 2026-11-01T23:17:54.061505+00:00.
A and B, original receipts/retries and current source/catalogue coverage passed through
ordinary public APIs. All 183 catalogue rows, 25 rule obligations, 21 self-model groups
and Q01–Q08 were audited. The model retains **3,452 explicit draft gaps**;
human acceptance and authority handover remain absent, and repository documents own definitions.

Population, first compact/scrub/indexed inspection, second-backend inspection, catalogue,
late-neighbour and rent fixtures, and coverage audit all exited successfully. See
[commands.json](commands.json), [result.json](result.json) and
[coverage-audit.json](coverage-audit.json). Fictional rent records are isolated fixtures.
The second backend returned identical semantic results and unchanged sealed cache packs.
Cold neighbour metadata cost 18,285,208 bytes,
cold text metadata 850,634 bytes; subsequent
warm metadata cost was zero. The actual three-hop composite used three batch lookups,
examined three associations and visited four states, with zero full-State loads.

Regular-file storage after the generated guide (directory entries excluded):

| Component | Encoded bytes | Allocated bytes | Files |
| --- | ---: | ---: | ---: |
| metadata/guide | 1,842 | 8,192 | 2 |
| cache/neighbours | 27,287,902 | 27,738,112 | 325 |
| cache/text | 2,146,726 | 2,240,512 | 48 |
| locks | 0 | 0 | 1 |
| objects | 12,587,057 | 12,595,200 | 3 |
| refs | 328 | 4,096 | 1 |
| **Total** | **42,023,855** | **42,586,112** | **380** |

The total is 42.02 MB encoded (40.08 MiB),
with 42.59 MB regular-file allocation. The objects directory
includes the accepted compressed pack and its reconstructible location metadata.
The authoritative logical object closure is 29,368,917 bytes
across 23,548 objects; record data is 8,513,219
logical bytes, captured artifacts 5,432,964, and the three
compact baseline payloads 1,851. These categories describe
overlapping representations and are not summed as independent disk costs.

Accepted compaction produced one sealed pack with 50 bounded blocks. Observed transient
compaction peak was 137,191,424 allocated bytes
(54,728,910 encoded bytes). The neighbour cache has
161 packs, 9,000,878 pack bytes and
18,274,371 sidecar bytes; text has 23 packs,
1,295,198 pack bytes and
849,043 sidecar bytes. Both remain within their
unchanged 256 MiB / 128 MiB allocation quotas. [Physical inventory](physical-files.json)
and result.json preserve exact measurements and peak/cold/warm counters. Inputs changed;
use controlled fixed-input benchmarks for storage-efficiency comparisons.

The first cutover found hub absent. The user's old_hub was never opened by a library
adapter or imported. Its 362 path/length/SHA-256 entries match before and after this
run and match the initial implementation inventory. See [before](old-hub-before.json)
and [after](old-hub-after.json). Source capture excludes it and generated stores/caches.

The checked/captured checkpoint precedes final task checkboxes, status prose, reports
and the generated hub guide. [Post-checkpoint record](post-checkpoint.json) lists exact
final prose-only source changes. Repository edits do not refresh the persisted model.
Original source books, archived planning and older execution evidence retain their scope.

Four active OpenSpec changes pass strict validation; 21 main specs pass ordinary
validation. Existing long-requirement warnings in three main specs make strict-all
validation report warnings as failures; they are documentation length diagnostics,
separate from the passing native/library and dataset checks. No publication, remote
adapter, power-loss guarantee or minimum-compiler execution was added.
