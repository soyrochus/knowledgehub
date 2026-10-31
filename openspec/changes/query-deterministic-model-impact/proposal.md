# Proposal

Planning draft · KH-SL-011 · 31 October 2026. No new query implementation is claimed.

## Why

A human or external AI asking which database tables are linked to a story should not issue one tool call per hop or download the whole graph. The library needs a bounded deterministic read that finds starting records, composes fast actual-neighbour lookups and returns exact dependency paths with honest coverage.

## What Changes

- Add versioned type/literal-phrase search over selected fields and exact document-owned narrative, using a rebuildable trigram candidate index and exact final verification.
- Add declarative mixed-direction typed traversal policies and ordered step plans with alternatives; execute deterministic batched frontier expansion using actual neighbour indexes, never precomputed routes or inferred edges.
- Combine start selection, traversal and compact record/source-fragment projection in one public read and snapshot-bound continuation. Return matched starts grouped, deduplicated targets and exact assertion/property witness paths.
- Make ambiguity, no match, budget truncation, unsupported/external mapping and insufficient traceability explicit. Candidate dependencies do not assert that a wording edit requires a schema migration.
- Supply an isolated rent-story fixture with explicit logical-to-physical mappings; no hardcoded rent-to-table routing or new universal database-table ontology type is introduced.

Outcome: KH-US-28–29; a caller submits a story phrase and a finite typed policy and receives candidate physical schema targets and their evidence in one request, plus pagination only when required. Hard prerequisite: [index-ontology-neighbours](../index-ontology-neighbours/proposal.md), transitively shared files/3 storage. Existing plain `Query`/`Page` behaviour remains supported through its own contracts.

Trace: ontology §§5/8/11/17/21.3/21.7/21.10; [file-storage proposal](../../../source/file-storage-proposal.md) §7.6–7.7; KH-DR-03/04/07/12/15/18/19/21/22, KH-D-18.

## Capabilities

### New Capabilities

- `model-text-search`: versioned deterministic exact/ASCII-insensitive phrase entry with indexed candidate verification and narrative bindings.
- `bounded-model-traversal`: generic typed query-time multi-hop expansion with finite policy states, budgets and exact witness paths.
- `composite-model-queries`: one-view search/traverse/project reads, grouped ambiguity and evidence/coverage results.

### Modified Capabilities

None in the empty main inventory. Add richer operations alongside the completed unarchived `model-navigation` contract, retaining its original maximum depth four, assertion semantics and cursor behaviour. No claim that fulltext or mixed-step plans are already supported.

## Impact

Extend owned query/result types and application/navigation orchestration, reuse files/3 point reads and derived adjacency, add a bounded derived text index, and retain the shared cursor admission/policy/drain boundary. No service, SQL backend, arbitrary query-language interpreter, scripts, LLM, embedding dependency or remote content acquisition is added.

Acceptance requires deterministic phrase/field/narrative matching, mixed directions, paths beyond four hops through the additive API, cycles/alternative policy states, late associations, one-view pages, revoked access, incomplete mappings, fan-out and byte/path budgets, and instrumentation showing batched indexed work. Search modes/default limits and fixture physical-schema classification are explicit proposed choices in design. Generic named query templates, synonym/Unicode folding beyond the first modes, automatic schema-change inference and canonical reset/rebuild or general conversion are excluded. Modelled rent examples are fixtures, not claims about KnowledgeHub's actual persistence tables.
