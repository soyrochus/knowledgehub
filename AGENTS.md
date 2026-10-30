# Agent Instructions

## Project and source of truth

This workspace is for implementing `knowledgehub`, a Rust library for the System Definition Ontology and governed Knowledge Hub model.

Read `source/system-definition-ontology-proposal.md` before planning or changing the project. It is the current requirements and design proposal; section 21 defines the proposed library, backend contract and first implementation criteria. Preserve its draft status and record unresolved decisions explicitly rather than treating proposals as approved or implemented behaviour.

## Current phase

Workspace preparation and planning only. Do not generate or modify implementation code, add dependencies, or create library modules, tests, examples or adapters until the user explicitly authorises implementation. Documentation, project metadata and planning configuration may be updated within the requested scope.

The existing `src/main.rs` is a Cargo starter placeholder. Converting the package to a library target is deferred to authorised implementation; the placeholder is not a working KnowledgeHub implementation.

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
