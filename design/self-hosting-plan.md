# Bootstrapping KnowledgeHub's own definition

Draft 0.6 · 1 November 2026 · Full ontology/self-model, compact persistence, indexed neighbours and deterministic queries technically verified at their recorded boundaries. Canonical regeneration is next; human acceptance and authority migration remain pending.

This plan owns the bootstrap sequence, representation authority and migration gates. It references the [domain analysis](domain-analysis.md), [slice plan](../delivery/slice-plan.md), [ontology proposal](../source/system-definition-ontology-proposal.md) and [Practical Design Manual](../source/practical-design-manual.md). The manual guides how we discover and specify the product; the ontology defines what the Hub can represent and preserve.

## Intended result

KnowledgeHub will be developed using its own governed system model. Begin with reviewable repository documents, implement a small complete library path, and progressively load the library's own definition through that public API. Later development changes start from the governed model and produce exact, version-bound specification representations. Implementation and execution remain in their appropriate code/build/test tools.

Uploading a directory of prose is only source capture. The self-model also needs identified concepts, typed relationships, ownership, revisions, provenance, selected baselines, explicit gaps and links from promises to actual evidence. The first seed can be small; it must already preserve those distinctions for its selected scope.

## One definition, several representations

| Material | During bootstrap | After an explicit scoped handover |
| --- | --- | --- |
| Original proposal and manual under `source/` | Immutable inputs to the analysis; retain original revisions and hashes. | Remain source evidence. A new source edition is a successor capture, never an edit of earlier evidence. |
| Domain analysis, journeys and slice definitions | Repository documents own these authored definitions while they are being reviewed. | Selected fields may remain document-owned or become graph-owned; the recorded authority map names exactly which. |
| OpenSpec specifications and change artifacts | Own the bounded behavioural delta used to direct an authorised implementation. | Bind each contract to a Hub model revision/baseline. Keep narrative document-owned where appropriate; graph-owned fields appear only as derived representations or references. |
| Graph records | The small seed is submitted through the implemented public API; broader mappings remain planned. | Own stable identities, typed assertions and the structured fields explicitly assigned to them. Derived extractions remain marked as derived. |
| Rust source, builds and tests | Git/build/test tools own implementation and actual execution evidence. | Continue to own them; the Hub retains exact versions, bindings and scoped evidence records. |
| Review and acceptance records | User adoption of the first-slice recommendations is recorded below; other draft content remains subject to review. | Hub review decisions bind exact scope. Actual acceptance and test results remain distinct from stored design status. |

Authority is per field, relationship and baseline scope. It need not move all at once, and moving definitions into the Hub does not require converting every paragraph to graph fields. A document-owned definition edited through the API must update its bindings coherently; a graph-owned field's Markdown rendering cannot become a competing editable definition.

## Bootstrap stages and gates

### B0 — Define the library outside the running library

Bootstrap foundation. Repository documents continue to own these definitions. Keep original sources intact. Author domain language, responsibility boundaries, rules, journeys, slices and the first specification as documents. Record what is sourced, what is proposed, and which adoption choices remain open. Stable document identifiers referenced in the analysis will become import mappings, not filenames used as semantic identities.

Exit evidence is a reviewed planning set and resolved first-implementation decisions. Structural validation can show consistency of the documents; it cannot complete domain review or authorise implementation.

### B1 — Retain a small self-description through the public API

Current implemented stage, pending human acceptance: KH-SL-001 supplies the smallest end-to-end path. See [technical evidence](../delivery/evidence/kh-sl-001.md) and [actual contract](file-backend.md). A versioned bootstrap profile declares `Requirement`, `UseCase` and `UseCase satisfies Requirement`, plus the shared identity/revision, document/provenance and model-scope structures they require. Only the source constraints applicable to this profile are required initially; unsupported types or profile versions are rejected explicitly.

The first seed has the following semantic roles. The [public fixture](../examples/support/mod.rs) implements this mapping and submits it through ordinary API operations in isolated stores. This establishes neither the complete self-model nor human acceptance.

