# Native publication history

Implemented C8 of MM-SL-02, `expose-native-model-history`. The library remains unpublished
0.10.0, with catalogue-3, semantic record/request codecs /3 and files/4. The new owned
response projection is `knowledgehub-history/1`; it changes none of those stored contracts.

A currently authorised reader can resolve the original publisher's accepted operations,
page native publications, and reconstruct original receipt vectors. Publication sequence
orders native changes. It supplies no timestamp, application event, review, execution,
model completeness or human acceptance claim.

## Authority and integration

Every lookup, initial page, continuation and release requires **both `Action::Read` and
`Action::ReadHistory`**, evaluated for `Context.actor` and the handle's actual `ModelScope`.
Create/open retain their existing permissions. Page/lookup calls recheck authority before
returning data. A token supplies no authority. Modelled roles supply no runtime grants.

Adding `ReadHistory` is a Rust source integration change for exhaustive `Action` matches.
Hosts must map it explicitly. Default policies deny it; an ordinary read grant is
insufficient. Policies that return `true` for every action already grant it and should be
reviewed accordingly. The example's reader permits only Open, Read and ReadHistory for
one scope and actor. The publisher remains a separate actor in every returned header.

`commit`, canonical request fingerprints, `OperationOutcome`, and actor-bound
`operation_result` are unchanged. Giving B access to A's publication never lets B recover
or reuse A's operation key as B. A's exact retry still returns A's unchanged original receipt.

## Exported operations and owned contracts

All methods below belong to `knowledgehub::KnowledgeHub`; request/result types are also
exported from `knowledgehub` and `knowledgehub::ontology`.

```rust,ignore
pub async fn publication(&self, context: Context, request: Publication)
    -> Result<PublicationResult, Error>;
pub async fn publications(&self, context: Context, request: Publications)
    -> Result<HistoryPage, Error>;
pub async fn publication_affected(&self, context: Context, request: PublicationAffected)
    -> Result<HistoryPage, Error>;
pub async fn continue_history(&self, context: Context, request: HistoryRequest,
    continuation: String) -> Result<HistoryPage, Error>;
pub async fn release_history(&self, context: Context, continuation: String)
    -> Result<(), Error>;
```

Construct requests with `Publication::new(anchor, operation_id)`,
`Publications::new(anchor)` or `PublicationAffected::new(anchor, operation_id)`.
Each owns `anchor: SnapshotId` and `limits: HistoryLimits`; operation-specific requests
also own `operation_id: String`. Capture an anchor with the existing `head` method once.
Every later request retains that exact snapshot. Unknown snapshots never become latest.

`PublicationResult` owns `format`, `scope`, `anchor`, `operation_id`, `outcome` and `work`.
`PublicationLookup::Published(Box<PublicationHeader>)` retains the original scope,
operation ID, actor, fingerprint version, fingerprint, previous head, resulting head and
sequence, plus `affected_count`. `PublicationLookup::NotPublishedAt` proves only that
the operation is absent from the verified anchor's ancestry. An operation published
later than that anchor also has this outcome. This never establishes current/global
operation absence or permits changing a mutation identity.

`HistoryPage` owns `format`, `scope`, `anchor`, `items`, `complete`, `initial_boundary`,
`stopped_by`, `continuation` and `work`. `HistoryItems::Publications(Vec<PublicationHeader>)`
is ordered by descending native sequence. `HistoryItems::Affected` owns `operation_id`,
`published`, `total`, `offset` and `references: Vec<ExactRef>` in original vector order,
including repetitions. `published: false` is explicit anchor-relative non-membership,
rather than a fabricated empty receipt. A published operation may have zero affected
references. Offsets start at zero and advance through continuations.

Only a verified boundary produces `complete: true`. For discovery, `initial_boundary`
identifies the initial no-receipt snapshot. Partial pages have a token and `stopped_by`
(`Items`, `Nodes`, `StorageBytes` or `OutputBytes`). They always make progress. A minimum
verification step or one indivisible value that cannot fit yields `Error::HistoryLimit`.
Complete affected pages finish their retained vector, rather than the whole history.

Collect all affected pages in order before calling
`PublicationHeader::with_affected(Vec<ExactRef>) -> Result<Receipt, Error>`. That helper
checks the count and reference validity; callers remain responsible for collecting the
actual returned sequence. A header alone is a projection, never a complete receipt.

## Limits, work and failure

| `HistoryLimits` field | Default | Hard maximum |
| --- | ---: | ---: |
| `items` | 100 | 1,000 |
| `nodes` | 256 | 8,192 |
| `storage_bytes` | 16 MiB | 128 MiB |
| `output_bytes` | 256 KiB | 4 MiB |

All limits must be positive. Invalid or overflowing values fail, with no clamping.
`HistoryLimits::MAX` explicitly selects all hard maxima. It does not override authority,
integrity or finite cursor admission. Identifiers and references are never shortened.

