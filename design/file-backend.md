# Calling and support boundary

Historical implementation record. The current 0.10.0 contract is documented in
[the full API guide](typed-ontology-api.md) and [current file storage](files3-guide.md).
Earlier seed APIs/decoders and the runnable files/1 fixture have been removed; this
record retains the scope of its original implementation and evidence.

KH-SL-001 · `knowledgehub` 0.1.0 · 31 October 2026.

The embedded API implements the fixed sourced seed and its guarded file commits.
The host authenticates actors, supplies current scoped policy, owns the Tokio
runtime and declares the support profile. Modelled actors and roles confer no
runtime privilege. `DenyAll` is the default policy implementation; policy errors
fail closed. There is no account system or inferred human approval.

## Public operations

Host composition constructs `backends::files::FileBackend` with an absolute
store path, explicit `FileOptions::local_ext4()` and optional finite `JobLimits`.
`KnowledgeHub::create` exclusively creates one Hub/model/branch; `open` requires
the same exact `Scope` and supplied `Profile::seed(ontology_revision)`.
Existing damaged stores are never recreated. One store contains one branch in
this slice. The storage port is internal, not a stable third-party adapter API.

`head`, `read_snapshot`, `read_element`, `read_assertion`, `read_document` and
`read_source` return owned values. Every new read uses current Read permission.
An explicit `SnapshotId` selects an accepted historical snapshot, including
genesis. Orphan generation IDs are not accepted snapshots. Source/document
bytes, exact revisions, interpretations and vocabulary definitions are visible
through these operations without adapter handles or physical paths.

`validate` requires Validate permission and returns a report tied to the
request's exact accepted base. It never commits and may inspect a historical
base. `commit` requires current Commit and OperationResult permissions, including
a final check under writer coordination; that check defines publication
authorisation. New intent must match the current expected head and every exact
revision/absence guard. The entire resulting graph is validated at admission.
No automatic rebase occurs. This slice supports additions; successor authoring
and named baselines remain KH-SL-002.

`operation_result` requires current OperationResult permission and returns the
original actor-bound receipt, or established absence after complete recovery.
Errors distinguish Denied, Unavailable, Capacity, Busy, Incompatible,
InvalidInput, Invalid (rule/subject/message violations), Conflict, GuardConflict,
OperationReuse, NotFound, AlreadyExists, Integrity, Storage and Uncertain.
Busy and Capacity admit no mutation and can be retried. A stale conflict needs
explicit reconsideration. After Uncertain or a lost response, resolve the same
operation before changing intent or identity. Integrity/unavailability never
establish absence. Actor mismatch refuses reuse without returning the receipt.

## Seed profile and authority

`knowledgehub-seed/1` contains exact, canonically hashed Requirement, UseCase,
satisfies and seed-integrity definitions, pinned to the host's exact ontology
source revision. `Profile::definition` returns their references, or None for
an unsupported name. Profile modification/evolution is rejected.

| Record | Validated obligations and owner |
| --- | --- |
| Scope | Graph-owned nonempty Hub/model/branch/owner and exact ontology/profile. |
| Requirement | Exact type, identity, revision, inherited snapshot scope and authored interpretation owned by graph; title and statement each owned by one nonempty exact document fragment. |
| UseCase | Same graph fields; title, actor, trigger and outcome each owned by one nonempty exact document fragment. |
| satisfies | Graph owns identity, revision, exact type, inherited scope, from, to and interpretation. Exactly one exact UseCase source and Requirement target resolve in the same snapshot. Each element may participate zero or more times. |
| Document | Original UTF-8 Markdown owns narrative. Graph owns stable document/fragment IDs, revisions and UTF-8 ranges. No editable normative graph copy of narrative exists. |
| Source | Original UTF-8 bytes are retained verbatim. Graph owns source ID, original SHA-256/length, exact revision and named range/quotation/section anchors. |
| Interpretation | Graph owns exact source/anchor, nonempty text/method, qualified interpreter and Authored status. No approval is inferred. |
| Snapshot/history/receipt | Graph infrastructure binds exact scope, selected records, parent, sequence, actor, operation intent fingerprint and original affected revisions. |

Each element/assertion has an explicit `authority` vector. Missing/duplicate
owners, unsupported normative fields, unresolved fragments, unsupported exact
types and wrong/missing endpoints fail validation. Kernel ownership is fixed
by the profile. Identities are caller-supplied opaque nonempty strings qualified
by scope, never filenames or content hashes. Actor IDs are host-qualified.
Within a model, record identities cannot collide or be overwritten. Historical
snapshots and their complete records remain immutable.

An empty scope is permitted and `seed_complete()` is false. Every non-genesis
commit must retain the complete Requirement/UseCase/satisfies seed, including
the captured pinned ontology with a section 21.4 anchor. Structural conformity
does not assess the truth of an authored interpretation. The runnable example
captures the exact repository ontology Markdown and cites its complete §21.4.

Coverage gaps: other ontology types, successor authoring, arbitrary profile
registration, named-baseline comparison, broad query/traversal, review workflow,
interchange/import/export, external references, complete self-model and authority
handover are deferred. This does not satisfy the full §21.10 implementation
programme. Repository documents retain their existing authority.

## Execution and shutdown

Rust 2024, declared minimum Rust 1.85; user-directed verification uses existing
Rust 1.99.1 and assumes platform compliance. Rust 1.85 has not been run here.
No toolchain is installed or automatically selected. The file adapter requires
a host-owned Tokio 1.x runtime and supports current-thread and multithreaded
callers. It creates no runtime. Semantic values expose no Tokio resources.

