# Full ontology implementation record

Historical implementation record. The current 0.10.0 contract is documented in
[the full API guide](typed-ontology-api.md) and [current file storage](files3-guide.md).
Earlier seed APIs/decoders and the runnable files/1 fixture have been removed; this
record retains the scope of its original implementation and evidence.

31 October 2026 · `express-full-ontology-and-self-model`.

The user explicitly requested “Apply express-full-ontology-and-self-model”.
This authorises its implementation and the concrete KH-D-11–15 design choices
in the reviewed proposal, design, coverage inventory and self-model map. Those
choices refine missing schema details; they do not rewrite the source draft or
constitute human acceptance of a working result. Original material stays intact.

The selected contract is complete catalogue support followed by the applicable
KnowledgeHub self-model. Preserve the existing files/1 seed and original replay
semantics. New typed records, binary artifacts and baselines explicitly opt into
files/2. V2 revision identity is separate from content hashing, with one exact
owner per definition/property. No modelled role or validator description grants
runtime access or arbitrary host-code execution.

Use existing Rust 1.99.1, install no compiler, and retain the user-directed
platform assumption separately from actual execution evidence. The inherited
single crate, host-owned bounded Tokio work and coordinated single-host ext4
profile continue. Interchange, deferred adapters and authority handover remain
outside this change. Human acceptance remains pending.

Starting source/build inventory and actual baseline checks are retained under
`delivery/evidence/full-ontology-start.*`; the original accepted seed is retained
as an immutable compatibility fixture under `tests/fixtures/files1`.

## Finishing evidence after user commit 807ed85

The user committed the implementation and requested completion of the remaining
verification and reporting. The finishing pass keeps the library boundary fixed.
It adds a direct traversal/cycle/budget/continuation test and repairs host-loader
bindings: declared semantic IDs replace path-derived definition IDs, and the
snapshot mapping uses the actual manifest field `files`. The rejected initial
attempt and final successful runs are retained separately.

Current technical evidence is indexed by
[full-ontology outcome](../delivery/evidence/full-ontology.md), including exact
checked source/build hashes, original source comparison, catalogue specimens,
self-model group/question coverage and final A/B replay results. This is technical
completion of the requested bounded change, not human acceptance or authority
handover. No source document or generated agent skill was rewritten, and no compiler
was installed. Broader query/review/interchange/adapters remain outside this work.
