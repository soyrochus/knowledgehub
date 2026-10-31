# Typed ontology API and files/2

Implementation record · 31 October 2026 · technical checks do not record project acceptance.

`knowledgehub::ontology::OntologyHub` is the storage-independent typed API.
`FileBackend::new_ontology(path, FileOptions::local_ext4())` opts into
`knowledgehub-files/2`; the original `KnowledgeHub`/`FileBackend::new` remain
files/1. Each API refuses the other format. There is no in-place migration or
rollback of a v2 store to v1. Retain the original store when adopting v2.

A host provides its existing Tokio runtime, owned `Context` and current `Policy`.
Both current-thread and multithread runtimes use the same bounded blocking-job
admission and drain contract. `RegisterOntology` is a separate admission action;
stored Role, Permission or AuthorizationPolicy records do not grant access.
The library installs no compiler, runtime, extractor or service. The platform
complies with the caller's selected profile; actual observed checks remain scoped.

Create/open a typed Hub, get `(head, state)` through `snapshot`, construct
`Change::at(&state, head, operation_id)`, set ordered operations, validate, then
commit. Retain this exact request before submission. Reuse it unchanged to
recover its original receipt after response loss or a later head movement.
Changed actor or intent under that operation ID fails. `operation_result` returns
an authoritative original result only after successful recovery and current access.

`builtin_catalogue()?.operations()` registers all definitions, original PDF and
Markdown artifacts and selects the full ontology/profile in one atomic change.
The closed native record codec supplies the trusted bootstrap grammar. The stored
Type/Relationship/Constraint/Ontology/Profile records describe the domain vocabulary;
registration checks the whole candidate, so forward and mutually recursive schema
references are legal. Inheritance cycles, unresolved schemas and modifications
under built-in exact tokens fail. New types can refer recursively through values;
this is distinct from cyclic inheritance. Exact built-in definition content and
source/refinement citations are discoverable through ordinary reads and queries.

Every `Record` carries a qualified model/identity/opaque revision token and a
separate `knowledgehub-record/2` canonical digest. Allocating tokens before sealing
records permits mutually referring records without hash fixed points. Once
published, a token cannot be reused, even for a similar record. `Put` successors
pin their exact predecessor and retain the semantic ID. `Unselect` guards the
exact selected revision and removes it from the normative selection while retaining
its immutable history. `Select` restores an explicitly named retained revision; it never chooses latest. A retained identity must select its exact predecessor before publishing a successor, and cannot start a second lineage. Selecting schema definitions requires current registration authority. An old receipt always names its original head and records.

Values use a closed tagged grammar: null, text, Boolean, arbitrary precision
canonical integer strings, decimal coefficient/exponent strings, enumerations,
exact references, artifacts, narrative bindings, ordered lists, canonical distinct
sets and declared structured values. Absent and null differ. Decimal coefficients
have no redundant trailing zeros; zero has exponent zero. No binary floating-point
rounding is introduced. Record/structure schemas reject undeclared properties.
Document-owned values bind an exact source revision and resolving anchor; graph
properties and recursive ownership paths have one owner. Coherent document,
property, representation and assertion successors must be submitted together.

The exact resolver returns actual native assertion/source/representation records.
Element revisions are the actual record envelopes, not duplicate ElementRevision
nodes. `state.identity_reference(&revision)` addresses a read-only `@identity`
projection of that identity's original lineage header. It has no independent
editable source of truth. `represents`, `supportsAssertion` and `contradicts`
therefore target actual native subjects.

Artifacts retain independent original bytes, SHA-256 and media type. Native UTF-8
anchors check exact byte boundaries and quotations. Binary/non-native locator
kinds use an exact source-bound `LocatorManifest` artifact, extraction tool/version,
typed original section/ID/table/pointer/symbol/UI/region/time/page coordinates and a separate derived text span. Derived revisions retain
their original revision and derivation. Validation checks these bindings and bytes;
it does not independently establish an extractor's semantic accuracy.

`assess` distinguishes integrity violations, explicit permitted draft gaps and a
complete structural result. Gaps cannot waive bad references, wrong types,
competing ownership or changed immutable content. Closed constraint primitives
are requiredness, uniqueness, acyclicity, same-owner references, cardinality,
integer range and logical composition, plus the versioned V-01–17 structural
rules. Full type schemas enforce the S01–08 contracts. Structural validity never
executes a stored algorithm or establishes human adequacy, implementation truth,
measurement success or acceptance. Test/review scopes remain exact and do not
transfer to successors.

A `ModelBaseline` selects one normative revision per identity plus an explicit
supporting closure, ontology/profile and gap assessment. Supporting history does
not become a normative selection. Reads never substitute latest revisions.
`baseline_data()` captures an explicit complete intent, which a caller can narrow
only while preserving its exact closure. Named-baseline comparison orders qualified
identities and reports additions, removals, successors, data, provenance and narrative
binding changes, under current read policy.

Queries select exact type revisions/IDs, or traverse forward/reverse exact declared
signatures from exact subjects. Selection order is qualified reference order;
traversal is breadth first with ordered starts and edges ordered by signature then
assertion reference. Assertions are retained independently of endpoint filtering.
The API does not interpret vendor queries or arbitrary predicates. Page defaults
are 100 records/assertions, maximum 1000; depth defaults to 1, maximum 4; visited
subjects and edges each default/max 10000. Budgets cover the whole traversal,
including filtered nodes, and persist across pages. Exhaustion and total/depth
truncation are explicit. Submit a narrower request after truncation.

Opaque cursors retain one bounded selection, actor, scope and exact query; changing
any binding fails. Each backend retains at most 64 cursors, with explicit release
and 30-minute idle expiry. Draining or reopening invalidates continuations.
Each continuation rechecks current policy. Head movement cannot mix new revisions
into its retained pages. Cursor state is temporary and not a durable result format.
