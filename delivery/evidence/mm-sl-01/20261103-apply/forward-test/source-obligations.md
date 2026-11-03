# Independent raw-source obligations

Read before plan construction: design.md (six statements) and rule.js (two retained lines). No maintained fixture plan or coverage was read.

1. A human borrower reserves Asset and later inspects Booking; the two tasks remain one end-to-end journey.
2. Booking retains exactly one logical Asset reference, Requested/Confirmed/Cancelled states, and the original reason. Reference format and reason value type are not supplied.
3. Notification adapter remains planned and has no implementation artifact. Source rule.js is not that artifact.
4. Cancellation and a two-hour waiting policy are descriptions only. No timer, transition implementation or rule entrypoint may execute.
5. One shared story has two distinct task placements. Cuts are cumulative; they do not clone the story identity. Names and allocation to reserve/inspect cuts are unreviewed interpretations.
6. Reference-format and implementation choices remain unresolved, with no invented decision owner or resolution.
7. Retain the exact JavaScript sentinel bytes and bookingPolicy entrypoint; never invoke it.

Expected source gaps: no UI, concrete identity scheme, starting/terminal lifecycle rules, nullability policy, schema compatibility, adapter port, implementation evidence, accounts, provider actions, application execution, or human acceptance is supplied.

Queries must exhaust typed EntityType, State, UserStory, StoryPlacement, OutcomeCut, Adapter, BusinessRule, OpenQuestion, and RelationshipAssertion results at page size one; property neighbours must independently recover Booking's direct lifecycle, the shared story's two placements, and the logical Asset reference target.