| Seed role | Representation and source | Required meaning |
| --- | --- | --- |
| Library system model | Definition of the `knowledgehub` library; ontology §21.1. | Describes the library being built, separate from the Hub instance that stores it. |
| Atomic-change requirement | One `Requirement`; ontology §21.4. | An admitted change set must preserve a coherent all-or-nothing outcome and original operation result. |
| Commit-change-set use case | One `UseCase`; ontology §§21.3–21.4. | Describes intended behaviour; does not prove a working commit implementation. |
| Satisfaction assertion | One identified `UseCase satisfies Requirement` assertion; ontology §8 and the cited behaviour sections. | Exact endpoint revisions and assertion provenance remain inspectable. |
| Narrative definition | A small Markdown document explaining these records with stable fragment bindings. | Authoritative narrative fields and graph-owned typed bindings are declared separately. |
| Source capture and interpretation | Exact ontology Markdown bytes, SHA-256, meaningful section anchors and authored interpretation. | Source claim and its interpretation are distinguishable; no approval or runtime result is inferred. |

The calling host submits the entire seed under one authorised, guarded branch change. Opening, reading, concurrent rejection, retrying the original identity and recovery all use ordinary public operations. The bootstrap seed must not be installed by directly patching accepted snapshot files.

This is already limited use of KnowledgeHub for its own definition. It is not the complete self-model, a reviewed authority handover, or an exemption from its own consistency contract.

### B2 — Express the full ontology, then the governed self-description

The user clarified that the library must express the entire ontology explicitly
defined in the supplied PDF and then describe KnowledgeHub itself. The next
proposed sequence is KH-SL-007 full ontology → KH-SL-008 self-model, detailed in
[express-full-ontology-and-self-model](../openspec/changes/express-full-ontology-and-self-model/proposal.md).
The [coverage inventory](../openspec/changes/express-full-ontology-and-self-model/ontology-coverage.md)
enumerates all kernel/type/signature/rule obligations, including schemas and
visible refinements where the source leaves detail open. Full support cannot
mean accepting a larger label list or deferring unused UI/AI families.

The [self-model map](../openspec/changes/express-full-ontology-and-self-model/self-model-map.md)
names actual library records, source/code/test bindings, exact A/B explanations
and public inspection questions. It covers all applicable library responsibilities,
while retaining proposed domain analysis, historical evidence and inapplicability
status. Earlier KH-SL-002–005 core revision/baseline/navigation needs are pulled
forward where necessary; broader review, portable exchange and routine authority
handover retain their own gates in the owning slice plan.

Capture both source documents and retained PDF bytes with independent original-
content hashes and explicit representation relationships; similar filenames do
not establish byte or semantic equivalence. These additions now exist in the
opt-in files/2 API; files/1 remains a separate compatibility path. Actual runs and
coverage are recorded in [full-ontology evidence](../delivery/evidence/full-ontology.md).
The loaded model remains a draft with explicit gaps. Technical completion does
not close the B3 authority/acceptance/reconstruction gates.

The user requested a visible canonical dogfooding instance at the repository
root. The retained files/2 store is currently in [`orig_hub/`](../orig_hub/README.md), retaining
the complete verified history and source/evidence baseline. Its move from `target/`
does not reauthor that baseline or transfer definition authority. The public
`inspect_self_model` example opens it and reads A/B and original receipts; fresh
walkthroughs use separate paths. Subsequent repository changes require explicit
API authoring to become new stored model revisions.

Map the authored model through the following families as needed:

| Project definition | Ontology family / type examples |
| --- | --- |
| Purpose, actors and scope | `BusinessGoal`, `DesiredOutcome`, `Actor`, `SystemModel`. |
| Domain language, responsibility and rules | `UbiquitousLanguageTerm`, `BoundedContext`, `AggregateDefinition`, `EntityType`, `Invariant`. |
| Journey and need | `Journey`, `JourneyStep`, `UserActivity`, `UserTask`, `UserStory`, `Requirement`, `Scenario`. |
| Library behaviour and contracts | `UseCase`, `CommandDefinition`, `QueryDefinition`, `FunctionDefinition`, `ErrorDefinition`. |
| Development choices and delivery | `DesignDecision`, `OpenQuestion`, `DeliverySlice`, `ChangeDefinition`, `ImplementationTask`, `ReleaseWorkPackage`. |
| Verification and acceptance | `AcceptanceCriterion`, `TestCase`, `TestRun`, `EvidenceArtifact`, `AcceptanceRecord`, `ReviewDecision`. |

