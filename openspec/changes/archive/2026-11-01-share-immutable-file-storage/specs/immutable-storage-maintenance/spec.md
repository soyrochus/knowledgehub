## Purpose

Reduce encoded and allocated storage through lossless compression, shared chunks and immutable packs while preserving accepted history and independently verifiable content. Trace: KH-US-25–26; KH-DR-02/05/08/10–12/20; ontology §21.5–21.6; file-storage §§8/10–12.

## ADDED Requirements

### Requirement: Encoding-independent identity
Compression, chunking and packing SHALL preserve exact original bytes/digests and logical object identities. Readers SHALL decode every required declared encoding without external dictionaries and reject unsupported codecs, malformed lengths, truncation and invalid hashes under finite resource limits.

#### Scenario: Recompress and repack
- **WHEN** identical logical objects are stored with another supported physical encoding
- **THEN** their reconstructed bytes, semantic records, public snapshot IDs and original receipts remain identical

#### Scenario: Excessive decoded size
- **WHEN** an encoded input exceeds configured decoded-resource limits
- **THEN** decoding refuses capacity or invalid framing explicitly without unbounded allocation or changing accepted state

### Requirement: Exact chunk reuse
Large artifact representation SHALL permit unaffected chunks to be reused and SHALL verify full original digest and length on complete reconstruction. A range read SHALL identify its exact source revision and range; it SHALL NOT claim full-artifact verification from a partial read.

#### Scenario: Append to a retained log
- **WHEN** a successor appends bytes without shifting earlier chunk boundaries
- **THEN** complete earlier chunks are reused, both full versions reconstruct exactly and their distinct original digests are retained

### Requirement: Rebuildable physical location data
Packing SHALL provide verified immutable object representations. Loss or corruption of derived location indexes/catalogues SHALL NOT erase accepted knowledge or prove object absence; valid closed packs and loose objects SHALL suffice to rebuild location information.

#### Scenario: Location cache is removed
- **WHEN** all derived location indexes are deleted from an otherwise intact store
- **THEN** required accepted objects can be rediscovered and verified without original repository files

### Requirement: Coordinated crash-safe compaction
Compaction SHALL install and persist verified replacements before removing the only old readable representation. It SHALL coordinate with readers and publication, preserve pinned views and add no model generation. Uncertain recovery/reachability SHALL stop reclamation.

#### Scenario: Interruption during cleanup
- **WHEN** a process stops after installing a replacement pack but during old-file removal
- **THEN** reopening retains every accepted object through at least one durable verified representation

#### Scenario: Reader is materialising an old baseline
- **WHEN** maintenance attempts to replace its physical representations
- **THEN** coordination prevents deletion of the reader's required content until it can finish or retry explicitly

### Requirement: Lifetime accepted closure retention
Cleanup SHALL reclaim only unreachable preparation debris and redundant physical representations. Accepted generations, old baselines, unselected revisions, semantic/source dependencies and lifetime operation outcomes SHALL remain recoverable. Derived caches SHALL be distinguishable from required knowledge.

#### Scenario: Revision is no longer selected
- **WHEN** a successor becomes current and cleanup runs
- **THEN** an earlier accepted snapshot still retrieves the old revision and its original operation outcome

### Requirement: Complete integrity scrub and honest measurements
Scrub SHALL verify all retained accepted roots, semantic/source digests, original receipts, ancestry and logical dependency closure. Storage reports SHALL distinguish logical, encoded, allocated, derived-cache and temporary peak bytes. Timing results SHALL identify input scope, platform and cache state.

#### Scenario: Corruption outside the latest selection
- **WHEN** an old required object has no valid redundant representation
- **THEN** full scrub reports the affected retained history rather than declaring the entire store healthy

#### Scenario: Small-edit storage report
- **WHEN** a benchmark adds many small accepted edits and compacts the store
- **THEN** it reports new payload/structural writes, retained history and allocated bytes without dropping knowledge to manufacture a size reduction
