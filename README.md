# KnowledgeHub

This workspace is dedicated to `knowledgehub`, a proposed Rust library for representing, validating, changing, reviewing and exchanging system-definition knowledge through explicit, versioned relationships.

The requirements and design baseline is [A Proposal for a System Definition Ontology for the Knowledge Hub](source/system-definition-ontology-proposal.md), draft 0.2 dated 30 October 2026. [Section 21](source/system-definition-ontology-proposal.md#21-proposed-rust-library-knowledgehub) defines the proposed Rust library and its first implementation criteria.

## Current status

Workspace preparation and planning. Library implementation has not been authorised or started. The source proposal remains a draft, and unresolved decisions must be recorded before implementation.

The Cargo package is named `knowledgehub` and currently retains the original `src/main.rs` starter placeholder. It does not yet expose a library API. Library targets, dependencies, modules, tests and examples will be introduced only after implementation is authorised.

## First implementation scope

- One Rust crate with a storage-independent application API and clear internal model, validation, use-case, interchange and storage responsibilities.
- A default file backend using Markdown documents, graph JSON and versioned artifacts, without a database server.
- Stable identities, immutable revisions, typed relationships, provenance, definition ownership and exact baseline selection.
- Asynchronous I/O contracts, snapshot-bound reads, atomic guarded changes, explicit revision conflicts, idempotent retries and interrupted-write recovery.
- Portable import/export and verification against the file-backend completion criteria in section 21.10.

MongoDB and PostgreSQL adapters, an HTTP service, remote SDKs, FFI and crate publication are outside the first implementation. Their future integration must preserve the same logical contracts.

## Repository guide

| Path | Purpose |
| --- | --- |
| `source/system-definition-ontology-proposal.md` | Requirements and design proposal, including open decisions and verification criteria. |
| `source/system-definition-ontology-proposal.pdf` | Accompanying source document. |
| `AGENTS.md` | Project guidance and the current preparation-only boundary. |
| `openspec/config.yaml` | Project context and rules for future specification work. |
| `openspec/specs/` | Main specifications; currently empty. |
| `openspec/changes/` | Bounded change proposals and their archive; currently no changes. |
| `Cargo.toml` | Rust package metadata; no dependencies selected. |
| `src/main.rs` | Original Cargo starter placeholder, pending the library implementation. |

## Before implementation

Use sections 19–21 of the proposal to define a small, coherent first outcome and its acceptance criteria. Resolve the initial ontology profile, document/graph authority and authoring route, review policy, portable query semantics, interchange rules, and file publication/recovery contract. Record the Rust edition, minimum supported Rust version, executor policy and supported host/filesystem profiles; the scaffold currently declares edition 2024, while the other toolchain decisions remain open.

Capture bounded implementation work through the existing OpenSpec workflow when requested. The source proposal and planning artifacts describe intended behaviour; actual library support requires the working operations and evidence specified in section 21.10.
