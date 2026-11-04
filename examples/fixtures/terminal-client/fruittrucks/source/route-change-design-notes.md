# Route-change design notes

> Authored summary of proposed FT-P01 design, not an actual workshop or approved client policy.

## Establish a changed stop order

1. Understand the disruption and effective route.
2. Prepare and check another order of the same unserved stops.
3. Release one exact offer.
4. The assigned driver acknowledges or declines.
5. Before activation, recheck expiry, plan, assignment and relevant progress.
6. Track communication about the established result.

## Offer lifecycle subset

Draft → Awaiting acknowledgement → Activated. Declined and Expired are alternative terminal outcomes. A draft can be revised; changed released content requires a fresh offer and acknowledgement. Withdrawal and supersession are left to the full reference case.

Exact-offer binding, stale-state refusal, retry recovery and an illustrative **five-minute expiry from release** are proposed design choices. The production expiry policy remains open; it is separate from Q1.

## Retained history

Retain both plans, exact offer, actor, reason, response and established result. A lost activation response requires resolving the original operation. Communication failure does not undo activation.

## Illustrative reference fixture

FT-ROUTE-EX01: Mercado Centro remains served. Move Frutería Plaza before Supermercado Norte; keep Supermercado Parque last. This is invented fixture data.

## Open questions

Authority thresholds, production expiry, emergency/manual acknowledgement, notification recipients/channels and retention remain client decisions. The previous effective plan does not itself resolve a physical disruption.