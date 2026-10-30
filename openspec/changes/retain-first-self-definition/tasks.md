# Tasks

Planned only. No task is completed by writing these documents. Review and resolve KH-D-01–06 in the [decision register](../../../design/self-hosting-plan.md#decisions-to-resolve), update affected design/specs, and obtain explicit implementation authorisation before starting code. This is an initial breakdown to refine against those recorded decisions, not permission to settle product or durability policy during apply.

## 1. Resolve the first-implementation gate

- [ ] 1.1 Record the adopted seed profile's required properties, relationship/endpoint rules and document/graph authority map; verify each seed field and assertion has one declared owner and a validation obligation (KH-D-01/02).
- [ ] 1.2 Record the supported Rust edition/MSRV, asynchronous executor boundary and bounded file-I/O policy; verify the public contract and dependency plan agree without exposing executor-specific resources (KH-D-03).
- [ ] 1.3 Record the initial supported host/filesystem, writer coordination, persistence/publication sequence and covered failure model; verify every claimed recovery guarantee has a feasible check and that process interruption is distinct from power-loss evidence (KH-D-04).
- [ ] 1.4 Record current host-authority integration, operation-ID namespace, versioned canonical fingerprint encoding, file/profile compatibility rules and ledger retention; verify denied reads/retries and changed-intent examples have unambiguous outcomes (KH-D-05/06).
- [ ] 1.5 Reconcile this design, behavioural deltas and task breakdown against the adopted decisions; verify strict OpenSpec validation passes and the implementation authorisation is explicitly recorded before any code task starts.

## 2. Establish the library and public contract boundary

- [ ] 2.1 Replace the Cargo starter target with one library crate and only the adopted dependencies; verify Cargo metadata exposes the intended library target and no deferred database/service dependencies.
- [ ] 2.2 Introduce owned scope/identity/revision, change-intent, validation, receipt and error contracts with asynchronous I/O operations; verify a public consumer can express the seed and distinguish denial, incompatibility, invalid input, conflict, identity reuse and uncertain storage outcomes without adapter-private handles.
- [ ] 2.3 Document the supported calling, policy and compatibility boundary as built; verify the documented minimal public consumer compiles under the adopted toolchain/executor profile.

## 3. Model and validate the sourced seed

- [ ] 3.1 Implement source capture, exact original-byte digests, meaningful anchors and authored interpretations; verify exact-byte retrieval and rejection of missing anchors or mismatched digests.
- [ ] 3.2 Implement the adopted seed profile, stable element/assertion identities, immutable revisions and document/graph authority bindings; verify valid `UseCase satisfies Requirement` content and independent invalid-type, wrong-endpoint, unresolved-revision and competing-owner examples.
- [ ] 3.3 Implement snapshot-bound exact reads and non-mutating candidate validation under current policy; verify a rejected candidate leaves the accepted head unchanged and a reader pinned to A does not mix in later B.
- [ ] 3.4 Document the actual seed profile and its deliberate coverage gaps; verify the documentation matches the validated public records and makes no broader ontology-support claim.

## 4. Publish guarded file snapshots and recover original outcomes

- [ ] 4.1 Implement compatible create/open, coordinated writers and immutable staged generations under the adopted format/profile; verify incompatible stores and incomplete staging are never reopened as a fresh empty accepted model.
- [ ] 4.2 Implement validated branch publication with expected-head and relevant graph guards, complete provenance/history and the original durable receipt; verify an acknowledged seed is complete and a same-base race yields a single valid new head plus an explicit conflict.
- [ ] 4.3 Implement current-access checks followed by scoped operation lookup, exact canonical-fingerprint replay and original-result lookup; verify exact retry after further head movement, changed-intent rejection, revoked disclosure and no second mutation after a lost response.
- [ ] 4.4 Implement the adopted persistence/recovery sequence and integrity reporting; verify interruption around staging, content persistence, head publication and acknowledgement yields the required complete prior/new state, and missing/corrupt accepted content is reported explicitly.
- [ ] 4.5 Document the implemented support profile and evidence limits alongside these checks; verify no untested host, arbitrary multi-host writing or unproved power-loss guarantee is advertised.

## 5. Demonstrate and review KH-SL-001 as one outcome

- [ ] 5.1 Run a public-contract walkthrough in an isolated store that creates the library self-description seed, inspects its exact source/assertion, closes/reopens and resolves the original receipt; verify identities, revisions, Markdown and original source digests remain exact.
- [ ] 5.2 Run the integrated denied/invalid/concurrent/replay/fault/restart scenarios across the three capability deltas; retain build, toolchain, filesystem profile and observable results against ontology §21.10, without claiming future adapter parity.
- [ ] 5.3 Review the outcome at its stated boundary and record actual findings, gaps and acceptance separately; verify the slice status and later sequence reflect the evidence and do not imply the full self-model or authority handover is complete.

## Workflow follow-up

- Archive only after the implementation, required verification and project review conditions are satisfied; retain requirement identity and the scoped evidence links.
- Use the observed KH-SL-001 result to revise the domain model and deepen the next ready slice. Full self-description handover remains KH-SL-006.
