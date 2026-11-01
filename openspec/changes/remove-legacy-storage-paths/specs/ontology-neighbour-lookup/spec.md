## REMOVED Requirements

### Requirement: Existing navigation compatibility
**Reason**: Old assertion-only ordering/depth preservation and legacy backend capabilities are no longer supported.
**Migration**: Use current full-model selection, neighbour and generic traversal operations. Retain finite authority-bound paging and deterministic ordering without the old route.

## ADDED Requirements

### Requirement: Unified current navigation semantics
Maintained Query/Page selection and traversal SHALL use current full-model/indexed semantics. Enumeration, type/ID filtering and finite paging SHALL remain available. The old assertion-order/four-hop implementation and compatibility-only traversal dispatch SHALL NOT remain.

#### Scenario: Use current selection and traversal
- **WHEN** a caller enumerates typed records or follows recorded relationships through the full API
- **THEN** it receives deterministic exact-view bounded results without a separate legacy traversal engine


