# Design

## Context

See [proposal.md](proposal.md) for the consuming outcome and scope. Applied in the KnowledgeHub workspace on 4 November 2026 under the user's explicit `apply expose-native-model-history` instruction. The decisions below record the bounded design; [the consumer guide](../../../design/native-model-history.md) owns the delivered signatures and [implementation evidence](../../../delivery/evidence/native-history/20261104-apply/README.md) records verification separately from human acceptance.

The inspected source already supplies:

- `src/contracts.rs`: `Context`, default-denying `Policy`, `Action`, original `Receipt` and actor-bound operation outcomes. Receipts include scope, operation ID, actor, fingerprint/version, previous/resulting heads, sequence and ordered affected references; they contain no publication timestamp.
- `src/ontology/api.rs`: current `operation_result` checks the caller against the original receipt actor; `compare` already compares two exact baselines. Exact historical record/artifact reads also exist.
- `src/ontology/types.rs`: `Record.predecessor`, exact `BaselineData` membership/parents and snapshot/baseline selections. Source revisions, model baselines and native generations are different concepts.
- `src/storage/shared/generation.rs`: accepted generation ancestry, per-generation original receipt links and an operation index. `original` validates receipt scope, accepted generation, sequence and parent binding, but decodes the whole receipt.
- `src/storage/shared/projection.rs`: stored receipt metadata is already separate from `affected_root`; current `decode_receipt` expands the entire affected sequence. Tree helpers already support sequence count/range reads. A bounded reader must use those selective paths rather than call full receipt decoding then slice its result.
- `src/storage.rs`: an internal storage port with `operation(id)` but no bounded history enumeration. Existing jobs, cursor lifetime/actor binding and file work instrumentation provide patterns to extend, not a requirement to reuse materialised record-query cursors.

The main public API spec mentions history broadly; branch-commit scenarios deliberately prevent another actor from recovering/reusing an original operation as their own. This design adds a separate disclosure contract, preserving that existing restriction.

## Goals / Non-Goals

**Goals:** small owned asynchronous read contracts; explicit current authority; original attribution; known-operation resolution before broader discovery; bounded reads over accepted native history; unchanged model/source/receipt meaning and observable resource costs.

**Non-Goals:** a second model of commits in ontology records, an audit database, automatic timestamps, arbitrary historical-record indexing, replacement comparison logic, transport/UI design, or a new backend/format. Existing ordinary read/compare performance is not silently redefined by this history API.

## Decisions

### 1. Give history its own permission and use-case boundary

Add proposed `Action::ReadHistory`. Each initial lookup, affected-page request and continuation checks both `Read` and `ReadHistory` for the current `Context.actor` and actual Hub/model/branch scope before storage disclosure. Opening still uses existing open permission. Policy failure remains unavailable/fail-closed. Historical actors are output attribution only.

Keep `commit`, `operation_result`, `OperationOutcome` and request fingerprinting unchanged. Do not temporarily construct the publisher's context, make actor equality optional, or route a history request through mutation recovery. Host policies with exhaustive action matches need explicit source updates; ordinary `Read` alone does not grant the new permission. All-allow trusted policies should be reviewed and documented as already granting new actions.

Alternative: interpret every ordinary read grant as full publication-history access. Rejected as an implicit expansion of disclosure, especially for actor identities and operation metadata. An explicit action keeps application permission mapping reviewable without requiring a user-account system in this crate.

### 2. Use one exact accepted snapshot as the history anchor

All history requests carry an exact `SnapshotId` within the handle's existing scope. A caller may first obtain current head through the existing API; subsequent requests and continuations preserve that captured anchor. This interface does not overload named-baseline semantics or silently resolve latest on each page.

Membership means a publication was accepted in that anchor's ancestry. A lookup returning `NotPublishedAt` proves only non-membership at that anchor. It is not `OperationOutcome::Absent`, does not establish current/global operation absence and cannot justify changing a mutation identity. An invalid, unsupported or unavailable anchor is a distinct failure.

Publication listing uses descending native sequence, newest to oldest, within that accepted ancestry. The initial empty generation has no receipt and is a completed boundary, not an invented publication. Multiple baselines in one change still form one publication; a baseline parent is not automatically a generation parent.

Alternative: use current head repeatedly or sort by consumer registration time. Rejected because both change historical meaning during paging and registration time is not a publication timestamp.

### 3. Separate small receipt metadata from affected-reference pages

Proposed Rust surface, with owned request/result types exported from the existing `KnowledgeHub` boundary:

