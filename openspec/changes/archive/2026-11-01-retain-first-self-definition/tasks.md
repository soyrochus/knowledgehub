# Tasks

Implementation authorised by the user's request “Implement the first openspec change” on 31 October 2026. Use the adopted KH-D-01–06 selections in the [decision register](../../../../design/self-hosting-plan.md#decisions-to-resolve). The user subsequently instructed that no compiler be installed, that existing Rust 1.99.1 be used, and that platform compliance be assumed. Compiler verification below therefore uses that existing compiler; Rust 1.85 execution is not claimed. Checkboxes require the stated implementation and scoped evidence, independently of human acceptance.

## 1. Confirm adopted contracts and implementation authorisation

- [x] 1.1 Check the adopted `knowledgehub-seed/1` properties, relationship/endpoint rules, source byte-range anchors and document/graph authority map against the seed; verify each seed field and assertion has one declared owner and a validation obligation (KH-D-01/02).
- [x] 1.2 Confirm Rust 2024 / Rust 1.85 minimum, host-owned Tokio, finite blocking-job limits and cancellation/shutdown ownership; verify the public contract and dependency plan agree without exposing executor-specific resources (KH-D-03).
- [x] 1.3 Check feasibility of the adopted Linux x86_64/local ext4, nonblocking branch locks, synced generation/reference publication and process-interruption profile; verify every claimed recovery guarantee has a feasible check and that process interruption is distinct from power-loss evidence (KH-D-04).
- [x] 1.4 Check the adopted default-deny host policy, branch-scoped operation IDs, RFC 8785/SHA-256 fingerprints, explicit format versions and lifetime receipt retention; verify denied reads/retries and changed-intent examples have unambiguous outcomes (KH-D-05/06).
- [x] 1.5 Review the reconciled design, behavioural deltas and task breakdown against the adopted decisions; verify strict OpenSpec validation passes and the implementation authorisation is explicitly recorded before any code task starts.

## 2. Establish the library and public contract boundary

- [x] 2.1 Replace the Cargo starter target with one library crate and only the adopted dependencies; verify Cargo metadata exposes the intended library target and no deferred database/service dependencies.
- [x] 2.2 Introduce owned scope/identity/revision, change-intent, validation, receipt and error contracts with asynchronous I/O operations; verify a public consumer can express the seed and distinguish denial, incompatibility, invalid input, conflict, identity reuse and uncertain storage outcomes without adapter-private handles.
- [x] 2.3 Document the supported calling, policy and compatibility boundary as built; verify the documented minimal public consumer compiles on the existing Rust 1.99.1 with both current-thread and multithreaded Tokio callers (user-directed platform compliance assumption; no toolchain installation or Rust 1.85 run); verify no hidden runtime, finite job admission and orderly draining of admitted jobs.

## 3. Model and validate the sourced seed

- [x] 3.1 Implement source capture, exact original-byte digests, meaningful anchors and authored interpretations; verify exact-byte retrieval, resolving byte-range/quote anchors and rejection of missing anchors or mismatched digests.
- [x] 3.2 Implement the adopted seed profile, stable element/assertion identities, immutable revisions and document/graph authority bindings; verify valid `UseCase satisfies Requirement` content and independent invalid-type, wrong-endpoint, unresolved-revision and competing-owner examples.
- [x] 3.3 Implement snapshot-bound exact reads and non-mutating candidate validation under current policy; verify a rejected candidate leaves the accepted head unchanged and a reader pinned to A does not mix in later B.
- [x] 3.4 Document the actual seed profile and its deliberate coverage gaps; verify the documentation matches the validated public records and makes no broader ontology-support claim.

## 4. Publish guarded file snapshots and recover original outcomes

- [x] 4.1 Implement compatible create/open, coordinated writers and immutable staged generations under the adopted format/profile; verify incompatible stores and incomplete staging are never reopened as a fresh empty accepted model.
- [x] 4.2 Implement validated branch publication with expected-head and relevant graph guards, complete provenance/history and the original durable receipt; verify an acknowledged seed is complete and a same-base race across processes and same-process instances yields a single valid new head, a retryable busy outcome while locked, and an explicit stale conflict on later admission.
- [x] 4.3 Implement current-access checks followed by scoped operation lookup, exact canonical-fingerprint replay and original-result lookup; verify exact retry after further head movement, changed-intent/actor rejection, equivalent canonical property ordering, byte-sensitive content, retained receipts after later commits, revoked disclosure and no second mutation after a lost response.
- [x] 4.4 Implement the adopted persistence/recovery sequence and integrity reporting; verify interruption around staging, content persistence, head publication and acknowledgement yields the required complete prior/new state, and missing/corrupt accepted content is reported explicitly.
- [x] 4.5 Document the implemented support profile and evidence limits alongside these checks; verify no untested host, arbitrary multi-host writing or unproved power-loss guarantee is advertised.

## 5. Demonstrate and review KH-SL-001 as one outcome

- [x] 5.1 Run a public-contract walkthrough in an isolated store that creates the library self-description seed, inspects its exact source/assertion, closes/reopens and resolves the original receipt; verify identities, revisions, Markdown and original source digests remain exact.
- [x] 5.2 Run the integrated denied/invalid/concurrent/replay/fault/restart scenarios across the three capability deltas; retain build, toolchain, filesystem profile and observable results against ontology §21.10, without claiming future adapter parity.
- [x] 5.3 Review the outcome at its stated boundary and record actual findings, gaps and acceptance separately; verify the slice status and later sequence reflect the evidence and do not imply the full self-model or authority handover is complete.

## Workflow follow-up

- Archive only after the implementation, required verification and project review conditions are satisfied; retain requirement identity and the scoped evidence links.
- Use the observed KH-SL-001 result to revise the domain model and deepen the next ready slice. Full self-description handover remains KH-SL-006.
