# Independent skill forward test

The raw equipment design was imported through knowledgehub-import and independently reopened/reconciled/verified through knowledgehub-verify. No maintained fixture model-plan or coverage was read. All authored work is under `/Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1`; no production, fixture, canonical hub/ or old_hub/ files were changed by this test. The JavaScript source was read and retained as data and never executed.

Native directory: `/Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1/native-hub`

Scope: `{"branch": "source-import", "hub": "forward-test:equipment-dueb5mj1", "model": "equipment-forward-dueb5mj1", "owner": "source:equipment-designer"}`

Exact head: `c04b08d6ab0080de89e28210c8a8f90ab347397f1c29a19132dedb863dca6952`

Named baseline: `{"id": "model:baseline:equipment-source-bound", "model": "equipment-forward-dueb5mj1", "revision": "1"}`

Catalogue/profile: catalogue-3, `sdo:ontology:system-definition` / `sdo:profile:system-definition`; record/3, request/3, files/4.

## Actual results

- Catalogue, capture, prepare, corrected disposable assess, create, independent resolve and fresh-process verify all exited 0.
- Native source coverage passed 10 independently source-derived obligations. All 12 queries exhausted across 24 pages at page size one; none truncated or reached a work limit.
- Verification read 90 records and verified 655 exact original bytes across both supplied files.
- Native validity is true. Native completeness is false: 15 genuinely omitted source details remain sourced gaps. The status `verified` and generation `ready: true` report technical/source checks; they do not mean complete application requirements or human approval.
- Two native Interpretation records remain unreviewed; shared-story task/cut names and singular retained-field interpretation are explicitly identified as agent interpretations. Source narratives/Body retain cancellation, the two-hour policy and open format/implementation choices beyond structured formalization.
- Implementation evidence is not established, application execution was not performed and human acceptance is pending. No accounts/provider actions/application tests are supplied.

## Reproducible commands

```sh
/Users/ivanderk/src/knowledgehub/target/debug/knowledgehub-model catalogue /Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1/catalogue.json
/Users/ivanderk/src/knowledgehub/target/debug/knowledgehub-model capture /Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1/import-config.json
/Users/ivanderk/src/knowledgehub/target/debug/knowledgehub-model prepare /Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1/run /Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1/model-plan.json /Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1/coverage.json
/Users/ivanderk/src/knowledgehub/target/debug/knowledgehub-model assess /Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1/run /Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1/host.json
/Users/ivanderk/src/knowledgehub/target/debug/knowledgehub-model create /Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1/run /Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1/host.json
/Users/ivanderk/src/knowledgehub/target/debug/knowledgehub-model resolve /Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1/run /Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1/host.json
/Users/ivanderk/src/knowledgehub/target/debug/knowledgehub-model verify /Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1/run /Users/ivanderk/src/knowledgehub/target/skill-forward-test/equipment-dueb5mj1/host.json
```

The destination already exists after this run. Do not rerun capture/create against it. For current independent verification use the final two commands with the retained host input. Any revised source or changed intent requires a newly reviewed run and fresh destination. The first assess failed on canonical Set ordering; a serialization-only correction was prepared before any main destination dispatch, then assessed again.

## Handover files

- `source-obligations.md`: independent raw-source expectation inventory.
- `model-plan.json`, `coverage.json`, `import-config.json`, `host.json`, `catalogue.json`, `author.py`: reviewable authored inputs.
- `run/source-manifest.json`, `run/plan.json`, `run/coverage.json`, `run/key-map.json`, `run/assessment.json`, `run/generation-manifest.json`, `run/journal.json`: native generation handover.
- `run/verification.json`, `run/verification.md`: final independent native report.
- `logs/independent-resolve.json`, `logs/independent-verify.json`: separate-process output.
- `independent-review.json`: retained-source/digest checks and explicit review of the reconciled immutable request, including absent implementation/resolution and non-execution distinctions. This supplements the native reads; it does not substitute for them.
- `run/assessment-57335/`: preserved failed disposable assessment (catalogue committed, model not published); `logs/assess.stderr` contains the V-02 /enum_values and /placements diagnostics plus consequent baseline closure violation.
- `run/assessment-57690/`: corrected successful disposable assessment; `logs/assess-canonical-sets.json` contains its full result.

## Known native receipts

- `authoring:register`: sequence 1, head `8d36ac89b78480f4cb05de0e1d074ce41b004a9d559b18b38cc2657795e730b6`, fingerprint `ebb026fe7e8adbf0d2a35fa16c64e1074b59a28e4bda950ac14b7b79dfb747c4`.
- `authoring:model`: sequence 2, head `c04b08d6ab0080de89e28210c8a8f90ab347397f1c29a19132dedb863dca6952`, fingerprint `8fac2b7deedf7f0ca8b221da1e46f9df8fe5c8a65300d35c036272ddc98eae84`.

## Workflow findings

1. The packaged contract at test start did not document canonical ordering of Value::Set. The native assessment correctly rejected unordered enum values and cut placement membership. Sorting canonical encodings preserved intent and resolved the failure. The failed store and diagnostic remain inspectable.
2. The packaged reference at test start described authoring contracts narratively but lacked complete JSON examples for Config/Input (including required status), exact Scope/Host, Coverage check kind and Query/Neighbours shapes, and native RecordData variants. I needed to inspect src/model_authoring.rs and native type definitions to construct a binary-compatible request. The catalogue supplies exact tokens and domain property schemas but not those outer authoring/native payload envelopes. No runtime prerequisite remained unmet; source insufficiency remains explicit below.

## Preserved structural source gaps

- `model:adapter/port`: A notification adapter is planned but no port or interface contract is specified.
- `model:asset/identity_contract`: Stable application identity and its representation are not supplied; reference-format choices explicitly remain unresolved.
- `model:booking/identity_contract`: Stable application identity and its representation are not supplied; reference-format choices explicitly remain unresolved.
- `model:field:reason/datatype`: The original booking reason is retained without a specified logical value type.
- `model:field:reason/nullable`: The retained field is described, but null-value semantics are unspecified.
- `model:field:state/nullable`: The retained field is described, but null-value semantics are unspecified.
- `model:lifecycle/initial_state`: The source names three states without specifying the initial state or transition ordering; no timer or cancellation transition is inferred.
- `model:question:implementation/decision_owner`: No authorized decision owner is named in the source; host authority does not supply a design decision owner.
- `model:question:reference-format/decision_owner`: No authorized decision owner is named in the source; host authority does not supply a design decision owner.
- `model:schema:booking/compatibility`: The raw design provides no schema evolution or compatibility policy.
- `model:state:Cancelled/terminal`: The source enumerates this state but does not define terminal-state semantics.
- `model:state:Confirmed/terminal`: The source enumerates this state but does not define terminal-state semantics.
- `model:state:Requested/terminal`: The source enumerates this state but does not define terminal-state semantics.
- `model:step:inspect/authority`: The source identifies a borrower but defines no participation or authorization policy.
- `model:step:reserve/authority`: The source identifies a borrower but defines no participation or authorization policy.
