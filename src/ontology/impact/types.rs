use crate::{Error, codec, ontology::*};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiteralMode {
    #[serde(rename = "literal-utf8/1")]
    Utf8,
    #[serde(rename = "literal-ascii-fold/1")]
    AsciiFold,
}
impl LiteralMode {
    pub(crate) fn bytes(self, s: &str) -> Vec<u8> {
        s.bytes()
            .map(|b| {
                if self == Self::AsciiFold {
                    b.to_ascii_lowercase()
                } else {
                    b
                }
            })
            .collect()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextField {
    pub type_ref: ExactRef,
    pub path: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextSelector {
    pub phrase: String,
    pub mode: LiteralMode,
    pub fields: BTreeSet<TextField>,
}
impl TextSelector {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if !(3..=4096).contains(&self.phrase.len())
            || self.fields.is_empty()
            || self.fields.len() > 64
            || self
                .fields
                .iter()
                .any(|f| !f.type_ref.is_valid() || !f.path.starts_with('/') || f.path.len() > 4096)
        {
            return Err(Error::InvalidInput("literal search needs 3–4096 UTF-8 bytes and 1–64 exact Text/Narrative field selectors".into()));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadBudgets {
    pub hops: usize,
    pub starts: usize,
    pub states: usize,
    pub examined: usize,
    pub candidates: usize,
    pub targets: usize,
    pub witnesses: usize,
    pub page_size: usize,
    pub bytes: usize,
    pub preparation: NeighbourPreparation,
}
impl Default for ReadBudgets {
    fn default() -> Self {
        Self {
            hops: 8,
            starts: 100,
            states: 2000,
            examined: 5000,
            candidates: 2000,
            targets: 1000,
            witnesses: 1,
            page_size: 100,
            bytes: 256 * 1024,
            preparation: NeighbourPreparation::default(),
        }
    }
}
impl ReadBudgets {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if [
            (self.hops, 32),
            (self.starts, 1000),
            (self.states, 10000),
            (self.examined, 10000),
            (self.candidates, 10000),
            (self.targets, 10000),
            (self.witnesses, 8),
            (self.page_size, 1000),
            (self.bytes, 1_048_576),
        ]
        .iter()
        .any(|(n, m)| *n == 0 || n > m)
            || self.preparation.owner_limit > 100000
            || self.preparation.fact_limit > 100000
            || self.preparation.byte_limit > 64 * 1024 * 1024
        {
            return Err(Error::InvalidInput(
                "read budgets exceed finite supported bounds".into(),
            ));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchText {
    pub selection: Selection,
    pub text: TextSelector,
    pub budgets: ReadBudgets,
}
impl SearchText {
    pub fn from_json(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() > 1_048_576 {
            return Err(Error::Capacity);
        }
        let q: Self = codec::decode(bytes)?;
        q.text.validate()?;
        q.budgets.validate()?;
        Ok(q)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceSpan {
    pub document: ExactRef,
    pub anchor: ExactRef,
    pub span_document: ExactRef,
    pub content: crate::ContentRef,
    pub start: u64,
    pub end: u64,
    pub original_location: Option<SourceLocation>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextHit {
    pub record: ExactRef,
    pub field: String,
    pub start: u64,
    pub end: u64,
    pub source: Option<SourceSpan>,
    pub snippet: Option<String>,
    pub omitted: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ReadLimit {
    Starts,
    Candidates,
    Hops,
    States,
    Examined,
    Targets,
    Witnesses,
    Bytes,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadWork {
    pub text_postings_examined: u64,
    pub expansion_attempts: u64,
    pub batch_lookups: u64,
    pub examined_associations: u64,
    pub candidates_verified: u64,
    pub visited_states: u64,
    pub record_reads: u64,
    pub hydrated_bytes: u64,
    pub index_objects_read: u64,
    pub index_bytes_read: u64,
    pub prepared_owners: u64,
    pub prepared_fields: u64,
    pub prepared_bytes: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchResult {
    pub selection: Selection,
    pub mode: LiteralMode,
    pub hits: Vec<TextHit>,
    pub reached: BTreeSet<ReadLimit>,
    pub exhausted: bool,
    pub work: ReadWork,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraversalStep {
    pub direction: NeighbourDirection,
    pub selectors: BTreeSet<NeighbourSelector>,
    pub categories: BTreeSet<NeighbourCategory>,
    pub opposite_type: Option<ExactRef>,
    pub include_subtypes: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum TraversalPlan {
    Repeated { alternatives: Vec<TraversalStep> },
    Ordered { steps: Vec<Vec<TraversalStep>> },
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetPredicate {
    pub type_ref: Option<ExactRef>,
    pub ids: BTreeSet<ExactRef>,
    pub include_subtypes: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchPolicy {
    AllGrouped,
    RequireOne,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum ImpactStarts {
    Exact { references: Vec<ExactRef> },
    Text { selector: TextSelector },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImpactProjection {
    pub target_records: bool,
    pub witness_records: bool,
    pub snippets: bool,
}
impl Default for ImpactProjection {
    fn default() -> Self {
        Self {
            target_records: false,
            witness_records: false,
            snippets: true,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelImpact {
    /// None resolves head once. Continuations retain that exact view.
    pub selection: Option<Selection>,
    pub starts: ImpactStarts,
    pub match_policy: MatchPolicy,
    pub plan: TraversalPlan,
    pub target: TargetPredicate,
    pub fact_scope: FactScope,
    pub projection: ImpactProjection,
    pub budgets: ReadBudgets,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Traverse {
    pub selection: Selection,
    pub starts: Vec<ExactRef>,
    pub plan: TraversalPlan,
    pub target: TargetPredicate,
    pub fact_scope: FactScope,
    pub projection: ImpactProjection,
    pub budgets: ReadBudgets,
}
impl Traverse {
    pub fn from_json(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() > 1_048_576 {
            return Err(Error::Capacity);
        }
        let q: Self = codec::decode(bytes)?;
        q.composite().validate()?;
        Ok(q)
    }
    pub(crate) fn composite(&self) -> ModelImpact {
        ModelImpact {
            selection: Some(self.selection.clone()),
            starts: ImpactStarts::Exact {
                references: self.starts.clone(),
            },
            match_policy: MatchPolicy::AllGrouped,
            plan: self.plan.clone(),
            target: self.target.clone(),
            fact_scope: self.fact_scope,
            projection: self.projection.clone(),
            budgets: self.budgets.clone(),
        }
    }
}
impl ModelImpact {
    pub fn from_json(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() > 1_048_576 {
            return Err(Error::Capacity);
        }
        let q: Self = codec::decode(bytes)?;
        q.validate()?;
        Ok(q)
    }
    pub(crate) fn validate(&self) -> Result<(), Error> {
        self.budgets.validate()?;
        match &self.starts {
            ImpactStarts::Exact { references }
                if references.is_empty() || references.len() > 1000 =>
            {
                return Err(Error::InvalidInput("1–1000 exact starts required".into()));
            }
            ImpactStarts::Text { selector } => selector.validate()?,
            _ => {}
        }
        let positions: Vec<&Vec<TraversalStep>> = match &self.plan {
            TraversalPlan::Repeated { alternatives } => vec![alternatives],
            TraversalPlan::Ordered { steps } => steps.iter().collect(),
        };
        if positions.is_empty()
            || positions.len() > 32
            || positions.iter().any(|p| p.is_empty())
            || positions.iter().map(|p| p.len()).sum::<usize>() > 64
        {
            return Err(Error::InvalidInput(
                "plan needs 1–32 positions and at most 64 alternatives".into(),
            ));
        }
        let mut refs = Vec::new();
        if let ImpactStarts::Exact { references } = &self.starts {
            refs.extend(references.iter());
        }
        refs.extend(self.target.type_ref.iter());
        refs.extend(self.target.ids.iter());
        if self.target.type_ref.is_none() && self.target.ids.is_empty()
            || self.target.include_subtypes && self.target.type_ref.is_none()
            || self.target.ids.len() > 1000
        {
            return Err(Error::InvalidInput(
                "exact target type or IDs required".into(),
            ));
        }
        for s in positions.into_iter().flatten() {
            if s.selectors.is_empty()
                || s.selectors.len() > 64
                || s.categories.is_empty()
                || s.include_subtypes && s.opposite_type.is_none()
                || s.selectors.iter().any(|x| !x.valid())
            {
                return Err(Error::InvalidInput(
                    "step needs finite exact selectors, categories and valid type filter".into(),
                ));
            }
            refs.extend(s.selectors.iter().map(NeighbourSelector::definition));
            refs.extend(s.opposite_type.iter());
        }
        if refs.iter().any(|r| {
            !r.is_valid()
                || [&r.model, &r.id, &r.revision]
                    .iter()
                    .any(|s| s.len() > 4096)
        }) || serde_json::to_vec(self).map_err(codec::encoding)?.len() > 1_048_576
        {
            return Err(Error::InvalidInput(
                "invalid or oversized exact plan references".into(),
            ));
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchStatus {
    Matched,
    NoMatch,
    Ambiguous,
    Incomplete,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dependency {
    pub start: ExactRef,
    pub target: ExactRef,
    pub witnesses: Vec<Vec<Neighbour>>,
    pub omitted: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Coverage {
    /// Always candidate_dependency; a recorded path never determines migration necessity.
    pub meaning: String,
    pub domain_complete: bool,
    pub no_witnessed_dependency: bool,
    pub gaps: Vec<Gap>,
    pub unresolved: Vec<ExactRef>,
    pub dead_ends: Vec<ExactRef>,
    pub omitted: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImpactPage {
    pub version: String,
    pub matching: Option<LiteralMode>,
    pub selection: Selection,
    pub ontology: Option<ExactRef>,
    pub profile: Option<ExactRef>,
    pub match_status: MatchStatus,
    pub starts: Vec<ExactRef>,
    pub hits: Vec<TextHit>,
    pub dependencies: Vec<Dependency>,
    pub records: Vec<Record>,
    pub omitted_records: Vec<ExactRef>,
    pub reached: BTreeSet<ReadLimit>,
    pub exhausted: bool,
    pub coverage: Coverage,
    pub work: ReadWork,
    pub continuation: Option<String>,
}
