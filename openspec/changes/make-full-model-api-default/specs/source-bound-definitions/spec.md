## REMOVED Requirements

### Requirement: Explicit model scope and bootstrap profile
**Reason**: The dedicated seed bootstrap is superseded by the sole full-model scope and governed vocabulary.
**Migration**: Use the full KnowledgeHub API with explicit current model scope and registered ontology/profile definitions; recreate demonstrations instead of converting seed stores.

### Requirement: Typed seed definitions and relationship identity
**Reason**: Hardcoded seed types and endpoint obligations are superseded by full registered schemas.
**Migration**: Express examples as ordinary full-model records; ontology-conformance and full-model-public-api govern exact typing and relationships.

