# Design

## Context

KnowledgeHub already supplies source artifacts/anchors, interpretations, typed records, guarded Changes, original-outcome lookup, baselines and bounded reads. Its self-description examples and host_sources.py/verify_full_ontology.py scripts target the canonical library model, not arbitrary source packages. They are not an import API and must not be repurposed to reset a user's Hub. C1/C2 supply the revised experience, subject, logical and inheritance contracts.

The new tool is deterministic. An external analyst/agent owns semantic interpretation; a reviewed plan and explicit source authority are its inputs. The target is a new native store, with no application database or Modelarium process.

## Goals / Non-Goals

**Goals:** catalogue discovery, exact package inventory/capture, typed compilation and assessment, safe fresh creation, durable original-intent recovery and independent readback/coverage.

**Non-Goals:** inference inside the Rust crate, arbitrary source execution, a universal extractor, incremental source reimport/merge, arbitrary history discovery, web registration, Modelarium changes, additional adapters, migrations or canonical store rebuilds.

## Decisions

### 1. Provide a standalone native tool

Add a knowledgehub-model binary in this crate with source-independent supporting authoring modules. Commands are catalogue, capture, prepare, assess, create, resolve and verify. Each accepts structured input/output files and emits machine-readable diagnostics; verify also writes a readable Markdown report. Document concrete invocations with explicit source, destination, run directory, profile and host actor. A command's success status must distinguish successful work from a ready verified model.

Use only the supported KnowledgeHub API for native state, the current file backend and existing Rust 1.99.1/host-owned async runtime. No Modelarium checkout, build bridge, HTTP server, SQL database, provider account or network call is needed. Do not route through the canonical self-model scripts or edit accepted backend files.

### 2. Make input and authority explicit

The versioned source manifest records stable source key, normalized relative path, original digest/length/media type, document/UI authority role, rendition group, local dependencies and disposition/reason for exclusions. Scan only the explicitly selected root; reject path traversal and symlink escapes. Exclude destination/run outputs from source discovery, refusing overlapping roots where that cannot be guaranteed. Ignore metadata only with a recorded exclusion. Missing local dependencies remain visible; never fetch a remote URL implicitly.

The authority manifest identifies normative field ownership and source precedence. Markdown/PDF originals remain separate even when an explicit interpretation maps their concepts together. The working UI retains its declared authority for interactions regardless of the directory's name. A plan may not quietly override an authoritative source with agent prose; material disagreements require a retained interpretation decision, with unreviewed status until actually reviewed.

The local operator supplies an explicit host policy/actor configuration separate from imported content. Only a registration-authorized context may bootstrap catalogue/profile definitions; a model writer uses ordinary model authority. Source-defined roles do not grant native privileges. Current native authorization governs recovery/disclosure as well as mutation; a recorded historical actor is attribution, not permission to impersonate it.

### 3. Use a typed neutral plan, not browser projections

The versioned plan uses stable logical keys, qualified exact schema/signature selections, schema-typed values, exact/logical reference tokens, source anchors, owned representations, interpretations and gaps. Preserve absence versus null and precision in native tagged values. Native kernel records use their declared typed forms, not shadow elements or arbitrary property bags. Preallocate identities before sealing mutually referring records. Emit a key-to-exact-reference map for independent inspection.

Compilation pins C1/C2's selected catalogue/profile and exact transitive parent closure. Lifecycle subjects point directly to application definitions; logical targets remain model-level definitions; reference and endpoint acceptance delegates to native assessment. Unknown/obsolete fields, ambiguous IDs, unsupported external resolution and invalid constructors fail with plan and source paths. No inverse-only fallback, string-kind compatibility or source-specific compiler branch.

UTF-8 anchors use verified byte ranges, not character offsets. Other media use the existing source-bound locator/extraction contract: retain original bytes and exact extraction/locator artifacts with method metadata. The tool validates supplied extraction results; it does not assume it can interpret every PDF or dynamic UI automatically. Unsupported extraction is an explicit coverage disposition. HTML/JS is captured and inspected as data, not executed by the CLI. External skills can supply source-bound observations without turning them into fabricated runs or approved reviews.

### 4. Separate assessment, creation and recovery

capture inventories and pins original bytes in a run workspace outside source/destination. prepare validates manifests and compiles a reviewable plan against one current contract. assess uses disposable native state to exercise the actual public validator; its result is not publication to the destination. The tool rejects any stale source digest or changed plan before creation. Model data is not accepted merely because compilation succeeded.

create exclusively creates a new destination. Refuse an existing path for fresh creation, even if it appears empty; never delete/reset/reinitialize it. Creation has explicit checkpoints: initialize -> register selected catalogue/profile -> capture original sources -> prepare/assess complete model change against the actual head -> publish model plus named baseline -> drain/reopen -> verify. Initialization and source capture can have separate native commits, but the promised application-model delta and its required bindings publish coherently. Do not split that delta to evade native atomic limits. Partial initialized stores are incomplete, not ready models.