| Proposed operation | Input | Output |
| --- | --- | --- |
| `publication` | context, anchor, operation ID, finite read limits | Original `PublicationHeader` or `NotPublishedAt`, plus anchor and actual work |
| `publications` | context, anchor, finite page limits | Descending publication-header page, boundary/completeness and opaque continuation |
| `publication_affected` | context, anchor, operation ID, finite page limits | Exact affected-reference page, total count, retained offset/order and continuation |
| `continue_history` | context, original request intent and opaque token | The corresponding typed page at the same anchor |
| `release_history` | context and opaque token | Bounded explicit cursor release under the same actor/scope rules |

Names are proposed implementation names, not assertions of existing API availability. Keep requests typed and errors structured; publish the final exported signatures in the example/docs before declaring this change complete.

`PublicationHeader` retains every scalar field of the original `Receipt` unchanged and adds `affected_count` as a read projection. It never calls a truncated reference set a complete receipt. The original full receipt can be reconstructed from that header plus all affected pages in the original order, including any repetitions present in the authoritative vector. Do not change canonical intent or rewrite historical receipts to produce this projection. No extra publication timestamp, inferred review, TestRun or application audit record is returned.

A versioned `knowledgehub-history/1` owned response envelope may document these new serialisable read projections; it is not a new semantic record/request codec or a Modelarium wire protocol. Required identifiers are never shortened to fit output budgets.

Alternative: return `Receipt` directly and slice later. Rejected because an operation may affect many references and `decode_receipt` currently materialises that entire sequence.

### 4. Make work and output limits explicit

Use the following proposed defaults/hard maxima, independently validated rather than silently clamped:

| Limit per request/page | Default | Hard maximum |
| --- | ---: | ---: |
| Returned headers or affected references | 100 | 1,000 |
| Visited generation/index/sequence nodes | 256 | 8,192 |
| Storage bytes read/decompressed for history verification | 16 MiB | 128 MiB |
| Encoded owned response bytes, including metadata | 256 KiB | 4 MiB |

Use checked arithmetic and positive minima. Count actual storage work, including accepted-anchor resolution, receipt checks, sequence/index reads, pack-block reads/decompression and hidden preparation triggered by the request. Cache hits still count logical verification visits; physical read counters separately identify actual I/O. Reuse/extend file work measurements so bounds can be asserted independently of response size. Do not count only emitted items while scanning all history internally.

A page reports actual work, explicit exhaustion/continuation and whether its requested range is complete. An unfinished but progressing traversal is partial; only a verified boundary is complete. A value or minimum verification step that cannot fit is an explicit resource-limit failure. Do not create endless empty continuations. Read budgets may reject a large indivisible stored metadata object or cold pack block; callers can retry with a larger permitted limit. No unbounded background rebuild is authorised to make the page appear cheap.

The defaults are design choices for the initial release, not measured performance claims. Verify them on loose and packed fixtures; changing these documented numbers requires updating this design and tests, without weakening finite bounds. Ordinary backend opening/maintenance has its existing separate contract; report those costs separately and do not move request work there merely to evade limits.

### 5. Keep continuation state small and authority-bound

Extend existing process-local cursor handling with a distinct history position: verified anchor/generation position for publication pages, or operation/affected-sequence offset for reference pages. Do not retain vectors of all publications or affected references.

Fingerprint scope, calling actor, exact anchor, operation kind/ID, ordering and all limits/query intent. Tokens remain opaque and expose no object addresses or physical paths. Continue/release operations validate actor/scope and current permission; tokens grant no authority. Reuse the existing finite admission policy: at most 64 live cursors per handle and a 30-minute idle expiry, shared with or explicitly accounted alongside other cursor families rather than an additional unbounded pool.

A process restart loses tokens, not retained history. Consumers restart a query at the same exact anchor. Expired, released, unknown, tampered, wrong-actor/scope/intent and capacity outcomes are explicit and never return a page from current head. Concurrent commits or compaction cannot retarget the logical anchor; reload physical positions from authoritative identity where necessary or fail explicitly.

Alternative: permanent resumable tokens containing storage offsets. Rejected because it exposes layout, complicates authority/maintenance and is unnecessary for a restartable read API.

### 6. Reuse retained storage with bounded verification

Add internal storage-port operations for header lookup, accepted-publication walking and affected-sequence range/count. Keep public DTOs independent of backend generations, object IDs, pack locations and directory structure.

For the file backend, resolve the accepted anchor using the ancestry structure, then use its operation root for known IDs. Reuse the integrity checks in `original`, factored so metadata verification does not expand `affected_root`. Verify the original scope, operation key, fingerprint/version validity, accepted generation, sequence, receipt binding and predecessor head before reporting authoritative results. Validate affected counts/ranges through existing sequence trees; returned references must equal the original stored sequence.

