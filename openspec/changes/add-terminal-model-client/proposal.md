# Proposal

**Alignment update (4 November 2026):** [expose-native-model-history](../expose-native-model-history/proposal.md) is implemented and technically verified (20/20 tasks). This plan consumes its [delivered contract](../../../design/native-model-history.md) and [handoff evidence](../../../delivery/evidence/native-history/20261104-apply/README.md). The user authorised this planning alignment; terminal implementation remains unstarted and requires a separate apply instruction.

## Why

Readers currently need Rust integration code or an authoring-run verification workflow to explore a native KnowledgeHub model. A repository-owned terminal client would let an instructor, designer or agent browse definitions, follow their evidence and produce faithful diagrams from a prebuilt distribution, without installing Rust or depending on Modelarium.

## What Changes

- Add a generic, read-only `kh` client with an interactive terminal browser (TUI) and non-interactive commands using one exploration service and one result contract.
- Add a persistent KnowledgeHub-owned Rust host using the ordinary public library API, with a Bun/TypeScript transport client. Distribute the compiled host and a compiled terminal entry point together; retain Bun source entry points for development.
- Support model selection, ontology/type discovery, alternative hierarchical browsing views, exact record inspection, incoming/outgoing association and property-reference navigation, literal search, source/provenance inspection, gaps, exact predecessor lineage and named-baseline comparison. Add original-publication lookup/discovery and affected-reference pages through the delivered native history API, under explicit current `Read` plus `ReadHistory` authority.
- Export bounded graph JSON and deterministic Mermaid diagrams with exact input references and coverage information. Supply a self-contained HTML preview of the same graph so the instructor can show a visual without an external renderer or network access.
- Supply a small, explicitly hypothetical FruitTrucks reference fixture and a second unrelated model proving that navigation is schema-driven. Package native stores and sources for a no-compilation session-4 instructor demonstration.
- Keep Modelarium's repository, host, client, private protocol, bindings and installation console unchanged. The new client neither imports that implementation at runtime nor replaces it.

### Outcome slice and prerequisites

**Proposed slice TC-SL-01 — Explore and explain a native model from the terminal.** A permitted reader opens an explicitly configured native Hub, finds a definition, follows an exact relationship and source, switches to an earlier baseline, and exports a traceable visual explanation. An agent obtains the same records through JSON commands. Completion requires a working distributed artifact and observed TUI/CLI/visual demonstrations, not only source code or screenshots.

Hard prerequisites are the current `knowledgehub::KnowledgeHub` API, catalogue-3, record/request codecs /3 and files/4, ordinary file-backend opening, snapshot-bound queries, neighbours, search, source reads, exact-baseline comparison and the completed native history change. That change delivers `publication`, `publications`, `publication_affected`, `continue_history` and `release_history`, with owned `knowledgehub-history/1` projections. These native operations exist; the terminal host, transport and interface do not. A native FruitTrucks fixture must be authored and verified as part of this change; the campus browser mock is not native model data. No Modelarium installation or open external host gate is a prerequisite.

### Boundary, acceptance and unresolved choices

This slice is a complete read-only explorer. Creating Hubs, editing definitions, registering schemas, validating candidates, committing changes, importing learner artifacts, remote services, FFI/native addons, new backends/platforms, publication and live attachment to Modelarium are excluded. Derived locks/indexes and ordinary library recovery may write operational files; read-only means no semantic model publication, not an OS read-only mount.

Acceptance is a prebuilt package opening a disposable native model without Cargo/source checkout or separately installed Bun, navigating requirement → use case/scenario → original source, retaining exact selection across paging, reporting incomplete results, comparing exact named baselines, and producing JSON, Mermaid and a local browser-viewable preview of the same bounded graph. A distinct current reader B must resolve publisher A's original metadata and affected references, discover accepted publications at a pinned snapshot, and retain exact A/B sources after reopen/packing. Ordinary read access must remain usable without implicitly granting history disclosure. The package must also navigate the unrelated fixture without case-specific client code. Existing library/authoring workflows remain intact.

Baseline listing distinguishes selected definitions and supplied exact handover references from an explicit bounded retained-history search; it never claims a native retained-baseline catalogue or paged comparison engine. Native publications, record predecessors, baseline parents and terminal back/forward entries remain distinct. Actor-bound `operation_result`, publisher impersonation, native receipt rewriting and library changes to add another history/diff engine are outside this consumer scope.

The first release target is the available macOS arm64/local APFS environment. Linux x86_64/local ext4 packaging remains a separately reported target until native distribution checks are executed; existing platform-verification gates are not closed by this proposal. Final terminal rendering dependency and target-specific binary signing/distribution policy are implementation/release choices; keyboard behaviour and artifact contracts below do not depend on them. This proposal authorizes no implementation or publication.

## Capabilities

### New Capabilities

- `terminal-client-host`: Local read-only host sessions, trusted configuration, versioned bounded transport and exact native value/error mapping.
- `terminal-model-cli`: Reproducible command-line exploration, reference resolution, JSON output and explicit selection/coverage/error semantics.
- `terminal-model-browser`: Keyboard navigation through hierarchical views, exact definitions, graph links, sources, search and history.
- `terminal-model-visualization`: Faithful bounded graph export, Mermaid generation and offline visual preview.
- `terminal-client-distribution`: Relocatable prebuilt packages, prepared generic fixtures and an executable instructor walkthrough.

### Modified Capabilities

None. Existing model-navigation, neighbour, search, baseline, source, native-model-history and host-tooling contracts remain their semantic owners. This change adds a consuming interface rather than redefining library semantics or the existing `knowledgehub-model` authoring command. The native history change remains the implemented prerequisite even while its artifacts are unarchived.

## Impact

Proposed implementation areas are `clients/terminal/` for the Bun client/TUI/CLI, `src/bin/knowledgehub-client-host.rs` plus a separate host-support module for the Rust executable, `examples/fixtures/terminal-client/` for portable source/plan/coverage inputs, and packaging/documentation/test scripts in this repository. The existing Rust crate remains the semantic implementation; no native storage files are decoded by TypeScript. New Bun dependencies and any Tokio host I/O features must be pinned and confined to terminal-host needs. No files in `/Users/ivanderk/src/modelarium` are changed or required by the delivered package.

Trace: ontology proposal §§3–5 (identity, source authority and complementary views), §§7–8 (journeys and typed relationships), §12 (representations versus meaning), §§17/19 (drafts and demonstrable adoption), §§21.2–21.3/21.7/21.9–21.10 (consumer boundary, finite reads, versions and verification); domain rules KH-DR-01–07/12/15/17–22; journeys KH-J-02/03/07 and stories KH-US-09/10/21/22/24/27/29. The practical manual's discovery → journey → slice → behavioural specification → verification loop supplies the method, not Memento product requirements.
