# Deterministic model reads

KH-SL-011 implementation contract · 1 November 2026. Technical evidence belongs in [the query evidence record](../delivery/evidence/impact/README.md); human acceptance and the canonical rebuild are separate. This document owns the initial API choices for KH-D-18, not the source ontology.

The public operations are KnowledgeHub::search_text(SearchText), traverse(Traverse), and model_impact(ModelImpact), with Context as the first argument. A composite read finds starts, follows an explicit policy and projects candidate dependencies in one public request. It uses ordinary exact types and selectors, so the same engine serves different vocabularies. It makes no agent, network, modelled-code or script calls and performs no accepted graph mutation.

Run the isolated rent model through the public API:

~~~sh
cargo run --offline --example rent_impact -- /absolute/unused/fixture-store
~~~

The parent must exist and the destination must be absent. The executable [request and fixture](../examples/rent_support/mod.rs) register a local physical DataSchema subtype with database/schema/table coordinates, a native DDL artifact, and MappingRule source/target subjects and validated field selectors. The fixture also contains explicitly synthetic catalogue conformance specimens; none describe the real KnowledgeHub application. It never opens or changes the canonical demonstration.

The actual seven-hop plan is: UserStory ←formalises— Requirement ←satisfies— UseCase —invokes→ FunctionDefinition —operatesOn→ AggregateDefinition —hasRoot→ EntityType ←MappingRule.source.subject— MappingRule —target.subject→ PhysicalTable. Exact stored assertions and property occurrences supply every hop. Type compatibility alone supplies no edge.

## Literal entry

TextSelector requires 3–4096 UTF-8 phrase bytes and 1–64 explicit TextField selectors. Each field pins an exact registered type and an escaped JSON-pointer property path. Nested records/structured values, union alternatives and collection wildcards are supported; inheritance resolves declared fields. Any, metadata IDs, unrelated artifacts and unselected history are excluded. A field selector applies to records of that exact type; name subtypes explicitly when selecting text.

- literal-utf8/1 matches contiguous original UTF-8 bytes.
- literal-ascii-fold/1 folds only ASCII A–Z bytewise. Other bytes, accents and Unicode case remain exact. No tokenisation, normalisation, relevance ranking, stemming, synonyms or embeddings occur.

The disposable trigram index intersects candidates within one exact owner/field and verifies the original field under the requested mode. Matching grams in different fields or in the wrong order cannot produce a hit. Each matching field returns its first contiguous occurrence, ordered by exact model/ID/revision, concrete field path and byte range. Every matching start remains available within the stated bounds; no ranking chooses a story.

TextHit offsets are UTF-8 byte offsets within the field/fragment. Narrative hits additionally bind the exact document, anchor, span document, content hash/length and absolute span bytes. Byte-range anchors refer to the original capture. Manifest locators identify the derived UTF-8 capture plus the original typed location; derived offsets never masquerade as PDF byte offsets. Snippets preserve original case. Full artifacts are never included implicitly. read_artifact_range retrieves a bounded exact follow-up span.

The files/4 cache/text/ namespace contains compressed immutable typed field maps and gram posting sequences. A completed manifest binds scope, selected/supporting roots, ontology/profile, exact field selection, analyzer and extractor version. It stores references and hashes rather than another authoritative text copy. Ready reads perform gram/field point lookups and selective authoritative fragment verification. Cold preparation can enumerate selected owners within explicit budgets. Missing, stale, incompatible or corrupt derived paths yield IndexNotReady or bounded reconstruction; authoritative corruption retains its integrity error. preparation.allow_rebuild=false explicitly requires readiness.

After a commit, already prepared current-view configurations update added/removed exact owners and their selected fields. Narrative values pin immutable document/anchor revisions: changing that binding changes the owner revision and its index entry. Publishing another source capture alone cannot silently refresh an old narrative. Schema/profile changes require new preparation. Derived maintenance follows durable publication and cannot replace the accepted receipt. Old views are reproducible even after cache eviction. Text cache retention is at most 32 advertised views and 128 MiB allocated regular files; construction reserves allocated space, with at most 100,000 distinct grams per field and 1,000,000 total postings. Incremental work shares unchanged tree nodes. No per-question route index is created.

## Closed plans, evidence and limits

TraversalPlan is either Repeated with finite allowed step alternatives or Ordered with alternatives at each successive position. A step names exact selectors, direction, categories and an optional opposite type/subtype filter. Types and selectors must resolve against the pinned ontology before evaluation; unknown/external or unsupported selectors fail explicitly. The data contains no executable callbacks. ModelImpact::from_json, Traverse::from_json and SearchText::from_json reject unknown fields/modes and inputs larger than 1 MiB; Rust callers receive the same finite semantic validation.

