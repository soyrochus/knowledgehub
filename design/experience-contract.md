# Experience contract

Current refinement, authorised by the apply request of 3 November 2026. Repository
definitions remain authoritative; source draft 0.2 retains its original status.

StoryMap.journeys and UserActivity.journeys are nonempty canonical sets of exact
Journey revisions. Each activity's scope is contained in its selecting map's
scope. Ordered map activities and activity tasks have no duplicate members;
UserTask.activity is the single task owner, with reciprocal membership.

StoryPlacement is an identified Experience definition with map, task, story and
nonnegative integer order. Its map selects it, its task belongs to a selected
activity, and order is unique within each map/task. Multiple placements may
select one story. OutcomeCut.placements is a nonempty canonical set of placements
from its map. Prerequisite cuts belong to the same map and are acyclic. Cuts may
overlap; story membership is derived from placements.

Scenario illustrates Requirement is a qualified signature without qualifiers.
UserStory dependsOn UserStory has dependency_kind required/conditional and a
required nullable condition Narrative. Required dependencies have null condition;
conditional dependencies have a resolving nonblank condition. Self-dependencies
and cycles of required edges are invalid. Conditions are never executed.

These refinements implement the general identity, experience, provenance and
integrity principles in source/system-definition-ontology-proposal.md §§5–8,
14 and 17. StoryPlacement and both signatures are additions, not entries
attributed to the original source vocabulary. Concrete application names and
business identifiers are model data. No TestRun or human acceptance is implied.

C1 allocated catalogue-2; the cumulative C1/C2 contract uses catalogue-3 and
system-definition-ontology/3. Old fields are
rejected; no reader, conversion or migration is supplied. Accepted stores are
not rebuilt by updating repository definitions.

Run `cargo test --offline --test experience_contract` for the creation, bounded
read, baseline, successor, reopen and original-receipt walkthrough. The equipment
reservation specimen in that test uses these same schemas and assessment path.
