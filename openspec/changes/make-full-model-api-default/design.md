# Design

## Context

See proposal.md and the four deltas. Current lib.rs exports the seed application/contracts; ontology::OntologyHub is opt-in. ModelScope::transport fabricates Profile::meta(), Snapshot stores seed vectors plus Option<State>, and Receipt stores affected plus qualified_affected. Full operations already implement the target ontology and query capabilities.

This is an unpublished breaking cutover, order 1/4 (KH-SL-014). The user selected one full API and 0.10.0; planning is not evidence that the cutover or version bump has occurred.

## Goals / Non-Goals

**Goals:** one public full application identity, actual typed scope/state/receipt contracts, consistent 0.10.0 metadata and a compiling current consumer checkpoint.

**Non-Goals:** duplicate facades, deprecated aliases, seed feature flags, automatic catalogue registration, migration, canonical population or crate publication. Registration/profile selection remain explicit governed operations.

## Decisions

### Public surface

Rename the actual full application to crate-root KnowledgeHub. Export current typed Record/Value/ExactRef/ModelScope/Change/Selection and query contracts at the ordinary boundary. Keep module organization only where useful to the current implementation; do not export OntologyHub or the former seed contracts as compatibility paths. Context/policy, errors, content/digests, snapshot IDs, runtime limits and host composition remain current shared responsibilities.

Preserve current typed Query/Page selection and the latest neighbour/traverse/search/impact capabilities. Their legacy traversal implementation is removed in step 2; do not accidentally delete enumeration/type/ID filtering while removing that old route. No new semantic query language is required.

### Direct contracts

Use the actual ModelScope for policy and application/storage boundaries. Keep ontology/profile bindings in typed State, not synthetic bootstrap scope. Make the accepted candidate carry metadata plus a mandatory State; remove the optional seed/full union. Expose one Receipt.affected collection of ExactRef and actual ModelScope. Remove qualified_affected duplication and seed RevisionRef transport. Current original-outcome semantics remain actor/fingerprint-bound.

Adapt current generation/projection/validation plumbing in this change where required to make these contracts work. Old structs may remain private only where a still-unremoved old implementation needs them, never as new wrappers around the current path. Step 2 removes those remaining implementations. If a structure is already eliminated safely here, step 2 verifies absence rather than recreating it.

### Version and serialization boundary

Set the package and own Cargo.lock entry to 0.10.0 and keep publish=false. Record/request codecs describe semantic data; class renaming alone does not require changing them. The changed stored scope/receipt projection cannot honestly share the old files/3 contract: introduce the current files/4 identity when the new payload becomes writable, refuse older stores, and supply no conversion.

Step 2 completes the single default constructor and removes residual format dispatch. Do not write changed payloads under an old format identifier or claim old receipts decode. The last canonical files/3 instance is retained by the user as old_hub and is outside current writes.

### Buildable boundaries and safety coverage

Update affected examples, host policy signatures and test callers in the same checkpoint so all targets compile. Port immediately affected safety cases to current types before removing their seed imports. Step 4 audits completeness and runs the final suite; it is not permission to leave early changes untestable. Do not add temporary public aliases to make callers compile.

Source books and archived plans remain historical. Repository documents retain definition authority. Rust 2024 / existing 1.99.1, host Tokio with bounded off-scheduler work and the declared local-ext4 profile remain; no compiler installation or broader durability claim.

## Risks / Trade-offs

- [Renaming hides a still-live seed facade] → public import checks and source inventory must show one application type and no deprecated aliases.
- [Removing shared types also removes security/recovery data] → classify shared responsibilities, use current typed policy/receipt tests and preserve exact retries.
- [Receipt changes silently reuse an old physical contract] → introduce files/4 at the first new write and reject old stores.
- [Intermediate changes fail to compile] → mechanical consumer/test updates belong to the change that breaks them; comprehensive cleanup/evidence follows later.

## Migration Plan

No persisted-data migration. Complete this source/API checkpoint, then apply remove-legacy-storage-paths, align-current-api-tooling and verify-v010-and-rebuild-hub in order. Retain the user's old_hub unchanged; a later fresh instance supplies its own history.

