# Tasks

**Sequencing update (4 November 2026):** postponed at the user's direction until [expose-native-model-history](../expose-native-model-history/proposal.md) is implemented and verified. Reconcile the delivered native history permission/contracts with this consumer plan before starting its implementation; this note does not implement or accept either change.

Status: planning only. All implementation and verification work remains unchecked. TC-SL-01 adds a read-only consumer; it authorises no Modelarium changes, canonical Hub refresh, publication or native-store conversion. Requirements live in this change's five capability specs; source verification trace is ontology §21.10 and design decision 9.

## 1. Open and inspect a disposable native model

- [ ] 1.1 Add the binary-local Rust host module and `knowledgehub-client-host` entry using the existing public library and installed toolchain; verify it opens a current disposable fixture and returns its exact scope/head without adding terminal exports to the library API.
- [ ] 1.2 Implement versioned workspace loading, relative-path/canonical identity checks and scoped Open/Read policy; verify missing grants, unknown fields/aliases, wrong scopes, missing/incompatible stores and request authority/path overrides fail without creating a replacement or changing semantic content.
- [ ] 1.3 Implement the versioned length-prefixed handshake, correlated read dispatch and native DTO/error mapping; verify codec/version refusals, malformed/oversized frames, exact large integers/decimals, absent/Null, assertion origins and artifact byte identity with real native host tests.
- [ ] 1.4 Add the isolated Bun transport package with pinned lockfile, lifecycle, finite pending work and source-range reading; verify host reuse, diagnostic separation, cancellation, EOF/drain, response loss and invalid post-restart cursors using process simulations plus a native open/read smoke test.
- [ ] 1.5 Verify the host rejects every model-writing method and preserves head, records, original bytes and operation history across exploration; document permitted recovery/index writes and quiescent independent store use in `design/terminal-client.md`.
- [ ] 1.6 Add host build/run instructions and a compatibility/limit table beside these tests; verify the documented commands work without reading canonical/historical Hubs or invoking any Modelarium component.

## 2. Interrogate through shared exploration functions and CLI

- [ ] 2.1 Implement one exact-selection context, native bounded query/neighbour preparation and authority-aware caches for CLI/TUI/local scripts; verify interleaved head advancement does not mix selections and filter changes release old cursors.
- [ ] 2.2 Implement configured model listing, info, type/schema discovery, typed listing and qualified reference resolution; verify a registered extension, subtype intent, duplicate display/type names and arbitrary kernel/assertion inspection retain exact identity without case-specific client branches.
- [ ] 2.3 Implement direction/category/selector-aware links, declared property-occurrence navigation and bounded tree projections; verify incoming/outgoing links, parallel assertions, nested references, shared targets, cycles and unassigned memberships against native results.
- [ ] 2.4 Implement literal field/narrative search, source/provenance inspection, bounded text previews and explicit original-byte export; verify native matching spans, minimum/unsupported query errors, binary preview refusal, partial flags and exact original hashes.
- [ ] 2.5 Implement gaps, exact predecessor history, baseline selection/listing and native A/B comparison; verify earlier definitions and source bindings remain pinned and arbitrary snapshot/record comparisons are not advertised as native baseline comparison.
- [ ] 2.6 Add the `kh` command grammar, help, human output, `knowledgehub-exploration/1` JSON and documented exit codes; verify success, empty search, ambiguity, missing subject, denial, native/transport failure and cancellation produce unpolluted structured stdout where requested.
- [ ] 2.7 Enforce finite aggregate paging/output budgets and reproducible partial-result intent for one-shot commands; verify no dead process-local continuation is presented as reusable and a later pinned rerun returns the same selection after head movement.
- [ ] 2.8 Document and execute a CLI/local-TypeScript walkthrough using the same exploration functions, including exact reference encoding, filters and limits; verify scripts can interrogate the model without TUI prompts, raw protocol knowledge or Modelarium files.

## 3. Prepare two small native reference models

- [ ] 3.1 Add portable FruitTrucks source excerpts and an explicit provenance/authority inventory under `examples/fixtures/terminal-client/`; verify retained origin/digests and the distinction between acknowledgement requirements, proposed expiry, open questions and illustrative route data against the campus reference process.
- [ ] 3.2 Author a small typed plan/coverage set for the requirement, command/use case, scenarios, context/journey, route/offer definitions and explicit lifecycle; generate a fresh disposable /3 Hub through existing authoring/public APIs and verify the expected links, sources, declared gaps and no execution/acceptance claims.
- [ ] 3.3 Prepare a coherent successor and named A/B baselines for FruitTrucks using ordinary guarded changes; verify reopening, predecessor lineage, original sources, baseline comparison and retained original receipts before the explorer reads it.
- [ ] 3.4 Prepare a small unrelated equipment-reservation fixture with exact sources, A/B selections and expected query results; verify identical generic commands/views work without changing client code or depending on large ignored Memento/reference-run outputs.
- [ ] 3.5 Add fixture preparation/copy documentation and relative workspace configuration; verify creation/copy refuses occupied destinations, copies only quiescent prepared stores into new locations, and never opens, resets or converts repository `hub/` or `old_hub/`.

