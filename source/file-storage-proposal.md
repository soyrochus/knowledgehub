# KnowledgeHub file storage format proposal

Draft 0.3 · 31 October 2026 · Proposed `knowledgehub-files/3`, generic neighbour indexing and deterministic queries; the canonical demonstration will be reset and regenerated when its rebuild change is applied. Implementation and verification remain required.

## 1. Purpose, authority and scope

Define a space-efficient filesystem representation of a Knowledge Hub that preserves exact knowledge, immutable history, provenance, atomic changes and original operation results. A small definition change must not require another complete copy of the model or its source files.

The [System Definition Ontology proposal](system-definition-ontology-proposal.md), especially §§3–5 and 21.4–21.9, supplies the semantic requirements. The [Practical Design Manual](practical-design-manual.md), especially §§3, 15–17, supplies the method: begin with a concrete outcome, specify normal and exceptional behaviour, implement only when authorised, and revise the design from evidence. Its Memento examples add no requirements here.

This document proposes physical storage choices. It does not replace the ontology, grant runtime authority, approve stored definitions or transfer repository definition authority to the Hub. The [domain analysis](../design/domain-analysis.md), [slice plan](../delivery/slice-plan.md) and [self-hosting plan](../design/self-hosting-plan.md) retain their existing ownership. The current [files/1 contract](../design/file-backend.md) and [files/2 contract](../design/typed-ontology-api.md) describe implemented behaviour; this proposal does not describe an implemented files/3 backend.

Within the proposed format, **must** identifies a conformance requirement, **should** an expected choice requiring a recorded reason to depart, and **may** an optional behaviour. These words do not establish adoption or successful verification.

Initial scope remains one embedded Rust crate, coordinated single-host writers, concurrent snapshot-bound readers, and Linux x86_64/local ext4 under the host's declared support profile. One atomic change affects one model branch. Objects can be shared within a store, but this proposal does not add multi-model transactions, network filesystem support, multi-host writers, database adapters or a service. Existing Rust 1.99.1 and the user's platform-compliance assumption remain given; no compiler installation is proposed.

## 2. The concrete problem and desired outcome

The canonical repository demonstration is [`hub/`](../hub/README.md). A read-only measurement on 31 October 2026 found approximately:

| Current files/2 content | Logical file bytes |
| --- | ---: |
| Complete graph files across eight generations | 91.93 MB |
| Source/artifact copies across those generations | 23.06 MB |
| Receipts, manifests and other store files | 1.04 MB |
| Total | 116.03 MB |

MB means 1,000,000 bytes. Filesystem allocated space and Git storage are separate measurements. The latest generation alone occupies 21.69 MB. Its graph contains about 5.40 MB of baseline records and 4.05 MB of source-anchor records. The ontology type, relationship, constraint, profile and ontology-definition records themselves occupy approximately 0.47 MB. The [retained run](../delivery/evidence/full-walkthrough.json), [relocation inventory](../delivery/evidence/hub-relocation.json) and [public inspection](../delivery/evidence/hub-inspection-run.json) identify the measured demonstration; these numbers are observations, not future size guarantees.

The demonstration is deliberately detailed: it retains individual source declarations and fields, planning definitions, source quotations, PDF/Markdown originals, extraction bindings and execution evidence. That granularity is an authoring choice. Storage optimisation must not silently discard those records or shorten their meaning.

The target outcome is that a maintainer can change one explanation, preserve A and B, restart, inspect both exact explanations and recover the original receipt, while storing each unchanged record or artifact once. A baseline must ordinarily add a small record and references to shared structures, rather than megabytes of copied membership lists. A generation must record the new accepted roots rather than another complete graph directory.

Space efficiency is balanced with bounded reads, understandable recovery and exact reconstruction. No universal minimum byte count or numerical compression ratio is promised.

## 3. Vocabulary: meaning versus storage

| Concept | Definition | Consequence |
| --- | --- | --- |
| Semantic identity | A stable, qualified name for a model subject. | Renaming a file or relocating the store does not create another subject. |
| Revision | An immutable version of that subject, addressed by model, identity and an opaque revision token. | A change creates a successor. Its semantic digest is separate from its token and physical object address. |
| Blob | Exact original bytes of Markdown, code, PDF, a log or another artifact. | Identical bytes can be shared by different source identities; their provenance and ownership remain distinct records. |
| Storage object | An immutable typed payload addressed by a hash of its logical bytes. | Records, content, tree nodes, receipts and generations can be shared without overwriting them. |
| Selection | One explicitly chosen normative revision per qualified identity. | Retained historical revisions do not automatically become current definitions. |
| Snapshot | A consistent, read-only state identified by a public snapshot ID. | A read resolves its roots once; it does not mix states as the head advances. |
| Baseline | An immutable model record selecting compatible exact definition, assertion, ontology/profile and artifact revisions for a declared scope, with explicit gaps and supporting dependencies. | It is a meaningful reference point such as “design assessed by test run X”, not a request for whatever is latest. |
| Generation | An immutable storage commit descriptor recording one accepted transition: its snapshot ID, parent, sequence, resulting roots and original receipt. | It is a backend history unit. It is not another ontology, a model copy or evidence of design approval. |
| Branch head | The mutable reference identifying the accepted current generation of one model branch. | Publishing this one reference is the commit point; content objects remain immutable. |
| Supporting closure | Exact retained revisions and artifacts required to interpret and validate a selection. | Supporting dependencies are accessible without becoming competing normative selections. |
| Derived index/view | Rebuildable search data, an object-location index or a rendered document. | It may accelerate access but cannot invent accepted content, replace a baseline or grant authority. |

### 3.1 Baseline semantics

A baseline pins exact revisions and their source/ownership bindings, the ontology/profile that interprets them, declared parent baselines and its recorded gaps. It may be a complete or explicitly incomplete draft. Recording a baseline does not establish review approval, execution success or human acceptance.

For example, A selects atomic-change requirement revision `self-1`. After an authorised successor change, B selects `self-2`. Both select the same revisions of unchanged test and code definitions. A continues to resolve its original narrative even after B becomes current. Reviews and test runs remain bound to the scope they actually assessed.

A baseline is neither a timestamp nor a backup. A selected-baseline export can omit unrelated history; a recovery backup cannot omit retained receipts or history promised by its backup profile. Comparison reports changed exact bindings and their definitions, not differences in compressed files.

Creating a baseline is an ordinary guarded model change and therefore normally creates a generation. Most generations need not create a baseline. Several baselines can share the same selection while describing different purposes or assessments. Recompressing or packing storage creates neither a baseline nor a model generation.

### 3.2 Generation semantics

There is one genesis generation at sequence zero, with no parent or operation receipt. Each accepted change adds one generation whose parent is the prior accepted head and whose sequence is parent sequence plus one. Rejection and exact retry add none. Failed preparation may leave unreferenced objects; their existence does not establish an accepted generation.

The sequence belongs to the model branch. It is unrelated to a requirement's revision token or a baseline's name. The first profile has one parent per generation and no merge-commit semantics. Alternative model designs may still be expressed as distinct baselines.

The public snapshot ID is preallocated and opaque. The storage generation's content address is calculated after its payload is complete. Keeping these separate allows receipts to name their resulting snapshot without circular hashes.

### 3.3 Worked history

