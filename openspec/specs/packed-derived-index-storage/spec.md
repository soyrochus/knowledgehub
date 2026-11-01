# packed-derived-index-storage Specification

## Purpose

Keep disposable exact-view neighbour and text indexes compact and bounded while preserving immutable object identities and deterministic reads. Trace KH-SL-013, KH-DR-20–22 and ontology §21.7/21.10.

## Requirements

### Requirement: Sealed variable-size physical packs
Derived index storage SHALL group immutable objects into variable-size sealed pack files with bounded compression blocks. Published packs SHALL never be modified or appended. Packing SHALL preserve each object's exact kind, payload and content hash without preallocating a large fixed-size file.

#### Scenario: New index nodes are stored
- **WHEN** preparation or an incremental update produces many small nodes
- **THEN** they occupy bounded packs with block checksums and independently verifiable object identities

#### Scenario: A later association is published
- **WHEN** an index update creates new nodes
- **THEN** new packs supply them while existing closed pack bytes remain unchanged

### Requirement: Unchanged allocated-space quotas
Preparation and incremental updates SHALL retain the 256 MiB neighbour and 128 MiB text allocated-file-space quotas. Staging, pack data, location metadata and publication files SHALL be accounted before allocation; insufficient capacity SHALL be explicit.

#### Scenario: Many sub-kilobyte objects fit in encoded storage
- **WHEN** a fixture exceeds 65,536 logical nodes while its packed representation fits the quota
- **THEN** preparation completes without one filesystem allocation per object

#### Scenario: Space is insufficient
- **WHEN** the next bounded pack or publication exceeds available quota
- **THEN** preparation fails explicitly and no incomplete view is advertised as ready

### Requirement: Complete publication after durable packing
The writer SHALL resolve pending nodes during construction and finish, verify and sync all required packs before publishing a completed view. Interrupted preparation SHALL leave accepted Hub data intact and permit bounded reconstruction or explicit unreadiness.

#### Scenario: Process stops before view installation
- **WHEN** an injected failure interrupts pack or catalog installation
- **THEN** no successful complete view references unfinished data and the underlying snapshot remains readable

### Requirement: Selective reads with reusable location metadata
Healthy ready reads SHALL reuse bounded pack-location metadata across local batches. They SHALL invalidate it when the cache catalog changes and account cold metadata loading separately from object/block work. They SHALL NOT rescan all pack contents or reload all location indexes for every traversal hop.

#### Scenario: Repeated lookup on an unchanged cache
- **WHEN** another batch reads the same prepared pack catalog
- **THEN** it resolves the requested objects without loading every location index again

#### Scenario: Another backend changes or evicts the cache
- **WHEN** a caller next uses its cached locations
- **THEN** the changed catalog is detected and stale locations cannot yield a falsely complete result

### Requirement: Exact compatibility and corruption handling
Packed and loose representations SHALL return identical neighbours, text hits, origin witnesses, ordering and semantic coverage for the same pinned request. Reads SHALL verify requested blocks and objects. Corrupt or missing derived representations SHALL cause verified reconstruction or explicit unreadiness/capacity, never fabricated facts or source-data loss.

#### Scenario: Reopen and reconstruct an index
- **WHEN** matching requests run after reopening, cache eviction or location-index damage
- **THEN** their complete semantic results agree with the original pinned read

### Requirement: Existing canonical data remains immutable
Verification SHALL exercise the existing canonical self-model through public APIs without reset, recapture, successor generation or receipt import. It SHALL report actual encoded/allocated/peak space and cold/warm work. The unfinished rebuild SHALL remain explicitly postponed with its failed attempt and uncompleted tasks preserved.

#### Scenario: The canonical index is prepared successfully
- **WHEN** packing resolves the previous allocation limit
- **THEN** actual neighbour, phrase and composite reads pass while original accepted files, head, baselines and operation outcomes remain unchanged

