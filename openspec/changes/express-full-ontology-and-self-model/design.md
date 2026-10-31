# Design

Planning draft · 31 October 2026. Source requirements are distinguished from
proposed refinements KH-D-11–15; this file establishes no implementation evidence.

## Context

See [proposal.md](proposal.md) for motivation and outcome boundaries. Inspection
found one implemented crate and no adopted main specifications. The previous
change is complete but unarchived. Preserve that change and its evidence.

Concrete extension points and limitations:

| Observed implementation | Required extension |
| --- | --- |
| `Profile::seed` defines four records with rule strings; `check_scope` accepts only equality with that profile. | First-class exact ontology/profile/type/constraint records and schema interpretation. Keep the seed codec/profile as a compatibility case. |
| `Element` has no typed property values; only narrative authority bindings exist. | Typed value and property schema contracts with explicit per-field authority/provenance. |
| Assertion validation searches only the ordinary element list. | A typed resolver for element revisions, native assertions, anchors and other kernel subjects. |
| Source validation requires UTF-8 and byte-range quotations. | Exact binary artifacts plus tagged native/derived locator bindings. |
| Candidate insertion rejects existing IDs; recovery requires prefix-preserved record arrays. | Append immutable revisions while selecting exactly one normative revision per selected identity; verify lineage and old content rather than forbidding successors. |
| Every non-genesis state requires the fixed atomic-change seed. | Profile-specific completeness; full ontology models need not contain this library's seed. |
| Public reads resolve exact snapshots only. | Exact baseline selections, stable typed filtering and finite directional traversal. |

The existing guarded publication, default-deny policy, original receipts,
nonblocking writer locks and bounded job ownership provide the foundation.
All five new capability specs are behavioural contracts; private signatures
can evolve without changing them.

## Goals / Non-Goals

**Goals:** one extensible semantic record model, explicit built-in catalogue,
measurable source coverage, meaningful validation and one real self-description
using the same public contracts as any other system model. Native infrastructure
and ontology records share exact addressing without duplicate owners.

**Non-Goals:** a Rust struct/module for every ontology type, execution of arbitrary
modelled code or processes, inference that a modelled aggregate is a live library
aggregate, a UI, automatic human approval or document-authority handover.
Interchange, database adapters and the broader query/review policies remain with
their later outcomes. Modelling their definitions is supported here.

## Decisions

### 1. Complete declared catalogue, schema-driven records

Proposed KH-D-11: supply `system-definition-ontology/1` as immutable bootstrap
data sourced from draft 0.2, with 17 kernel kinds, 128 family kinds, 32 qualified
catalogue relationship signatures and separately identified prose refinements.
Provide `knowledgehub-self-description/1` as a selected applicability/completeness
profile. Their contract versions are independent of source draft, crate and files.

Use one owned typed record envelope plus value/schema descriptors. Each property
descriptor defines name, value kind, cardinality, nullability, allowed reference
types/roles, list/set semantics, authority and constraints. Enumerations and
structured records have explicit member/field schemas; unknown properties fail.
Inheritance is explicit, acyclic and flattened for validation/discovery. Template
reuse in [ontology-coverage.md](ontology-coverage.md) does not merge type identities.

Values include text, boolean, signed integer, decimal, enumeration, exact reference,
artifact reference, record, ordered list and set, with explicit null only where
allowed. Integers/decimals use tagged canonical textual encodings when beyond
I-JSON safe integers; arbitrary binary floats are not admitted. Integer text is
canonical signed base-10 without redundant zeros; decimal is a signed integer
coefficient and integer base-10 exponent, removing trailing coefficient zeros
and representing zero as coefficient 0/exponent 0. Lists preserve order; schema
sets sort unique canonical value encodings. Narrative values reference exact
document fragments; graph text is permitted only where declared graph-owned.

Alternative: one Rust enum variant per modelling type would require recompiling
the semantic engine for vocabulary-only additions. An arbitrary property bag
would miss the source's explicit schema/constraint obligations. Schema-driven
records preserve a single versioning and validation foundation.

### 2. First-class ontology and kernel references

Proposed KH-D-12: keep a small trusted meta-schema in code to validate the first
ontology registration, then retain the registered definitions as ordinary exact,
readable governed content. Registration is an atomic scoped operation with
current host permission; definitions cannot run arbitrary callbacks/scripts.
The built-in catalogue is not an unreviewed runtime plug-in mechanism.

