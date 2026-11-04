# Route-change information

> Authored FT-P01 logical schema projections, not database tables or a validated native KnowledgeHub model.

## Route

| Field | Meaning |
| --- | --- |
| id | Stable route identity |
| driver_id / vehicle_id | Unchanged assignment in this slice |
| plan_revision | Effective operational plan revision |
| stop_ids | Ordered stops, with served progress retained |
| progress_revision | Relevant execution-state guard |

## Change offer

Contains id, route_id, expected_plan_revision, candidate_stop_ids, expires_at and status. An exact offer revision also captures the relevant assignment and progress. Released content cannot be silently edited.

## Acknowledgement

Contains id, offer_id, offer_revision, driver_id, responded_at and response. The offer reference and revision identify the exact instruction; response distinguishes acknowledgement from decline.

The three cards are concise projections, not the full record set. Operational revisions are distinct from revisions of these design definitions in Loom.