# Proposal

Planning draft · KH-SL-010 · 31 October 2026. Implementation remains a separate request.

## Why

The current traversal scans all selected assertions for every visited subject, and cannot retrieve schema reference occurrences as neighbours. Associations may be inserted long after endpoints, so the reader needs generic fast direct lookup over actual facts, independent of future paths or model-specific indexes.

## What Changes

- Add storage-independent batch neighbour reads for incoming/outgoing/both directions, exact association/type filters, category selection and finite pagination/work limits.
- Derive endpoint-first bidirectional adjacency: element → selector → compact postings carrying opposite exact reference and assertion/property origin. New types and later associations use the same mechanism.
- Extract nested tagged schema references generically, preserving occurrence paths, parallel assertions, native kernel references and exact revision pins. Keep provenance/administrative membership separately selectable.
- Bind rebuildable persistent indexes to exact view/schema/extractor versions; update changed postings incrementally, share unchanged structures, and refuse stale/incomplete indexes as complete answers.
- Add selective root-bound record hydration and work counters proving neighbour lookup does not perform a full graph/history/artifact scan once a valid index exists.

Outcome: KH-US-27; a human or external AI retrieves actual neighbours, commits a later link and reads the changed neighbourhood while the old baseline remains unchanged. Hard prerequisite: [share-immutable-file-storage](../2026-11-01-share-immutable-file-storage/proposal.md) for files/3 root reads and immutable map/sequence sharing. Build the generic extractor independently, but do not claim the slice complete with files/2 full-history materialisation on each call.

Trace: ontology §§5/8/21.3/21.7/21.10; [file-storage proposal](../../../../source/file-storage-proposal.md) §7.4–7.5/7.7; KH-DR-03/04/07/12/15/17/19/21 and proposed KH-D-17.

## Capabilities

### New Capabilities

- `ontology-neighbour-lookup`: generic exact one-hop lookup, schema-reference occurrences, view-consistent derived indexes and bounded batch results.

### Modified Capabilities

None in main specs, which are empty. Preserve the existing unarchived `model-navigation` API's assertion-only filters, ordering and depth/page limits; its internal scans may use the new index without changing those results. New property/category semantics use additive operations.

## Impact

Work centres on `src/ontology/navigation.rs`, `types.rs`, schema/reference inspection in `validation.rs` and the private files/3 read/index layer. Indexes stay adapter mechanisms; callers receive owned references/records, not paths or database cursors. Native immutable maps are the initial implementation; SQLite or another database dependency is not part of this change.

Acceptance compares indexed results against an independent scan oracle across generated ontologies, late links, parallel edges, nested/duplicate references, exact old pins and baseline subsets. Fault/stale-cache/rebuild tests and increasing-unrelated-graph work measurements are required, with cold preparation distinguished from warm reads. Excluded: inferred links, vector nearest neighbours, all-pairs closure, per-model index code, text search, multi-hop impact decisions, agent behaviour and changes to canonical `hub/`. Numeric limits/index schema versions are proposed in design, not runtime evidence or human acceptance.