All application file work, hashing, serialization, recovery and graph validation
run in `spawn_blocking` jobs. Each backend instance defaults to 4 active and 32
waiting jobs. Host-configured limits are finite: 1–65536 active, 0–65536 waiting.
Total admission is bounded before spawning; exhausted capacity returns Capacity.
Separately constructed instances have independent limits but coordinate writers
through the same branch lock. Construction/capture/sealing helpers are synchronous
host-side preparation; prepare large candidate values off the async scheduler.

After admission, a detached coordinator owns queue admission, candidate data and
the blocking job through completion, even if the caller drops/aborts its future.
`hub.drain().await` or `backend.drain().await` closes admission across clones
and waits for all queued/running work. The host must keep its runtime alive
until draining finishes. Cancellation is not rollback. Runtime destruction
before draining is outside the orderly-shutdown contract.

## Filesystem and persistence

The first-slice Linux protocol below is historical context. The current sole files/4 adapter also implements the [native APFS profile and checked creation/location separation](macos-file-backend.md); its evidence and pending Linux run remain separate.

The supported target is coordinated single-host Linux x86_64, local ext4,
ordinary files and one filesystem for staging/generations/references. The host
declares these assumptions; the adapter checks architecture, filesystem magic,
the ext4 mount type in `/proc/self/mountinfo`, ordinary managed directories/files,
same device, explicit versions and exact scope/profile. External modification,
network filesystems, multi-host writers and other platforms are unsupported.
The path cannot prove hardware behaviour or that every external writer cooperates.

A stable never-renamed branch lock uses nonblocking exclusive `flock`. Each
acquisition uses a separate file description, coordinating processes and separate
same-process opens. Creation uses a stable sibling creation lock and exclusive
directory creation. Busy returns immediately; no blocking worker waits for a
writer lock. Exact reads/recovery also briefly acquire that lock and may return
Busy. Lock files and unaccepted staging may remain after process interruption.

1. Persist creation metadata, stable locks and containing directories.
2. Preallocate an opaque commit ID and original receipt. Stage a complete graph,
   exact `.md` documents, original source artifacts and receipt. A manifest binds
   every required object's SHA-256 and byte length, with no circular self-hash.
3. Under the branch lock, recover authoritative history, check current policy,
   consult original committed intent before stale-base checks, then validate
   guards and the full resulting candidate. Required records publish together.
4. Sync all staged files, manifest, nested directories and staging parent.
   Install the unique immutable generation; sync staging and generations parents.
5. Write and sync a temporary reference in the reference directory, atomically
   replace the head there, then sync that directory before acknowledging.
   Any failure after replacement reports Uncertain, never confirmed non-commit.
6. Recovery follows only the accepted reference and hash-bound parent chain,
   verifies every required object/record/receipt and immutable history, then
   re-establishes reference and directory persistence before reporting outcomes.
   It never promotes orphans or falls back over missing/corrupt history.

Implementation stages the complete candidate under the lock. This conservatively
serializes preparation as well as publication, keeping admission and final
validation simple for small models. Storage growth and full-history scans are
deliberate first-slice limits; no garbage collection, expiry or pruning exists.

## Encoding and compatibility

Crate/API 0.1.0, `knowledgehub-files/1`, `knowledgehub-seed/1`,
`knowledgehub-request/1` and the exact ontology revision are separate versions.
Unsupported metadata/profile/fingerprint versions fail explicitly; no automatic
migration or stable pre-1.0 API guarantee is offered.

Request fingerprints use SHA-256 over a tagged RFC 8785 canonical typed envelope
containing actor, full scope/profile/ontology, operation ID, expected head,
revision/absence guards and ordered operations. Every submitted content value
is verified against its original-byte SHA-256/length before publication; the
fingerprinted envelope binds those references instead of duplicating raw bytes.
The supported JSON subset has strings, booleans, null, arrays, objects and safe
integers only. Object keys sort by UTF-16 code units; no whitespace or Unicode
normalisation changes strings. Floating point, unsafe integers, duplicate keys,
invalid Unicode and unknown typed fields fail strict `ChangeSet::from_json`.
Operation/array order and every authored content byte remain significant.

Semantic revisions use separately tagged `knowledgehub-record/1` encoding
(excluding their own revision); source revisions/digests hash original bytes.
Definition revisions use `knowledgehub-definition/1`. The accepted reference
hashes the manifest; manifests hash graph JSON, Markdown, sources and receipt.
Formatted graph JSON is never a substitute for original-source hashing.

The operation key is `(Hub, model, branch, operation ID)`, independently bound
to actor and fingerprint. Every committed generation retains its original receipt
and hash-bound parent history for the store lifetime. Rejected/staged operations
reserve no key. Lookup and exact replay apply current permissions first.

## Evidence boundary and references

See [implementation evidence](../delivery/evidence/kh-sl-001.md) for actual runs.
SIGKILL checkpoints and injected persistence failures establish process-interruption
and lost-response behaviour on the recorded host. They do not establish power-loss,
kernel-crash, media durability or support for other filesystems/adapters. Human
acceptance and bootstrap authority handover remain separate pending decisions.

Primary mechanism references: [RFC 8785](https://www.rfc-editor.org/rfc/rfc8785.html),
[Tokio spawn_blocking](https://docs.rs/tokio/1.53.1/tokio/task/fn.spawn_blocking.html),
[flock](https://man7.org/linux/man-pages/man2/flock.2.html),
[fsync](https://man7.org/linux/man-pages/man2/fsync.2.html) and
[rename](https://man7.org/linux/man-pages/man2/rename.2.html).
