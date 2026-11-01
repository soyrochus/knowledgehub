## REMOVED Requirements

### Requirement: Existing canonical data remains immutable
**Reason**: That requirement bound the earlier KH-SL-013 verification to a then-postponed rebuild. The user has separately authorised a fresh current instance.
**Migration**: Preserve accepted knowledge during ordinary cache work; regenerate the demonstration only through its explicit current-only lifecycle.

## ADDED Requirements

### Requirement: Derived maintenance preserves accepted knowledge
Index preparation, reconstruction and physical cache maintenance SHALL leave the current instance's accepted objects, history, baselines and receipts unchanged. A separately authorised host demonstration rebuild SHALL establish a new instance and SHALL NOT be presented as an ordinary cache update.

#### Scenario: Rebuild a current derived index
- **WHEN** a cache is evicted or corrupted and then prepared against an existing exact view
- **THEN** verified semantic query results return without accepted-data mutation or another model generation


