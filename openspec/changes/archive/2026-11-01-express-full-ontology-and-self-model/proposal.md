# Proposal

Planning draft · 31 October 2026 · Full ontology and then KnowledgeHub's own model.
This proposes implementation; it does not implement or accept either outcome.

## Why

The user wants the library to express the entire System Definition Ontology in
`source/system-definition-ontology-proposal.pdf`, then express KnowledgeHub
itself using that vocabulary. The implemented seed accepts only two element
types and one relationship; a reliable seed store is not full ontology support.

## What Changes

- Supply an exact versioned catalogue covering all 17 kernel concepts, all 128
  types in the 14 families of §6, all 32 relationship declarations in §8, the
  specialised contracts of §§9–14, and validation obligations V-01–17. Preserve
  source definitions and separately identify proposed schema refinements.
- Introduce explicit property schemas, typed values, endpoint roles, qualifiers,
  multiplicities, applicable constraints and completeness obligations. Ontology
  and profile definitions become inspectable versioned data; accepting arbitrary
  labels or JSON bags does not satisfy this change.
- Represent exact element revisions, assertions, anchors and representation
  bindings as addressable subjects, including the kernel-to-assertion edges in
  §8. Retain original binary artifacts, including the PDF, alongside independent
  Markdown captures and exact provenance.
- Add coherent successor revisions, exact named baselines and bounded typed
  navigation needed to retain and inspect the complete self-model. Preserve
  authority, policy, expected-head guards, canonical retries and recovery.
- Deliver a public consumer that registers the complete ontology, creates a
  sourced KnowledgeHub system model, reads requirement → behaviour → component
  → test/evidence connections, revises one definition, closes/reopens and recovers
  both exact baseline explanations.
- **BREAKING, explicit opt-in:** new record contracts and `knowledgehub-files/2`
  are needed for ontology records, revision selections and binary artifacts.
  Existing files/1 seed records, hashes and original receipts remain supported
  through their original codec. No automatic in-place migration or replay-domain
  transfer is proposed.

The [coverage inventory](ontology-coverage.md) enumerates the actual vocabulary,
schemas and source rules. The [self-model map](self-model-map.md) identifies
concrete records, sources, expected links and executable acceptance questions.

Two ordered outcomes belong to this change:

1. **KH-SL-007 — Express the complete ontology:** a model author discovers every
   source definition, submits valid instances and relationships from every family,
   receives meaningful violations for invalid examples, and reopens the same
   exact records. Prerequisite: the KH-SL-001 consistency/recovery foundation.
2. **KH-SL-008 — Express KnowledgeHub itself:** the maintainer creates and
   navigates the scoped self-model, revises it under two explicit baselines and
   retrieves real evidence separately from intended behaviour. Prerequisites:
   KH-SL-007 plus this change's revision/baseline/navigation operations.

The [slice plan](../../../../delivery/slice-plan.md) owns the revised sequence.
Source basis: PDF/Markdown draft 0.2, §§2–17 and 21.2–21.7/21.9–21.10;
domain rules KH-DR-01–15 and proposed KH-DR-16–19 in the owning analysis.

## Capabilities

### New Capabilities

- `ontology-definitions`: versioned, discoverable, complete ontology/type/
  relationship/profile/constraint definitions and measurable source coverage.
- `ontology-conformance`: schema-driven typed records, specialised graph rules,
  addressable kernel subjects, binary artifacts and purpose-specific completeness.
- `model-baselines`: immutable successor lineage and exact coherent named
  baselines with reproducible comparison.
- `model-navigation`: bounded, permitted, snapshot/baseline-bound typed selection
  and relationship traversal for inspecting the ontology and self-model.
- `knowledgehub-self-description`: repeatable public loading and inspection of
  KnowledgeHub's purpose, domain, behaviour, architecture and scoped evidence.

### Modified Capabilities

None in the main-spec inventory, which is currently empty. This change depends
on the completed but unarchived `retain-first-self-definition` contracts
(`source-bound-definitions`, `branch-commits`, `file-snapshot-recovery`), preserving
their seed-profile behaviour. It does not silently archive or duplicate them.

## Impact

Authorised implementation would extend `src/contracts.rs`, replace seed-only
checks in `src/model.rs`, add public registration/discovery/revision/baseline/
navigation operations in `src/application.rs`, and evolve the private storage
port and file codec. The current history checks that require append-only ID
sets must distinguish immutable history from selected revisions. The single
crate, bounded host-owned Tokio work, current host policy and single-host ext4
publication boundary remain. Use existing Rust 1.99.1; install no compiler and
keep the user-directed platform assumption separate from execution evidence.

Acceptance requires exact catalogue coverage, meaningful valid/invalid fixtures
for every declared type/signature and each deterministic rule, public self-model
walkthroughs, revision/baseline/navigation consistency, v1 replay preservation,
and extended race/retry/interruption checks on the recorded file profile.

Excluded: executing arbitrary modelled functions/processes/AI, a product UI,
database adapters, services/SDK/FFI, publication, general-purpose query languages,
portable exchange/backup and automatic document-authority handover. All ontology
families remain representable even where they do not apply to KnowledgeHub's
runtime. No fictional screens, deployments, test successes or acceptance records
are required to fill the self-model.

Schema details absent from the source are proposed decisions KH-D-11–15 in
the [decision register](../../../../design/self-hosting-plan.md). They are explicit
review subjects; source draft status and human acceptance remain separate.
