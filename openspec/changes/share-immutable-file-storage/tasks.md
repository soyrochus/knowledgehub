# Tasks

Planning only; all work remains unchecked until implemented and verified. KH-SL-009 / KH-US-25–26; acceptance scenarios live in both capability specs. Use existing Rust 1.99.1 and installed host tooling; install no compiler. Do not reset, rebuild or convert `hub/` in this change.

## 1. Exact object grammar and projections

- [ ] 1.1 Implement the versioned logical OID/framing and strict canonical payload decoder in the private file adapter; verify golden bytes, kind separation, invalid Unicode/duplicate keys, unknown versions, bad lengths/hashes and finite decoder limits.
- [ ] 1.2 Implement typed persistent maps/sequences and companion retained-reference keys; verify unchanged-subtree sharing, insert/select/delete boundaries, count/key equality, collision handling, ordered duplicates and corrupt-node rejection.
- [ ] 1.3 Implement record/baseline/anchor/receipt factoring and reconstruction; verify original record/2 semantic digests, exact source spans, array order/null distinctions, actual kernel projections and cyclic logical-reference fixtures. Document the projection boundary in a files/3 guide.

## 2. Explicit backend and shared readable history

- [ ] 2.1 Add the explicit files/3 constructor/version and private pinned-root read boundary; verify files/1–2 constructor/golden/replay compatibility and explicit cross-format refusal without rewriting existing stores.
- [ ] 2.2 Implement complete generation/state/ancestor/operation roots and selective exact record/content-range reads; verify genesis, accepted historical lookup, orphan exclusion, correct full State materialisation and no unrelated history/artifact loads for point reads.
- [ ] 2.3 Deliver a public fresh-store A/B example using the new host constructor; run create → sourced commit → baseline → successor → reopen → exact reads and document the working command without using canonical `hub/`.

## 3. Guarded durable publication and original outcomes

- [ ] 3.1 Implement coordinated object installation and one synced head publication with current policy, expected-head/record guards and complete candidate validation; verify denied/invalid/same-base race cases preserve atomic outcomes.
- [ ] 3.2 Implement authoritative lifetime operation lookup/retry and original actor/request-codec checks; verify exact stale-base retries, changed actor/intent refusal, absence proofs and missing-node/receipt failures.
- [ ] 3.3 Extend process-interruption and injected persistence tests across object install, head replacement and acknowledgement; verify old-or-new complete recovery, Uncertain semantics, stable locks and both existing host runtime configurations, documenting the actual process-fault evidence boundary.

## 4. Compact encoding and maintenance

- [ ] 4.1 Add/review the lossless zstd dependency and raw/zstd framing plus fixed 1 MiB content manifests; verify exact chunk/full-source digests, unchanged-chunk reuse, range semantics, unsupported dictionaries and bounded malformed/decompression cases.
- [ ] 4.2 Implement immutable block packs and rebuildable object-location data; verify every logical OID across raw/compressed/packed forms and rediscovery after catalogue/index deletion. Document pack/readable-materialisation commands and run them against isolated fixtures.
- [ ] 4.3 Implement full retained-closure scrub and coordinated debris/redundant-encoding cleanup; verify unselected history/receipts survive, logical cycles terminate marking, reader/maintenance races are safe, and interruption at each replacement/removal boundary leaves a readable complete store.

## 5. Integrated evidence and compatibility

- [ ] 5.1 Run the existing files/1–2 suites plus the files/3 public/fault examples, formatting, Clippy and relevant doctests; retain actual commands, input/build hashes and results separately from historical evidence.
- [ ] 5.2 Measure small/large fixed models and long tiny-edit histories for logical/encoded/allocated bytes, changed-path writes, compaction peak space and point-read work; demonstrate no full-model rewrite, record tradeoffs and update the owning slice/decision evidence without implying human acceptance.

## Workflow follow-up

- Review the delivered scope/evidence before requesting archive; leave existing unarchived changes and main specs untouched during implementation planning.
- Begin KH-SL-010 only after the shared-root read and recovery foundation is verified. Canonical deletion/regeneration and full-suite verification remain KH-SL-012; general conversion stays deferred.
