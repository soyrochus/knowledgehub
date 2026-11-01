# Proposal

## Why

The unpublished 0.1.0 crate exposes the original seed as its default KnowledgeHub while full ontology functionality is opt-in. The user selected a single full API with no backward-compatibility facade and requested version 0.10.0 before a new canonical rebuild.

## What Changes

- **BREAKING:** make the full typed implementation the crate-root KnowledgeHub and expose its current model/query contracts as the ordinary API. Remove the seed facade and the OntologyHub alias instead of deprecating them.
- Use one public model scope, a mandatory full state and one qualified affected-reference collection. Remove public seed bootstrap types, synthetic profile transport and dual seed/full receipt fields.
- Retain complete ontology registration, schema conformance, revision/baseline/history, current-policy reads, deterministic selection/neighbours/traversal/search/impact and recovery guarantees.
- Set Cargo.toml and the knowledgehub Cargo.lock package entry to 0.10.0; keep publish=false and existing dependencies/toolchain.
- Adapt the current adapter and affected callers sufficiently for this cutover to compile and demonstrate behaviour. Do not defer broken consumers to later changes or add temporary public aliases.

## Capabilities

### New Capabilities

- `full-model-public-api`: one asynchronous full-model entry point, typed contracts and complete current operations.
- `library-release-metadata`: consistent unpublished 0.10.0 package/build/capture identity.

### Modified Capabilities

- `source-bound-definitions`: remove mandatory seed bootstrap and seed-only typed definitions.
- `ontology-definitions`: remove the commitment to preserve the older seed API and stores.

## Impact

Order 1/4, proposed KH-SL-014. Actor: library consumer/host integrating a full governed model. Completion: ordinary imports use the full API, old facade names are absent, package metadata is 0.10.0 and focused full-model checks pass. No prerequisite change; follow with [remove-legacy-storage-paths](../remove-legacy-storage-paths/proposal.md).

Trace ontology §§4–8/17/21.2–21.4/21.7/21.10 and KH-DR-01–13/15–19. Changes touch lib/application/contracts/model, ontology API, current adapter/receipt projection, source bindings and affected compile-time callers. Later changes remove residual storage implementations, finish tooling and run final evidence/rebuild. No new adapter, migration, crate publication, compiler installation, automatic ontology registration or authority handover. Existing Rust 1.99.1/Tokio/local-ext4 decisions remain; minimum-platform compliance is the user's assumption, distinct from observed checks.