The two supplied sources do not settle every type property or relationship in this mapping. The selected profile must define endpoint, multiplicity, ownership and completeness rules explicitly before it is used. Mark missing coverage as gaps rather than adding generic links and claiming the self-description is complete.

### B3 — Hand over a selected scope and use it for the next change

KH-SL-006 establishes this gate. Import and review a candidate baseline containing the selected library model and exact repository/source bindings. Compare its identities, content, relationships, provenance, reviews and gaps against the bootstrap documents. Export/reconstruct it independently, including the declared recovery history where the backup profile promises it.

Record an explicit handover decision naming:

1. The exact source repository revision and Hub baseline being compared.
2. Which fields, documents and relationships are document-owned or graph-owned.
3. Which earlier editable definitions become references or derived views.
4. The reviewer, current status, unresolved gaps and conditions for reversing or superseding the handover.
5. How to rebuild or restore the Hub, and which representation owns work during recovery.

After this gate, demonstrate one subsequent change: follow a need to its rule and slice, prepare exact behavioural deltas, review them, implement outside the Hub when authorised, attach actual build/test evidence, and record scoped acceptance. A newly stored requirement or a successful import cannot mark its behaviour implemented.

If the Hub is temporarily unavailable, restore the last verified backup or declare an explicit document-authority recovery branch. Reconcile it against the base before re-import. Editing a stale derived export and silently promoting it is outside the write protocol. A broken definition of the library does not change the already compiled recovery implementation.

## Decisions to resolve

Decision record · 30 October 2026 · Owner: the user. In this chat the user accepted the KH-D-03 recommendation and instructed adoption of the recommendations from the other proposals. KH-D-01–06 now have adopted directions and the concrete first-slice selections below. These selections refine the source proposal; they do not revise it. They authorise planning choices, not implementation, successful verification, acceptance of a running library or authority handover.

Later-slice directions are retained, but details that have not been proposed remain open. The original proposal/manual remain draft source material; this decision does not approve every illustrative type or future capability.

| ID | Status and selected direction | Consequence / remaining work |
| --- | --- | --- |
| KH-D-01 | Adopted: versioned kernel + `Requirement`, `UseCase` and `satisfies`; fixed seed contract below. | Implement and verify only this profile first; reject arbitrary types/properties. |
| KH-D-02 | Adopted: Markdown owns narrative; graph owns semantic IDs, types and assertions; coherent API submission owns admission. | Bind fields/fragments exactly; later authoring round-trip remains KH-SL-002. |
| KH-D-03 | Adopted: Rust 2024, fixed Rust 1.85 minimum, host-owned Tokio 1.x for the file backend; bounded blocking jobs. | User's later directive selects existing Rust 1.99.1, no compiler installation and assumed platform compliance. Record actual runs separately; no minimum-compiler execution claim. |
| KH-D-04 | Adopted: Linux x86_64/local ext4, coordinated single-host writers, immutable generations and synced atomic reference publication. | First evidence covers process interruption and lost responses; power-loss and other filesystem/host support await separate evidence. |
| KH-D-05 | Adopted: trusted host supplies stable actor identity and current scoped policy; deny by default. | No built-in account system or privilege from modelled roles; no mandatory human-review workflow in the seed. |
| KH-D-06 | Adopted: separate explicit versions, SHA-256, RFC 8785 canonical request encoding, branch-scoped operation IDs and lifetime committed-receipt retention. | Implement compatibility, replay and integrity checks under the contract below; no pruning in the first slice. |
| KH-D-07 | Direction retained: bounded portable snapshot queries. KH-D-14 proposes the exact-type/ID and directional subset needed next; broader operators remain open. | Initial seed has exact-ID reads only. Review the proposed finite ordering/cursor contract before implementing the new subset. |
| KH-D-08 | Direction retained: host-configured review of exact revisions, separate from commitment and evidence. Detailed roles, outcomes and dispute policy remain open. | Resolve before KH-SL-004. |
| KH-D-09 | Direction retained: distinct model-exchange and recovery-backup profiles. Collision mapping and replay-domain continuity remain open. | Resolve before KH-SL-005; ordinary exchange cannot silently promise recovery continuity. |
| KH-D-10 | Direction retained: progressive self-description with an explicit scoped handover. The next change proposes a full applicable self-model; exact acceptance/handover baseline remains open. | KH-SL-008 proposes the model, while KH-SL-006 still owns actual authority transfer. Documents retain authority until that decision. |

