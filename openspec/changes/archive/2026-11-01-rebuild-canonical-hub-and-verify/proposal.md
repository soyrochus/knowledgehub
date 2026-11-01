# Proposal

Resumed by user direction on 1 November 2026 after completion of [pack-derived-index-storage](../2026-11-01-pack-derived-index-storage/proposal.md). The earlier failed 7/11 attempt remains historical; this execution repeats checks, capture, reset and population against the current packed-index implementation. The fresh run is now technically verified; see delivery/evidence/hub-rebuild/20261101T190114Z-9b3a2f55/README.md. No human acceptance or archive is claimed.

Authorised refreshed scope · KH-SL-012 · 1 November 2026. Replaces the proposed migration outcome at the user's direction; applying this change will explicitly discard and regenerate the demonstration.

## Why

The canonical `hub/` is a generated self-description whose inputs live in the repository. The maintainer wants a fresh compact files/3 demonstration and actual test-suite evidence; preserving the old store's history through a converter adds unnecessary scope.

## What Changes

- **BREAKING for the generated demonstration:** explicitly delete all existing content under repository-root `hub/`, then create a fresh files/3 Hub through ordinary public APIs. Do not preserve or import old snapshots, revision history or operation receipts.
- Extend the capture/loader/inspector host tooling for files/3 and the new storage/neighbour/query implementation, retaining explicit semantic origin bindings for newly added project sources.
- Run the complete native test suite, doctests, formatting and Clippy against final code before deletion; bind fresh source capture to those actual successful checks using new run-specific evidence paths.
- Populate the full catalogue and current applicable KnowledgeHub self-model, create new A/B explanations and receipts, drain/reopen, scrub and perform the public and indexed-query walkthroughs.
- Mark failed/incomplete rebuilds explicitly and publish completion only after the new store's checks pass. Retain historical evidence outside `hub/` as older-run records, without relabelling it as current.
- Check the files/3 and sealed-cache compatibility boundary, pack accepted objects through existing compaction, verify packed cache hashes/quotas and cold metadata reuse after reopening, and preserve exact query results.
- Report actual new source/model counts, draft gaps, logical/encoded/allocated bytes and cold/warm lookup work. A fresh rebuild has a new replay domain and history, not migration continuity.

Outcome: KH-US-30; a maintainer applies the reset/rebuild workflow, obtains a newly verified canonical self-description and a complete test/walkthrough report. Hard prerequisites: [shared files/3 storage](../2026-11-01-share-immutable-file-storage/proposal.md), [neighbours](../2026-11-01-index-ontology-neighbours/proposal.md) and [composite queries](../2026-11-01-query-deterministic-model-impact/proposal.md), plus completed [sealed derived packs](../2026-11-01-pack-derived-index-storage/proposal.md).

Trace: ontology §§4–5/17/21.3–21.7/21.10; [file-storage proposal](../../../../source/file-storage-proposal.md) §13.3/14–15; KH-DR-02/04–13/15/17–23 and user-selected KH-D-19. This does not implement the residual KH-SL-005 migration/backup/interchange scope or KH-SL-006 authority transfer.

## Capabilities

### New Capabilities

- `canonical-hub-rebuild`: explicit deletion and fresh public-API population of the generated canonical Hub, with complete native checks and reopened/query verification.

### Modified Capabilities

None in the empty main-spec inventory. The removed conversion and recovery-backup draft capabilities were never implemented or archived. Existing completed unarchived library/self-description contracts remain the compatibility baseline.

## Impact

Authorised implementation updates host capture/verification scripts, public examples, explicit source-origin bindings, inspector and documentation. Reset is a narrowly scoped host demonstration operation, not a destructive library mutation or ordinary open behaviour. It uses existing Rust 1.99.1, Python and installed extraction tooling, with host-owned Tokio and the existing filesystem support profile; no compiler installation.

Acceptance requires all native tests/doctests and required checks to pass on exact captured code, prior Hub content to be removed, a fresh files/3 scope with new receipts, and successful full-catalogue/self-model A/B/reopen/scrub/indexed reads. New evidence records actual counts and failures. General conversion, old-receipt recovery, backup/restore implementation, atomic migration cutover, new ontology families and human acceptance are excluded.
