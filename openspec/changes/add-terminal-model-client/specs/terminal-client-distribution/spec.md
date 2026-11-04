# Terminal client distribution

## Purpose

Let an instructor or reader run the terminal explorer and prepared native examples from a relocatable package without compiling or installing development runtimes. Trace: KH-DR-02/04–07/12/17–19/21; KH-US-09/10/21/22/24; ontology §§19/21.2/21.9–21.10.

## ADDED Requirements

### Requirement: Prebuilt end-user package
The distribution SHALL contain the terminal entry executable, matching KnowledgeHub host, runtime assets, version/checksum manifest and quick start. End users SHALL need neither Rust/Cargo, source checkouts, a separately installed Bun/Node runtime, Modelarium nor a service installation. Build tools SHALL be required only when preparing the package.

#### Scenario: Start on a clean supported machine
- **WHEN** a user extracts a matching verified package without developer tools on PATH
- **THEN** its documented command opens help, runs a JSON read and starts the TUI without compilation or dependency download

### Requirement: Relocatable explicit configuration
Packaged executables/assets and example workspace entries SHALL resolve independently of the original checkout and current working directory. Relative configured paths SHALL resolve against their workspace file. Model storage SHALL be separate from executables, and an occupied destination SHALL never be reset implicitly.

#### Scenario: Move into a directory containing spaces
- **WHEN** the extracted package and prepared workspace are relocated and launched from another directory
- **THEN** the same alias opens the same exact configured model without referring to the original developer paths

### Requirement: Honest target support and compatibility
Each package SHALL identify OS, architecture, filesystem requirements, library/protocol/codec/catalogue versions, native history projection/method support and actual verification evidence. Unsupported targets SHALL fail explicitly. Native checks for one target SHALL NOT establish another target's support or close existing file-backend gates.

#### Scenario: Linux package has no native run evidence
- **WHEN** release information lists a Linux build that has not passed native distribution checks
- **THEN** it is identified as unverified rather than ready for a classroom deployment

### Requirement: Portable prepared reference models
The package SHALL supply small hypothetical FruitTrucks and unrelated models with exact sources/scopes, A/B baselines, original publication handovers, expected reads and draft gaps. Preparation SHALL use ordinary authorised library/authoring workflows. Workspace reader B SHALL be distinct from publisher A, with explicit Open/Read/ReadHistory grants. Generic client behaviour SHALL require no fixture-specific branches.

#### Scenario: Change the selected case
- **WHEN** the reader switches from FruitTrucks to the unrelated fixture
- **THEN** types, properties, relationships and sources are browsable without client changes

#### Scenario: Inspect the actual route-change example
- **WHEN** the reader opens the FruitTrucks acknowledgement requirement
- **THEN** the native use case, scenario and exact source are available and proposed expiry policy remains distinct from established requirements

#### Scenario: Inspect a pre-consumer publication as a different reader
- **WHEN** B opens a fresh course copy containing A's earlier publications and a coherent successor
- **THEN** known operation IDs resolve original metadata/affected references, bounded native discovery finds accepted publications and exact A/B definitions/source hashes survive reopen and supported packing

#### Scenario: Course workspace grants only ordinary reading
- **WHEN** the companion Open/Read-only configuration is used
- **THEN** ordinary browsing and known baseline inspection/comparison remain usable while publication history and retained-baseline discovery are denied

### Requirement: Protected originals and separate working models
Example preparation and rehearsal SHALL use new disposable stores outside protected canonical or historical Hubs. No command SHALL automatically inspect, refresh, convert, reset or import repository hub/old_hub. Course copies SHALL preserve exact model/source bytes and be made only from quiescent prepared stores.

#### Scenario: Example preparation finds an occupied destination
- **WHEN** a fixture generation or copy destination already exists
- **THEN** it refuses or requires a separately selected new destination and leaves existing models and historical stores untouched

### Requirement: Runnable instructor demonstration
The distribution SHALL include a short session-4 walkthrough covering open, hierarchical browse, relationships, exact source, JSON interrogation, original publications/affected pages as a different reader, baseline inspection/comparison with discovery coverage, and offline visual export. It SHALL require no learner prework or later learner authoring/full Loom integration.

#### Scenario: Run the instructor's demonstration offline
- **WHEN** the instructor follows the packaged walkthrough using a fresh disposable copy
- **THEN** the TUI and CLI inspect the same pinned definitions and the exported preview renders the selected native relationships

#### Scenario: Instructor demonstrates original publication history offline
- **WHEN** the instructor resolves a packaged handover ID, pages accepted history and follows an affected reference
- **THEN** the package retains the original publisher, exact anchor, revision and source without a running publisher application, SQL audit, development tools or actor impersonation

### Requirement: Scope-specific delivery evidence
Verification SHALL record build inputs, target, commands, exact fixture selections and observed TUI/CLI/preview outcomes. Planning, fixture source coverage, technical execution and human usability/acceptance SHALL remain separate. Existing authoring/library regressions SHALL be checked without requiring Modelarium changes.

#### Scenario: Publish the local verification report
- **WHEN** distribution checks pass on macOS arm64/local APFS
- **THEN** the report states that exact boundary, retains pending target/rehearsal gates and makes no claim of human acceptance or production deployment

#### Scenario: Native history evidence already exists
- **WHEN** the package report cites the completed native-history prerequisite
- **THEN** it separately records new host/CLI/TUI/distribution outcomes and does not present native C8 tests as proof that this terminal client or instructor workflow has been implemented