### Full ontology and self-model refinements — proposed 31 October 2026

The user's desired breadth and order are explicit: full source ontology, then
KnowledgeHub's own model. The source remains draft 0.2 and is not rewritten.
The following technical/schema details are concrete proposals for that scope,
not already adopted user decisions or implemented features. Their detailed
definitions have one owner in the linked change's design/inventory.

| ID | Proposed selection | Consequence / review boundary |
| --- | --- | --- |
| KH-D-11 | `system-definition-ontology/1`, explicit shared typed schemas and per-type refinements covering 17 kernel kinds, 128 profile kinds, all 32 catalogue signatures plus separately sourced/refined prose edges; `knowledgehub-self-description/1` selects applicable completeness. | The coverage inventory defines every row; source omissions and chosen field requiredness remain visibly proposed refinements. No family may be silently omitted to declare completion. |
| KH-D-12 | Small trusted meta-schema, first-class exact ontology/kernel addressing, closed deterministic validators and independent binary/original source capture with resolving locator manifests. | One owner per subject/property; no shadow assertion/revision nodes, arbitrary script execution or inferred PDF/Markdown equivalence. Host-derived extraction observations remain distinct from review. |
| KH-D-13 | Append-only successors plus selected normative revision maps and immutable exact named baseline closure; explicit relationship pins and deterministic comparison. | Preserve original explanations and review scope; no automatic latest lookup, rebase or repinning. Historical supporting artifact revisions are not competing normative owners. |
| KH-D-14 | Exact type/ID filters, qualified forward/reverse traversal and stable UTF-8-ID ordering; default/max depth 1/4, page 100/1000, total visited subject/edge caps 10000 each. Opaque actor/query-bound instance cursors: 64 active, explicit release, 30-minute idle expiry, invalid after close/reopen. | Concrete next subset of KH-D-07. Current policy applies to every page; limits and invalid-cursor outcomes are public. Broader operators and durable cursors are not promised. |
| KH-D-15 | Explicit opt-in files/request/record version 2; preallocated opaque revision tokens separate from canonical content digests, enabling cyclic exact references. Keep original version-1 codecs, seed semantics, hashes and receipts exact. | Full catalogue/self-model use fresh v2 stores. No silent in-place migration, circular hash scheme or transfer of old committed keys to a new replay domain. |

### Compact storage and indexed navigation — staged decisions, 31 October 2026

The user requested the file-format proposal, selected actual local-neighbour lookup and requested bounded OpenSpec changes. They subsequently replaced canonical migration with a fresh rebuild, explicitly deleting previous hub content and performing the test suite when that change is applied. “Implement the first open change” authorised KH-SL-009, verified in [its evidence](../delivery/evidence/files3/README.md); the subsequent explicit request to continue with index-ontology-neighbours authorised KH-SL-010, verified in [its evidence](../delivery/evidence/neighbours/README.md). On 1 November, “go with the next change” authorised KH-SL-011, verified in [query evidence](../delivery/evidence/impact/README.md). KH-SL-012 was subsequently applied and reached an incomplete files/3 dataset before its derived cache exhausted its allocated quota; it is now deferred at 7/11. KH-SL-013 implements and verifies sealed derived packs against that existing dataset without another rebuild, acceptance or authority handover. [File-storage draft 0.3](../source/file-storage-proposal.md) owns the proposed physical format; KH-SL-009–012 own outcome order.

