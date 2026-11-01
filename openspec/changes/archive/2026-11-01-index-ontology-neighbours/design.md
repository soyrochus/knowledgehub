# Design

## Context

See [proposal](proposal.md). `navigation::materialise` collects/sorts all selected assertions and scans them for each dequeued subject. The public `Query` exposes one direction and assertion-signature filters; `Page` has no property-occurrence identity. `Value` already has closed reference, artifact and narrative variants and registered nested schemas. Reuse those facts rather than parse arbitrary strings or add editable shadow assertions.

This change depends on files/3's private root-read port. Source [file-storage §§7.4–7.5](../../../../source/file-storage-proposal.md) and KH-D-17 own the proposed generic direction. Existing Rust 1.99.1, host Tokio, finite jobs/cursors, current scoped policy, local ext4 and document/graph ownership stay unchanged.

## Goals / Non-Goals

**Goals:** direct batched actual-neighbour lookup, generic new vocabulary, compact exact origins, incrementally maintained view-bound indexes and root-selective hydration.

**Non-Goals:** anticipating query routes, causal inference, text similarity, transitive closure materialisation, per-type storage indexes, eager baseline-membership edges or a SQL dependency.

## Decisions

### Endpoint-first nested maps

Use outgoing/incoming persistent maps keyed by exact endpoint, then selector, then ordered posting structures. A flat hash of `(endpoint, unknown selector)` cannot efficiently discover an endpoint's actual selector groups; nested maps can. Each posting carries opposite reference, association origin and occurrence path; optional opposite-type metadata is verified against the pinned record. Two postings per actual fact keep entry growth linear. High fan-out still costs enumeration and requires budgets.

Assertion selectors are exact relationship-definition references, with assertion revision as origin. Property selectors are owning type revision plus escaped schema-field path; the posting's occurrence path includes concrete list indices. Preserve parallel assertions and repeated occurrences. Sort by selector kind/reference/path, opposite qualified reference, origin and occurrence path, using UTF-8 lexical components. Both-direction self-loops are returned once per origin/occurrence with both direction markers, not duplicated accidentally.

### Generic extraction and explicit fact scope

Walk closed tagged values/schema definitions recursively with finite depth. Index actual `Ref` values under their declared field descriptors; artifact/narrative/representation/provenance links receive distinct categories and native origins. A newly registered type needs no hardcoded storage branch. `Any` fields only expose explicitly tagged references, never inferred string IDs. Administrative baseline membership remains in its persistent sets and is not expanded into default graph postings.

Default fact scope uses selected assertion records and reference fields of selected owners. Explicit closure scope includes the pinned baseline's retained supporting owners; it never means “all latest history”. Both include exact targets named by eligible facts, even when a target is a historical pin. Vertex availability and selected-ness are returned separately. An unselected old assertion cannot leak into the default current graph. Type filters pin exact definitions; subtype inclusion is explicit and validated through the registered hierarchy.

### Derived index readiness and incremental change

Fingerprint scope, selected/supporting membership, fact scope, ontology/profile, categories and extractor grammar as `neighbour-index/1`. A manifest binds this fingerprint to both roots and a completed-build checksum/inventory. Persist under the derived cache namespace using known map/sequence grammars extended with typed index keyspaces/items. It has no authority over accepted knowledge and is excluded from required backups.

Admission computes posting removals/additions from changed selected owners/assertions, including unselect/select and successor operations. Unchanged nodes share storage. Publish a complete cache manifest only for its exact recovered view. Failed post-commit index installation cannot undo or replace the original committed receipt; the next reader sees index-not-ready and rebuilds. A narrower baseline's first build may scan its declared fact scope once. Cold preparation has its own finite work/space limits and is measured separately.

Missing/wrong-version caches are rebuilt under a declared preparation budget or return explicit not-ready/capacity. Corrupt cache data is discarded/rebuilt only from verified accepted records, with a diagnostic; authoritative record corruption fails integrity. A cached root for another view is never served as an empty or complete result. Maintenance can evict a complete index under coordination and readers can reconstruct it. No disk cache is required to restore knowledge.

### Additive owned API and compatibility

Introduce owned neighbour request/result types, working name `Neighbours`, accepting multiple starts, direction, selector/category/type/fact-scope filters, projection and finite budgets. Initial defaults/maxima: starts 100/1000, page 100/1000, examined postings 10000 maximum and projected bytes 256 KiB/1 MiB. Reuse current 64-cursor/30-minute-idle instance capacity and current policy per page; fingerprints bind full intent and pinned view. Unsupported modes fail before query execution.

The initial fast operation requires files/3 capability and fails explicitly on unsupported legacy stores. Existing files/2 Query/continuations keep their original contracts. On files/3, legacy assertion-only traversal can use an adjacency adapter preserving its established sorting/filtering and maximum depth four; new property/category behaviour stays additive. Point hydration reads only returned/required records, never calls full public `snapshot()` to discover one node.

## Risks / Trade-offs

- [Schema evolution reinterprets old links] → selectors/extractor pin exact definitions and version; register successors without altering old view indexes.
- [Index misses a late reverse link] → update both directions from one fact delta; compare incremental/rebuilt results with an independent scan oracle.
- [Baseline or provenance fan-out overwhelms useful routes] → explicit categories/fact scope, stable paging and examined-posting/byte budgets.
- [Caches hide O(E) cold work] → separate ready-index lookup metrics from first build, and expose not-ready/capacity rather than return partial completeness.
- [Older pinned endpoints are replaced silently] → use exact references through every posting/hydration and assert old/new baseline fixtures.

## Migration Plan

Keep legacy stores/canonical `hub/` intact. Exercise new operations in disposable files/3 stores and generated registered vocabularies. Remove the per-node full-edge scan only after old Query/continuation equivalence passes. Record posting/object reads while unrelated edges and retained generations increase; no absolute latency promise substitutes for structural evidence. Canonical deletion/regeneration and complete-suite verification belong to the later rebuild change.
