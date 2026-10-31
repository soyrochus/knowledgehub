# KH-SL-001 implementation record

31 October 2026 · Implementation authorised; human acceptance pending.

The user explicitly requested implementation of the first OpenSpec change,
`retain-first-self-definition`. This authorises the code, dependencies and
verification for that bounded change. Original proposal/manual bytes remain intact.

The user then directed: install no Rust compiler, use existing Rust 1.99.1,
and assume the deployment platform complies. No toolchain was installed.
Rust 2024 and the declared Rust 1.85 minimum remain the selected contract;
execution on Rust 1.85 is assumed rather than verified here. No toolchain pin
that could trigger an installation is introduced.

## Contract feasibility review

- KH-D-01/02: `knowledgehub-seed/1` fixes Requirement (title, statement),
  UseCase (title, actor, trigger, outcome) and exact UseCase → Requirement
  `satisfies` endpoints. Each narrative field has one document-fragment owner;
  semantic IDs, types, scope and provenance have graph ownership. Source ranges
  and quotations resolve against exact captured UTF-8 bytes. The complete seed
  is validated atomically; an empty created model has no seed-completion claim.
- KH-D-03: host-owned Tokio; owned semantic values; bounded blocking work,
  default 4 active / 32 waiting. Admitted jobs outlive cancelled callers and
  orderly shutdown closes admission and drains them. Dependencies are limited
  to serialization, SHA-256, Tokio and Linux system calls; no database/service.
- KH-D-04: stable nonblocking flock locks can coordinate separate file
  descriptions across processes and same-process opens. Complete immutable
  generations precede synced atomic head replacement. Recovery verifies the
  accepted manifest chain and never promotes orphans. Fault checks cover each
  boundary and post-replacement uncertainty. Process interruption cannot prove
  power-loss durability. The observed workspace is local ext4, Linux x86_64.
- KH-D-05/06: default-deny trusted host policy, evaluated before disclosure
  and at final admission. Operation keys are branch scoped and actor bound;
  canonical typed intent is consulted before stale-base guards. Only committed
  keys reserve receipts, which remain in the immutable accepted history.
  RFC 8785 encoding is restricted to strings, booleans, null and safe integers;
  object ordering is canonical, content bytes and operation order significant.

Strict OpenSpec validation passed before code tasks started. This review
confirms a feasible implementation direction, not library verification or
human acceptance. Actual results are recorded after implementation below.

## Implemented result

The one-crate library, fixed seed validation, owned asynchronous API and default
file adapter now implement the bounded change. See [the actual calling and
support contract](file-backend.md) and [recorded technical evidence](../delivery/evidence/kh-sl-001.md).
The public seed walkthrough and integrated denial, invalid-input, concurrent,
replay, cancellation, corruption and process-interruption checks passed on the
existing Rust 1.99.1. The README consumer compiles; formatting, Clippy with
warnings denied and strict OpenSpec validation pass.

Review found no domain-rule contradiction. Full history scans and generation
preparation under the branch lock are deliberate small-model limits. The first
seed remains authored intended behaviour, independently of the native run
evidence. Human acceptance, later slices and document authority handover remain
pending; no archive or broader implementation completion is inferred.