A fresh v2 store starts with the fixed trusted meta-schema and an empty governed
branch. Registration can submit the complete source/type/constraint/profile
closure and initial model profile selection in one guarded change: validate the
closure under the meta-schema first, then assess model records under the selected
profile. Bootstrap is explicit; the source ontology does not have to exist before
its definitions can be admitted. V2 metadata pins stable Hub/model/branch IDs,
format and meta-schema; exact model ontology/profile bindings live in immutable
selected model-scope records, not a silently mutable constructor value. Requests
name those exact bindings and their expected head. V1 scope semantics remain as-is.

A qualified exact reference includes model/namespace, semantic identity, revision
and subject kind/role. Native subjects such as an assertion or source anchor
have their own actual identity and content, accessible by this resolver. The
`ElementRevision` endpoint role denotes the actual exact typed element revision;
it does not create a second editable node describing that same revision.
`RelationshipAssertion` and `SourceAnchor` refer directly to their native records.
`RepresentationBinding` similarly resolves its actual representation and subject.

For v2, preallocate opaque immutable revision tokens in the caller's retained
candidate and store a separate tagged canonical record digest. Exact references
use the tokens; the accepted manifest binds each token to its one content digest.
This permits mutually referring type/schema definitions, recursive schema
references and a baseline/run selection without circular content hashes. Resolve
the whole candidate's symbol table before checking references. Reusing a token
for different content is always an integrity/validation failure. Preserve v1's
digest-as-revision encoding unchanged. The caller reuses all original tokens on
exact retry; it never regenerates them to resolve uncertainty.
The supplied built-in catalogue distributes its fixed revision tokens with its
immutable definition data, so registering the same catalogue in another fresh
store preserves definition references. New instance/successor tokens are allocated
before submission and remain part of the retained original intent.

Relationship signature IDs qualify source type/role and predicate, for example
`sdo:relationship:UseCase.invokes` and `sdo:relationship:ProcessStep.invokes`.
The same visible verb does not erase their different endpoint rules. Exact
pinning, schema compatibility and multiplicities are assessed against the
selected normative revision map plus explicitly retained supporting revisions.

Kernel materialisation maps are enumerated in the coverage inventory. Native
envelopes and modelled type definitions are different modelling levels. Both are
inspectable; one identity has one owner. No infinite bootstrap recursion is needed.

### 3. Validation has integrity, graph and purpose layers

Proposed KH-D-11/12: implement a closed set of deterministic constraint primitives
bound by exact `ConstraintDefinition` records: required/type/enum/range/uniqueness,
reference resolution/type/scope, cardinality, single ownership, exact membership,
acyclic declared containment, one aggregate root, lifecycle/process membership,
schema compatibility, contract completeness and exact review/evidence scope.
Compose these with typed parameters and finite conjunction/disjunction rules;
do not execute arbitrary business-rule text as a validator.

Apply record validity first, then selected graph rules, then profile adequacy.
`ontology-coverage.md` maps V-01–17 and every specialised source obligation.
Documented descriptions or exact native algorithm artifacts can satisfy declared
representation obligations; this does not assert algorithm execution/correctness.
Event schema compatibility is exact identity/version agreement by default,
or an explicit declared compatibility mapping; no guessed structural subtyping.

Draft profiles can permit missing completeness obligations through explicit
subject/path/reason/source gap records. They never permit malformed schemas,
wrong endpoints, dangling required references, two aggregate roots, competing
owners or rewritten revisions. Complete assessment requires all applicable
obligations. Human adequacy and actual run/acceptance facts remain separate.
Inapplicability is a reasoned profile claim, not removal of the ontology family.

Alternative: treating every missing field as an error would prevent progressive
modelling; treating every failure as a gap would defeat graph integrity.

### 4. Exact source bytes, artifacts and locators

Proposed KH-D-12: use a native artifact envelope with stable ID, exact original
digest/length, media type and provenance. Retain binary PDF/source-code/log bytes
without converting them to Markdown. Documents still own declared narrative.
Capture PDF and Markdown independently; bind claimed representations explicitly.

