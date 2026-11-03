# Standalone source-bound authoring

The `knowledgehub-model` binary is a deterministic local host over the public
KnowledgeHub API. It interprets no source scripts or rule bodies. An analyst or
external agent supplies source authority, typed definitions and independent
source expectations. No source application or service is needed.

The delivered path is source package → analyst/agent interpretation → typed plan
→ native assessment/publication → independent verification. Interpretation and
ambiguity decisions belong to the analyst or external coding agent. The tool
compiles the supplied plan and delegates semantic checks to the library; source
capture by itself supplies neither a completed interpretation nor a fully
formalized model. The maintained import/verify skills guide that external work.

Build with the installed Rust 1.99.1: `cargo build --offline --bin knowledgehub-model`.
Commands emit structured JSON diagnostics. `verified` means the reported native
and source checks passed; interpretation review, implementation and human
acceptance remain separately reported. `incomplete` is a successful report of
missing coverage, never a ready import.

Read the report dimensions separately:

| Dimension | What the result establishes |
| --- | --- |
| Native validity | The exact selected model conforms to native schema, reference, ownership and integrity rules. |
| Native completeness | The selected profile's structured obligations are satisfied; explicit unresolved gaps make this false. |
| Source coverage | The supplied obligation inventory, source mappings and bounded query expectations pass. The inventory's business adequacy still needs review. |
| Interpretation review | The recorded analyst/agent/human review status; automated checks do not promote it. |
| Implementation and acceptance | Separately evidenced facts; captured code and described workflows do not establish execution or acceptance. |

A report can be `verified` with `native_completeness: false`: all declared source
checks pass while the model retains valid draft omissions. For example, exact
source code may be captured and anchored while its detailed structured contract
remains unresolved. Narrative/Body/artifacts preserve that detail for further
formalization; the gap reasons and human review status remain visible.

```sh
target/debug/knowledgehub-model catalogue target/catalogue.json
target/debug/knowledgehub-model capture target/import-config.json
target/debug/knowledgehub-model prepare target/import-run model-plan.json coverage.json
target/debug/knowledgehub-model assess target/import-run target/host.json
target/debug/knowledgehub-model create target/import-run target/host.json
target/debug/knowledgehub-model resolve target/import-run target/host.json
target/debug/knowledgehub-model verify target/import-run target/host.json
```

All inputs use `knowledgehub-authoring/1`; selected catalogue/profile must match
`catalogue-3`. Use `catalogue` to obtain actual exact schema/signature tokens,
including qualified selectors. JSON rejects duplicate and unknown contract
fields. Native scalar values retain their tagged types and exact integer/decimal
strings. Absent fields and explicit Null are different.

Native Set arrays require unique members in strictly increasing canonical JSON
byte order, including each tagged Value envelope. Sending the same members in a
different order fails assessment before destination publication. For ordinary
text/reference specimens, sort serialized members with ordered object keys and
compact separators. Native assessment checks the final canonical encoding.
Never remove duplicate members to make invalid supplied data pass.

The capture config contains explicit `source`, new `destination`, new private
`work`, qualified host `actor`, grants, model `scope`, finite `limits`, every
source file's `inputs` entry and an `authority` manifest. Each input has stable
key, normalized relative path, media_type, authority role, rendition_group,
dependencies, exclude_reason and optional expected digest/length. The authority
manifest has field_owners, source-key precedence, unresolved_conflicts and
review_status. Declare working UI authority explicitly regardless of directory
name. Renditions retain separate originals. Every source file needs a disposition;
symlinks, traversal and source/output-root overlaps are refused. Missing local
dependencies remain coverage failures. Remote dependencies are never fetched.

The tool defaults to 10,000 files, 32 MiB/file, 256 MiB captured originals,
100,000 planned records, 256 MiB plan/coverage JSON, 1,000 pages/check,
1,000 records/page, 10,000 subjects/query and depth 32. Requests above native
limits fail explicitly. Original byte capture and JSON/native payload sizes are
different measurements. Oversized input is an explicit failure, never clipped.

A plan has format, catalogue, profile, baseline name, records, gaps and optional
selection `[ontology_key, profile_key]` for governed extensions. Each record has
key, exact type_ref, native tagged data, provenance anchor keys and optional
capture bytes. `{"$ref":"logical-key"}` replaces an exact reference slot;
identities are allocated before sealing, so mutual references work. Source keys
are `source:KEY`, `capture:KEY`, and `anchor:KEY` for nonblank valid UTF-8.
Custom anchors use verified UTF-8 byte ranges. Binary media require supplied
source-bound extraction and locator manifests using the native contracts;
capture success alone claims no extraction. Never run HTML/JavaScript or stored
business code to populate a deterministic import. No arbitrary external schema
resolution or obsolete-shape conversion is provided.