For discovery, walk the accepted generation parent chain from the anchor, reading each original receipt header selectively and stopping at the work/output limits. Preserve the next logical position in the cursor. Generation indexes and parent metadata are authoritative inputs, not a model-content scan. Initial anchor resolution and lookup paths consume the same budgets; inability to verify within a budget is explicit rather than a reason for an unbounded scan.

This is a selective read projection over existing files/4 data. It needs no new committed fields, history store or event rewrite. Factor helper code without changing the existing full-receipt decoder/recovery semantics. Read-only means no semantic publication; ordinary lock/recovery/derived-index mechanisms keep their existing documented effects. If existing storage cannot satisfy this design without a format change, revise the proposal before implementing that change; do not regenerate handovers or add old-format support implicitly.

### 7. Compose with current lineage/baseline primitives

A consumer reads a known exact record at a publication's resulting snapshot and follows `predecessor` through the existing exact read API. Known baselines resolve their retained membership and use existing `compare` for A/B; this change does not introduce a generic snapshot diff or claim existing comparison is paged.

The publication API lists native publications, not every historical record or every baseline revision directly. Known baseline references come from handovers/model records; affected-reference pages can expose earlier baseline records for subsequent exact inspection under caller limits. Document that ordinary selected-record queries do not enumerate every superseded baseline. A dedicated retained-baseline catalogue or bounded comparison engine is deferred unless separately scoped; it is not a hidden prerequisite for known A/B history inspection.

Native publication, source revision, baseline, review and application activity stay distinct. Later Modelarium C9 maps these owned contracts through its own host and UI. `add-terminal-model-client` is postponed until this change's actual implementation/evidence, then its consumer policy and command scope can be aligned before apply; no terminal implementation is added here.

### 8. Verify the consumer boundary with two actor identities

Add focused history tests and a runnable public-API example using fresh small current-contract fixtures. Actor A publishes an initial sourced definition/baseline, coherent successor and later unrelated changes; actor B has current read/history permission but no publish/recovery impersonation. Cover the lookup and paged discovery paths even when no application registry/audit exists.

Use `tests/public_contract.rs` policy/retry patterns, `tests/ontology_contract.rs` A/B/source assertions, `tests/shared_storage.rs` packing/reopen checks and internal fault hooks rather than building another application harness. The new history oracle may compare a small fixture's known original receipts; production queries must not use a full-state/history oracle.

Test revocation and policy failure at continuation, actor/scope/intent/token changes, initial empty history, later-than-anchor operations, concurrent appends, count/order preservation, large affected sets, encoded limits and actual work. Inject receipt/ancestry and storage faults on disposable stores; no failure may become absence. Existing exact retries, baseline/source preservation and maintenance checks must still pass.

An implementation evidence report should name actual build inputs, API signatures, permissions, defaults/maxima, counters, fixture anchors and outcomes. If an MM-SL-01 retained handover is available, exercise a disposable copy; the portable small equivalent remains committed/reproducible. Unavailable large runs, Linux/cross-host checks and human review remain explicitly unexecuted. No test touches protected root `hub/` or `old_hub/`.

## Risks / Trade-offs

- **History permission integration:** exhaustive Rust `Action` matches need adjustment. Document the source change and deny-by-default behavior; Modelarium/terminal map permissions later in their own scopes.
- **Bounded output can hide unbounded decoding:** metadata/range reads must bypass whole affected-sequence expansion and expose storage counters. Exercise a large receipt and packed cold reads.
- **Corrupt history can resemble absence:** verify accepted anchor/receipt/parent bindings and propagate failures. A lookup at an older anchor never establishes current mutation absence.
- **Operational costs differ by packing/cache:** count actual work and report preparation separately. Finite resource failure is preferable to pretending every minimum request can complete.
- **No native timestamps:** sequence provides ordering; callers must not label registration/edit UI times as original publication times.
- **Scope growth into a general audit browser:** limit the contract to accepted branch publications and their exact references. Baseline diff UI, generic retained-record search and application events remain separate.

## Migration Plan

Implementation is authorised separately in a KnowledgeHub session. First add the new permission/contracts and selective reads, then verify lookup, affected pages, discovery, cursors and regressions before updating docs/example/evidence. Preserve current catalogue-3, record/request /3, files/4, existing publication receipts and exact sources. No migration, canonical-store refresh or compatibility reader is planned.

Deploy consumers only after the native contract has actual passing evidence. This change precedes the postponed terminal change and later Modelarium C9 integration; planning completion alone must not unblock those runtime acceptance claims. Rollback of this read extension uses the earlier library/consumer build without rewriting stored models. Do not archive, synchronise main specs, publish or transfer model authority as part of this planning request.
