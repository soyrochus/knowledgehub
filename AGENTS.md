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

The user authorised MM-SL-01 after the earlier KH-SL-014–017 cutover. The experience-contract, logical/design-contract and standalone-authoring changes are implemented and technically verified; the skills/reference change has one external host execution gate open. See README.md, design/model-authoring.md, examples/fixtures/README.md and delivery/evidence/mm-sl-01/20261103-apply for the delivered boundary. The OpenSpec task files own precise completion and pending gates. Further implementation requires its own scope.

The crate remains unpublished 0.10.0 with one full `knowledgehub::KnowledgeHub` API and one default files/4 `FileBackend::new`. The current catalogue/profile is catalogue-3; semantic record/request codecs are /3. Older codecs, API aliases and format modes are refused; no compatibility reader, converter or migration is provided. Storage layout version and semantic codec versions are separate contracts.

The user also authorised C8 `expose-native-model-history` of MM-SL-02 and TC-SL-01 `add-terminal-model-client` on 4 November 2026. The separate publication API requires current scoped `Read` plus `ReadHistory`; original attribution and actor-bound mutation recovery remain distinct. The binary-local terminal host, shared CLI/TUI explorer, two disposable source-bound draft fixtures and offline diagrams consume that API without library/codec changes. See `design/native-model-history.md`, `design/terminal-client.md`, the change tasks and their dated evidence for precise contracts and verification. The local macOS arm64 package does not establish Linux execution, signing/publication, human meaning/usability acceptance or Modelarium C9; those retain separate scope and gates.

The standalone workflow is source package → analyst/agent interpretation → typed plan → native assessment/publication → independent verification. The external analyst/agent supplies source authority and interpretation; deterministic tools use the ordinary library API. Current reference runs are separate Hubs under target/rich-design, with exact sources, A/B baselines and original receipts. They are valid drafts: the recorded Memento run has 1,504 explicit structured omissions. Source-accounting success does not establish complete formalization, implementation, human meaning review or acceptance.

Use existing Rust 1.99.1; install no compiler. Accept the user's platform-compliance assumption separately from actual run evidence. Preserve the domain → journey/slice → specification → implementation/evidence → review loop. Explicit draft gaps, repository definition ownership, technical verification, human acceptance and authority handover remain distinct.

Preserve hub/ and old_hub/ byte-for-byte. Never decode old_hub through a compatibility backend, import it, reset it or include it in source capture. The retained root hub is the earlier codec-/2 self-description; the current /3 decoder cannot reopen it. Do not run the current inspect_self_model against that historical instance as a current-contract check. A future canonical refresh or replacement requires an explicitly scoped decision, exact-source verification and new public-API evidence; neither this implementation nor a documentation update authorises it.

Inspect current reference runs through the documented knowledgehub-model verify workflow using their retained run and current host inputs. Within a current instance preserve accepted history and exact captured sources, changing definitions through public APIs. Repository edits do not refresh a stored model or transfer authority. Modelarium registration/in-app editing, archive, crate publication and remote adapters remain outside this delivered scope.

The user authorized TC-SL-02 `add-kitty-terminal-graph-view` on 5 November 2026. Its optional `kh view REF`/browser `V` consumer adds bounded embedded Kitty presentation without library, host RPC, codec or fixture changes. See `design/terminal-client.md`, the change tasks and `delivery/evidence/terminal-kitty-graph/20261105-apply`. Automated protocol/PTY/package evidence is separate from actual Kitty/Ghostty display; both native-terminal gates remain open, as does human meaning/usability acceptance. This does not authorize canonical refresh, Modelarium C9, archive or authority transfer.

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