Native ElementData explicitly owns every supplied property path. Native kernel
records use their real RecordData variants, not shadow elements. Lifecycle
subjects, logical targets and subtype matching follow the
[logical contract](logical-design-contract.md). Native assessment owns schema,
reference, inherited and graph integrity checks. Compilation has no second
assignability implementation. Source ambiguity needs an actual retained
interpretation decision or an unresolved conflict. Do not mark agent prose
human-reviewed.

Coverage inputs contain items (id, source key, disposition, reason, targets),
queries, interpretation_review and source_gaps. Each target supplies logical key,
optional native-record JSON pointer and expected value, plus its original-source
anchor key. Use one semantic item for duplicate renditions, with separate exact
source bindings. Query entries contain name, the public Query JSON shape and
expected_ids. Verification replaces selection with the named baseline, follows
continuations within configured limits and reports exhaustion/truncation. An
unmapped requirement or control fails coverage even with zero structural gaps.
Exact retained artifacts can carry details beyond structured formalization; name
those limits. A missing external implementation is a source gap, not an artifact.

Query entries have name, kind (`query` or `neighbours`), query and expected_ids.
Public Selection JSON is
`{"selection":"Snapshot","reference":"prepared-selection"}`; verification
replaces it with the retained named baseline. Ordinary Query requires type_ref
(or null), include_subtypes, ids, starts, signatures, direction, depth, page_size,
subject_limit, edge_limit and preparation. Preparation contains allow_rebuild,
owner_limit, fact_limit and byte_limit. Neighbour queries compare opposite
endpoint IDs and support derived reverse subject reads.

Element plan data is
`{"kind":"Element","data":{"properties":{...},"owners":[...]}}`.
Properties are tagged Values, for example
`{"kind":"Text","value":"Reserve equipment"}` and
`{"kind":"Ref","value":{"$ref":"entity:Asset"}}`.
Narrative is
`{"kind":"Narrative","value":{"document":{"$ref":"capture:design"},"fragment":{"$ref":"anchor:design"}}}`.
Ownership entries have path and owner, e.g.
`{"path":"/name","owner":"Graph"}`. Supply one per value path, including
collection indices and nested properties. Narrative paths are Document-owned;
other paths are Graph-owned. Conflicting supplied ownership is rejected.
Mapping paths address the serialized Record, e.g.
`/data/data/properties/statement`. The mapping's original-source anchor must also
appear in the target record's provenance. Input entries additionally require the
source claim status (observation/hypothesis/authored-requirement/design-decision/
implementation-observation/validation-result); do not reclassify simulations as
observed execution.

`assess` publishes only into a separate disposable store under the run workspace.
`create` requires the destination absent and lets the native API create it
exclusively. It commits catalogue registration, then one atomic model/source/
baseline delta. It never splits a model to evade native limits. Invalid data
leaves that model unpublished. Partial initialization remains incomplete.

Each native dispatch first persists the exact Change, actor, fingerprint, guards
and stable operation ID. The run uses a nonblocking exclusive lock. Journal
replacement uses file/directory persistence and the native Mac full-flush fence.
Responses are retained afterward. `resolve` reconciles known original IDs under
current authority. Errors/denial are not absence. `resume WORK HOST.json` continues
only that recorded run; changed source/plan/context is refused. Never reset a
partial store or replace an uncertain operation ID. Corrupt/incomplete native
initialization is an explicit diagnostic requiring inspection, not reinitialization.

Every assess/create/resolve/resume/verify invocation requires a current separate
host JSON with actor and grants. Grants name native actions (Create, Open, Read,
Validate, Commit, RegisterOntology, OperationResult). This host configuration is
trusted operator input; imported roles grant nothing. Historical attribution
does not authorize recovery. No credentials are part of a model plan.

Handover artifacts are source-manifest.json, exact plan/coverage, key-map.json,
assessment.json, verification.json/.md, journal.json and generation-manifest.json.
The native model also captures exact plan/coverage/source/authority manifests;
final receipts stay outside their own commit hash. The journal enumerates only
known generation operations, not arbitrary native history. `verify` needs the Hub
and retained run, not the original source checkout or agent conversation. Later
ordinary native edits preserve the imported exact baseline. Canonical `hub/` and
`old_hub/` are outside this workflow.

`cargo test --offline --test model_authoring` exercises the local command boundary.
Core exhaustive semantics remain in ontology_contract/logical_design_contract;
the complete source reference and agent-host acceptance are additional evidence.
