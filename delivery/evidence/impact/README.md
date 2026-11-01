# KH-SL-011 deterministic query evidence

1 November 2026 · Implemented and technically verified; human acceptance and canonical regeneration remain separate. This evidence applies to query-deterministic-model-impact and the [documented API/bounds](../../../design/deterministic-query-api.md). Earlier evidence is preserved with its original scope.

The existing Rust 1.99.1 toolchain and host Tokio runtime were used offline. No compiler or dependency was installed. [Build inputs](build-inputs.json) bind the production, catalogue, source, example and original test bytes, toolchain, host and base commit. [Final rent inputs](rent-final-inputs.json) bind the subsequently strengthened absent-mapping and compact-witness tests; production inputs are unchanged. [Verification summary](verification.json), command logs and binary hashes retain actual executions. The supported host-declared filesystem profile remains coordinated single-host local ext4; the statfs tool reports its shared ext-family magic as ext2/ext3.

## Checks and behavioural boundary

- The [complete native run](native-tests.log) passed **92 native tests and one doctest**: 39 internal, 6 new generic query, 6 neighbour, 25 ontology, 12 original public, 3 rent and 1 shared-storage test. Helper-only integration crates contain zero tests. Recovery, cancellation, process interruption, legacy formats and the original depth-four queries pass.
- The [final rent run](rent-final-tests.log) passed all three scenarios after adding actual mapping Unselect operations and a 4096-byte seven-hop witness projection check. No change to production code followed the full-suite build.
- Formatting, Clippy over all targets with warnings denied, strict OpenSpec validation and release example builds passed. [Final check results](final-check-runs.json) include the final strengthened test input.
- [Generic query tests](../../../tests/model_impact.rs), [independent path oracle](../../../tests/impact_oracle.rs), [internal cursor/work tests](../../../src/ontology/impact/tests.rs) and [rent scenarios](../../../tests/rent_impact.rs) cover literal bytes/ASCII-only folding, false gram candidates, field boundaries, exact source/anchor successors, readiness/corruption, seven-hop mixed directions, state-aware cycles/revisits, parallel origins, near/far targets, grouped starts/shared hydration, finite limits, current-policy cursors, head movement, expiry/capacity and fresh-backend invalidation.
- Identical pinned requests preserve targets, witnesses, snippets and coverage across reopen and reconstruction. Late mapping/association insertion affects new views. Missing selected mappings, declared gaps and unresolved external starts remain distinct from real-world no-impact or migration claims. The generic engine contains no model-specific route, code executor, network or agent call; the read call graph uses exact view/record/search/neighbour ports and derived-cache preparation. Accepted graph publication is not invoked by these reads.

## One-call work measurement

[Raw benchmark output](impact_benchmark.json) and [resource observations](impact_benchmark-resources.log) come from the release impact_benchmark example. It grows unrelated selected records and retained commit history while leaving one text start and its seven-hop neighbourhood fixed. Each timed ready sample is **one public request**, with no output continuation. Cold/adapted preparation is reported separately. Concurrent suite activity and file synchronization affect these single observations; they are not latency guarantees or a long-history stress claim.

| Unrelated records | Local batches | Examined associations / verified text candidates | Hydrated bytes | Index nodes | Text posting probes | Ready milliseconds | Full State loads |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 8 | 7 / 1 | 232,777 | 35 | 18 | 55.591 | 0 |
| 100 | 8 | 7 / 1 | 232,777 | 47 | 19 | 39.757 | 0 |
| 1,000 | 8 | 7 / 1 | 232,777 | 66 | 22 | 80.164 | 0 |

All three samples read 49 required records (including repeated schema/origin hydration in separate local batches) and returned 5734 encoded bytes. Increased gram binary-search probes and tree paths explain index growth; the unrelated graph/history is not materialised or rescanned by ready execution. Initial combined preparation reported 462 owners, 11 text fields and 599,631 source-record/text bytes; subsequent selected-owner updates reported zero cold preparation during the measured requests. One extra empty frontier lookup after the seven associations accounts for the eighth local batch. Peak process RSS, including fixture construction, was 114,704 KiB.

The additional 1,000-edge filtered fan-out case reached the explicit 100-unit work cap: 22 text posting probes plus 78 examined/rejected associations, one local batch, no fabricated target and exhausted=false. No per-hop external request was needed. Persistent index/cache storage has its own documented allocated-space quotas; these measurements do not assert a canonical Hub size or general database performance.

## Isolated demonstration and preservation

The [rent output](rent_impact.json) is an actual public read of a fictional fixture with a UserStory narrative, the seven-hop typed plan, explicit MappingRule selectors and a locally registered physical DataSchema subtype with captured DDL. Its table is a candidate review dependency, not a claim that wording changes require schema migration. Fixtures live under target/ and are independent of the canonical model.

The first release invocation used /tmp, whose filesystem is outside the existing supported backend profile. It failed explicitly before creating a Hub; the tmpfs-attempt logs retain that observation. The successful examples ran on the repository filesystem without weakening platform validation. [Example commands/results](example-runs.json) and resources record both successful release executions.

[Canonical inventory](canonical-inventory.json) verifies all **426** original published files byte-for-byte under the externally existing orig_hub/ directory. hub/ remained absent. This change performed no reset/rebuild, recapture, source revision, definition-authority transfer, archive or human acceptance. The next planned change remains rebuild-canonical-hub-and-verify.
