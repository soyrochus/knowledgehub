## Purpose

Establish exact current-only 0.10.0 safety evidence and a newly verified self-model without preserving old API implementations. Trace KH-SL-017, KH-DR-01–13/15–23 and ontology §21.10.

## ADDED Requirements

### Requirement: Safety coverage follows guarantees
Verification SHALL map original seed/full safety cases to current full-API tests before deleting old harnesses. It SHALL retain coverage of canonical intent, actor-bound retries, publication/conflicts, source integrity/ownership, recovery/cancellation, baselines and finite policy-bound queries. Obsolete cross-version matrices SHALL be removed without requiring the old numerical test count.

#### Scenario: Retire a legacy test harness
- **WHEN** its former API and backend are removed
- **THEN** every still-required safety guarantee has a current-API case or an explicit unresolved gap that blocks completion

### Requirement: Exact current-only native evidence
A successful final batch SHALL bind actual 0.10.0 source/build/executable/log identities and observed results for all targets, doctests, formatting, Clippy, host checks and required release examples. Existing Rust 1.99.1 SHALL be used without installing a compiler.

#### Scenario: A passing log predates the cutover
- **WHEN** its source/version/build identity differs from the final implementation
- **THEN** it remains historical and cannot satisfy current verification

### Requirement: Verified current canonical model
Completion SHALL require new public-API population, fresh A/B/history/receipts, reopen and original retry, complete scrub, packed-storage checks and actual deterministic queries. Catalogue and isolated fixture walkthroughs SHALL pass separately; draft completeness and human acceptance SHALL remain explicit.

#### Scenario: Only the code tests pass
- **WHEN** population or a required dataset check subsequently fails
- **THEN** the new canonical attempt remains failed/incomplete rather than verified

### Requirement: Old instance is retained outside current knowledge
The user-owned old_hub SHALL remain outside reset, current capture and migration/import. Fresh creation SHALL establish a new replay domain and original operation history. Retained-store integrity SHALL be compared through host file inventories without adding a compatibility reader.

#### Scenario: Complete the new demonstration
- **WHEN** hub is freshly created after the user's rename and all current checks pass
- **THEN** old_hub is byte-for-byte unchanged and its old receipts are not imported as current outcomes

