# KnowledgeHub

This workspace is dedicated to `knowledgehub`, a proposed Rust library for representing, validating, changing, reviewing and exchanging system-definition knowledge through explicit, versioned relationships.

The requirements and target model come from [A Proposal for a System Definition Ontology for the Knowledge Hub](source/system-definition-ontology-proposal.md), draft 0.2 dated 30 October 2026. [Section 21](source/system-definition-ontology-proposal.md#21-proposed-rust-library-knowledgehub) defines the proposed Rust library and its first implementation criteria. The [Practical Design Manual](source/practical-design-manual.md) supplies the method for domain analysis, journeys, story maps, outcome slices and behavioural specifications.

## Current status

Domain analysis, slice planning and specification. The initial planning set is drafted for review. The user has adopted the first-slice recommendations KH-D-01–06, including the toolchain/executor and initial file support profile. Library implementation has not been authorised or started; later decisions and verification remain pending.

The Cargo package is named `knowledgehub` and currently retains the original `src/main.rs` starter placeholder. It does not yet expose a library API. Library targets, dependencies, modules, tests and examples will be introduced only after implementation is authorised.

## Read the development model

1. [Domain analysis](design/domain-analysis.md): concrete situation, shared language, context map, aggregate responsibilities, invariants, operations and counterexamples.
2. [Journeys and slice plan](delivery/slice-plan.md): story map, six outcome slices, hard prerequisites, release work packages and progressive specification order.
3. [Self-hosting plan](design/self-hosting-plan.md): begin with documents, retain a small model of KnowledgeHub through the working library, then progressively govern its own definition inside the Hub.
4. [First OpenSpec change](openspec/changes/retain-first-self-definition/proposal.md): KH-SL-001, retaining and reopening one sourced self-definition, with three behavioural deltas, a proposed design and unchecked tasks.

The method is domain analysis → slices → specifications → authorised implementation → evidence and review. The full self-model grows after the first complete path works. An explicit authority handover prevents repository documents and Hub records from becoming competing definitions; stored requirements remain distinct from actual implementation and run evidence.

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
| `source/practical-design-manual.md` | Practical design method; Memento material is illustrative. |
| `design/` | Library domain analysis and self-description/authority plan. |
| `delivery/slice-plan.md` | Journeys, stories and ordered demonstrable outcomes. |
| `AGENTS.md` | Project guidance and the current preparation-only boundary. |
| `openspec/config.yaml` | Project context and rules for future specification work. |
| `openspec/specs/` | Main specifications; currently empty. |
| `openspec/changes/` | Bounded change proposals and their archive; the first slice is drafted. |
| `Cargo.toml` | Rust package metadata; no dependencies selected. |
| `src/main.rs` | Original Cargo starter placeholder, pending the library implementation. |

## Implementation gate

The [decision register](design/self-hosting-plan.md#decisions-to-resolve) records adopted KH-D-01–06: the small sourced seed and field ownership, Rust 2024 / minimum Rust 1.85, host-owned Tokio with bounded blocking work, Linux x86_64/local ext4 with coordinated immutable publication, host policy, and versioned canonical fingerprints with retained receipts. These are planned support targets awaiting implementation and evidence. Broader query, review, interchange and full self-model details remain open for their later slices.

Explicit implementation authorisation is still required. Complete planning artifacts and adopted decisions do not establish actual library support; that requires working operations and the scoped evidence specified in section 21.10.


## Principles of Participation

Everyone is invited and welcome to contribute: open issues, propose pull requests, share ideas, or help improve documentation. Participation is open to all, regardless of background or viewpoint. 

This project follows the [FOSS Pluralism Manifesto](./FOSS_PLURALISM_MANIFESTO.md), which affirms respect for people, freedom to critique ideas, and space for diverse perspectives. 

## License and Copyright

Copyright (c) 2026, Iwan van der Kleijn

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
