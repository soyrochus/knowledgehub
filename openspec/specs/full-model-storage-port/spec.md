# full-model-storage-port Specification

## Purpose

Publish and read full typed knowledge through one mandatory storage port without seed envelopes or old-backend capability fallbacks. Trace KH-SL-015, KH-DR-08–12/15/17/20 and ontology §21.2/21.4–21.7.

## Requirements

### Requirement: Direct full-model storage contract
The storage port SHALL carry actual full-model scope/state, guarded publication and typed receipts directly. It SHALL NOT require a seed snapshot envelope, synthetic profile transport or dual affected-reference representations.

#### Scenario: Publish the full state
- **WHEN** a valid current change reaches storage admission
- **THEN** the port receives one mandatory typed state and its typed original receipt under exact guards

### Requirement: Complete current backend operations
The current adapter SHALL implement all required exact, range, operation-outcome, neighbour and deterministic read operations. Legacy-only unsupported defaults and full-history fallbacks SHALL be removed rather than used for current selective reads.

#### Scenario: Read a fixed neighbourhood
- **WHEN** unrelated current history and facts grow while the view is prepared
- **THEN** the operation follows current required paths and postings without an old-backend scan fallback

### Requirement: No superseded persistence implementation
The maintained library SHALL contain no files/1–3 readers/writers, compatibility feature switches, deprecated constructors or conversion adapters. Historical fixtures MAY identify rejected formats without implementing their schemas.

#### Scenario: Build the supported library
- **WHEN** the crate is built with its declared current features
- **THEN** only the current persistence implementation is available