| ID | Proposed selection / explicit user direction | Consequence and review boundary |
| --- | --- | --- |
| KH-D-16 | Implemented/technically verified under KH-SL-009: explicit files/3, shared immutable objects/trees, compact lossless baseline/anchor data, fixed 1 MiB chunks, raw/zstd block packs and lifetime history/receipts. | Existing record/request digests and files/1–2 remain exact. [Guide](files3-guide.md) records finite bounds; [evidence](../delivery/evidence/files3/README.md) records existing Rust 1.99.1, local ext4, process faults and size/read work. Power-loss/other-filesystem evidence and human acceptance remain separate. No automatic conversion or canonical rebuild. |
| KH-D-17 | Implemented/technically verified under KH-SL-010: owned batch direct neighbours, exact assertion/property/native selectors, selected/closure scopes and both-direction persistent adjacency. | [Guide](neighbour-api.md) fixes extractor/index grammar 1, numeric request/cursor/cache bounds and explicit files/3 capability. [Evidence](../delivery/evidence/neighbours/README.md) covers generic/inherited/tagged extraction, late/successor facts, cache faults/eviction, current-policy cursors, legacy equivalence and measured lookup/storage work. Existing assertion endpoint guards remain; no per-model index code, inferred edges, all-pairs closure, impact search or human acceptance. |
| KH-D-18 | Implemented/technically verified under KH-SL-011: deterministic literal entry and typed bounded query-time traversal with grouped candidate dependencies and exact evidence. | [API contract](deterministic-query-api.md) fixes literal-utf8/1 and literal-ascii-fold/1, closed repeated/ordered plans, finite defaults, grouped/require-one matching, exact source spans, shared cursors and explicit coverage. [Evidence](../delivery/evidence/impact/README.md) records 92 native tests plus one doctest, isolated rent/alternate vocabulary, cache reconstruction, policy/cursor bounds and one-call work measurements. No semantic AI search, automatic migration interpretation or canonical regeneration. |
| KH-D-19 | User-selected replacement: delete the generated canonical hub's previous contents and populate a fresh files/3 self-description from current checked repository inputs. Run the complete native suite and public/reopen/scrub/indexed checks. | New replay domain, snapshots and receipts; do not claim migration or import old history. Preserve older evidence outside hub/ as historical. The unfinished attempt is now deferred at 7/11; no further reset occurs under KH-SL-013; general conversion/backup/restore remains deferred. |
| KH-D-20 | Implemented/technically verified under KH-SL-013: sealed variable-size derived packs, bounded compression blocks, finish-before-manifest publication and catalog-stamp-bound shared locations. | Preserve 256 MiB neighbour / 128 MiB text regular-file quotas and immutable accepted files. [Guide](packed-cache-storage.md) fixes the physical protocol and cold/warm counters; [evidence](../delivery/evidence/packed-cache/README.md) records 100 native tests, one doctest, 11 host tests, canonical public queries/reopen/scrub and unchanged original outcomes. KH-SL-012 stays deferred; no reset, recapture or authority handover. |

KH-D-16's map leaf 64, sequence leaf/fanout 128, normal pack block 1 MiB/512 entries, fixed content chunk 1 MiB and zstd level 3 are the implemented bounded profile. KH-D-17 adds disposable neighbour maps/postings, the shared 64-cursor/30-minute pool and 256 MiB/32-view neighbour cache bounds. KH-D-18 adds a 128 MiB/32-view text cache and documented finite query defaults (eight hops, 2,000 states, 5,000 combined work, 100 starts/page groups and 256 KiB response). Cache allocation is measured separately from encoded authoritative storage. These choices remain distinct from source ontology requirements and human acceptance. A minimal-closure authoring helper and broad portable interchange remain separate semantic scope.

These proposals preserve the adopted single-crate/host/runtime/file support
boundary. An implementation request must identify this change; completed
planning alone neither adopts every refinement nor proves support or acceptance.

### KH-D-01/02 — Seed contract and definition authority

