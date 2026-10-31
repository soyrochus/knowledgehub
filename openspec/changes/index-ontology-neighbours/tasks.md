# Tasks

KH-SL-010 / KH-US-27, implemented after verified KH-SL-009 files/3 root reads. Checked tasks record implementation and automated verification, not human acceptance; actual results are in `delivery/evidence/neighbours/README.md`. Existing Rust 1.99.1 was used. No canonical reset/rebuild ran; the final audit found the original byte-exact data in `orig_hub/` and left that layout unchanged.

## 1. Generic facts and owned neighbour contract

- [x] 1.1 Add additive owned neighbour request/result, selector/category/origin and fact-scope types with documented ordering/limits; verify strict input/unsupported-capability cases and legacy Query/Page contract compatibility.
- [x] 1.2 Implement schema-guided extraction for assertions, nested tagged references, native kernel/provenance links and occurrence paths; verify generated newly registered types, Any-tagged references, repeated list items, parallel assertions and both-direction self-loops against an independent direct-record oracle.
- [x] 1.3 Implement selected versus explicit closure fact scopes and exact pinned-target resolution; verify unselected old assertions do not leak, historical endpoint pins stay exact and administrative membership is excluded by default. Document these semantics with an executable isolated example.

## 2. View-bound adjacency and incremental lifecycle

- [x] 2.1 Implement endpoint → selector → postings maps in both directions with a versioned exact-view manifest; verify selector-free endpoint discovery and result equality between full initial build and direct-record oracle.
- [x] 2.2 Derive incremental removals/additions from selected owner/assertion changes; verify later links to existing endpoints, select/unselect, successors and schema-version changes match a clean rebuild while old baseline index roots remain valid.
- [x] 2.3 Implement finite cache build/readiness, corruption discard/rebuild and coordinated eviction; inject cache installation failures after accepted commits and verify the original receipt survives while stale/partial caches never yield false complete/empty answers. Document cold preparation and cache recovery.

## 3. Selective reads, pagination and legacy integration

- [x] 3.1 Connect batch neighbour operations to pinned files/3 point reads and bounded hydration; verify index/record read counters show no call through full State/history/artifact materialisation for a ready lookup.
- [x] 3.2 Add stable page/byte/work bounds and bind continuations to actor, full filters, view, projection and fact scope in the shared finite cursor pool; verify head movement, revocation, expiry/capacity and changed-intent refusal.
- [x] 3.3 Adapt files/3 legacy assertion-only traversal to neighbour reads without changing original ordering/filter/depth semantics; run equivalence cases with the existing public navigation tests and document which new operations legacy stores explicitly do not support.

## 4. Measured end-to-end outcome

- [x] 4.1 Deliver/run a public late-link create → query → commit → query-old/new → reopen example, retaining exact assertions/property origins and original receipts; record the actual code/input scope and current-policy results.
- [x] 4.2 Benchmark fixed neighbourhoods with increasing unrelated edges/generations and separate cold preparation, cold page reads, warm lookup, high fan-out, posting counts and allocated cache bytes; show neighbourhood-plus-tree-path work and linear actual-fact entry growth.
- [x] 4.3 Run relevant native public/internal suites, formatting, Clippy and the documented neighbour example; retain scoped evidence and update KH-SL-010/decision status without claiming implementation of multi-hop search or human acceptance.

## Workflow follow-up

- Review evidence before requesting archive or starting KH-SL-011. Do not archive earlier changes or convert the canonical store implicitly.
