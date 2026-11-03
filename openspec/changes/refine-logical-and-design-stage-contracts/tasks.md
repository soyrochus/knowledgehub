# Tasks

## 1. Resolve governed exact type contracts

- [x] 1.1 Introduce exact tagged type/subject-role selectors and qualified parent resolution through authorized registration; verify exact/direct/transitive acceptance, unrelated/reverse/same-name rejection, denied registration and native payload boundaries (G6.4-01/02/04).
- [x] 1.2 Implement origin-based inherited field/constraint resolution; verify valid diamond/multiple inheritance and rejection of cycles, unresolved parents, ambiguous fields and shadowing, with no inherited obligations dropped (G6.4-03/04).
- [x] 1.3 Update public schema/catalogue documentation with selectors, extension authority and profile limits; verify documented fixtures register through the supported API and a profile cannot inject undeclared fields (G6.4-01/07).

## 2. Enforce inherited semantics and consistent reads

- [x] 2.1 Apply one matcher to reference properties and both assertion endpoints, and execute inherited/local integrity constraints; verify descendant aggregate, lifecycle and UI-binding violations fail despite different concrete names (G6.4-02/03).
- [x] 2.2 Update typed selection/lookup/neighbour/impact filtering with explicit exact versus inclusive intent and snapshot-bound parents; verify stable paged membership, finite bounds, current policy, and relevant schema/base guard conflicts (G6.4-05).
- [x] 2.3 Document supported inclusive filters and unsupported-query diagnostics; verify example reads agree with native assessment and disclose truncation rather than reporting a complete result (G6.4-05).

## 3. Bind lifecycle subjects and logical application types

- [x] 3.1 Replace subject_type and the authoritative inverse with singular direct LifecycleDefinition.subject and derived reverse lookup; verify permitted kinds/cardinality, metatype/unresolved rejection, coherent guarded edits and exact current/A states after reopen (G6.1-01/02/03).
- [x] 3.2 Add explicit enum/record/item/reference/date/timestamp/duration logical contracts and field target narrowing; verify valid structured definitions, wrong metatype targets, widening, incompatible constructors and malformed values, with source-bound representation retained.
- [x] 3.3 Document the model/metamodel boundary and new lifecycle/logical examples; verify both a rich shape and unrelated-domain fixture use identical production schemas and code paths (native G6.1-04, G6.4-07).

## 4. Preserve honest design-stage and draft status

- [x] 4.1 Add planned/implemented architecture status with conditional real-artifact obligations; verify complete planned records, exact resolving implemented bindings, permitted omissions and rejection of supplied empty/nonresolving bindings (G6.3-04).
- [x] 4.2 Validate gap permission, exact subject/path, reason and provenance without waiving supplied-invalid data; verify every G6.3-01/02/03 case and valid/incomplete reason retention after baseline/reopen.
- [x] 4.3 Add non-execution checks and documentation using retained sentinel rule artifacts and source-bound time/cancel policies; verify capture/assessment/commit/read do not invoke code/actions/timers and never manufacture execution, TestRun or human acceptance (G6.2-01/02/03/04).

## 5. Cut over and demonstrate the native contract

- [x] 5.1 Assign new affected contract/catalogue/profile/codec identities, update current helper pins/examples and regenerate disposable specimens; verify obsolete selectors/subject payloads fail explicitly with no reader/alias/converter/migration and no writes to hub/ or old_hub/ (G6.4-06).
- [x] 5.2 Publish extensions and rich/unrelated models, baseline, revise, drain/reopen and recover original receipts; verify immutable schemas/assertions/assessments/baselines and denied/stale/changed-intent outcomes through the supported native API (G6.1-03, G6.4-06/07).
- [x] 5.3 Run affected catalogue/conformance/navigation/public-API checks and required crate checks with existing Rust 1.99.1; retain a criterion-by-criterion result matrix for the 19 G6 items, marking future Modelarium portions outside this change rather than passed.

## Workflow follow-up

- C3 verifies these contracts through compilation; C4 verifies the complete supplied source package. Neither is replaced by the small core fixtures.
- Do not create Modelarium artifacts or implement rule execution/backward compatibility. Stop if a required native deliverable demands a Modelarium-owned edit.
