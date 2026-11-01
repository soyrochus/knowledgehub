# KH-SL-012 — completed fresh canonical rebuild

Verified on 1 November 2026 after the user resumed [rebuild-canonical-hub-and-verify](../../../../openspec/changes/rebuild-canonical-hub-and-verify/proposal.md) following KH-SL-013. [status.json](status.json) is `verified`; [result.json](result.json) records the fresh instance, physical measurements and reopened indexed work. The earlier failed attempt remains unchanged under [20261101T123801Z-d7b406f3](../20261101T123801Z-d7b406f3/status.json); it has not been relabelled as a successful run.

## Exact inputs and execution

[checked.json](checked.json) and [native.log](native.log) bind the actual inputs, executable hashes, toolchain/filesystem observations and complete native checks: 100 Rust tests across all targets, one doctest, 11 Python host tests, formatting, Clippy with warnings denied and release example builds. Existing Rust 1.99.1 was used; no compiler was installed.

[self-model-input.json](self-model-input.json) captures 149 source artifacts and 2,049 extracted units, including the current files/3, neighbour, impact, sealed-pack and rebuild definitions. Capture and in-memory model validation passed before deletion. All source and checked executable bytes were verified again before final task/status prose updates. The capture is an exact working-tree checkpoint, not a claim that future repository edits refresh the model.

[commands.json](commands.json) retains all nine required command results: capture, input validation, population, inspector/compaction/scrub/indexed reads, a second inspector, full catalogue conformance, late neighbours, rent impact and coverage audit. Every exit code is zero. [coverage-audit.json](coverage-audit.json) verifies all 183 catalogue rows, 25 rule/refinement obligations and the sourced self-model walkthrough. Fictional rent data exists only in isolated target/ fixtures.

The optional concurrent measurement in [compatibility-before-reset.json](compatibility-before-reset.json) overlapped reset and was not completed or counted as verification. Required measurements below ran successfully against the fresh populated instance.

## Fresh instance and model

The exact canonical target was checked, coordinated and completely removed after successful preflight. New public create/register/commit calls produced replay domain `aa574094b460be73b70ecfab007bb216dc1d6f434bb69debdd870d4cbbe7c7cf`, distinct from the deleted instance. No old history or receipts were imported. [fresh-instance-check.json](fresh-instance-check.json) also confirms the unrelated 426 original files under orig_hub/ remain unchanged.

The fresh head is `9859e2dd20dd059a920aee85f1553a3eb8d26415d7e3cc51b72d214fa371ea4e`. It retains eight generations, three baselines, 9,264 selected records and 9,270 revisions. A/B hold distinct exact explanations; reopened original operation lookup and committed retry pass. The model remains structurally valid with 3,815 explicit draft gaps, purpose completeness false and no invented human assessment or authority transfer.

## Physical storage and deterministic reads

The files/3 logical format and object hashes remain compatible. Existing compaction places 23,317 accepted logical objects in one sealed pack with 48 bounded blocks. Neighbour/text writes use sealed variable-size packs directly. [inspect.json](inspect.json) retains compaction peaks, complete scrub and actual public results; [inspect-reopened.json](inspect-reopened.json) proves exact agreement from a second backend.

| Component | Encoded regular-file bytes | Allocated regular-file bytes | Packs |
| --- | ---: | ---: | ---: |
| Accepted objects, native locations and metadata | 12,404,028 | 12,419,072 | 1 |
| Neighbour cache | 27,829,391 | 28,286,976 | 164 |
| Text cache | 1,286,639 | 1,327,104 | 11 |
| Total at scrub | 41,520,058 | 42,033,152 | 176 |

The total is about 40.1 MiB allocated across 361 regular files, excluding directory/Git blocks and the small generated guide written after verification. Logical accepted-object bytes are 29,999,678. `scrub.derived_file_bytes` also includes the native pack's rebuildable location index; it is broader than cache-only bytes. Source/model inputs differ from earlier demonstrations, so these are actual current sizes rather than a controlled compression ratio.

Neighbour preparation retains 75,400 nodes in 164 packs; its peak reservation is 28,295,168 bytes against the unchanged 268,435,456-byte quota. Text has 2,749 nodes in 11 packs, with a 1,327,104-byte peak against its unchanged 134,217,728-byte quota. All cache pack filenames match complete-file hashes; sealed bytes remain unchanged on reopen.

On the new backend, cold neighbour metadata loading is 18,611,743 bytes and cold text metadata is 683,548 bytes. Following warm direct/composite batches load zero location metadata. The complete composite request uses three local batches and three associations, verifies one text candidate and loads zero full States. It reads 4,829,730 encoded index bytes in bounded blocks; metadata reuse does not remove decompression. Reopened dependencies, hits, coverage, snippets and origins agree exactly.

Read the [generated canonical guide](../../../../hub/README.md). The completed report remains technical evidence for its captured checkpoint. Final task/status documentation is tracked separately in post-verification-sources.json; library/test/host-tool bytes remain the checked inputs. Human acceptance, authority handover, power-loss guarantees and general migration/backup remain separate.
