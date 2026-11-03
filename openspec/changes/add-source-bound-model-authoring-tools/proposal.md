# Proposal

## Why

The library already has guarded source-bound publication, but its examples and self-model scripts are not a reusable package-to-model authoring interface. An external analyst needs deterministic tools to compile a reviewable plan, create a native Hub and demonstrate source coverage independently of Modelarium and structural Assessment completeness.

## What Changes

- Provide a standalone Rust command over the ordinary KnowledgeHub API for catalogue discovery, package capture, plan preparation/assessment, fresh-store publication, original-operation reconciliation and exact verification.
- Define versioned source/authority manifests, logical model plans, retained exact prepared requests, known-operation receipts and source-to-model coverage reports.
- Preserve Markdown/PDF/HTML/CSS/JavaScript/image originals and source-bound locators, while distinguishing duplicate editions, derived text, interpretations and unresolved dependencies.
- Bound work and payloads; checkpoint the actual sources and exact schema/parent closure; never infer completeness from previews or a zero-gap structural result.
- Use C1/C2's generic definitions and direct-subject/assignability semantics. No Memento filename, entity, field, ID/count or UI assumption in production tooling.
- This is C3 of MM-SL-01. Actor: developer/analyst supplying a source package and reviewed plan. Completion: fresh native Hub, baseline, exact sources/known receipts and independent coverage verification after reopen. Hard prerequisites: C1 and C2.

## Capabilities

### New Capabilities

- `standalone-model-authoring`: Source-bound plan preparation, native publication, exact retry/reconciliation and independent inspection through a local tool.
- `source-package-coverage`: Input identity/authority inventory, stable source mappings and explicit coverage/exclusion/dependency reports.

### Modified Capabilities

None. Existing source-bound definitions and atomic library guarantees are consumed rather than replaced; canonical self-model rebuild tooling keeps its separate destination contract.

## Impact

KnowledgeHub only: a maintained src/bin entry point with source-independent supporting modules, integration fixtures/tests and tool documentation. Trace: ontology proposal §§5, 11–12, 14, 16–17, 21.3–21.7/21.10; KH-DR-01–13/15/17–19; MM O-05/O-06 and native G6 acceptance.

No web server, Modelarium installation, SQL registration, in-app AI, embedded model provider, arbitrary code/rule execution, remote SDK, backend addition or registry publication. Work uses the current native profile and existing Rust 1.99.1; no compiler installation. Fresh fixtures replace obsolete contracts; no migration/read-compatibility layer. Input paths can be anywhere explicitly selected, including a read-only source example in another checkout, without depending on that application's code or changing it.