| Accepted operation | Storage result | Meaning retained |
| --- | --- | --- |
| Load the initial self-model | G1 points to its selection and retained-revision roots. | Original definitions and sources. |
| Record baseline A | G2 adds A's record and changed tree paths. | A pins the selection from G1. |
| Revise one explanation | G3 adds the successor records, changed content/chunks and changed tree paths. | The original revisions remain available. |
| Record baseline B | G4 adds B's record and changed tree paths. | B pins the selection from G3. |
| Retry the original explanation change | Return G3's original receipt; create no generation. | Original operation identity and outcome. |

G1–G4 are illustrative labels. There are no four complete model copies. Unchanged objects are addressed by the same hashes throughout.

## 4. Invariants

1. Published logical object bytes, semantic revisions, source bytes and original receipts never change under their existing identities.
2. Stable semantic IDs, opaque revision tokens, semantic digests, original-byte digests, snapshot IDs and physical object hashes are different addresses with different purposes. None is silently substituted for another.
3. A generation names a complete, resolvable state through shared roots; reconstruction does not require replaying every earlier change.
4. A baseline resolves its exact selection and dependencies without consulting a mutable head or external “latest” content.
5. Graph JSON owns structured definitions and relationships. Exact Markdown owns declared document-owned narrative. Physical factoring does not create a second authoritative copy of a field.
6. A source anchor resolves against an exact retained source revision. Sharing bytes does not collapse different source identities, interpretations or review histories.
7. Head, history, operation deduplication and original receipt become accepted together. No side-file update can make only part of a change accepted.
8. The original actor, scoped operation ID, request fingerprint and receipt remain available for the store lifetime. Neither packing nor cleanup expires committed operations.
9. Ordinary publication and maintenance never delete accepted history. Only unreachable preparation debris and redundant physical encodings are reclaimable under the protocol below.
10. Corruption, missing content, unsupported codecs and incomplete recovery produce explicit failure. They never establish operation absence or justify falling back silently to an older head.

These refine the physical realisation of KH-DR-01–15 and KH-DR-17–19 in the domain analysis; they do not introduce alternative domain-rule ownership.

## 5. Versions, directory layout and authority

The proposed file format is `knowledgehub-files/3`. Storage object grammar `knowledgehub-object/1`, map/sequence grammar, packing, request fingerprint codec and semantic record codec are separately identified. A physical-format revision must not silently change the ontology or `knowledgehub-record/2` and `knowledgehub-request/2` semantics.

```text
hub/
  hub.json                         immutable format, identity and support metadata
  refs/<scope-key>.json             mutable accepted branch reference
  locks/store.lock                 stable publication/maintenance lock
  objects/loose/<hh>/<rest>.obj     immutable encoded objects
  objects/packs/<pack-sha>.khpack   immutable packs of objects
  objects/packs/<pack-sha>.idx      rebuildable pack-location index
  objects/catalog.json             rebuildable active-pack/location catalogue
  staging/<attempt-id>/            unaccepted candidate and maintenance work
  cache/                           disposable queries and verified-object caches
  views/                           disposable or explicitly exported readable views
```

`scope-key` is SHA-256 of the canonical JSON tuple `[hub_id, model_id, branch_id]`. Semantic IDs are never interpolated as path components. Object paths use the first two lowercase hexadecimal hash characters as the shard and the remaining 62 as the filename stem. Managed files must be ordinary files under ordinary directories on the selected local filesystem; symlink traversal and path escape are refused.

`hub.json` declares at least: format; Hub identity; immutable replay-domain identity; supported scope and owner; semantic/request codecs; object hash and grammar; required compression, tree and pack features; and the filesystem/durability profile. Opening a store with unknown required features or unknown normative fields fails explicitly. Initial creation is exclusive and durable before use. A new incompatible configuration requires an explicit conversion, not a silent metadata edit.

The branch reference is small JSON containing its format, exact scope, public snapshot ID, sequence and generation object address. Its values must agree with the generation. `hub.json` and accepted refs are authoritative roots. Object presence, pack membership, cache entries and filenames alone never make anything accepted.

Only refs, staging and disposable derived data are mutable during ordinary use. Lock files have stable names and inodes; they are not replaced to release a lock. Published object/pack paths are installed without overwriting an existing immutable representation. The catalogue is derived physical-location data, not another model head.

The initial writer profile coordinates all publication and maintenance through one nonblocking store lock. This deliberately favours correctness over parallel writes across scopes. Public snapshot-bound reads use the same maintenance coordination described in §11. Finer-grained locking is a later implementation choice requiring equivalent evidence.

## 6. Logical objects, hashes and lossless factoring

### 6.1 Canonical payload and object address

