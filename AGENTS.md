# Agent Instructions

## Project and source of truth

This workspace is for implementing `knowledgehub`, a Rust library for the System Definition Ontology and governed Knowledge Hub model.

Read `source/system-definition-ontology-proposal.md` and `source/practical-design-manual.md` before planning or changing the project. The ontology proposal supplies the requirements, target model and proposed library/backend contract; the manual supplies the practical domain, journey, story-map and specification method. Its Memento examples are not KnowledgeHub requirements. Preserve draft/review status and record unresolved decisions explicitly.

## Development model

Use domain analysis → journeys and outcome slices → behavioural specifications → authorised implementation and verification → review and revised sequence. Apply the manual's learning loop; revisit earlier modelling when examples expose a contradiction.

- `design/domain-analysis.md` owns the working language, candidate contexts, aggregate responsibilities and domain rule catalogue.
- `delivery/slice-plan.md` owns journeys, story mapping, ordered outcome slices, dependencies and specification sequence. Prepare later changes progressively.
- `design/self-hosting-plan.md` owns bootstrap stages, document/graph authority and the open decision register.
- OpenSpec change artifacts define bounded behavioural deltas and their design/tasks. Completed planning artifacts do not establish approved decisions, implementation or acceptance.

The library's domain model and the ontology it stores are different modelling levels. Start with repository definitions and a small self-description seed. Progressively represent KnowledgeHub's own domain, slices, specifications and evidence inside the implemented Hub through its ordinary API. Transfer authority by explicit scope/baseline decisions; keep one owner per definition and bind derived documents to exact revisions.

## Current phase

The user authorised the ordered KH-SL-014–017 cutover: make-full-model-api-default, remove-legacy-storage-paths, align-current-api-tooling, then verify-v010-and-rebuild-hub. The crate is unpublished 0.10.0 with one full `knowledgehub::KnowledgeHub` API and one default files/4 `FileBackend::new`. Superseded API aliases, readers, constructors and format modes are removed. See design/typed-ontology-api.md, design/files3-guide.md and delivery/evidence/v010 for current contracts and focused evidence. Further work requires its own scope.

Use existing Rust 1.99.1; install no compiler. Accept the user's platform-compliance assumption separately from actual run evidence. Preserve the domain → journey/slice → specification → implementation/evidence → review loop. Explicit draft gaps, repository definition ownership, technical verification, human acceptance and authority handover remain distinct.

The user renamed the previous demonstration to old_hub. Preserve that directory byte-for-byte: never decode it through a compatibility backend, import it, reset it or include it in source capture. First cutover creation requires hub absent. The authorised final workflow runs the complete exact-source batch and creates a separate current canonical hub through ordinary APIs, with new history/A/B/receipts, then reopens/compacts/scrubs and verifies actual queries and catalogue/isolated fixtures. Its verified run evidence must precede completion claims.

Inspect the current demonstration with `cargo run --offline --release --example inspect_self_model`. Within a current instance preserve accepted history and exact captured sources, changing definitions through public APIs. Repository edits do not automatically refresh that historical model or transfer authority. No archive, crate publication or remote adapter is implied by the current implementation authorisation.

## Scope and design constraints

- Start with one Rust crate named `knowledgehub`, with clear internal responsibility boundaries.
- The first implementation covers the embedded library and a default file backend using Markdown, graph JSON and versioned artifacts.
- Keep public contracts storage-independent. Model and application semantics belong in the library; persistence mechanisms belong in adapters.
- Preserve stable semantic identities, immutable revisions, typed relationships, exact baselines, provenance and explicit definition ownership.
- Keep asynchronous I/O, snapshot-bound reads, guarded atomic change sets, revision conflicts, idempotent retries and recovery in the planned contract.
- MongoDB and PostgreSQL adapters, HTTP services, remote SDKs, FFI and crate publication are deferred. Do not add placeholder adapters or claim backend parity.
- Resolve the initial profile, authoring/authority rules, Rust toolchain and executor policy, and supported filesystem/durability profile before implementation. See sections 20 and 21 for the full decision and verification requirements.

## Working conventions

- Inspect repository files directly; use `rg` to locate text and paths for library development.
- Use the existing OpenSpec workflow for bounded proposals and specifications when requested. Keep planning artifacts traceable to the source proposal and make scope exclusions and acceptance criteria explicit.
- Keep original material under `source/` intact unless the user requests a source revision. Distinguish source requirements from implementation choices.
- Preserve unrelated local and untracked work. Do not edit generated agent skills or command templates for project-specific guidance.
- Distinguish planned behaviour, implementation, verification results and human acceptance. Do not report documentation checks as working-library evidence.