The bootstrap profile is `knowledgehub-seed/1`, pinned with the exact ontology and type/constraint definition revisions. It is supplied by the host through supported create/open operations; it is not injected by editing accepted snapshots. Arbitrary profile registration/evolution is deferred. Required kernel structures are Hub/model/branch scope, identity/revision references, profile/type/constraint definitions, document/fragment and authority bindings, source artifact/revision/anchor, authored interpretation, snapshot/history and original-operation receipt. These infrastructure structures are distinct from the two system-definition element types.

| Record | Required fields and owner |
| --- | --- |
| Model scope | Graph-owned stable Hub/model IDs, branch identity, scope owner and exact ontology/profile bindings. |
| `Requirement` | Graph-owned stable identity, exact revision, type and model scope; document-owned nonempty title and requirement statement through exact fragment bindings; required source/interpretation binding. |
| `UseCase` | Same identity/type/scope rules; document-owned nonempty title, actor, trigger and outcome through exact fragment bindings; required source/interpretation binding. |
| `satisfies` assertion | Graph-owned stable assertion identity/revision, exact relationship-type revision, one exact `UseCase` source endpoint and one exact `Requirement` target endpoint in the same model/snapshot; required supporting interpretation/source binding. |
| Source and interpretation | Graph-owned source identity, original-byte digest, exact source revision and resolving anchor; interpretation text, interpreter identity, method and `authored` status. Approval is absent until an actual review exists. |
| Markdown and fragments | Original UTF-8 narrative bytes, stable document/fragment IDs and exact revision bindings. The document owns narrative values; graph bindings own their location/identity. Extracted narrative is derived. |

Both element types may participate in zero or more `satisfies` assertions; each assertion has exactly one endpoint of each specified type. A complete first seed contains the atomic-change Requirement, Commit Change Set UseCase and at least one such assertion. Creating an empty scope is permitted; it is not a completed seed. Each required narrative field binds one nonempty document fragment. Source anchors use an exact UTF-8 byte range plus expected quoted bytes and a human-readable section label; boundaries and quotation must resolve against the captured revision. The seed captures the exact ontology Markdown, with the section 21.4 atomic-change anchor and any additional cited sections. Missing fields, unknown normative properties, unsupported types, invalid endpoints, unresolved revisions and competing owners are validation failures.

The host retains its candidate until a receipt establishes commitment. Initial authoring submits the complete document, typed records, authority map and provenance together through the ordinary API. Accepted snapshots cannot be edited in place. Managed working copies, successor authoring and profile expansion remain later slices.

### KH-D-03 — Toolchain and executor policy

