# Standalone reference inputs

From the repository root, build
`cargo build --offline --release --bin knowledgehub-model`, then run:

```sh
python3 examples/fixtures/run.py unrelated-design --binary target/release/knowledgehub-model
python3 examples/fixtures/run.py memento --binary target/release/knowledgehub-model
```

Every replay uses a fresh target/rich-design run, separate native Hub and private
journal. It executes catalogue, capture, prepare, disposable assessment, create,
resolve and independent verify. Existing paths are never reset. Read the emitted
verification.json/.md and generation-manifest.json before making any completion
claim. `incomplete` and permitted native gaps remain meaningful outcomes.

## Recorded native demonstrations

The 3 November 2026 [implementation evidence](../../delivery/evidence/mm-sl-01/20261103-apply/README.md)
records these separate current-contract Hubs:

| Fixture | Declared source obligations | Native queries | Native validity | Native completeness | Structured gaps |
| --- | --- | --- | --- | --- | --- |
| Memento | 1,091 | 25 | Valid | Incomplete | 1,504 |
| Equipment reservation | 10 | 4 | Valid | Incomplete | 10 |

Both exhausted their query continuations, reopened successfully and preserved
baseline A, exact original bytes and original receipts after an ordinary coherent
successor B. The obligation counts include source identity/artifact accounting;
they are not counts of fully formalized business behaviours. Memento includes
530 individually located UI source constructs. Human meaning acceptance remains
pending, and no source application, business rule entrypoint or timer was executed.

The retained local runs are `target/rich-design/apply-memento-acceptance` and
`target/rich-design/apply-equipment-acceptance`. They are generated outputs and
will exist in a fresh checkout only after a new replay. Each run's
`run/generation-manifest.json` identifies its native directory, scope, original
generation head, baseline and receipts. `successor.json` identifies B, and
`run/verification.json` distinguishes the original head from `current_head`.

To verify the recorded runs in a new process, review the demo host inputs against
current authority, then use:

```sh
target/release/knowledgehub-model verify target/rich-design/apply-memento-acceptance/run target/rich-design/apply-memento-acceptance/host.json
target/release/knowledgehub-model verify target/rich-design/apply-equipment-acceptance/run target/rich-design/apply-equipment-acceptance/host.json
```

These reads use retained sources and manifests; no source application checkout
or original agent conversation is needed. The root `hub/` remains a protected
codec-/2 self-description. These /3 demonstrations do not refresh it or transfer
definition ownership. Modelarium registration and in-app editing remain later work.

## Reproduction, source authority and limits

Use `--binary target/release/knowledgehub-model` after a release build for the
large fixture. Each run freezes the selected executable and its digest. Deliberate
negative fixtures use `--variant missing-requirement` or `--variant missing-ui`;
they replace one logical identity and its model references while preserving
record totals and the independent coverage expectations.

For ordinary evolution after import, build `--example model_successor` and run
`target/debug/examples/model_successor GENERATION-MANIFEST LOGICAL-KEY HOST.json OUTPUT.json`.
It publishes a coherent native successor and baseline B, then reopens and checks
baseline A, original bytes and generation receipts under current host authority.

Memento retains 33 exact supplied files, including five Markdown/PDF document
pairs, working UI HTML/CSS/JavaScript, image/prompt and auxiliary research. The
source-origin inventory binds copied bytes; no source application is changed or
required at runtime. Finder metadata has an explicit exclusion. PDF text/page
locators are actual Poppler extractions; originals remain independent. The static
illustration has an authored whole-image observation and lower authority than
the designated working UI. No UI or stored script executes in the native tool.

The fixture-specific build_plans.py owns application IDs, counts and authored
mapping choices. It produces plans/source maps/coverage/query inputs from exact
source spans. It is example data preparation, not a source-specific production
compiler. Its 5 contexts, 13 journeys, 36 US/R pairs, 72 scenario IDs, 26 DR,
11 FA, 30 AC, 6 templates, P1–P5 map and 5 working surfaces are source obligations.
Map cells explicitly retain partial-coverage language; selecting a placement is
not a claim that the whole story has been implemented in a cut.

Current fixture plans retain explicit gaps where detailed structured fields have
not been established. Their exact source sections and UI code remain retained.
This is an agent-authored interpretation awaiting human meaning review. Passing
native/source checks cannot establish completed business design, a running
Memento system, real research, provider actions, successful application TestRuns,
or human acceptance. Inspect each mapping and formalization gap, especially
script-defined controls and the authority of historical illustrations.

ui-source-review.json enumerates individual static/rendered controls, script
functions, event handlers, dispatcher aliases and observable messages/dialogs
with exact original byte locators. Details beyond the bounded typed bindings
remain inspectable artifacts. This is source accounting and agent review, not
a claim that every callback is formalized as an executable graph. The coverage
report separately lists linked context that was not supplied (release plans,
external-service instructions, spike manifests and similar material).

The authority manifest follows the product document's five-document reading
order and assigns distinct owners for product, experience, domain, engineering,
platform and visible UI. Historical comparisons, concept art, simulated research
and prototype regression sources do not establish implementation or acceptance.

The unrelated equipment-reservation fixture has different entities, fields,
states and IDs and a harmless throwing rule entrypoint retained as native Body
metadata. It exercises shared story placements, logical constructors, direct
lifecycle subjects and planned architecture through the same production schemas.

Regenerate authored fixture plans only deliberately:
`python3 examples/fixtures/build_plans.py all`. A changed original source digest
requires inventory and interpretation review; never silently reuse earlier run
evidence. Installation/discovery and actual agent-host evidence are described in
[the skills package](../../skills/INSTALL.md).

The separate [terminal teaching fixtures](terminal-client/README.md) contain a
small FruitTrucks interpretation and a portable equipment reference, with their
own neutral plans, coverage, exact source inventories and disposable A/B native
preparation. They do not refresh the root Hub or the larger reference runs.
