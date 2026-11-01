# Current shared immutable file storage

Implementation guide for KH-SL-009, `share-immutable-file-storage`. The wire grammar follows [file-storage proposal draft 0.3](../source/file-storage-proposal.md). Implementation and automated verification do not establish human acceptance or transfer document authority.

Use `FileBackend::new(absolute_path, FileOptions::local_ext4())` with the full
`knowledgehub::KnowledgeHub` API. Crate 0.10.0 supports only files/4. Older
constructors, modes and decoders are removed; open rejects older stores without
changing them. A fresh root hub is generated only after the final checked batch.
The user-retained old_hub is excluded from capture and remains untouched. Use the
existing Rust 1.99.1; install no compiler.

A generation is one accepted atomic change: an opaque snapshot ID, immutable roots, its original receipt and a parent generation. A baseline is a model record naming an exact selection and supporting closure. Creating a baseline normally creates a generation. Packing creates neither. All accepted history and lifetime operation outcomes remain retained.

The adapter hashes type-separated logical bytes, then chooses raw or zstd encoding. Records reconstruct the original record/2 body and verify its original semantic digest. Generic data has a removable physical wrapper. Baselines factor membership into maps and retain an explicit order sequence when needed; duplicates permitted by the semantic codec remain in the order sequence. Anchor quotes factor into verified exact UTF-8 spans, preserving optional null quotes. Cyclic model references remain opaque exact semantic references and introduce no physical hash cycle. Native identity/revision projections still come from their original owner; storage does not create competing kernel definitions.

The selected, retained revision, companion reference, artifact, gap, operation and accepted ancestor roots are separate. Old immutable record envelopes are reused, including their original baseline artifact roots. A commit validates the complete current candidate and current host policy under the publication lease. It installs and syncs missing objects before atomically replacing and syncing the sole accepted branch reference. Original operation lookup precedes stale-head rejection. A post-replacement failure returns `Uncertain`; resolve it with `operation_result` using the original actor and scoped operation ID.

`head`, exact `read`, `read_artifact_range`, historical snapshot selection and operation lookup follow the needed immutable paths. They do not replay all generations or load unrelated artifacts. `snapshot`, `read_state`, current query/navigation and commit validation still materialise the full promised state. An unselected identity projection may enumerate retained reference keys to locate its original lineage. Corruption outside a point-read path is detected by full scrub. A verified operation-map search establishes absence; missing map nodes/receipts are integrity failures, never absence.

Artifacts larger than 1 MiB use fixed 1 MiB chunks and a manifest retaining full original SHA-256/length. Range results carry the original reference, offset and exact selection. A partial chunked read verifies its chunk OIDs and immutable manifest, and reports `full_content_verified=false`; a complete reconstruction verifies the original digest. Small blobs are fully verified even for a partial range. Ranges perform no external I/O or interpretation.

The initial finite adapter bounds are 64 MiB per logical object, 4 KiB framing headers, 64 levels of sequence traversal, one million entries per map/sequence and per location cache, 32 MiB of decoded-object cache plus one 1 MiB decoded pack block per admitted operation, 64 MiB per owned range and 256 MiB per owned full artifact. Exceeding a valid capacity returns `Capacity`; malformed lengths, bindings and payloads fail integrity checks. The host also bounds active/waiting jobs through `FileOptions`. Full states necessarily consume memory proportional to their returned contents; this is not a streaming state API.

Closed packs contain independently verifiable blocks of at most 1 MiB/512 entries, with a dedicated bounded block for a larger object. Pack filenames hash their complete encoded bytes. Installation and scrub verify full pack hashes; point reads seek and verify their block and logical OID. The per-backend location cache is bounded and tied to pack/index inventory metadata. Cold index preparation is reported separately through `FileWork.index_bytes_read` and `index_rebuilds`. Index/catalogue deletion or a wrong/missing index entry triggers verified pack rediscovery. These files never decide model acceptance.

`scrub_ontology(context, scope)` verifies complete retained physical closure, every state/source digest, history transitions and lifetime maps. `compact_ontology(context, scope)` requires current commit permission, acquires exclusive store coordination, scrubs first, installs and verifies a durable replacement pack and only then removes loose encodings, superseded packs and preparation debris. It never prunes accepted knowledge. Readers hold a shared lock until their owned result is materialised; publication/maintenance use the same stable exclusive lock and return `Busy` on contention. Keep the host Tokio runtime alive and drain before shutting it down.

Disposable neighbour/text nodes now use [sealed variable-size packs](packed-cache-storage.md) directly. Normal blocks remain bounded by 1 MiB/512 entries; finish makes every required pack durable before the exact-view manifest. These namespaces share catalog-stamp-bound location maps across ready batches, account cold metadata separately and preserve the existing 256 MiB/128 MiB allocated-file quotas. Accepted-object writes and explicit compaction retain the protocol above. [KH-SL-013 evidence](../delivery/evidence/packed-cache/README.md) records unchanged canonical files, successful public queries, reopen and full scrub.

Run these commands with fresh absolute paths whose parent already exists. Replace the paths with unused directories under this checkout's `target/`; do not use `hub/`.

```sh
cargo run --offline --example shared_storage -- create /absolute/checkout/target/files3-demo
cargo run --offline --example shared_storage -- scrub /absolute/checkout/target/files3-demo
cargo run --offline --example shared_storage -- pack /absolute/checkout/target/files3-demo
cargo run --offline --example shared_storage -- export /absolute/checkout/target/files3-demo /absolute/checkout/target/files3-export
```

The public example creates sourced catalogue/specimen commits, baseline A, a coherent successor, baseline B, reopens, reads exact old/new records and original bytes and retries the original change. Export uses the ordinary semantic read API to write byte-exact Markdown, an owned graph and an exact snapshot watermark into a new host-owned directory. It is a derived readable view, not another authority.

Zstd 0.13.3 is pinned with default features disabled. Its transitive native codec is used only for lossless local encoding; external dictionaries, concatenated/skippable frames and excessive windows/output lengths are rejected. Dependency provenance, executed checks, fixed-input measurements and the precise process-fault evidence boundary are recorded in `delivery/evidence/files3-*`, separately from earlier files/1–2 evidence.

The fault harness actually kills a separate writer process at eight publication checkpoints (staging, staged content, content sync, generation installation, staged reference, head replacement, head sync and acknowledgement), and at seven pack/cleanup checkpoints (completed staged pack, installed pack, index, catalogue, cleanup admission and before/after removal). Injected errors and cancelled callers exercise the same publication boundary; current-thread and two-worker Tokio hosts are covered. These are process-interruption and injected-error observations on the declared filesystem, not power-loss, kernel-crash or hardware durability tests. They do not broaden the supported filesystem profile.

Diagnostics count unique logical OIDs separately from encoded regular-file bytes and allocated regular-file blocks (`st_blocks * 512`; directory blocks and Git storage excluded). Temporary hard-link aliases count once by device/inode. Reports separate artifact blob bytes, generic record data, baseline data, anchor data and the remaining envelopes/history/tree overhead. Shared derived location bytes and readable views are identified separately. New-object counters split logical payload bytes from structural metadata and report actual loose encoded writes. Compaction records encoded and allocated high-water observations after the replacement pack/index/catalogue are installed while original encodings remain. Final external benchmark runs record process peak RSS, actual timing and cache conditions.

`FileWork` counts decoded object requests requiring physical retrieval, encoded object/block bytes, loose-object writes and cold location-index work. It does not count every filesystem metadata call, directory enumeration, head/reference fsync or returned allocation. Full candidate validation/materialisation remains proportional to model size even when disk writes share all unchanged content.
