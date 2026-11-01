# Sourced self-model consumer

Status: the canonical rebuild is postponed by user direction. Its unfinished attempt remains at 7/11 under openspec/deferred/rebuild-canonical-hub-and-verify/. The cache-storage prerequisite pack-derived-index-storage is implemented and technically verified; see [packed-cache evidence](../delivery/evidence/packed-cache/README.md). It performed no reset or recapture.

`examples/full_self_description.rs` authors through public owned contracts only.
It never reads private backend files. The host's `capture_self_model.py` reads the
repository and extractor output before admission, verifies the checked-run inventory,
and records exact paths, hashes, spans, toolchain/filesystem observations and actual
working-tree status. Git HEAD is recorded as context and never substituted for
modified bytes. Native run timestamps refer to the observed check batch; individual
subcase timings are not invented.

The canonical persisted instance is the repository-root [`hub/`](../hub/),
including all accepted generations, original artifacts, A/B and receipts. Run
`cargo run --offline --release --example inspect_self_model -- --format files3` to reopen it through
the public API without authoring changes. The inspector reads both explanations,
compares the exact baselines and recovers the original operation outcomes.
The recorded source baseline remains exact; moving the store or editing current
repository files does not refresh its historical model or evidence. The
[relocation inventory](../delivery/evidence/hub-relocation.json) describes the older files/2 store now retained under `orig_hub/`. The current files/3 dataset comes from the deferred rebuild attempt; [packed-cache evidence](../delivery/evidence/packed-cache/README.md) records its unchanged checkpoint and successful indexed verification.

To reproduce authoring separately, run the commands in [README](../README.md)
with fresh store paths, outside the existing canonical Hub. The verifier
retains original command output, source/Cargo hashes and exact native test-executable
hashes. The capture script refuses a different log or changed checked input. The
loader rechecks every captured file, so a stale inventory cannot label changed bytes
as the checked snapshot. PDF originals and Markdown originals remain independent;
all physical PDF pages bind to retained `pdftotext -layout` output and a source-bound
manifest. Extraction consistency establishes the declared binding, not human adequacy.

The model separates the vocabulary (`sdo:ontology:system-definition`), this governed
Hub (`kh:hub:self-description`) and the described library (`urn:knowledgehub:library`).
The seed's requirement, use case, assertion, document and ontology source keep their
semantic IDs in a fresh files/3 replay domain. Public types, named/tuple/enum payload
fields, constants, I/O operations and modules have individual typed source bindings.
Rules, terms, candidate contexts/aggregates, decisions, journeys, stories, slices,
tasks and actual cases expand the selected owning documents. Private file manifests
are modelled as implementation mappings rather than exposed consumer handles.

Declared Rust shapes are source interpretations, not a new executable serde/schema
compiler. Native source artifacts preserve attributes, conditional code and full
bodies. Trait declarations have sourced descriptive bodies; actual implementations
have symbolic Rust entrypoints and exact artifacts. An explicit gap records a
semantic refinement that the source has not settled. Draft assessment must not be
reported as purpose completeness. Proposed responsibility boundaries stay proposed;
no synthetic UI, runtime AI, deployment success or human acceptance is loaded.

The definition baseline pins the exact cases, code and sources available for the
observed native test batch. A later change loads its actual log/build manifest,
TestRun and EvidenceArtifact, then selects the self-description profile. Historical
logs retain their own captures and are not current-code success. A includes the
sourced model and available evidence. The retained authoring request changes only
the atomic-change explanation and its document/fragment revisions, with explicit
assertion/representation successors. B selects the successor while A retains its
old source and narrative. Old tests and reviews keep their exact assessed scope.

After drain/close/reopen the consumer compares A/B, reads both exact narratives,
recovers the original load outcome, retries the exact successor intent and traverses
Requirement ← TestCase ← TestRun. Q01–Q08 and group IDs are retained in the result
JSON beside the store. Q02–Q05 contain actual publicly read records; source/build/log
artifacts remain retrievable through the same public snapshot/baseline API.

The repository continues to own its definitions. The new explanation is a sourced
walkthrough interpretation; it does not rewrite original material or transfer
routine authority. Human review, full exchange/backup reconstruction and explicit
KH-SL-006 handover remain separate work. Use release mode for the larger walkthrough:
the host validates the full generated model and retained history, and debug builds
are slow at this model size. Selective files/3 reads avoid full-State materialisation. No broad performance or untested durability claim follows.

Semantic identity bindings are declared by `module_id`, `namespace` and `source_key`
and the explicit origins.json inventory shared with capture/audit. They map current origins to stable source/module/contract
names. A relocated file updates its origin binding while retaining that name.
Task IDs use change/task identity; contracts and fields use their declared Rust
namespace and symbol. Exact byte/span anchors describe provenance, not a second
editable definition. Unknown new source/module origins require an explicit binding.

The [canonical rebuild workflow](canonical-rebuild.md) uses a fresh run directory,
complete exact-input native checks, source/module/API bindings, actual A/B and
original receipts, full scrub and indexed library-model reads. The older relocation
inventory above remains historical. The captured checkpoint precedes final run
reports, task status and generated hub guide. It is not a live working-tree mirror.
