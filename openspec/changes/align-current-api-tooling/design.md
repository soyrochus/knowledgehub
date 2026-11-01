# Design

## Context

Hard prerequisite: remove-legacy-storage-paths. Current examples/file_format chooses Files2 by default and dispatches explicit old/new constructors. Capture and its origins map feed authoring and coverage; archive-aware identity resolution is now part of the current authoring workflow. source_paths already bounds its top-level groups, but its explicit exclusion set lacks old_hub. The reusable reset host accepts files/2–3 and chooses old lock layouts.

## Goals / Non-Goals

**Goals:** every maintained command uses only the current implementation; source/version identities are exact; old_hub remains isolated; current guides agree with code.

**Non-Goals:** rewriting archived source/evidence, importing the retained store, renaming it for the user, running canonical population, or claiming a fresh native batch.

## Decisions

### Simplify host invocation

Remove the format enum/selector/parser and old-versus-current benchmark modes. Catalogue, loader and inspector always construct FileBackend::new and use KnowledgeHub/full contracts. Rename maintenance methods and example references consistently if changed in prior steps. Keep --verify/--compact/--result/--validate-input where they perform current useful work. Reject --format explicitly rather than silently accepting a historical alias.

Keep small demonstrations using the ordinary full ontology API. Update full-model host policy/support helpers; delete seed-only builders where they are no longer used. Do not delete semantic fault/coverage examples that have current value.

### Source and authority

Update actual public/module/function origins after the API restructuring, including changed files and runtime package-version observations. A source model must not claim OntologyHub or seed facade names are current. Keep archive-aware source resolution and exact byte/range bindings: they describe current repository movement/provenance, not old software support.

Current documentation and main requirement text describe the single implementation. Keep source/ originals and archived plans as original historical material; label older native outcomes with their old versions. Do not replace an older captured model's source artifacts or human-review status.

### User-retained directory

Treat old_hub as read-only external demonstration material. Explicitly exclude that directory name from recursive source/checkpoint enumeration and generated-capture guards, alongside hub/cache/outputs. Respect existing ignore preferences and add an appropriate root old_hub ignore entry if needed to keep generated retained data out of accidental capture/versioning.

The first cutover workflow requires hub absent. At planning inspection hub was absent and old_hub present, but recheck immediately before applying rebuild; do not treat this observation as permanent authorization to delete a later directory. If an older instance reappears at hub, refuse before changing either path and tell the user their rename is still required. Do not implement an old-format decoder solely for reset.

The canonical target and scope remain the only writable demonstration destination. Existing safe current-only --reset may be retained for explicit future current-instance retries, never for old_hub or arbitrary older versions.

### Verification boundary

Use disposable host tests to prove capture exclusion, wrong-target rejection, active writer refusal and preflight failure preservation. Exercise archived sources/current public identities by capture plus in-memory author validation. These are tooling/model checks, not an observed new full native run or a canonical rebuild.

Necessary mechanical changes may already have been made by steps 1/2 to keep builds green; verify them here and remove residual branches rather than delaying earlier compilation fixes.

## Risks / Trade-offs

- [A renamed store enters source capture] → explicit old_hub exclusion and nested/root fixture tests.
- [Unknown origin is hidden by an alias] → require actual new source/contract bindings; preserve source semantic identities only where they still describe the same concept.
- [Host reset still accepts old formats] → one current metadata/lock layout plus absent-destination cutover checks.
- [Guides mislabel historical success] → keep previous reports and the captured baseline unchanged, point current instructions at current code.

## Migration Plan

No data migration. Finish source/command/origin validation and current docs, then apply verify-v010-and-rebuild-hub. That final change performs fresh native checks, capture and new public-API population after rechecking the user's retained directory.