Before every dispatched native change, atomically persist the exact request, fingerprint, base guards, actor, model scope and stable operation identity in the run journal. Flush the checkpoint before dispatch. Store returned receipts and resulting exact heads afterward. resolve/resume first reconciles each dispatched original identity using the ordinary outcome API. Same-intent retry returns the original result before stale-base checks; changed intent is refused. An uncertain or currently denied lookup remains uncertain/denied, never proof of absence and never permission to generate a new operation ID. Process loss at each checkpoint must be tested.

An existing destination may only be resumed with its exact recorded run and validated identity. Relevant source/plan/context changes require a deliberate new run and new destination, not conversion of the partial one. Subsequent ordinary native edits are supported by the library; incremental source synchronization is not added here.

### 5. Retain a bounded generation manifest

The run journal lives in an explicit private local work directory, with source/plan digests and known native requests/receipts. The ready handover contains a generation manifest identifying its destination scope, contract/profile, baseline, exact head, source inventory, source mappings and known operation IDs/receipts. Retain these inspection artifacts beside the generated Hub and bind their exact content through ordinary native artifact/representation records where applicable. Do not invent a recursive final-receipt self-hash; the final receipt is recorded after that commit.

verify accepts the native destination and retained handover artifacts in a new process; it needs no conversation state. Known original operations can be resolved under current authority. This manifest is not an enumeration of unknown commits or all native history: broad history discovery belongs to later C8. Reopening preserves exact source bytes and the named current-contract baseline, even if subsequent authorized native edits advance the head.

### 6. Measure source coverage independently

The expected inventory and query manifest are user/model data. Each expected source concept/control maps to exact model subjects/properties/assertions and resolving source fragments. Every file has a disposition. Report separate dimensions: original-byte retention, structured source coverage, valid native integrity, native completeness/gaps, interpretation review, implementation evidence and human acceptance. Zero native gaps cannot compensate for an omitted source requirement or UI behavior.

Missing expected in-scope semantics fails full-import coverage. A genuine absent external artifact or undecided source policy remains a named source gap; a complete account of that absence is not a claim that the missing fact exists. Duplicate renditions do not double the expected concept count. Read back source IDs and consequential UI states/controls/bindings/effects from native data; aggregate counts alone do not pass.

All verification reads select the declared baseline and consume continuations until exhaustion or a finite declared bound. Report actual records, bytes, page counts, limits, truncation and omitted items. An unexhausted query is incomplete. Keep original media and previews separately labelled; previews never replace original digests or establish verified interpretation by themselves.

### 7. Bound work and make failure inspectable

Initial tool defaults: at most 10,000 inventoried files, 32 MiB per source file, 256 MiB captured source bytes, 100,000 planned records and 256 MiB serialized plan. Initial verification limits: 1,000 records per requested page, 1,000 pages per check, 100,000 visited subjects/edges and depth 32, further constrained by native limits. Publish these as tool limits, not application ontology constants. Explicit operator configuration can choose other finite limits within native backend limits; unsupported/excessive requests fail before dispatch. Report requested and effective limits; never silently clip them. Do not inherit Modelarium's projection/payload caps.

Large inputs are inventoried where possible, with explicit oversize dispositions; no silent truncation or automatic partial-success assertion. Source/plan/coverage files must be read with finite byte/entry bounds, including extraction artifacts. Keep accepted-store mutation and staged external work separate. Cancellation retains the journal and truthful operation state.

### 8. Carry G6 semantics through the tool

Repeat native subject acceptance/rejection and current/A reopen checks (G6.1-01–04 native portion); non-executing rule/time-policy retention and structural-check labels (G6.2-01–04); permitted exact omission versus each malformed supplied-value class and planned realization (G6.3-01–04); authorized extensions, property/endpoint subtype matching, inherited constraints, invalid hierarchies, exact/inclusive reads, guards and reopen (G6.4-01–07 native/compiler portions). Core tests own the exhaustive matrix; tool tests demonstrate those semantics are neither bypassed nor reimplemented divergently. Use source-independent fixtures, including the unrelated domain, before C4's full package.

## Risks / Trade-offs

- Arbitrary source formats exceed one deterministic tool -> retain exact originals and explicit supported/unsupported extraction; skills own interpretation.
- Multiple bootstrap commits can leave partial stores -> no ready result until final reopen/verification, durable checkpoint recovery, no reset path.
- Full reports can be mistaken for human approval -> separate evidence dimensions and retain unreviewed interpretations.
- Captured sources can change during work -> digest-pin the bytes and refuse changed-input resume.
- Large graphs/payloads may exceed native limits -> measure and fail explicitly; do not weaken atomicity or reuse truncated projections.

## Cutover and acceptance boundary

Use a single versioned manifest/plan/tool contract aligned with C1/C2, regenerate fresh fixtures and reject obsolete inputs. No translators, old readers, aliases, migration or dual-version modes. Preserve hub/, old_hub/ and unrelated work. Acceptance is a native-only command sequence with failure/recovery tests and independent source/trace readback. If that outcome requires any Modelarium-owned change, abort instead of expanding the change.
