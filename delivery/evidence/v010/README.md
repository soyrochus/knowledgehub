# KnowledgeHub 0.10.0 cutover

All four changes are implemented in order; all 22 tasks are complete.

- [Stage 1: sole full API and version](stage1.md)
- [Stage 2: current-only storage and unified queries](stage2.md)
- [Stage 3: current tooling and source bindings](stage3.md)
- [Safety guarantee map](coverage-map.md)
- [Final exact-source native and canonical dataset evidence](../hub-rebuild/20261101T230838Z-9b8133d6/README.md)

Final results: 95 Rust tests, 3 documentation tests and 17 host tests, formatting,
Clippy and release builds passed. Fresh hub has 9,578 selected
records, 9,584 retained revisions and 3,452
explicit draft gaps. Its regular files use 42,023,855 encoded bytes;
old_hub's 362 original file hashes remain exact. Technical verification establishes
no human acceptance or authority transfer. The changes remain active for review.