`HistoryWork` reports checked `visited_nodes`, `encoded_bytes_read`, `decoded_bytes`,
`storage_bytes`, `object_reads` and `output_bytes`. Logical object visits include cache
hits; file/index/block verification also consumes node visits. `object_reads` counts
logical object fetch attempts, while `FileBackend::file_work()` independently reports
physical reads. `storage_bytes` equals encoded bytes read plus decoded object/block
bytes (raw payloads included). Reads cover lease metadata, head/anchor/index/receipt/
parent/sequence checks, pack indexes and decompression. Output counts the actual
compact serde JSON envelope, including metadata, counter digits and continuation.
Pretty printing or a transport wrapper has its own additional size.

The request budget is active before acquiring the read lease, and lives only on that
bounded worker invocation. Errors and unwinding clear it. Storage never calls the
full-state or full-affected decoder to prepare a history page. Cursors retain only a
logical next snapshot/offset and a fingerprint of the full intent, with no history vector.

Pack indexes/blocks are existing indivisible storage values. A cold index/block that
cannot fit fails explicitly. Missing or invalid derived pack indexes report
`IndexNotReady`; history requests do no hidden rebuild/full-pack scan. Run supported
explicit maintenance separately. Ordinary opening/recovery and maintenance retain
their existing contracts and costs; request counters exclude a previously completed
open, but include any discovery actually triggered by the request. Cold-request tests
explicitly evict the adapter location cache after opening.

Typed errors distinguish `Denied`, policy `Unavailable`, `InvalidInput`,
`HistoryAnchorUnavailable { anchor }`, `HistoryLimit { resource, limit, work }`,
`InvalidCursor`, capacity, busy, incompatibility, integrity, storage and uncertain
recovery. None becomes authoritative non-membership. A missing original receipt or
inconsistent operation/accepted-generation/scope/sequence/parent binding fails closed.

## Pinned paging and existing model reads

Continue with `HistoryRequest::Publications(original_request)` or
`HistoryRequest::Affected(original_request)` and the opaque token. Actor, scope, anchor,
kind, operation and every limit are bound by the intent fingerprint. Altered, unknown,
released, expired or wrong-actor/scope tokens fail explicitly. There are at most 64 live
cursors shared with existing query/neighbour/impact cursors, with 30-minute idle expiry.
Use `release_history` when stopping early. Generic `release_cursor` also requires history
authority when releasing a history token.

Concurrent commits preserve the traversal's original ancestry. Tokens are process-local;
after drain/reopen or loss, start a new request at the same retained anchor. This
reproduces the same logical sequence and does not silently switch to current head.
Supported packing/index maintenance changes physical locations without changing that
logical sequence or creating publications. Retry a Busy maintenance overlap explicitly.

Read exact records at a header's `head` using existing `read` and
`Selection::Snapshot`; follow `Record.predecessor` through those exact reads. Known
baseline A/B references use existing `Selection::Baseline` and `compare`. A native
generation parent and a baseline parent are different contracts. History discovers
publications; affected pages may expose baseline references for subsequent inspection.
Ordinary selected-record queries do not enumerate all superseded baselines. A retained
baseline catalogue, generic snapshot diff and bounded comparison engine remain deferred.

## Executable walkthrough

From the repository, with existing Rust 1.99.1 and an unused absolute destination:

```sh
cargo run --release --example native_history -- "$PWD/target/native-history-demo"
cargo test --test native_history
cargo test --lib storage::shared::history
```

The example creates two unrelated fixtures (`library-loans` and `garden-irrigation`).
Actor A publishes a sourced Requirement and baseline A, a source-bound successor and
baseline B, then unrelated changes. B resolves known `history:original-A` and
`history:successor-B`, discovers the complete pinned sequence, reconstructs original
receipts, reads both exact meanings/sources and compares A/B. It appends after anchor
capture, loses a token across reopen, restarts at that anchor, packs and verifies again.
Output includes actual anchors, original fingerprints/heads, sources, limits/work,
explicit missing/denied outcomes and pending human acceptance. No application audit exists.

For a retained current-contract MM-SL-01 run, first make a quiescent disposable copy of
its complete Hub directory, then run:

```sh
cargo run --release --example native_history -- retained \
  target/rich-design/apply-equipment-acceptance/run/generation-manifest.json \
  target/rich-design/apply-equipment-acceptance/successor.json \
  "$PWD/target/disposable-equipment-history-copy"
```

This mode refuses the manifest's original store path and inspects only the supplied
copy through public APIs. It compares known original receipts, exact original/successor
definitions, baseline membership and source hashes as a different current reader.
Protected root `hub/` and `old_hub/` are never inputs. See the
[implementation evidence](../delivery/evidence/native-history/20261104-apply/README.md)
for executed commands and outcomes. Modelarium C9 and the postponed terminal client
consume this native contract in their own later changes; neither is implemented here.
