# Exact ontology neighbours

KH-SL-010 adds owned, deterministic one-hop reads over actual recorded facts. The existing Query/Page types retain their assertion-only meanings. The new operation requires the explicit files/3 backend; files/1–2 refuse its capability. Canonical regeneration remains KH-SL-012.

OntologyHub::neighbours(context, Neighbours::new(selection, starts)) accepts an exact snapshot or baseline, exact qualified starting references, outgoing/incoming/both directions, exact selectors, categories, an optional exact opposite type and explicit subtype inclusion. Definitions of legal endpoints do not create links. Plain text resembling an ID is never interpreted as a reference. Results contain owned references, selectors, origins and vertex availability/selection status. Records projection additionally returns deduplicated exact endpoint/origin records.

Selectors distinguish assertion signatures, registered property descriptors and native kernel fields. A property selector pins the owning element-type revision (including its pinned inherited fields), or the relationship-definition revision declaring a qualifier. Schema paths use escaped JSON-pointer components and * for collection items; origin occurrence paths retain concrete indices. Any exposes only explicitly tagged references and retains the path of its declared descriptor. Artifact/narrative links use the representation category. Provenance, native definition/header links and administrative metadata are separately selectable.

Default categories are associations and reference properties. Baseline selected/supporting membership remains in its existing membership maps and is not eagerly duplicated as adjacency. Native baseline ontology/profile/parent/gap references remain explicitly categorised. Default fact owners are selected records; FactScope::Closure additionally includes the exact snapshot's retained owners or the exact baseline's declared supporting owners. It never means all latest revisions. Selected facts can pin an older endpoint; the endpoint is returned exactly with selected=false. Native identity references resolve through their original record lineage and are not invented editable nodes.

Existing admission rules still govern which facts are valid. Selected assertion records require selected exact endpoints, so revising an endpoint also requires coherent assertion successors. Historical property pins and explicit supporting-closure facts can reach unselected retained revisions. Explicitly restoring an older endpoint selection is also supported; valid selected assertions then retain that older exact pin rather than following a newer retained revision.

Ordering is UTF-8 lexical by exact starting reference, selector variant/definition/path, opposite exact reference, origin owner/type/occurrence and category for any remaining tie. Starts are sorted/deduplicated. Parallel assertions and repeated occurrences are preserved. A self-loop queried in both directions appears once with outgoing and incoming markers.

| Bound | Default | Maximum |
| --- | ---: | ---: |
| Starting references | Supplied explicitly | 1,000 |
| Reference component bytes / aggregate filter bytes | Supplied explicitly | 4 KiB / 1 MiB |
| Results per page | 100 | 1,000 |
| Examined merged posting candidates per page | 10,000 | 10,000 |
| Projected neighbour/vertex/record bytes | 256 KiB | 1 MiB |
| Selector filters | None: discover actual groups | 1,000 |
| Groups discovered in one batch | Actual endpoint groups | 10,000 |
| Cold preparation owners | 10,000 | 100,000 |
| Cold preparation facts | 100,000 | 100,000 |
| Preparation record/fact bytes | 64 MiB | 64 MiB |
| Retained cursor pool, shared with Query | — | 64 per backend instance |
| Cursor idle lifetime | 30 minutes | 30 minutes |

The reached field distinguishes page, examined-posting and projected-byte boundaries. A continuation retains the exact position and full request binding; exhausted is true only when no eligible enumeration remains. Empty budget-limited pages can continue. A single item/start projection that cannot fit returns Capacity, preventing a continuation that cannot progress. Directional self-loop duplicates are merged before the examined-candidate budget; physical posting reads are separately reported. Index/record buffers and inherited-type traversal have finite bounds too.

Use continue_neighbours(context, original_request, token) and release_cursor(context, token). Tokens bind actor, scope, exact selection, every filter, direction, fact scope, projection and limits. Current read authority is checked again per request. Head movement does not change membership. Changed intent, wrong actor, expired/released/wrong-mode tokens fail. Concurrent consumption of one token cannot deliver duplicate pages; a competing request gets an explicit cursor/busy error.

The adapter stores two postings per actual fact in endpoint → selector → sequence maps. Its manifest fingerprints scope, selected/supporting roots, ontology/profile, fact scope and extractor/category grammars, then binds both roots and completion inventory with a checksum. The separate cache/neighbours/ namespace uses typed map/sequence codecs and immutable sharing. It is disposable and excluded from accepted backup closure.

The first lookup can prepare its exact view within the request budget. prepare_neighbours makes that cold work explicit; preparation.allow_rebuild=false requires readiness. Missing, incompatible or corrupt derived paths trigger bounded reconstruction or IndexNotReady/Capacity. Authoritative corruption remains an integrity error. A cache never supplies a false complete/empty answer for a different view. Ready lookups retrieve the relevant endpoint/selector paths and only needed exact records; they do not materialise full State/history or unrelated originals. Returned postings are checked against their exact authoritative origins.

When a prepared current view changes, publication computes removed/added owner facts and updates both directions, sharing unchanged nodes. Ontology/profile changes invalidate readiness and require a new build. New caches are installed only after graph publication is recoverable. Cache installation errors are recorded as adapter diagnostics and cannot change the original accepted receipt. Exact retries remain authoritative.

Preparation/publication/eviction hold the stable store's exclusive lease; a complete read holds its shared lease. Cache space is bounded to 256 MiB of allocated regular files, with at most 32 advertised views. A new build reserves space before immutable object installation; quota/view eviction can discard indexes together and reconstruct old views later. FileBackend::clear_neighbour_cache(context, scope) explicitly evicts only derived data under coordination. Cache mutation may inventory its own files for quota accounting; ready lookups do not perform that inventory.

NeighbourWork separates prepared owners/facts, index node/byte reads, required record reads, examined candidates, physical postings read and rebuild diagnostics. FileWork.state_materialisations helps verify the selective boundary. Timings must distinguish cold preparation, a fresh backend/page read and subsequent lookup; index growth includes actual native/provenance facts, not merely user assertions.

On files/3, existing assertion traversal uses adjacency while preserving its original signature/assertion ordering, filtering, depth-four and subject/edge/page limits. Empty-start type/enumeration queries keep their existing full-state implementation. New text entry and declarative multi-hop impact operations belong to KH-SL-011; this slice implements no AI calls, inferred edges, arbitrary execution or canonical rebuild.

The isolated public command is:

~~~sh
cargo run --offline --example neighbours -- /absolute/unused/store
~~~

The parent directory must exist. The example registers its vocabulary, prepares an initial view, inserts later links, compares old/new views, preserves historical endpoint pins and original receipts, then reopens. Actual verification and measurements are retained separately in delivery/evidence/neighbours/.

JSON request decoding refuses input larger than 1 MiB before parsing. Diagnostic inventory counts cover all indexed categories, while examined/record/node counts describe the actual request. The initial persisted cache uses compressed loose objects: filesystem block allocation exceeds encoded bytes, and a sufficiently large cache may be evicted/rebuilt at its quota. This cache-space tradeoff is measured explicitly; compact authoritative storage does not imply equally compact derived cache allocation.