## 4. Produce faithful visual artifacts

- [ ] 4.1 Implement bounded `knowledgehub-view/1` extraction using the shared pinned selection and native graph operations; verify exact references, incoming/outgoing orientation, assertion/property origins, parallel occurrences and coverage against both fixtures.
- [ ] 4.2 Implement relationship, explicit journey and explicit lifecycle projections with deterministic JSON/Mermaid/manifest output; verify absent transitions/order/ownership remain missing or explicitly unsupported rather than inferred from labels or unrelated links.
- [ ] 4.3 Add a pinned offline renderer and self-contained HTML preview with legend, selection/coverage banner, zoom/pan and reference/source inspector; verify actual browser rendering with network blocked and inspect a node and an edge against their native origins.
- [ ] 4.4 Implement explicit output paths and replacement, store-path exclusions and text/data escaping; verify existing-file refusal, symlink/path escapes, terminal/HTML/Mermaid control content, original-byte preservation and absence of model publication.
- [ ] 4.5 Verify complete, limited and unresolved exports in all formats, including proposed FruitTrucks expiry status; document that editable diagrams/graph exports are derived inspection artifacts rather than native import, backup, implementation or acceptance evidence.
- [ ] 4.6 Add executable Workbench-consumption examples for graph JSON/Mermaid and the local preview; verify the examples retain exact input identities and do not need a remote model, external diagram renderer or automatic learner-artifact import.

## 5. Browse and navigate in the terminal

- [ ] 5.1 Implement the TUI state machine and visible model/selection/connection/read-only header over the shared service; verify configured opening, switching models, loading/empty/denied/incompatible/disconnected states and non-TTY help behaviour with a real PTY.
- [ ] 5.2 Add by-type/context/journey/source discovery panes with lazy expansion and generic fallback; verify virtual grouping, actual containment, ordering, cycles, shared identities and missing memberships remain distinct across both fixtures.
- [ ] 5.3 Add generic property/narrative/assertion inspection, incoming/outgoing link following, exact source view and status/gap display; verify the requirement → use case/scenario → source path through keyboard actions and safe display of control characters.
- [ ] 5.4 Implement pane focus, search/filter/disambiguation, paging, help, no-colour mode and resize; verify every supported operation is keyboard-reachable, search cancellation restores the prior view and compact layouts retain selection and usable controls.
- [ ] 5.5 Implement exact back/forward history, explicit snapshot/baseline switches, refresh and reconnect; verify an A history entry still resolves A after refresh to B and invalid cursors are replaced by fresh pinned queries without false empty results.
- [ ] 5.6 Integrate diagram-scope selection and export with the shared visualization path; verify a keyboard-only export renders the same native scope as the equivalent CLI command and clearly reports partial results.
- [ ] 5.7 Add keyboard reference and TUI recovery walkthrough beside PTY integration tests; record an observed instructor-style browse/source/export run, distinguishing automated checks from human usability acceptance.

## 6. Deliver a package that requires no development tools

- [ ] 6.1 Add maintainer packaging for the matching Rust host, Bun-compiled `kh` entry, pinned offline assets and checksums/build provenance; verify the macOS arm64 artifact identifies exact protocol/API/codecs/catalogue/target and introduces no Modelarium runtime paths.
- [ ] 6.2 Package the two quiescent native examples, retained source/coverage inputs and relative workspace settings; verify relocation into a path with spaces and launch from another cwd preserves exact scopes, baselines and sources.
- [ ] 6.3 Execute help, JSON exploration, real compiled TUI keyboard/resize/exit and offline HTML rendering from the extracted package with Rust, Cargo, Bun, Node and source checkouts unavailable; verify no download, compilation or separately managed service occurs.
- [ ] 6.4 Ship a runnable 15–20 minute session-4 instructor walkthrough and quick start; verify a fresh disposable copy covers browse, requirement links/source, identical pinned CLI JSON, A/B inspection/comparison and a rendered diagram without learner prework.
- [ ] 6.5 Document supported target/filesystem boundaries and package troubleshooting alongside distribution checks; report Linux native verification and signing/publication separately, with no Windows/new-filesystem claim or automatic closure of existing backend gates.

## 7. Review the integrated outcome

- [ ] 7.1 Run the relevant existing Rust library/authoring/navigation suites, new native-host tests, Bun type/static checks, CLI integration and compiled PTY/visual/package checks; verify ordinary authoring still works and no existing semantic contract changed.
- [ ] 7.2 Produce a TC-SL-01 evidence report with exact build inputs, native fixtures/selections, command results, observed terminal/browser outcomes and pending target/human-acceptance gates; verify each capability's normal and consequential failure scenarios has evidence or an explicit unresolved gate.
- [ ] 7.3 Review final repository scope and deliver the proposed slice for human review; verify changes are confined to KnowledgeHub-owned client/host/fixture/docs/build areas, Modelarium remains untouched, protected stores remain intact and no implementation, publication or acceptance claim exceeds its evidence.