Anchor locators are tagged schemas covering the source's section/requirement IDs,
table rows, JSON pointers, code symbols, UI elements, image regions and time
ranges, plus the existing UTF-8 byte-range/quote and a PDF page locator. Each
resolves against exact captured content or a retained, source-bound locator/
extraction manifest. The manifest names original digest, extraction method/tool
version, indexed pages/symbols/regions and exact derived artifact digests. A
quotation is verified against its retained derived text, never against guessed
bytes in the original PDF. Bounds, source IDs and manifest bindings are checked.
Locator-resolution evidence is identified as host-authored observation; it is
not review approval or independent proof that an extractor is correct.

No universal renderer/parser is required inside the semantic engine. For the
walkthrough the host captures an actual page/extraction manifest for the supplied
26-page PDF and byte-range quotes for Markdown/code/logs. Additional native
locators retain the same resolving-manifest contract. A missing required resolver
manifest produces an unresolved binding, not a fabricated successful anchor.

### 5. Revision history and exact named baselines

Proposed KH-D-13: separate immutable records keyed by (identity, revision) from
the branch's selected normative revisions. Successor requests name the exact
predecessor and create append-only lineage. They never overwrite accepted bytes.
Document successors and affected narrative/semantic bindings are validated as one
candidate; unchanged historical support remains pinned and retained.

A named baseline contains its own stable ID/revision, scope, exact ontology and
profile, explicit element/assertion selections, exact supporting source/document/
artifact references, permitted gaps and optional parent-baseline references.
It does not include its own revision in a circular self-hash. There is one
selected normative revision per modelled identity. Multiple historical source or
document revisions can support different exact claims without becoming competing
normative owners of the same field. Every required reference resolves in that
explicit closure; no latest-resolution fallback exists.

Comparison orders by qualified semantic-ID UTF-8 bytes, then revision; it reports
selected additions/removals/successors, assertion endpoint changes and supporting
provenance/representation changes. Nothing is automatically rebased or re-pinned.
Named-baseline creation and revision selection publish with the complete receipt
under the existing branch lock and expected-head guard.

Alternative: named timestamps cannot reproduce exact cross-record membership.
Retaining every old record without an explicit selected map cannot establish
which revision a baseline describes.

### 6. Small explicit navigation contract

Proposed KH-D-14: select by exact type revision and/or semantic ID within one
resolved snapshot or baseline. Type filters mean exact type, not implicit subtype
expansion. There are no property predicates, full text, aggregation or vendor
query strings in this change. Returned values retain missing/null distinctions.

Traverse from exact subjects using qualified signatures and forward/reverse
direction. Use deterministic breadth-first expansion; order subjects by qualified
ID UTF-8 bytes/revision and edges by signature ID/assertion ID/revision. Deduplicate
exact subjects while retaining distinct assertions. Cycles terminate through the
visited set. Default depth 1, maximum 4; default page 100, maximum 1000; maximum
10,000 visited subjects and 10,000 edges per traversal. Reject invalid bounds.
Results state exhausted or truncated and identify the reached bound. A page
continuation can resume within remaining total traversal budgets; exhausted total
budgets require a narrower new request rather than unbounded continuation.

Cursors bind exact selection, scope, actor, canonical request, ordering, frontier,
visited sets and remaining budgets. Their opaque value is integrity-checked
against retained cursor state or an adapter-authenticated encoding; cursor payload
is not authority. This design selects retained adapter-private cursor state with
an opaque random ID and request binding, bounded by 64 active cursors per backend
instance; capacity refuses a new paginated query without mutation. Cursors can
be explicitly released, expire after 30 minutes of inactivity, and become invalid
on backend close/reopen. Snapshot/baseline references remain durable and allow
restarting the same query from the beginning. These are proposed limits, not
performance evidence. New pages always recheck current policy. External references
remain qualified without implicit remote fetch.

The public navigation contract reports invalid/expired continuation distinctly
from exhaustion. Cursor capacity, lifetime and total traversal bounds are explicit
consumer-visible limits, documented alongside the public API.

Alternative: broad query algebra would delay the user's model without helping
its first concrete questions. Exact-ID reads alone cannot conveniently follow
typed evidence connections or inspect catalogue coverage.

### 7. Preserve original intent across format boundaries

