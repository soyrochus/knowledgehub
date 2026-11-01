## MODIFIED Requirements

### Requirement: Explicit narrowly scoped demonstration reset
Rebuild SHALL require explicit creation/reset intent and validate the exact canonical demonstration path, expected scope and ordinary-directory boundaries. The first cutover SHALL require an absent hub after the user's rename; later explicit resets SHALL touch only a current-format canonical target. It SHALL coordinate with existing users and never modify old_hub. Ordinary library create/open SHALL NOT perform this reset.

#### Scenario: Reset the generated canonical store
- **WHEN** the authorised rebuild reaches reset after successful preflight
- **THEN** fresh creation uses an absent destination or a separately authorised current-only reset; old_hub and unrelated paths remain untouched

#### Scenario: Unsafe reset target
- **WHEN** the requested target is a repository root, parent/unrelated path, symlink escape or unexpected Hub scope
- **THEN** reset is refused before any deletion

### Requirement: Complete native suite on exact final inputs
Rebuild SHALL run all unit/integration/internal and target tests, doctests, formatting and Clippy against final 0.10.0 implementation inputs. It SHALL retain actual output, exit codes, build/source hashes and observed counts. A failed required check SHALL stop before creating or resetting the canonical Hub.

#### Scenario: A regression test fails
- **WHEN** any required native test or check fails
- **THEN** its failure is reported and the previous canonical Hub remains intact

#### Scenario: Passing result belongs to changed code
- **WHEN** current checked inputs differ from a retained successful manifest
- **THEN** the result is refused as current verification and the required checks/capture must be refreshed

### Requirement: Fresh current source capture
Population SHALL use a newly verified current repository/source capture with exact PDF/Markdown/extraction and actual native evidence bindings. It SHALL reject stale or unknown source origins and exclude old_hub and generated store/cache/output recursion. Historical evidence SHALL remain labelled as its own older run.

#### Scenario: New APIs and modules are implemented
- **WHEN** the current capture includes the sole current full API, neighbour/query contracts and 0.10.0 metadata
- **THEN** their declared stable origins and exact current bytes are represented rather than silently omitted or replaced by filename identities

### Requirement: New instance without migration continuity
After the absent-destination check or authorised current-only reset, ordinary public APIs SHALL create and populate a new 0.10.0 current-format Hub and replay domain with the full catalogue and applicable current self-model. The old_hub snapshots, history and operation receipts SHALL NOT be imported or presented as recovered outcomes. New counts and draft gaps SHALL be measured.

#### Scenario: New A and B are generated
- **WHEN** the fresh loader creates original and successor explanations
- **THEN** they have this run's baselines, snapshot history and receipts, and their existence does not imply preservation of the deleted instance

### Requirement: Explicit post-reset failure and retry
An interruption after new-store creation or a current-only reset SHALL report absent or incomplete new data honestly and retain attempt diagnostics outside the reset target. A later explicit current-target reset/retry SHALL rebuild from repository inputs and never touch old_hub. It SHALL NOT silently reuse a partial store as verified or promise recovery of deleted history.

#### Scenario: Process ends during model loading
- **WHEN** loading stops after only part of the fresh demonstration is committed
- **THEN** the attempt is marked incomplete and a clean retry requires explicit reset intent for that same canonical target

### Requirement: New run evidence and honest storage accounting
Successful 0.10.0 rebuild SHALL publish new test/walkthrough results, captured checkpoint, fresh domain/baseline/receipt identities and measured storage/read/count data. Older reports SHALL NOT be relabelled as current. Changed model inputs SHALL be distinguished from controlled same-input format comparisons.

#### Scenario: Fresh model contains more implementation definitions
- **WHEN** current API and storage definitions increase source/model coverage
- **THEN** the report records actual new counts and qualifies comparison with the older 116 MB demonstration rather than claiming migration equality

## REMOVED Requirements

### Requirement: Packed-storage compatibility and selective cache reads
**Reason**: Current-only storage verification replaces compatibility framing after old implementations are removed.
**Migration**: Run current sealed-pack integrity, quota and reopen checks against the new instance.

## ADDED Requirements

### Requirement: Packed-storage integrity and selective cache reads
The fresh run SHALL retain the current full-model logical contracts, use existing explicit compaction for accepted packs, and verify sealed variable-size neighbour/text cache packs under unchanged allocation quotas. It SHALL account encoded, allocated and peak regular-file space and separately charge cold location metadata. A newly opened backend SHALL return identical semantic results and reuse metadata on subsequent warm batches.

#### Scenario: Rebuild after derived packing is implemented
- **WHEN** the current source model is regenerated and indexed
- **THEN** required packs have valid content-addressed names, actual queries and reopened results agree, the original quotas hold and subsequent warm batches do not reload all sidecars


