# model-text-search Specification

## Purpose

Find typed starting elements by deterministic literal phrases in selected text and exact document-owned narrative, with reproducible matching and bounded candidate work. Trace: KH-US-29; KH-DR-04/07/12/18/19/22; ontology §21.3/21.7; file-storage §7.7.

## Requirements

### Requirement: Explicit literal matching modes
Search SHALL support versioned byte-exact UTF-8 substring and ASCII-case-insensitive substring modes. Case-insensitive mode SHALL fold only ASCII A–Z. It SHALL preserve original text, require 3–4096 UTF-8 query bytes and reject unsupported modes/lengths explicitly without hidden semantic matching.

#### Scenario: Case-insensitive rent phrase
- **WHEN** a reader searches for “increase the rent” in the ASCII-insensitive mode
- **THEN** a selected field containing “INCREASE THE RENT” matches and the original case/source span is returned unchanged

#### Scenario: Unsupported short or semantic request
- **WHEN** the request uses a two-byte phrase or asks for an unsupported synonym/vector mode
- **THEN** it receives an explicit unsupported/invalid request result rather than an unbounded scan or guessed matches

### Requirement: Exact typed and narrative field scope
Search SHALL pin type, field projection and selection and include declared Text values and resolved Narrative fragments. It SHALL NOT silently search all artifact bytes, unselected history or arbitrary metadata. A match SHALL lie within one requested field/fragment and identify its exact record and source binding.

#### Scenario: Statement is document-owned
- **WHEN** the phrase exists in the requested narrative fragment rather than a graph Text value
- **THEN** search finds its owning selected record and returns the exact document/anchor and source byte range

#### Scenario: Phrase pieces lie in different fields
- **WHEN** separate requested fields contain only different pieces of the phrase
- **THEN** search does not invent a contiguous phrase match across their boundary

### Requirement: Candidate verification and bounded work
Candidate indexing SHALL be followed by exact final matching under the requested mode. The result SHALL distinguish verified matches, true exhaustion and reached candidate/result/byte limits. Rebuildable indexes SHALL bind their exact view, field projection and analyzer version and follow explicit readiness/failure rules.

#### Scenario: Candidate has matching grams but no phrase
- **WHEN** an indexed candidate contains phrase fragments in the wrong order
- **THEN** exact verification excludes it while counting its examination against the work budget

#### Scenario: Text index predates a narrative successor
- **WHEN** the requested view uses a newer source/anchor binding
- **THEN** search prepares a matching index or reports readiness explicitly and never labels the stale result complete

### Requirement: Stable permitted matching results
Search SHALL return all verified starts under the requested limits in stable qualified-reference/field/range order, with current disclosure checks and exact view identity. No match, ambiguity and truncated search SHALL be distinguishable. Relevance ranking SHALL NOT choose a start implicitly.

#### Scenario: Two stories share the phrase
- **WHEN** both selected stories match within complete search bounds
- **THEN** both exact references are available for grouped traversal or an explicit require-one ambiguity result

#### Scenario: Rebuild and reopen
- **WHEN** identical search runs against the same view and analyzer after index reconstruction
- **THEN** it returns identical verified matches and ordering under the same authority and budgets