Proposed KH-D-15: keep `knowledgehub-files/1`, `knowledgehub-request/1`,
`knowledgehub-record/1` and seed definition codecs exact for existing stores and
receipts. Add opt-in `knowledgehub-files/2`, `knowledgehub-request/2`,
`knowledgehub-record/2` and explicit schema/catalogue bindings for new stores.
The file manifest binds ontology records, immutable revision index, selected maps,
baselines, original binary content, source manifests, history and original receipt.
Every required object participates in the same file/directory sync and reference
publication sequence. Readers reject unsupported format or damaged content.

New request fingerprints retain RFC 8785/SHA-256 over scope, actor, expected head,
ontology/profile definitions, ordered operations, guards and verified content
hashes/lengths, including all property/qualifier values and revision/baseline
selections. Distinct tags prevent treating v2 interpretation as an old retry.
Existing v1 exact retries use their original codec before stale-base logic.
No copying or re-keying committed old receipts into a fresh v2 replay domain.

Alternative: silently revising the old record encoding could invalidate retained
semantic hashes and original operation results. No in-place migration is needed
to demonstrate this change; the v1 walkthrough store remains valid.

### 8. A real self-model and native evidence

Use [self-model-map.md](self-model-map.md) as the concrete fixture contract. The
host captures exact source/repository bytes and actually runs relevant checks,
registers the catalogue through public operations, creates the library model and
publishes named baseline A. It navigates requirement → use case/command/function
→ component/artifact and requirement → case → actual run/evidence. Then it revises
one explanation and its explicit pins, establishes B, drains, reopens and compares.

Generate IDs from declared semantic namespaces, not paths/hashes. Names can
change while IDs remain. Every repository binding names an exact Git revision
when available and retained content digests; uncommitted work uses the exact
captured bytes and a declared working-tree snapshot, not a fictional commit.
Historical KH-SL-001 results retain their actual checked hashes. The implementation
of this change must produce new run evidence before claiming current-code success.

The self-model profile includes all applicable obligations, not an instance of
every available vocabulary type. Isolated catalogue conformance fixtures cover
UI, external integrations and other families without inventing them in this
embedded library. Native review records require actual assessment provenance;
no AcceptanceRecord is fabricated. Repository definitions retain authority until
a later explicit KH-SL-006 handover.

## Risks / Trade-offs

- [A type list masquerades as implementation] → Require schema mapping and valid/invalid persisted examples for every inventory entry, plus cross-family semantic checks.
- [New schema details are attributed to the PDF] → Keep source statements and proposed KH-D-11–15 refinements visibly separate, with review status.
- [Native records are duplicated as graph nodes] → Use one exact typed resolver and kernel materialisation map.
- [Full snapshots/history grow quickly] → Keep existing immutable-generation correctness; measure the full catalogue/self-model and record limits before claiming scale. No pruning of committed receipts.
- [Schema expressiveness creates an evaluator] → Use closed validation primitives and retain modelled algorithm artifacts without executing them.
- [New revisions invalidate retry hashes] → Version codecs explicitly and run unchanged v1 conformance plus new v2 recovery cases.
- [Evidence claims exceed what ran] → Bind each observation to exact captured build/source inputs and report gaps/inapplicability separately.

## Migration Plan

1. Retain existing v1 consumer paths and immutable fixtures; do not rewrite stores
   or archive the earlier change as part of this work.
2. Add new owned contracts, trusted meta-schema and opt-in v2 codec. Run old
   fingerprint/reopen/receipt checks unchanged before loading new models.
3. Register the catalogue and load catalogue conformance graphs in fresh isolated
   v2 stores through public operations. Add baseline/navigation operations.
4. Run the self-model consumer and the extended policy/race/replay/interruption
   cases; retain measured data size, run inputs and observable results.
5. On code rollback, v1 stores remain usable with the v1-capable version. Older
   binaries must refuse v2 stores; rollback never rewrites their accepted history.
   Portable migration and authority handover remain separately specified work.

## Open Questions

No behaviour-changing choice is left implicit. KH-D-11–15 are concrete proposed
refinements for review, owned by the decision register. Final private module
factoring, diagnostic wording and catalogue file splitting can be chosen during
implementation without changing these contracts. Measured performance and the
human acceptance outcome must be recorded from actual work, not predicted here.
