# Tasks

Planning only; KH-SL-011 / KH-US-28–29 depends on verified generic neighbour lookup. Keep legacy Query/Page semantics and canonical model content unchanged. All tasks remain unchecked until implementation and corresponding evidence exist.

## 1. Deterministic text entry

- [ ] 1.1 Add strict owned search contracts for literal-utf8/1 and literal-ascii-fold/1, exact types/fields and finite phrase/work limits; verify ASCII folding, non-ASCII byte preservation, empty/short/oversized queries, unknown modes and stable ordering.
- [ ] 1.2 Implement selected Text/Narrative field projection and a root/version-bound trigram candidate index with exact final verification; compare with direct source-fragment matching for false gram hits, cross-field boundaries, successors and document/anchor changes.
- [ ] 1.3 Implement bounded text-index preparation/incremental updates and exact snippet references; verify stale/corrupt cache handling, read policy and original byte ranges, and document supported matching/readiness semantics with a working isolated example.

## 2. Generic bounded multi-hop execution

- [ ] 2.1 Add strict typed policy/ordered-step alternatives and additive traversal request/result contracts; verify exact schema-selector validation, finite plan sizes, unknown/external selectors and unchanged legacy maximum-depth-four requests.
- [ ] 2.2 Implement deterministic batched frontier expansion with subject/policy-state visitation and origin witnesses; verify mixed directions beyond four hops, alternative-state revisits, cycles, parallel origins and nearer plus farther targets against a small independent path oracle.
- [ ] 2.3 Enforce examined-work, state, hop, target, witness and byte budgets, including rejected/duplicate work; verify explicit truncation/exhaustion, bounded oversized-item projection and no empty-page continuation loop. Document the proposed defaults and observable statuses.

## 3. One-view composite reads and evidence

- [ ] 3.1 Compose typed phrase/ID starts, explicit all-grouped/require-one policy, traversal and compact target/source projection in one application read; verify no-match/ambiguity/grouped shared targets and deduplicated hydration at one exact selection.
- [ ] 3.2 Add bounded composite continuations to the shared finite cursor pool; verify full-intent fingerprints, head movement, revoked access, expiry, capacity and reopen invalidation without mixing views.
- [ ] 3.3 Deliver the isolated rent fixture through ordinary ontology/type/record registration, using explicit MappingRule selectors and a local physical DataSchema subtype with DDL evidence; verify generic engine behaviour across another vocabulary and missing mappings without modifying the built-in catalogue or canonical Hub.
- [ ] 3.4 Add coverage/candidate-dependency reporting and executable user examples; verify missing mappings, declared gaps, unresolved external references and wording-only edits do not become fabricated no-impact/schema-migration conclusions.

## 4. Integrated deterministic and work evidence

- [ ] 4.1 Compare complete bounded results, witnesses, snippets and statuses across reopen, index rebuild and late-association insertion; verify identical pinned requests produce identical results and the read makes no agent/network/script calls or graph mutation.
- [ ] 4.2 Measure one-public-request composite execution over fixed neighbourhoods with growing unrelated graph/history, recording batch lookups, examined edges/candidates, hydrated bytes, index preparation and high fan-out cases; show no per-hop external calls or hidden repeated full scans.
- [ ] 4.3 Run relevant public/internal/legacy suites, formatting, Clippy and documented examples under the existing host runtime/toolchain; retain input/build-bound evidence and revise KH-SL-011 status separately from human acceptance and canonical regeneration.

## Workflow follow-up

- Review the scoped result before archive. Named query registries, semantic/vector matching and automatic change interpretation remain separate proposals.
- KH-SL-012 uses supported queries on the actual self-model; the fictional rent fixture remains isolated.