Execution expands breadth-first, sorting starts and transitions by UTF-8 exact-reference/selector/origin order. Each frontier/position is batched through local neighbour lookups (chunks of at most 100 subjects), never external calls per hop. Visitation includes start, exact subject and ordered position (the repeated policy has one state). Revisiting a subject in another position is permitted. A discovered target does not end exploration. Endpoint grouping and witness origin grouping are distinct; the bounded witnesses are deterministic shortest paths per start/target, not an all-paths completeness claim. Parallel origins remain distinct within the witness bound; cycles terminate.

| Bound | Default | Maximum |
| --- | ---: | ---: |
| Hops | 8 | 32 |
| Ordered positions / total alternatives | explicit | 32 / 64 |
| Matched starts | 100 | 1,000 |
| Visited start/subject/policy states | 2,000 | 10,000 |
| Combined examined work | 5,000 | 10,000 |
| Exact text candidates verified | 2,000 | 10,000 |
| Retained start/target groups | 1,000 | 10,000 |
| Shortest witnesses per start/target | 1 | 8 |
| Groups per response page | 100 | 1,000 |
| Encoded response bytes | 256 KiB | 1 MiB |
| Cold owners / fields / source-record bytes | 10,000 / 100,000 / 64 MiB | 100,000 / 100,000 / 64 MiB |

Combined work includes text posting examinations/binary-search probes, examined neighbour candidates (including rejected/duplicate edges) and transition attempts for origins/states. ReadWork reports these separately, along with batch lookups, verified text candidates, record reads/hydrated bytes, index objects/bytes and cold preparation. An overlapping alternative or another path cannot bypass the work budget. Small budgets can stop before a candidate target is established. Diagnostics include necessary repeated schema/origin hydration across batches, not just returned payload.

Reached Starts, Candidates, Hops, States, Examined, Targets and Bytes identify incomplete requested work or omitted projection. Witnesses identifies additional paths omitted by shortest/finite witness selection; it alone does not imply missing reachable endpoints. exhausted describes bounded evaluation completion and is false while output pages remain. Ordinary pagination has a continuation independently of reached work limits. A limit is conservatively reported when reached, even if an extra examination could have proved completion. Graph/record cache buffers are capped at 64 MiB, frontier and retained witnesses at 8 MiB each, retained hydration at 8 MiB, and a composite cursor result at 16 MiB. Capacity errors identify unsupported resource retention rather than silently changing semantics.

ModelImpact resolves head once when selection is None. Every phase uses that exact returned selection. MatchPolicy is explicit AllGrouped or RequireOne. NoMatch, Ambiguous and Incomplete differ; require-one never traverses an incomplete or ambiguous match set. Shared targets hydrate once in the page's compact record dictionary, while each start retains its own dependency/path group. Oversized records are named in omitted_records for exact read follow-up; oversized witnesses reduce to exact start/target references with omitted=true. Metadata omission is explicit. If even the essential exact reference/header cannot fit, Capacity is returned. A continuation always consumes at least one group, so tiny limits cannot create an endless empty-page loop.

continue_model_impact/continue_traverse require the original entire request and token. The existing backend-wide pool admits 64 cursors with a 30-minute idle lifetime; it is shared with Query and Neighbours. Composite cursors retain bounded evaluated evidence, with a completed evaluation frontier, then page that evidence without new traversal. They bind actor, scope, resolved selection and full original intent (including the original head-selection form, matching, plan, projection and budgets). Head movement cannot mix views. Changed intent/actor, wrong cursor mode, release, expiry, drain and a fresh backend invalidate reuse. Every continuation rechecks current disclosure. This extends the API without changing legacy Query/Page depth-four behaviour.

## Meaning and coverage

Every result means candidate_dependency under the supplied plan. Coverage records declared gaps, unresolved local/external starts or endpoints, dead ends, bounded omissions and whether no dependency was witnessed. domain_complete remains false: structural exhaustion does not prove authors captured all real-world dependencies. Accepted graphs already reject required dangling references; an unsupported selector is rejected rather than silently acquired remotely. No remote reference is fetched.

Missing mappings, no text match, ambiguity, incomplete work and no witnessed path have different outcomes. A wording-only story successor can keep the same candidate table targets. A path to a physical table never proves its structure must change or that a schema migration is required. That interpretation belongs to the calling human/AI. Named query registries, richer analyzers, automatic interpretation and canonical regeneration remain separate changes.

The fixed-neighbourhood measurement command is:

~~~sh
cargo run --release --offline --example impact_benchmark -- /absolute/unused/benchmark-store
~~~

It reports cold preparation separately from one ready public request, growing unrelated graph data, source/index work, response bytes and a filtered high-fan-out case. These measurements establish a scoped implementation property, not a platform-independent latency promise.
