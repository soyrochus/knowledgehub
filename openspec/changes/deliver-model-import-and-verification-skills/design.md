# Design

## Context

C1/C2 define reusable rich-design contracts and C3 supplies deterministic native authoring/verification. This final MM-SL-01 change makes the actor-to-outcome path usable from coding agents and proves it against the complete supplied design. Existing generated OpenSpec agent skills are unrelated managed files and are not edited.

The original package is currently supplied under the sibling example directory. Its mockups directory defines the working target UI. Copying selected source bytes with provenance into a KnowledgeHub fixture during implementation removes the runtime dependency on that checkout; no Modelarium code or OpenSpec change is needed.

## Goals / Non-Goals

**Goals:** portable import/verify skills; complete source interpretation and native model; independent coverage/trace evidence; truthful unresolved review and actual host-run status; unrelated-domain reuse.

**Non-Goals:** in-app AI, Modelarium registration/read/edit features, a running Memento implementation, arbitrary rule execution, graph-only UI rendering, general source round-trip editing, old-contract support, canonical hub rebuild, public package publication or editing generated host/OpenSpec skill files.

## Decisions

### 1. Ship two skills over one modelling contract

Maintain skills/knowledgehub-import/SKILL.md and skills/knowledgehub-verify/SKILL.md with general references for source ownership, typed plan examples, exact references, lifecycle/logical/inheritance rules, UI interpretation, gaps and recovery. Keep Memento-specific walkthroughs in an examples reference, not the core instructions. Both skills call C3's current knowledgehub-model commands; neither writes accepted backend files.

knowledgehub-import establishes source/destination/host prerequisites, inventories authority, interprets definitions and UI behavior, resolves material ambiguity in the external agent conversation, produces a typed reviewable plan/coverage inventory, assesses, creates and invokes verification. knowledgehub-verify can run independently with only the native Hub and retained handover artifacts; it does not trust the generating agent's claims.

Use thin host packaging/install guidance for local Codex, Claude Code and GitHub Copilot agent execution. Keep source semantics and tool commands common; adapters only express discovery/invocation conventions. At implementation, verify the exact supported host versions/modes and discovery paths, documenting installation without overwriting unrelated skills. Do not promise cloud/web-only modes can execute local Rust tools. Missing execution capability is a prerequisite failure, not a simulated successful import. No provider SDK or inference engine is added to KnowledgeHub.

### 2. Keep interpretation review explicit

Use the five-document ownership hierarchy, source stable IDs and the UI's declared authority. Analyse HTML/CSS/JS and local assets beyond top-level screens: states, interactions, confirmations, failure conditions, permissions, command/action effects and visual details. Retain exact source for semantics that the bounded ontology does not fully formalize. Body descriptions and artifact links are legitimate design content; do not claim an executable UI renderer.

Do not mark agent interpretations human-reviewed or convert simulated research into real stakeholder evidence. Material conflict is either clarified with an authorized person or retained with exact source support and unresolved state. A source can explicitly leave an implementation absent; represent that absence/planned status honestly. Missing in-scope semantics or unresolved material coverage prevents an unqualified full-import claim. A full account of supplied source gaps is reported separately from completed business design.

No skill imports an application entity by extending the built-in ontology with its business fields. New application definitions use EntityType, FieldDefinition, DataSchema, BusinessRule, LifecycleDefinition and other generic contracts; reusable schema extensions, if genuinely needed, must have domain-independent meanings and governed registration.

### 3. Keep the reference package local and reproducible

Use examples/fixtures/memento/source/ for exact selected original files, preserving relative asset paths. Record source origin, input revision/digests, authority and metadata exclusions in source-manifest.json beside source/, not inside captured input. Keep model-plan.json, coverage.json, source-map.json, queries.json and extraction/locator artifacts separately beside it. Capture documents and PDF renditions independently; preserve research status and auxiliary supporting material with a disposition. Do not silently exclude files because they are not one of the five top-level documents.

Inputs initially come from the explicitly selected read-only source package, never from hub/ or old_hub/. No task edits Modelarium. The fixture's maintained source bytes and manifests then support a KnowledgeHub-only checkout; no absolute developer path is required. Use fresh output destinations under target/rich-design/<run-id>/ and explicit separate run workspaces. Do not check native backend snapshots in as the substitute for reproducible generation.

Use examples/fixtures/unrelated-design/ with the same input/plan/query conventions for a small equipment-reservation design. Its Asset and Booking definitions, different attributes/states/IDs, lifecycle, multi-journey map, planned adapter, logical reference/enum/record/collection and harmless rule artifact are fixture data only. Add a small general extension hierarchy for property/endpoint/constraint reuse. This is a focused reuse check, not a third slice or second full application project.

### 4. Make source obligations independently checkable

