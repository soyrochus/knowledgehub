# Spec Delta

## MODIFIED Requirements

### Requirement: Unambiguous current commands
Maintained authoring, catalogue, inspection, benchmark and maintenance commands SHALL use the full KnowledgeHub API and sole current backend by default. On supported native hosts they SHALL declare the corresponding local filesystem profile without requiring Linux configuration on macOS. Old format selectors and old-constructor branches SHALL be absent rather than accepted aliases. Unsupported native profiles SHALL fail explicitly.

#### Scenario: Run an ordinary current command
- **WHEN** a user invokes the maintained inspector or authoring example without a format option
- **THEN** it uses only the current full implementation

#### Scenario: Supply an obsolete format flag
- **WHEN** a caller supplies an old files2/files3 selector
- **THEN** the command rejects the obsolete argument explicitly

#### Scenario: Use maintained tools on the Mac
- **WHEN** a maintainer runs minimal creation, catalogue/specimen walkthroughs, isolated self-description authoring/inspection, queries and storage maintenance on supported local APFS
- **THEN** those commands run through the ordinary API using the checked Mac profile
- **AND** the same commands retain Linux/ext4 operation on the supported Linux host

## ADDED Requirements

### Requirement: Native observations without Linux-only assumptions
Verification and benchmark commands SHALL collect platform-appropriate OS, architecture, filesystem/mount and resource observations. They SHALL identify metric units and unavailable optional observations explicitly. A missing required profile observation SHALL fail verification; missing optional metrics SHALL NOT panic, fabricate zero measurements or silently substitute another platform's values.

#### Scenario: Record Mac native evidence
- **WHEN** the complete native verification batch runs on supported macOS/APFS
- **THEN** it records actual compiler/target, macOS build, observed local volume/profile and source/executable/log identities without depending on Linux-only mount tools or procfs
- **AND** benchmark resource measurements use documented native units or explicit unavailable values

#### Scenario: Required profile observation is unavailable
- **WHEN** the verification host cannot establish its required filesystem/platform observation
- **THEN** the batch records the failure and does not issue a successful native-support manifest

#### Scenario: Retain the existing Linux observations
- **WHEN** the batch runs on supported Linux/ext4
- **THEN** it retains actual Linux mount and compiler observations and the same safety coverage, without claiming the Mac log as Linux evidence

### Requirement: Isolated support verification and honest evidence
Platform-support verification SHALL use isolated current-format fixtures and fresh evidence paths. It SHALL preserve repository `hub/`, user-retained `old_hub/` and historical evidence; no canonical reset, source refresh or history import SHALL occur. Current source bindings SHALL describe new platform APIs/modules explicitly without rewriting historical captures or transferring definition authority.

#### Scenario: Verify the platform extension
- **WHEN** the new native batch creates, reopens, compacts, scrubs and interrupts its fixtures
- **THEN** it modifies only its isolated stores and new evidence outputs
- **AND** existing canonical/retained data and historical reports are unchanged

#### Scenario: A new native-profile contract is captured
- **WHEN** current source inventory/audit encounters an added native support API or module
- **THEN** it resolves an explicit current semantic binding or reports an unresolved origin
- **AND** it does not silently refresh the historical model in `hub/` to make the audit pass

#### Scenario: Compilation passes without runtime evidence
- **WHEN** a report contains only builds, no-run test compilation or rejection-path tests
- **THEN** the report labels those observations as build/rejection evidence and leaves native persistence/recovery unverified