Structured payloads are UTF-8 JSON canonicalised with [RFC 8785 JCS](https://www.rfc-editor.org/rfc/rfc8785.html). They use the existing restricted contract: strings, booleans, null, arrays, objects and safe structural integers; arbitrary domain integers/decimals retain their typed string encodings. Duplicate keys, invalid Unicode, unknown typed fields and invalid lengths are rejected. Strings and source bytes are not Unicode- or newline-normalised. Original Markdown and binary content are byte payloads, not canonicalised JSON.

For kind `K` and logical payload bytes `P`, define:

```text
OID = lowercase_hex(SHA256(
  UTF8("knowledgehub-object/1\n" + K + "\n" + decimal(byte_length(P)) + "\n")
  || P
))
```

`decimal` has no sign or redundant leading zeros, except `0`. Kind names are closed ASCII tokens from the object grammar. Payloads do not embed their own OID. Object hashes bind kind and logical uncompressed bytes; compression level, pack position and storage path are excluded.

Original artifact digests remain SHA-256 of exact original bytes, without this storage prefix. Existing semantic record digests remain computed by their declared semantic codec. A storage OID therefore need not equal either digest. A reader verifies all applicable bindings; a matching filename is not verification.

### 6.2 Required object kinds

| Kind | Logical payload |
| --- | --- |
| `blob` | Exact original byte sequence, also used for content chunks. |
| `blob-manifest` | Original-byte digest/length and an ordered sequence of chunk OIDs. |
| `record` | Exact reference, semantic codec/digest, type, predecessor, provenance and a data-object OID. |
| `record-data` | Generic typed logical record data, retaining all declared fields. |
| `baseline-data` | Factored selection/supporting roots, order descriptors, exact ontology/profile/parents and a gap-sequence root. |
| `anchor-data` | Complete anchor metadata with a lossless quotation descriptor instead of a repeated quote string. |
| `gap` | One exact logical gap; identical gap payloads are shared. |
| `map-node` | An immutable node of a typed persistent map. |
| `sequence-node` | An immutable node of an ordered persistent sequence. |
| `receipt` | Original public receipt fields and factored affected-reference sequences. |
| `generation` | Accepted transition descriptor defined in §9. |

All JSON kinds declare their schema version. Type/relationship definitions, assertions, source revisions, interpretations and other ontology records use the same record envelope; there are no competing shadow copies of native kernel subjects. Generic `record-data` is an envelope with exactly `schema` (`knowledgehub-record-data/1`) and `value` (the complete existing typed `RecordData` value). Its wrapper is removed when reconstructing a public record.

### 6.3 Reconstruction contract

A stored `record` factors physical data out of the public logical record. `record-data` reconstructs the generic typed payload verbatim. `baseline-data` and `anchor-data` have the specific projections below. The adapter must reconstruct the complete record accepted by its declared semantic codec and verify its original semantic digest. The full expanded record need not be stored alongside the factored form.

Missing/null distinctions, typed values, ordered arrays, predecessor references, ownership, provenance and all original strings survive reconstruction. Factoring must not substitute an equivalent-looking summary or change a record's digest. A legacy array's order or duplicates, where accepted by its codec, are preserved with an order descriptor; they are not silently normalised into a set.

Logical references between model records remain exact semantic references. They need not become physical OID dependencies, so mutually referring definitions can use preallocated revision tokens without requiring a hash fixed point. The physical object dependency graph itself must be finite and acyclic.

### 6.4 Concrete record example

This is a schematic `record` payload; angle-bracket strings stand for actual digests/OIDs and are not valid conformance fixtures. Its fields define the `knowledgehub-stored-record/1` envelope. `semantic_sha256` projects to the public record's `sha256`; `data_oid` is expanded according to its verified object kind.

```json
{
  "schema": "knowledgehub-stored-record/1",
  "semantic_codec": "knowledgehub-record/2",
  "reference": {"model": "knowledgehub", "id": "urn:requirement:atomic-change", "revision": "self-2"},
  "semantic_sha256": "<original-semantic-digest>",
  "type_ref": {"model": "sdo", "id": "sdo:type:Requirement", "revision": "catalogue-1"},
  "predecessor": {"model": "knowledgehub", "id": "urn:requirement:atomic-change", "revision": "self-1"},
  "provenance": [{"model": "knowledgehub", "id": "kh:anchor:atomic-explanation", "revision": "self-2"}],
  "data_oid": "<shared-record-data-object-address>"
}
```

All fields are required; `predecessor` may be null and `provenance` may be empty only where the semantic profile allows it. Changing `self-1` to `self-2` creates another record/payload as needed; unchanged source, type and other records keep their OIDs. Native identity projections such as files/2's `@identity` continue to resolve from their pinned lineage under the semantic codec; the storage layer must not invent editable identity records to provide them.

## 7. Shared state trees and compact baselines

### 7.1 Persistent maps

A persistent map is an immutable tree whose unchanged nodes are reused. An update writes the affected leaf and its ancestor path; it does not rewrite every entry. Map roots denote complete maps, not deltas needing a walk through parent generations.

Required map keyspaces are:

| Map | Key | Value |
| --- | --- | --- |
| Selection | `[model, identity]` | Exact revision reference. |
| Retained revisions | `[model, identity, revision]` | Record OID. |
| Exact-reference set | `[model, identity, revision]` | `null`, signifying membership. |
| Artifact bindings | `[model, source_identity, revision]` | Content OID and original-byte digest/length. |
| Operations | Scoped operation ID; scope inherited from the generation. | Original receipt OID. |
| Accepted ancestors | Public snapshot ID. | Earlier generation OID and sequence. |

Keys are canonical JSON tuples, or a JSON string where the table specifies one scalar. Routing uses the hexadecimal SHA-256 of their canonical bytes. A leaf contains at most 64 entries ordered by canonical key bytes. A larger subtree splits on successive hexadecimal nibbles into a branch with up to 16 children. Branches carry child-OID bindings instead of complete entries. Delete/collapse rules restore the same leaf threshold; empty maps use one shared empty leaf per keyspace. At depth 64, distinct keys with the same routing hash remain distinct sorted entries in a collision leaf; routing hashes do not replace keys. Readers verify prefixes, unique keys, child bindings and declared counts.

Each node declares its keyspace, prefix and node variant. This prevents a receipt-map node from being interpreted as a revision map. Public query ordering remains its API-defined ordering, not the tree's hash traversal order.

The `knowledgehub-map-node/1` JSON schema has required fields `schema`, `keyspace`, `prefix`, `count`, `variant`, and either `entries` for a leaf or `children` for a branch. `prefix` is the consumed lowercase hexadecimal routing prefix; `count` is a canonical decimal string. A leaf's entries are objects with exactly `key` and `value`. A branch's children are an object keyed by one next hexadecimal digit, with child OIDs as values. The root prefix is empty. Child prefixes extend the parent's prefix by their digit, and subtree counts add exactly. Unsupported keyspaces or variants fail decoding.

The retained-revision registry binds every accepted exact reference to one immutable record OID. A companion retained-reference set contains exactly the same keys without physical record addresses. Maintaining both costs another changed tree path when a revision is added, but permits baselines to reuse membership roots without depending recursively on the physical records they describe. These are storage structures with a checked equality invariant, not two authoritative definitions of the knowledge.

### 7.2 Persistent sequences

Ordered lists use `sequence-node` leaves with up to 128 values or object references, and branches with up to 128 child OIDs and subtree counts. The schema declares the item type. Append, insertion and replacement copy the affected path and split full nodes; unchanged nodes are reused. Order and multiplicity are exact. Empty sequences use a shared empty leaf. An implementation must bound depth, decoded node size and item counts, and reject inconsistent counts or cycles.

The `knowledgehub-sequence-node/1` schema has required fields `schema`, `item_type`, `count`, `variant`, and either `items` for a leaf or `children` for a branch. Children are ordered objects with exactly `oid` and `count`. Counts are canonical decimal strings; all descendants use the same declared item type and their counts add exactly. Empty sequences share an empty leaf per item type. Known item schemas include exact references, gap OIDs and chunk descriptors; a chunk descriptor has exactly `oid` and canonical decimal `length`.

Sequences factor gap lists, changed/affected-reference lists, chunk lists and noncanonical legacy membership order. Flattening for an existing owned public API is permitted; persisting that flattened result in each generation is not.

### 7.3 Baseline representation

`baseline-data` stores:

```text
ontology             exact ontology revision reference
profile              exact profile revision reference
selected_root        selection map root
supporting_root      exact-reference set for the declared supporting membership
artifacts_root       exact source/content bindings required by this baseline
selected_order       null for canonical key order, otherwise sequence of exact refs
supporting_order     null for canonical key order, otherwise sequence of exact refs
parents              exact parent-baseline references in their declared order
gaps_root            ordered sequence of gap OIDs
```

`schema` is additionally required and is `knowledgehub-baseline-data/1`. `selected_root` expands to its exact references in canonical key order unless the explicit order sequence is present. `supporting_root` expands to its keys under the same rule. Order sequences must resolve to precisely their declared map membership, preserving any codec-permitted multiplicity. Parent, ontology/profile, type, provenance and logical record dependencies must resolve within the declared closure. Extra entries physically shared in an artifact map do not expand normative selection or declared supporting membership.

Membership contains semantic exact references, not record OIDs. A baseline read pins an accepted generation containing that baseline revision, then resolves every member through that generation's immutable retained-revision registry and the baseline's pinned artifact bindings. The registry is append-only: a later accepted generation can add exact references but cannot redirect an existing one to different content. No lookup substitutes the latest revision or resolves the head again midway. A standalone exchange/backup must provide its own fixed resolution registry covering the baseline's full declared closure; the baseline cannot depend on access to an external current Hub.

Thus a baseline can reuse the selection, retained-reference-set, artifact and gap roots that already describe its source snapshot. Two baselines with the same selection reuse those roots. When they differ by one selected revision, only affected map paths and genuinely new content are added. A baseline record's own identity, provenance and meaning remain separate from its shared selection payload.

A convenient creation API normally captures the selection before adding its own baseline record. It resolves “current” once, before publication, and records explicit roots rather than a deferred “latest” query. If an accepted logical baseline explicitly refers to itself or participates in a cyclic semantic graph, its reference-set representation remains finite: neither membership root nor logical references contain a physical back-pointer to that baseline's OID. The containing generation's registry is sealed only after all records exist. Physical hash cycles are forbidden; cyclic logical references are not rejected merely to simplify storage.

The supporting closure should contain what interpretation and validation require, rather than every unrelated historical revision. However, choosing that scope is an authoring/semantic decision. The adapter must preserve a caller's declared supporting set exactly, including a larger historical set in an existing record. Changing the current `baseline_data()` convenience method to select a minimal closure requires its own authorised behavioural change, not a hidden files/3 optimisation.

### 7.4 Index actual neighbours, independently of future queries

A nearest neighbour here is one recorded typed association away. It is not a text-similarity result or an association inferred merely because the ontology permits one. Future relationships may be added after either endpoint; future traversal questions are not known when inserting elements.

Maintain generic bidirectional adjacency over actual facts:

```text
outgoing[exact subject reference][association selector] -> posting sequence
incoming[exact target reference][association selector] -> posting sequence
posting -> opposite exact reference + exact association origin + occurrence path
```

The outer map is keyed by endpoint, with an inner selector map. This supports discovering all actual relationship kinds at one endpoint without enumerating every ontology relationship or scanning composite hash keys with an unknown suffix. An assertion selector pins the exact relationship-definition revision; its origin is the exact assertion revision. A property selector pins the owning type revision and schema field path; its origin is the owning record revision, with an exact occurrence path distinguishing list items or repeated references. Posting order is deterministic by selector, opposite reference, origin and occurrence path. Distinct assertions/occurrences are retained even when they connect the same endpoints.

Extract tagged reference-bearing values using the closed record grammar and exact registered schemas, including nested structured fields, lists and sets. New registered types/signatures use this generic mechanism without a storage index per type. Plain strings are not guessed to be IDs. Separate model associations, reference properties, provenance/representation links and administrative membership into explicit query categories. Baseline membership uses its existing maps; it is not automatically expanded into thousands of ordinary impact edges. Default impact traversal does not include provenance or administrative categories.

Adding `Requirement R formalises UserStory S` adds R's outgoing posting and S's incoming posting, even if S was inserted long before. Removing a selected assertion removes its postings from the new view; its historical postings remain available in old views. An exact endpoint is never repinned to its newest revision. Artifact, narrative and identity projections retain their native semantics rather than becoming invented editable assertions.

### 7.5 Snapshot-bound index lifecycle and space

Adjacency is rebuildable derived data. An index manifest under `cache/neighbours/<view-key>/` binds scope, exact selection/supporting roots, ontology/profile revisions, category rules and extractor version to the outgoing/incoming roots. Their map keyspaces and posting item types belong to the versioned neighbour-index grammar, not arbitrary untyped index payloads. The read pins this manifest and graph view under maintenance coordination; it never uses an index for another revision selection.

When the view changes, prepare incremental postings from admitted selection/record changes and publish a complete matching manifest only after the accepted graph is recoverable and its index nodes are durable. A publication interrupted between graph and derived-index readiness leaves valid knowledge with an absent index, not a partially indexed complete answer. Missing, corrupt or incompatible caches trigger a bounded complete rebuild, an explicit index-preparation/capacity result, or a documented correct fallback; they cannot establish absence. No stale-index fallback may be labelled complete. Old indexes remain valid for old views and unchanged persistent map/sequence nodes are shared.

A baseline naming an already indexed identical view reuses its roots. An arbitrary narrower baseline may need its own first build. Exact identity lookup uses the selected/retained maps; type lookup may use a similarly bound type-membership cache. Search indexes bind their own text projection and analyzer version. Derived index data is excluded from required backup closure and may be discarded by maintenance under the same reader coordination; rebuilding requires accepted records, not external sources.

Two postings per actual association give linear index entry count in the actual reference graph, plus map overhead. No all-pairs reachability table, precomputed story-to-table index or index for every possible ontology route is required. Adjacency postings contain compact references and optional verified type metadata, not copies of source documents. Physical packing/compression and changed-path sharing apply to persistent index structures where retained. In-memory caches and persisted derived files have finite host quotas and eviction; retained knowledge is never evicted with them.

### 7.6 Deterministic traversal at query time

Provide storage-independent, asynchronous batch neighbour lookup and multi-hop traversal. A neighbour request supplies a pinned selection, multiple exact starts, incoming/outgoing/both directions, association category/selector filters, optional endpoint type filters and finite result/work budgets. It returns exact endpoints, origin assertions or property occurrences, stable ordering and explicit continuation/truncation. Current policy applies on every operation/page.

Multi-hop traversal composes those lookups inside the library. Its caller supplies a finite declarative policy of permitted typed/directed steps, or a bounded ordered step plan with alternatives. The ontology validates selector/type applicability; it does not assert missing dependencies or predict the user's intent. Execution is deterministic breadth-first frontier expansion, with batching, visited subjects/policy states, retained witness paths and explicit limits. The result can include compact target records and source fragments in the same response. There are no LLM calls, autonomous query rewriting, network acquisition, scripts or inferred graph mutations.

The phrase-search → UserStory → Requirement → UseCase → function/domain definition → explicit persistence mapping → physical table example is a query-time route, not an insertion-time index. Different directions and newly added associations can be followed using the same local indexes. “Nearest neighbour” describes each indexed hop; finding one nearby table must not silently exclude farther reachable targets requested by an impact query. Cycles, fan-out, path count and projected bytes are bounded. A proof path identifies exact origin records, not just endpoint names.

### 7.7 Text entry, completeness and measuring fast lookup

An optional composite read first selects by exact type and literal phrase, then feeds matching starts into traversal at the same pinned view. Initial proposed search modes are byte-exact UTF-8 substring and ASCII-case-insensitive substring with no Unicode rewriting of original content. Search includes declared narrative fragments via exact document/anchor bindings. A versioned trigram candidate index plus exact final substring verification supports these modes; queries shorter than three UTF-8 bytes are explicitly unsupported initially. No semantic/vector matching, synonym expansion or unrecorded extraction is implied.

Return all matching starts grouped, or require exactly one as an explicit match policy. Report ambiguity, no match, incomplete coverage, unsupported external bindings and budget truncation distinctly. A table is an explicitly classified physical schema/mapping target; domain entities do not become tables automatically. Results identify potential dependencies and their traces. Missing mappings produce unknown/insufficient traceability, not proof that no tables are affected; a wording change alone does not establish a required schema migration.

Measure first index preparation separately from warm neighbour lookups, cold object-page reads and full scrub. With a valid index, one-hop work is tree lookup plus actual matching posting enumeration; multi-hop work follows visited neighbourhoods instead of scanning all assertions for every node. Public full-snapshot materialisation remains a different operation. Benchmarks must add unrelated edges and retained generations while holding the queried neighbourhood fixed, count index/object reads and bytes, test high fan-out explicitly, and report hardware/cache conditions rather than promise an unmeasured microsecond latency.

## 8. Artifacts, source anchors and readable Markdown

### 8.1 Store original bytes once

All source revisions and representations bind content in the shared object store. Identical byte sequences are stored once even when referenced by several sources, baselines or generations. Media type, source identity, origin, capture time, ownership and derivation remain in their logical records; sharing content must not merge those facts.

Small content is one `blob`. For content longer than 1,048,576 bytes, the initial proposed chunk profile splits it into consecutive 1,048,576-byte chunks; the final chunk contains the remaining 1–1,048,576 bytes, including a full chunk for an exact multiple. Chunks are ordinary shared blobs. A `blob-manifest` records the full original-byte SHA-256 and length and an ordered persistent sequence of chunk OIDs/lengths. The threshold and chunk rule are format parameters, not host-dependent heuristics. Empty content is one zero-length blob.

The manifest has exactly `schema` (`knowledgehub-blob-manifest/1`), `sha256` (the full original-byte digest), `length` (canonical decimal) and `chunks_root` (the chunk-descriptor sequence OID). Chunk lengths must add to the declared full length and obey the fixed chunk profile. A content binding identifies the `blob` or `blob-manifest` OID together with its original-byte digest and length, so readers need not infer original content identity from a storage OID.

Reassembly must reproduce the original bytes and full digest exactly. Readers can resolve a bounded range from affected chunks; full integrity verification streams the entire content. Appending a log or changing a portion can reuse unaffected chunks. Insertion can shift fixed chunk boundaries; content-defined chunking is a possible separately versioned improvement, not an unstated guarantee of this profile.

Compression is below this logical representation and cannot change original content hashes, chunk order, source anchors or artifact identity.

### 8.2 Avoid repeated source quotations

For a native UTF-8 anchor whose quotation equals a retained source span, `anchor-data` stores the exact source revision, its content OID, half-open byte offsets, quote digest and other declared anchor fields. Offsets must be in range and align with UTF-8 code-point boundaries. The literal quote is reconstructed from the retained bytes and checked for exact equality through the semantic digest. The same code body or document section is not embedded again in every anchor record.

Binary/PDF coordinates continue to bind the exact original artifact and exact locator/extraction manifest. A derived textual quotation references a span of the retained extraction, with its derivation and tool/version intact. A byte offset in extracted text is not presented as a byte offset in the original PDF.

An authored interpretation or quotation that cannot be reconstructed exactly from an existing retained span keeps its exact literal value, optionally in a shared blob. It must never be replaced by an approximate extraction. Moving source files or refreshing a source adds explicit successor bindings; historical anchors remain unchanged.

The `knowledgehub-anchor-data/1` payload retains `source`, `label` and the original typed `locator` fields other than its `quote`, plus a separate required `quotation` descriptor. A null descriptor preserves an absent optional quote. Otherwise the descriptor's closed `kind` is `literal` (exact `text`), `blob` (`content_oid`, original-byte `sha256` and decimal `length`), or `span` (exact `source_ref`, `content_oid`, decimal `start`/`end` and quote `sha256`). Reconstruction restores the quote at the original locator field, including a present empty string. The descriptor must agree with that locator and, for extracted text, its declared manifest. These storage fields do not become extra public anchor properties.

### 8.3 Human-readable files

Markdown remains a logical original artifact with exact UTF-8 bytes and stable document/fragment identities. Compression or packing may remove a direct `.md` pathname from the authoritative physical store. The library must offer byte-exact materialisation of a selected document or baseline into an explicit export/workspace. A readable `views/` tree may display Markdown, graph JSON, filenames and semantic IDs with the exact snapshot/baseline watermark.

Those views are derived until an explicit authoring/import operation admits changes. Editing a rendered Markdown file must not edit accepted objects in place. Graph-owned definitions and document-owned definitions keep their declared owners through export and guarded import. Source capture excludes generated views, staging, caches and the store's own physical objects, avoiding recursive ingestion of copies of itself.

Keeping all originals as loose readable files is an optional space/readability policy, not a second normative copy required in addition to packed content. The user should be able to inspect knowledge through supported tools without learning object paths.

## 9. Generations, receipts and accepted history

A generation payload contains the following fields. OIDs are full lowercase SHA-256 addresses; `null` is used only where explicitly allowed. Structural sequence numbers in files/3 metadata use canonical nonnegative decimal strings and are range-checked when projected to public types.

| Field | Required meaning |
| --- | --- |
| `schema` | `knowledgehub-generation/1`. |
| `scope` | Exact Hub, model, branch and owner. |
| `snapshot_id` | Preallocated public snapshot identity. |
| `sequence` | Branch sequence, zero for genesis. |
| `parent` | Prior accepted generation OID; null only for genesis. |
| `ontology`, `profile` | Exact active bindings, or the explicitly permitted empty genesis state. |
| `selected_root` | Complete resulting normative selection map. |
| `revisions_root` | Complete retained exact-reference-to-record map. |
| `retained_refs_root` | Exact-reference set with the same keys as `revisions_root`, available for shared baseline membership. |
| `artifacts_root` | Complete exact source/content binding map. |
| `gaps_root` | Complete ordered current gap sequence. |
| `operations_root` | Complete scoped operation-to-original-receipt map. |
| `ancestors_root` | Accepted snapshot-ID-to-generation map, excluding this generation itself. |
| `receipt` | Original receipt OID; null only for genesis. |

For each child, `ancestors_root` is the parent's ancestor map plus the parent itself. The current generation is resolved directly by the head, not entered into its own ancestor map. This avoids a circular hash and permits direct accepted historical snapshot lookup without walking the entire parent chain. Publication validates that ancestry and operations maps only extend accepted history; existing bindings cannot be replaced or removed.

A receipt retains exactly its original public scope, actor, operation ID, request codec/fingerprint, previous and resulting public snapshot IDs, sequence and affected revisions. Large ordered affected lists use shared sequence objects with lossless reconstruction. Receipts refer to public snapshot IDs, not their containing generation's OID. A generation can therefore reference both its receipt and an operation map containing that receipt without recursion.

`knowledgehub-stored-receipt/1` stores required `schema`, `fields`, `affected_root` and `qualified_affected_root`. `fields` contains the original public receipt fields other than those two lists; the lists reconstruct from typed legacy revision-reference and qualified exact-reference sequences, respectively, respecting the original public codec's empty-field convention. A `gap` object similarly uses exactly `schema` (`knowledgehub-gap/1`) and `value` (the complete logical gap). Factoring neither object adds, removes or assesses evidence.

The immutable replay-domain identity binds the meaning of `(Hub, model, branch, operation ID)`. The operation map is authoritative generation state, not an eventually updated side index. Exact retry returns the same original receipt even after many later generations. Changed actor or intent under that operation ID is rejected under current policy. No storage-maintenance action issues a new receipt for an old operation.

Unselecting a record changes `selected_root`, not its retained revision or content. All accepted generation parents, historical snapshot aliases, original receipts and transitively required objects remain retained. If a future product introduces history expiry, it needs a separate explicit semantic and recovery contract; it is not garbage collection under this proposal.

## 10. Compression and physical packing

### 10.1 Encoding is separate from identity

Readers support `raw` and lossless `zstd` encodings. [RFC 8878](https://www.rfc-editor.org/rfc/rfc8878.html) defines the Zstandard framing used here. Initial encodings use self-contained frames without external dictionaries; decoding must not require an installed compressor or an unrecorded dictionary. A compressor version or level is not part of a logical OID.

Writers should use a fixed documented default level, initially proposed as level 3, and retain raw bytes when compression does not save at least 32 encoded bytes. Small metadata benefits from block compression with other objects; already compressed PDF/image content may remain raw. Higher levels are an explicit maintenance policy and require measured CPU/memory tradeoffs. Recompression creates another verified physical representation of the same logical objects, never successor model revisions.

Each decoder checks declared lengths, supported kind/codec, complete frame consumption and the recomputed logical OID. It enforces finite configured bounds on header size, logical object size, decoded block size and total work before allocating. Oversized valid content produces a capacity refusal; truncated, inconsistent or hash-invalid content produces integrity failure. Trailing unexplained bytes, external dictionary requests and unbounded nested decompression are rejected.

### 10.2 Loose object framing

An `.obj` file is:

```text
5 bytes       ASCII KHO3 followed by LF
4 bytes       unsigned big-endian header length
header length canonical UTF-8 JSON header
stored length encoded payload
```

The header has exactly `schema`, `kind`, `oid`, `codec`, `logical_length` and `stored_length`. `schema` is `knowledgehub-loose/1`; lengths are canonical decimal strings. Kind and OID obey §6; the decoded payload is exactly its logical payload. There is no trailing data. Header lengths and physical file lengths are validated before reading payloads. The prefix is framing, not part of the logical OID.

Loose objects make new writes simple. They must not be the only long-term representation: thousands of tiny files can consume much more allocated space than their payload bytes.

### 10.3 Immutable packs

Packing combines many small objects into bounded compressed blocks, removing per-file allocation overhead and allowing repeated JSON keys and reference prefixes to compress together. It is physical maintenance, not a rewrite of model history.

A `.khpack` consists of eight-byte ASCII magic `KHPACK3` followed by LF, an unsigned big-endian 64-bit block count, and that many blocks. Each block is a four-byte unsigned big-endian header length, a canonical JSON header and its encoded bytes. The block header has exactly `schema` (`knowledgehub-pack-block/1`), `codec`, `logical_length`, `stored_length`, `object_count` and `logical_sha256`. Numeric lengths/counts are canonical decimal strings.

The decoded block is a concatenation of entries:

```text
2 bytes       unsigned big-endian ASCII kind-token length
kind length   kind token from §6
32 bytes      raw OID digest
8 bytes       unsigned big-endian logical payload length
payload length logical payload bytes
```

Every entry's OID is verified from its kind and payload. A block's logical SHA-256 covers its complete decoded entry stream. Entries are sorted by OID within each pack; duplicate OIDs in a pack are rejected. A normal block targets at most 1 MiB of decoded entries and at most 512 entries. An individual larger object occupies a dedicated bounded block. These targets guide packing; configured safety limits still apply to oversized single objects. A closed pack ends exactly after its declared blocks.

The pack filename is SHA-256 of the complete encoded pack bytes. Packing validates every entry and syncs the finished immutable file before publishing its availability. Full pack hashes are checked during installation and scrub. A bounded lookup can instead verify its decoded block and requested object; it need not hash an entire large pack on each read.

The `.idx` is derived, sorted object-location data recording OID, block file offset, block length and entry offset/kind/length. Offsets and lengths use canonical decimal strings. It can be rebuilt by scanning and verifying pack blocks. `catalog.json` accelerates pack discovery; losing it must not lose knowledge. Recovery can discover and verify closed packs directly. An index entry cannot prove object absence or override a mismatching object hash.

Publish new packs and their directory entries durably before advertising them. Readers prefer a valid indexed representation and fall back to verified loose/other packed copies. Redundant representations may be removed only under §12. Never append to or overwrite a published pack. This initial pack grammar uses compression and structural sharing, not inter-object binary deltas or chains of patches that must be replayed to recover an object.

## 11. Guarded publication, reading and recovery

### 11.1 One accepted publication point

The host supplies current actor/policy context. Modelled roles do not confer runtime access. File I/O, hashing, compression, validation and recovery use bounded blocking work under the existing host-owned asynchronous runtime and drain contract.

1. Prepare exact requested intent, opaque revision/snapshot tokens and candidate objects. External acquisition and extraction happen outside publication. Preparation alone reserves no operation identity.
2. Acquire the stable store lock exclusively with nonblocking admission. Recover the accepted head and check current required authority. Consult the authoritative operation map before treating an exact retry's original base as stale. Return an existing original receipt or reject changed intent without another publication.
3. For new intent, verify expected head, exact revision/absence guards and all relevant graph/profile/ownership constraints. Validate the coherent candidate, including baseline closure and affected relationships. Reusing tree nodes is not evidence that graph-wide invariants remain valid. Incremental validation is allowed only with a justified dependency scope and equivalent results.
4. Build resulting state roots, original receipt and updated operation/ancestor roots. The ancestor map adds the prior head, not the candidate itself. Seal the generation after all its dependencies exist. No candidate depends on its own OID.
5. Write any missing objects to unique staging files. Verify their logical addresses, sync them, install without replacement, and sync every affected object/shard directory, including newly created directory parents. If an OID already exists, verify its logical content and reuse it; never overwrite it. Partially installed objects are still unaccepted.
6. After all candidate dependencies and the generation are durably installed, write and sync a temporary branch reference in `refs/`. Atomically replace the accepted reference and sync `refs/` before success acknowledgement. [Linux `rename`](https://man7.org/linux/man-pages/man2/rename.2.html) provides replacement atomicity; [Linux `fsync`](https://man7.org/linux/man-pages/man2/fsync.2.html) explains why the directory entry also requires persistence.
7. Return the original durable receipt. Failures before reference replacement leave the old accepted head. Any failure after replacement but before established durability reports an uncertain outcome; retry/recovery resolves the same original operation. Cancellation or a dropped response is not rollback.

Store creation uses an exclusive creation protocol and persists metadata, genesis, empty roots, lock files, initial ref and containing directories before acknowledging. A damaged existing store is not recreated as empty. Staged candidates may be built outside the exclusive lock, but final revalidation and publication remain under it; no stale automatic rebase is allowed.

### 11.2 Snapshot-bound reads without repeated history scans

A read checks current authority, acquires maintenance coordination, resolves one accepted generation and keeps its roots for the operation. Exact historical snapshots resolve only through that head or its authenticated ancestor map. Objects merely found on disk are not historical snapshots.

The initial coordination profile uses a shared store lock for a complete read and an exclusive lock for publication/maintenance. Public results are owned values; the lock is released after materialisation. Long-lived queries keep exact selection roots and recheck access on each new operation/page. Since accepted history is retained, a subsequent coordinated read can resolve those same roots after packing. Capacity and Busy remain explicit outcomes. Preparation outside the exclusive lock writes only uniquely owned staging; it holds shared maintenance coordination while consulting existing objects. Collection therefore cannot remove its dependencies, and exclusive admission revalidates everything before publication.

Readers verify the reference, generation and hash-bound tree/object paths they actually use. A full `State` snapshot still materialises everything its public contract promises, including retained revisions and artifacts where exposed; a bounded record or range read need not materialise the whole model. Neither operation requires re-reading every older generation's complete copy. Verified-object caches are optional and discardable; they cannot stand in for missing authoritative bytes after restart.

This is layered integrity verification, not a claim that an ordinary read has checked all retained history. Object hashes detect alteration when verified; they do not enforce physical write protection by themselves. The supported host excludes external edits to managed files. Full history health is established by explicit scrub and backup verification, not inferred from a successful small query.

### 11.3 Recovery and operation outcomes

Recovery validates the accepted ref and its generation, re-establishes reference/directory durability where publication was uncertain, and resolves required immutable roots. It never promotes staging, chooses the highest sequence found on disk, or silently falls back from a corrupt head. Caches and physical-location indexes can be discarded and rebuilt from verified object/pack content.

For an operation lookup, a verified path in the accepted generation's complete operation map proves either its original receipt binding or that the key is absent. An entry with an unreadable receipt is integrity/unavailability, not absence. A missing map node or failed recovery also cannot prove absence. This map removes the need to scan every historical receipt for each retry while preserving lifetime deduplication.

An explicit scrub traverses all retained roots, generations, tree nodes, logical records, receipts and original content. It verifies hashes, original-byte and semantic digests, scope, ancestry/sequence consistency, operation-map continuity and reference closure, and reports corruption without repairing accepted knowledge silently. Full backup restoration and migration require this complete check before activation. A cache of previous checks is not independent restoration evidence.

## 12. Retention, garbage collection and safe compaction

All accepted history is retained in the initial profile. Space savings come from sharing, lossless factoring, compression, chunk reuse and packing, not from dropping old baselines or receipts.

Reachability starts from immutable Hub metadata, all accepted branch refs, all their retained generations/ancestors and retained receipts. It follows map/sequence nodes, record payloads, baseline closures, source/locator bindings, content manifests and chunks. A logical exact reference must be resolved through the read's pinned registry or the exchange bundle's fixed registry and included; following only literal OIDs in JSON is insufficient. Semantic reference cycles use a visited exact-reference set, not recursive physical hashes. Backups/exports keep maintenance coordination for their complete source-copy operation in this initial profile; no unspecified pin/lease side file is required.

The initial collector/packer runs with exclusive maintenance access, so no publication or coordinated reader can race it. Its finite roots are captured after recovery. It may reclaim abandoned staging and objects not reachable from any retained root, and may remove redundant encodings once another verified durable representation is installed. It must retain all accepted records and operation results even when unselected from the current model.

Compaction protocol:

1. Recover and lock; mark the complete retained closure and identify reclaimable debris.
2. Build new immutable packs containing the required objects; verify every logical OID and content binding.
3. Sync new pack files, indexes and directories; publish/update the derived catalogue durably.
4. Confirm that every object being removed has a readable verified replacement, or is unreachable debris under the captured roots.
5. Remove obsolete loose files/packs and sync affected directories. Release coordination only after a consistent readable representation is established.

A crash before cleanup leaves duplicate encodings; a crash after cleanup still has the durable replacement. A catalogue failure is recoverable by scanning verified packs. An unindexed old pack is not automatically garbage. If reachability or recovery is uncertain, stop reclamation. Maintenance adds no accepted generation and never changes OIDs, public revision/snapshot IDs, record digests or receipts.

Hard links, filesystem deduplication, transparent compression and Git delta storage may reduce local allocation, but none is required for the portable format's sharing or correctness. Encryption and secure deletion need separate policies; logical garbage collection cannot promise erasure from backups or storage media.

## 13. Backup, interchange, deferred conversion and demonstration rebuild

### 13.1 Two distinct outputs

A **model exchange** names a baseline and its exact declared content/provenance/ontology closure. It declares omitted history and replay information. A **recovery backup** includes immutable store metadata, accepted references, every retained generation and receipt, the complete operation/ancestry maps and all reachable content needed for lifetime recovery. Derived caches, views, staging and location indexes are unnecessary. A backup may pack the objects differently without changing their logical OIDs.

Create backups from a captured root set while holding shared store coordination throughout the source copy, or use an offline/exclusively locked copy. This blocks publication/cleanup for the copy's duration. A future nonblocking backup lease/pin protocol requires its own specified persistent roots. An arbitrary live recursive directory copy is not an atomic backup. Restoration verifies the declared closure and original/semantic hashes independently before activation. A restore report identifies its exact snapshot/baseline roots and any omitted scope.

### 13.2 Deferred history-preserving conversion from files/2

Files/3 is incompatible with the existing files/1 and files/2 directory layout. An explicit converter is deferred future work for stores whose accepted history must survive; ordinary open never rewrites a store. The user selected fresh regeneration for the repository demonstration instead, as specified in §13.3. Neither full conversion nor recovery-backup implementation is a prerequisite for that rebuild. Files/1 needs a separately specified mapping if included later.

Conversion must:

1. Read a frozen recovered files/2 source and inventory every accepted generation, snapshot ID, exact record/revision, artifact, baseline, gap, receipt and request codec/fingerprint.
2. Build a separate files/3 destination, sharing content and factoring representations while preserving every public logical record, its exact semantic digest, original bytes and digest, selected revisions, baseline ordering/closure, historical snapshot IDs and original receipts. Factoring a full supporting set into a smaller semantic set is forbidden.
3. Record original-format provenance and a verifiable mapping from every old public snapshot ID to its new physical generation OID. Original receipts keep their old public previous/resulting heads. Physical directory names or generation OIDs need not match.
4. Preserve the original replay domain and request codecs only if a recovery-preserving conversion proves continuity for the complete retained history. Demonstrate old exact retries returning original receipts. A partial model import uses a fresh replay domain and must not present copied operation IDs as recovered commitments.
5. Compare complete source/destination public projections, all source-byte and semantic digests, every historical A/B result and the full original receipt set. Scrub the destination, reopen it, and run concurrency/interruption/retry checks under the selected profile.
6. Report measured sizes and verification evidence for review. Cut over only as an explicit store replacement with one active writer and a documented rollback decision. Retain the original store unchanged; never run both copies as independent writable continuations of the same replay domain.

Before the first new files/3 commit, rollback can reactivate the frozen source after exclusive cutover coordination. After any new acknowledged files/3 commit, simply reopening the old source as current would lose accepted history and is forbidden. Recovery then needs the complete current files/3 backup, or a separately supported reverse conversion preserving those new commitments. If the legacy format has no explicit replay-domain ID, the converter must record its frozen scope, accepted history inventory and provenance as the continuity basis for the new immutable domain identifier; it must not invent evidence that the old metadata already contained one.

Request/record codec changes, if eventually needed, require their own compatibility decisions. Merely changing a physical encoding does not authorise rehashing old semantic definitions, changing review status or relabelling historical execution evidence.

### 13.3 Selected canonical demonstration reset and rebuild

The repository's canonical `hub/` is generated from repository inputs. The user explicitly selected deletion of its previous content and generation of a fresh files/3 demonstration, together with the complete test suite. This is a new store lifetime/replay domain, not a history-preserving conversion. It does not weaken immutability for ordinary revisions, commits, baselines, maintenance or existing user stores.

The `rebuild-canonical-hub-and-verify` change must finalise files/3 host examples, current source/module bindings and verification/capture tooling. Run formatting, `cargo test --offline --all-targets`, `cargo test --offline --doc` and Clippy against the final code with original output and exact source/build hashes in new run-specific evidence paths. A failed required check or stale capture stops before deletion. Use existing Rust 1.99.1 and installed extraction tools; install no compiler.

After successful checks and fresh source capture, drain users, validate the exact repository-root demonstration path/scope with no symlink escape, explicitly delete its complete previous contents and create/populate files/3 through ordinary public APIs. Do not keep an automatic rollback store or import old snapshots/receipts. Old evidence outside `hub/` remains historical; new generation IDs, native evidence bindings, A/B baselines and original receipts belong to the new run. Generated stores/caches/output are excluded from recursive source capture.

Then drain/reopen, run full scrub, the full catalogue/self-model public walkthroughs, exact new A/B/narrative/outcome checks and indexed neighbour/phrase/composite examples. Rent and late-link examples use isolated fixtures or clones. Report the actual current record/source/gap counts, bytes and query work; preserve draft/applicability/authority distinctions. The captured checkpoint need not include later task/result status updates and must not be presented as a live mirror.

Only successful native and generated-dataset checks establish completion. Failure after deletion can leave absent or incomplete new data; report that state with attempt evidence outside the reset target and require explicit clean retry. There is no promise to recover intentionally deleted old history. Ordinary library create/open remains non-destructive. Full conversion and general backup/restore stay deferred to their own future bounded changes.

## 14. Behavioural acceptance and measurements

The bounded specification/implementation sequence should prove the applicable outcomes through public library operations and independently inspected storage, following the manual's learning loop. Backup/conversion checks apply to their deferred future capabilities, not the selected fresh rebuild. These are future checks, not results from this document.

| Scenario | Required observable result |
| --- | --- |
| Load an identical artifact under two source identities | One shared logical content representation; both exact source identities and provenance remain distinct. |
| Create A, revise one explanation, create B | A reads the original and B the successor after restart; unchanged records/content share OIDs. |
| Add a baseline selecting the current state | Reuse existing selection/reference-set/artifact/gap roots; add baseline metadata and changed tree paths, with no flat copy of the whole selection. |
| Retain a cyclic logical graph, including explicit baseline membership cycles | Exact references resolve through one pinned registry; physical objects remain acyclic and original semantic digests are preserved. |
| Change one field in one record | Add its actual successor payload/envelope and affected tree paths; unchanged artifact OIDs and tree substructures remain shared. |
| Change an artifact region or append a log | Preserve old bytes and digests; reuse unaffected fixed chunks where boundaries are unchanged. |
| Read exact source anchors | Reconstruct exact native quotations and original-byte offsets; PDF extraction coordinates retain their distinct provenance. |
| Race two changes from the same head | One coherent commit; the other gets a stale-head conflict; no mixed maps or lost receipt. |
| Interrupt each installation/reference persistence step | Recover the old or new complete accepted state; never a partially accepted change. |
| Lose a success response and retry after later commits | Return the exact original actor-bound receipt without a new generation. |
| Reuse an operation ID with changed intent or actor | Refuse reuse without disclosing an unauthorised original result. |
| Corrupt a blob, tree node, pack block or receipt | Explicit integrity failure for affected reads/scrub; never silent fallback or false operation absence. |
| Delete caches/catalogue/location indexes | Rebuild location data from verified immutable files; accepted roots and knowledge remain unchanged. |
| Pack/recompress, interrupt cleanup, reopen | Every retained OID resolves to identical logical bytes; snapshot IDs, baseline results and original receipts are unchanged. |
| Read concurrently with maintenance | Coordination preserves the pinned view; maintenance cannot delete the only readable representation. |
| Deferred recovery backup/restore | Independently recover all declared definitions, original artifacts, baselines, history and lifetime receipts when that capability is implemented. |
| Rebuild the canonical demonstration | Full native checks pass on current inputs; old hub content is deleted; a fresh files/3 self-model passes reopened A/B, scrub and indexed/public walkthroughs with new evidence. |
| Deferred history-preserving conversion | Complete logical/digest/history equality and original outcomes for an existing store when conversion is separately implemented. |

Measure at least: unique logical object bytes; compressed/packed file bytes; filesystem allocated bytes; file/object/block counts; retained generation and baseline counts; bytes newly written per operation; temporary compaction peak space; read/open/scrub time; and peak working memory. Report artifact bytes separately from model definitions, source-anchor metadata, baseline metadata and history/index overhead. Git repository size is a separate metric.

Keep the older 116 MB measurement/inventory as historical context and use controlled fixed-input fixtures for format-efficiency comparisons before resetting the generated demonstration. The fresh self-model may contain more source/API definitions; report its actual scope rather than claim byte-for-byte migration equivalence. Also exercise a larger synthetic model and a long sequence of tiny edits. Baseline creation and a small edit must not write another full model; changed-path writes may grow with tree depth, but not by copying every unchanged record. Repeated artifact references must not add artifact payload bytes. Packing must demonstrate reduced allocated space as well as logical sharing. No specific final megabyte figure is accepted without actual measurements.

## 15. Proposed delivery sequence and review decisions

These are progressively prepared outcomes, not newly approved slice IDs or implementation tasks. The owning slice plan and a future bounded OpenSpec change must establish the authorised sequence.

1. Resolve format choices and write executable behavioural specifications around exact baseline/generation/retry semantics and failure cases.
2. Establish immutable shared objects and persistent state roots; prove a small public create → commit → reopen → historical read → original-receipt path.
3. Add lossless baseline/anchor factoring and artifact chunk sharing, including legacy record-digest reconstruction.
4. Add compression, packing, bounded reads, full scrub and crash-safe maintenance; measure structural and allocated-space improvements.
5. Add generic bidirectional neighbour indexes and batch neighbour reads; prove late association insertion and view-correct incremental updates without per-question index design.
6. Compose phrase lookup and bounded typed traversal into deterministic projected evidence bundles, retaining explicit ambiguity, coverage and work limits.
7. Finalise host bindings/examples, run the complete test/check batch, capture current verified sources, explicitly delete previous canonical content and rebuild a fresh files/3 self-model; verify reopened A/B, scrub, public/indexed reads and actual new evidence. General conversion and recovery backup/restore remain deferred.

The corresponding planning changes, in implementation order, are
[shared storage](../openspec/changes/share-immutable-file-storage/proposal.md),
[neighbour lookup](../openspec/changes/index-ontology-neighbours/proposal.md),
[deterministic queries](../openspec/changes/query-deterministic-model-impact/proposal.md)
and [canonical rebuild and verification](../openspec/changes/rebuild-canonical-hub-and-verify/proposal.md).

| Review item | Proposed direction / unresolved consequence | Decision owner |
| --- | --- | --- |
| Physical format and codecs | Adopt files/3 with shared immutable JSON/byte objects, persistent roots, raw/zstd encodings and the pack grammar above; no in-place change to files/2. | User, informed by implementer evidence. |
| Initial tree/block/chunk parameters | Map leaf 64, sequence fanout/leaf 128, normal pack block 1 MiB/512 entries, fixed content chunk 1 MiB, proposed zstd level 3. Validate against measured metadata, allocation and memory costs before freezing the format. | Implementer proposes; user reviews. |
| Baseline authoring scope | Physical factoring preserves existing supporting sets. A minimal-closure convenience API is desirable but requires a separately specified behavioural change. | Definition Modelling / user. |
| Human-readable materialisation | Keep Markdown original bytes exact; provide explicit views/exports rather than mandatory duplicate loose files. Confirm the desired default repository browsing experience. | User. |
| Integrity profile | Verify required paths on ordinary reads and all retained closure on scrub/restore; document the change from files/2's repeated full-history scans and establish equivalent semantic/recovery guarantees. | Implementer and user. |
| Neighbour/index contract | The user selected local ontology-guided lookup. Use generic endpoint-first forward/reverse indexes of actual facts, shared by exact view; future routes are composed at query time. Cache format/default budgets remain proposed implementation details. | Definition Modelling / implementer. |
| Canonical demonstration lifecycle | User selected complete deletion and fresh regeneration with the full test suite and public/indexed checks. New replay domain/history; older external evidence remains historical. | User-selected scope; implementer verifies. |
| Deferred conversion/replay continuity | A future history-preserving converter must retain complete accepted history/receipts and one continuation. It is not needed for the generated demonstration rebuild. | Deferred Governance / user decision. |
| Longer-term optimisations | Content-defined chunking, compression dictionaries, alternative tree encodings, finer writer coordination and history-retention changes require separate versioned proposals and evidence. | Deferred; no implied support. |

Accepted decisions belong in the self-hosting plan's decision register; domain-language refinements belong in the domain analysis. This draft records the proposed choices and outstanding review points without silently editing those owners. The current store and all historical evidence remain unchanged by writing this proposal.