The Memento fixture inventory names every expected context, journey, US/R/S/DR/FA/AC identifier, template, cut and consequential UI control/state/effect. Baseline totals are five contexts, 13 journeys, 36 US/R pairs, 72 S identifiers, 26 DR, 11 FA, 30 AC, six templates and five working surfaces. These numbers and source identifiers live only in fixture expectations and query manifests. A source revision requires explicit inventory review; never tune generic validators to these totals.

The model must represent the actual P1–P5 product-wide story map across the existing journeys. Reuse story identity through placements and cuts. Verify US-32/R-32/S-32a/b and the source-supported rule/command/interaction/ActionBinding/artifact chain directly. Do not fabricate a journey or TestCase as a relationship bridge. Other independent checks exercise representative behavior/error contracts, entity/field/logical targets, lifecycle subjects/states/transitions, architecture/context boundaries, UI composition/state/binding, templates, assurance criteria and source provenance. Every family/identifier required by the source has an explicit mapping; a handful of representative queries is not a substitute for complete obligation accounting.

Compare exact identifiers and semantic mappings, not just counts. Remove a mapped requirement/control in a negative fixture to prove independent coverage fails even when native assessment remains complete. Record source completeness, native validity, native completeness, interpretation review, implementation evidence and human acceptance independently. Unknown external facts and missing implementation artifacts stay explicit; no actual account, provider action, successful application TestRun or stakeholder approval is fabricated.

### 5. Demonstrate persistence and ordinary native evolution

Run the maintained skills/tool path to create a real native store and baseline; drain/reopen in a new process and independently retrieve original bytes, locators, exact model definitions and requested traces. Resolve the known generation operation receipts. Exercise interrupted publication/response-loss using C3's journal and original identities. Record actual record counts, payload bytes and read budgets; exhaust relevant pages or fail full-coverage verification.

Use the public native API in the reference acceptance harness to publish one coherent definition/binding successor after baseline A, then drain/reopen. Current bindings must resolve coherently while A preserves exact original definitions, source/evidence and receipts. This verifies native history guarantees within the new contract; it does not implement incremental source reimport or Modelarium's future edit planner/history presentation.

Repeat relevant G6 criteria through the tools and skills: direct exact subjects, structural non-execution, valid gaps versus malformed values, planned realization, generic extensions and selected inheritance semantics. C2's acceptance table owns the exhaustive native cases; the full reference demonstrates application fidelity. Modelarium portions of G6.1-04/G6.4-05 remain MM-SL-02 gates, not a reason to add application changes here.

### 6. Separate evidence levels and host acceptance

Retain delivery/evidence/mm-sl-01/<run-id>/ with input/tool/host versions, actual commands, source/plan/coverage digests, exact baseline/head, known receipts, technical results, unresolved interpretations, platform assumptions and human-review status. The native Hub contains its ordinary exact source/provenance; the evidence directory records how verification was performed. Capture the real outcome of every check, including failure or unexecuted status.

Automated native fixture replay proves deterministic tooling, not portable agent interpretation. Require actual end-to-end import and independent verification in each claimed supported local host mode, recording whether the agent generated/reviewed the plan or merely replayed an existing plan. Do not label documentation inspection or a tool-only replay as an agent-host pass. Human acceptance additionally reviews meaningful source-to-model mappings, the authoritative UI behavior and explicit gaps. Until performed, that gate stays pending; this planning change proves neither implementation nor acceptance.

### 7. Use one current contract and a native handover

Package skills with explicit compatible current tool/manifest/catalogue identities. Refuse obsolete input rather than adding legacy readers, aliases, translators, dual modes or migrations. Regenerate disposable models as contracts settle. Preserve hub/, old_hub/ and unrelated/user stores byte-for-byte.

Return native directory, scope, exact head, baseline, source/mapping/coverage artifacts and generation receipts. Later Modelarium registration consumes this handover in MM-SL-02; it is not invoked, generated or tested here. Rust 1.99.1 and the supported native runtime/storage are prerequisites, with no compiler installation or new adapter.

## Risks / Trade-offs

- A complete-looking model may omit interactions -> independent source obligation inventory plus deliberate omission tests.
- Agent hosts differ and may be unavailable -> thin versioned guidance and actual recorded host gates; no fabricated passes.
- Reference data could leak into generic contracts -> unrelated-domain execution and review for application-specific production branches.
- Rich source leaves decisions open -> preserve unresolved interpretations and assess human adequacy separately.
- Exact original assets increase fixture size -> retain originals and explicit finite capture budgets rather than reducing them to screenshots/previews.

## Scope gate

All artifacts and later implementation tasks are KnowledgeHub-owned. Stop if producing this standalone outcome requires a Modelarium code, schema, test, documentation or OpenSpec change. Copying source bytes without editing their owner is an input-capture operation. There is no application integration hidden in reference acceptance.