Keep edition 2024 and Rust 1.85 as the declared minimum compiler target. Implementation adds `rust-version` and constrained dependencies. On 31 October 2026 the user explicitly directed no compiler installation, use of existing Rust 1.99.1 and assumed platform compliance. This supersedes the planned toolchain pin and minimum-compiler execution requirement for this change. No `rust-toolchain` selection is added; verification uses the existing compiler, and Rust 1.85 execution is not claimed. An incompatible dependency requires an explicit revised decision, not an unnoticed minimum increase. The locally inspected Rust 1.99.1 is environment information, not minimum-version evidence. Edition 2024 starts at Rust 1.85 ([Rust guide](https://doc.rust-lang.org/stable/edition-guide/rust-2024/)); Tokio 1.54 currently declares that same minimum ([Tokio README](https://docs.rs/crate/tokio/1.54.1/source/README.md)). No dependency is installed by this decision.

The host owns and drives the Tokio runtime for the initial file backend; KnowledgeHub creates no hidden runtime. The core model, storage port and public semantic request/result types do not expose Tokio resources. Runtime independence of those types does not mean this adapter can run on every executor. Verify both Tokio current-thread and multithreaded callers; blocking file work must not run on their async scheduler threads.

The adapter submits complete bounded file jobs through `spawn_blocking`, with finite host-configurable active and queued limits per backend instance (initial defaults: 4 active, 32 waiting). A full queue returns an explicit capacity/unavailable outcome before submission, with no mutation. Writer coordination is still per branch across instances/processes. Any publication job owns its writer lock and required data until its publication/recovery boundary finishes, even if its caller stops waiting. The host keeps the runtime alive and drains admitted jobs on orderly shutdown. Started blocking jobs cannot be aborted ([Tokio documentation](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html)); cancellation is not rollback and original-operation lookup resolves uncertainty. Unbounded CPU-heavy validation/hashing on the async scheduler is also excluded.

### KH-D-04 — Initial file support and publication protocol

The first target is Linux x86_64 with local ext4, ordinary files, and staging/generations/branch references on one filesystem. All writers use the supported library protocol; accepted content is immutable and no external process modifies it. Network filesystems, multi-host writers and other platforms are outside this initial target. The host declares this support profile; open checks observable filesystem/format requirements and rejects detected mismatches. It cannot infer honest hardware durability or the absence of external writers from a path alone. Evidence must record kernel, filesystem/mount configuration and relevant storage assumptions.

Use a stable, never-renamed lock file per branch with an exclusive OS advisory `flock` spanning final admission, guards, publication and recovery; scope creation uses equivalent store-level coordination. Separate opens must coordinate in-process instances too. Readers pin immutable generations. The adapter uses nonblocking lock acquisition and returns a retryable busy outcome when another writer holds the lock; it does not occupy the blocking pool indefinitely waiting for a lock.

1. Create/open metadata under coordination; creation uses exclusive creation and never overwrites an existing/damaged store. Persist new metadata and containing directories before acknowledging creation.
2. Prepare a unique full candidate generation. Include manifest, exact document/source/graph bytes, provenance, history and original receipt; hash every required content object. The manifest binds them without a circular self-hash. Preallocate the commit identity and construct its receipt before publication.
3. Acquire the branch lock, check current policy, consult committed operation history, recheck expected head and all applicable graph guards. Finalise the accepted candidate against that base. A rejected candidate remains unaccepted.
4. Persist all candidate files and directories, install the immutable generation under its unique name, and persist both sides' containing directories if moved. Required artifacts must already be persisted and manifest-bound. Cross-filesystem publication is refused.
5. Write and sync a temporary branch reference in the reference directory, atomically replace the head, then sync the containing directory. Retain the lock until this sequence finishes. Acknowledge only after all required persistence calls succeed. Failure after replacement is an uncertain outcome, never an assertion that the old head still owns the branch.
6. Recover under writer coordination from the accepted reference and its complete hash-verified generation/history. Ignore orphan staging/generations as unaccepted; do not promote them on directory scan. Re-establish required reference/directory persistence before reporting a recovered durable receipt; if this cannot be established, preserve an uncertain outcome. A valid recovered new head contains the original receipt. Missing/corrupt accepted objects cause an integrity error, not silent fallback or a fresh empty model.

Directory persistence is separate from file persistence ([fsync](https://man7.org/linux/man-pages/man2/fsync.2.html)); atomic reference replacement ([rename](https://man7.org/linux/man-pages/man2/rename.2.html)) and advisory writer coordination ([flock](https://man7.org/linux/man-pages/man2/flock.2.html)) supply different parts of this protocol. This is an engineering selection, not proof it has been implemented correctly.

Initial verification must cover process termination at each publication boundary, cancelled waits, lost acknowledgements, two processes and two same-process instances racing from one head. Power-loss, kernel-crash and hardware/media-corruption durability are not established by those tests. Any broader advertised guarantee needs additional scoped evidence. No automatic reclamation of accepted history is included.

### KH-D-05/06 — Policy, formats and original intent

The trusted host authenticates the caller and supplies an immutable qualified actor ID plus a current policy evaluator. Evaluate scoped create/open, read, validate, commit and operation-result permissions with default denial. Recheck write authority at final admission and disclosure authority on every new read/retry/lookup; policy failure is unavailable/denied, never allow. The admission check is the authorisation point for that publication; later revocation governs subsequent requests. Recorded actors/roles in the model grant no runtime privilege. A seed commit neither requires nor fabricates human approval.

Use initial file format `knowledgehub-files/1`, profile `knowledgehub-seed/1` and request fingerprint format `knowledgehub-request/1`. Rust crate/API, ontology, file, fingerprint and future bundle versions remain separate. Persist exact version bindings and reject unsupported versions explicitly; no automatic format migration is included.

The request fingerprint is SHA-256 over RFC 8785 canonical JSON for a typed envelope containing fingerprint version, Hub/model/branch, operation ID, actor ID, expected head, exact profile/ontology bindings, guards and the complete ordered requested operations. Content references include SHA-256 and byte length for every submitted Markdown/source/artifact; verify bytes against these references before publication. Unknown fields, duplicate JSON keys, invalid Unicode and unsupported numeric values are rejected. The seed envelope uses strings, booleans, null, arrays, objects and safe-range integer lengths; arbitrary numeric property bags are outside this profile. Preserve string contents and operation order; no whitespace or Unicode normalisation of authored content. Object property order alone is not different intent. Canonical encoding follows [RFC 8785](https://www.rfc-editor.org/rfc/rfc8785.html).

Semantic revision hashes use their separately tagged canonical record encoding; original-source and document digests hash exact retained bytes. Do not use formatted graph JSON or re-serialised source text as a substitute for original-byte hashing. Semantic identities are caller-supplied opaque nonempty IDs qualified by their declared namespace/scope, never filenames or content digests.

The operation key is `(Hub ID, model ID, branch ID, operation ID)`. The stored actor and fingerprint bind that key, so a changed actor cannot create a second operation under it. Current policy checks precede lookup/disclosure. Exact committed retries return the original receipt before stale-base checks; changed intent or actor is rejected without disclosing another actor's receipt. Only committed operations reserve a durable key; rejected/never-admitted attempts do not. Uncertain callers must establish authoritative outcome/absence before changing intent. Committed keys, original fingerprints/actors and receipts are retained for the lifetime of this store, including across later head movement. There is no expiry, pruning or branch deletion in this slice. Missing history prevents an authoritative absence claim. Staging receipts are not evidence of commitment.

### Implementation and remaining gates

The user explicitly authorised KH-SL-001 implementation on 31 October 2026 and directed use of existing Rust 1.99.1 without installing another compiler. The [implementation record](implementation-record.md) records feasibility and the platform assumption; [evidence](../delivery/evidence/kh-sl-001.md) records actual technical checks. Human acceptance is pending. KH-D-07–10 remain progressively specified later decisions. No document authority has transferred, and no later slice is authorised by this implementation.

### KH-SL-007/008 implementation follow-up

The user's explicit apply request authorised KH-D-11–15 as concrete implementation
choices for this bounded change. The source ontology and proposed domain analysis
retain their source/review status. The public consumer loads through ordinary API
operations, retains actual native logs/build/source hashes, establishes A/B and
recovers them after reopen. Missing semantic refinements remain explicit gaps;
UI/runtime-AI inapplicability does not remove those supported catalogue families.

Repository documents continue to own their definitions. There is no new human
AcceptanceRecord, automatic promotion of a stored claim to observed execution,
or KH-SL-006 handover. Review the resulting gaps and actual scope before choosing
the next outcome. [Consumer contract](self-model-consumer.md) and
[technical evidence](../delivery/evidence/full-ontology.md) own the reproducible run.

### Sealed derived indexes — 1 November 2026

The user authorised proposing and implementing KH-SL-013 / pack-derived-index-storage with sealed variable-size files and bounded compression blocks. Keep 256 MiB neighbour and 128 MiB text allocation quotas, exact logical objects and existing source authority. KH-D-20 records this physical cache-storage choice; no preallocated mutable file is introduced. KH-SL-012 is postponed at 7/11 and its old attempt is obsolete as a completion plan. The failed-run evidence and existing Hub remain retained. A successful packed-cache check does not relabel that rebuild as completed or capture the current repository into the older stored model.

KH-SL-013 is implemented and technically verified; see [packed-cache evidence](../delivery/evidence/packed-cache/README.md). The old rebuild remains unfinished, and repository documents still own their definitions.
