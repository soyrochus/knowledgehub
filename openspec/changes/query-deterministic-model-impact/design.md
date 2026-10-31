# Design

## Context

See [proposal](proposal.md). Existing `Query` only filters type/IDs and traverses one direction up to four hops. Native `Value::Narrative` already pins a source revision and anchor, so search must index resolved owned fragments rather than only graph strings. The catalogue has `MappingRule` selectors, `DataSchema.native_artifact` and `DataStoreDefinition.schemas`; it does not declare a universal SQL-table node. Neighbour extraction and selective reads come from KH-SL-010.

Trace source file-storage §7.6–7.7, ontology §§8/11/21.7 and KH-D-18. These are additive read operations in the same crate under existing Rust 1.99.1, host Tokio, finite jobs/cursors/current policy and the supported file profile. They do not execute modelled bodies or change definition authority.

## Goals / Non-Goals

**Goals:** one pinned-view read composing deterministic literal entry, typed actual-neighbour expansion and compact witnessed projection; explicit coverage/resource status.

**Non-Goals:** LLM/embedding search, inferred edges or causality, a rent-specific query engine, unrestricted query languages, scripts, automatic external acquisition or automatic database migration decisions.

## Decisions

### Closed text projection and matching

Add `literal-utf8/1` and `literal-ascii-fold/1`. The second folds only ASCII A–Z to a–z bytewise, without Unicode normalisation; every other byte remains exact. Require a nonempty phrase of at least three UTF-8 bytes, with a proposed 4096-byte query maximum. Unsupported shorter/mode requests fail explicitly instead of triggering an unbounded scan.

Build versioned trigram postings from explicitly selected Text and resolved Narrative property paths. Candidate intersections are followed by exact contiguous substring verification in the same field/fragment; matching grams across different fields cannot create a phrase hit. Exclude unselected history, arbitrary artifact bytes, metadata IDs and complete source PDFs unless an explicit supported field projection includes their narrative. Cache only compact positions/references where possible and rehydrate exact source spans for snippets. Incremental updates track changed selected fields and document/anchor dependencies. Snapshot/schema/extractor/profile and field selection bind the index; missing/stale indexes use the readiness rules of the neighbour change.

This native index avoids another dependency and keeps exact matching portable. Rich Unicode folding, token/synonym analyzers and external search engines require new versioned modes rather than silently changing these results.

### Finite policy data and deterministic frontier evaluation

An owned plan supplies start exact references or a typed text selector; explicit match policy; target exact type/ID predicates; association categories; direction/selector/type step constraints; and finite budgets. Support (a) a repeated finite allowed-step set and (b) an ordered list of steps with finite alternatives. Validate all selectors/types against the pinned ontology before execution. Step data is closed, contains no callbacks/code, and requests no mutation or remote lookup.

Batch each frontier through the neighbour operation. Ordered plans track `(exact subject, step position)`; repeated policies track subject plus policy state. Do not globally discard a subject reached in a different state. Sort starts/transitions/postings by the documented UTF-8 exact-reference/selector order; use breadth-first expansion and deterministic deduplication. Keep distinct origins as evidence even for the same endpoint. Finding a close target does not terminate discovery of other permitted targets within bounds. A requested shortest-witness projection selects the first deterministic shortest path per start/target; it is not an all-paths completeness claim.

### Proposed finite defaults

| Budget | Default / maximum |
| --- | --- |
| Hops in the additive API | 8 / 32 |
| Plan positions and alternatives | 32 positions / 64 total alternatives |
| Matched starts | 100 / 1000 |
| Visited subject-policy states | 2000 / 10000 |
| Examined postings/associations | 5000 / 10000 |
| Text candidates verified | 2000 / 10000 |
| Returned targets per page | 100 / 1000 |
| Witnesses per start/target | 1 / 8 |
| Projected response bytes | 256 KiB / 1 MiB |

Exhaustion, each reached work bound, omitted additional witnesses and response paging are separate statuses. Work counters include rejected candidates, duplicate attempts and filtered edges. No caller can bypass bounds with projection or alternatives. If one item exceeds the byte cap, return a compact exact reference with explicit omitted content and a bounded follow-up option; do not loop with an empty continuation page.

### One-view results and cursors

Resolve head once when requested; return its exact snapshot ID. Search, traversal, snippets and target hydration all use that pinned view. Group targets/witnesses under every matching start; match policy is `all_grouped` or `require_one`, with ambiguous candidates returned/refused explicitly rather than guessed. A compact record dictionary avoids repeating the same record along several paths. Include exact assertion/property origins, source refs, field/fragment byte ranges, query/analyzer versions, budget counts and coverage status. Full artifacts are not downloaded by default.

Share the existing backend-wide 64-cursor/30-minute-idle pool. Cursors bind actor, scope, pinned selection, full plan/projection/budgets, matching modes and evaluation frontier. Every continuation rechecks current disclosure. Store bounded frontier/visited/evidence state; if capacity cannot retain it, return a bounded capacity result rather than silently change evaluation. Old Query and continuation fingerprints remain separate and retain depth-four semantics.

### Rent example and honest impact scope

Use an isolated fixture with the existing UserStory/Requirement/UseCase/function/entity types, MappingRule source/target selectors and explicit assertion links. Register a fixture-local subtype of DataSchema with graph-owned database/schema/table coordinates and bind its native DDL artifact; query target predicates name that exact type. This extends test vocabulary through the ordinary registration API without modifying the built-in catalogue or hardcoding a SQL-table class in the engine. The route uses inverse formalises/satisfies, forward invokes/operatesOn/hasRoot and typed property occurrences through explicit mappings. Add alternate mappings/tables and late links to prove generality.

Report `candidate_dependency`, witnessed paths and model-declared gaps. No path means no witnessed dependency under the chosen policy; absent mappings/external resolution prevent claims of complete real-world impact. Structural/recorded coverage is qualified, not proof that authors captured every dependency. A wording-only edit does not prove that a schema migration is required. The external human/AI interprets that meaning.

## Risks / Trade-offs

- [Text dependencies change without an owner record change] → include exact narrative source/anchor bindings in incremental invalidation and compare against direct fragment verification.
- [Route or path enumeration explodes] → state-aware visited sets, finite examined-work and witness budgets, deterministic compact evidence.
- [A missing mapping looks like no impact] → distinguish exhausted trace search from unknown domain coverage, with explicit gaps and external references.
- [One tool call sends a huge graph] → project only requested targets/paths/snippets with deduplicated records and byte limits.
- [Cache/ranking changes alter results] → pinned mode/schema versions and exact verification; initial modes have no relevance ranking or synonym inference.

## Migration Plan

Add request/result contracts alongside legacy Query/Page. Exercise literal entry first, then ID-start mixed traversal, then the composite fixture over files/3. Record deterministic results across reopen/rebuilt indexes and concurrent head movement. No rewrite of existing stores or current self-model is needed. The later rebuild change deletes the old demonstration, regenerates the current canonical model and verifies indexed reads without inventing rent concepts inside it.
