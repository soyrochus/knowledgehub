use super::*;
use crate::{Error, ontology::*, storage::BranchStorage};
use std::collections::{BTreeMap, BTreeSet};
pub(crate) struct ViewInfo {
    pub ontology: Option<ExactRef>,
    pub profile: Option<ExactRef>,
    pub gaps: Vec<Gap>,
    pub gaps_omitted: bool,
}
struct Records<'a> {
    lease: &'a dyn BranchStorage,
    selection: &'a Selection,
    values: BTreeMap<ExactRef, Record>,
    bytes: usize,
    reads: u64,
}
impl Records<'_> {
    fn get(&mut self, r: &ExactRef) -> Result<Record, Error> {
        if let Some(v) = self.values.get(r) {
            return Ok(v.clone());
        }
        let v = self.lease.record(self.selection, r)?;
        self.reads += 1;
        let n = serde_json::to_vec(&v)
            .map_err(crate::codec::encoding)?
            .len();
        if self.bytes + n > 64 * 1024 * 1024 {
            return Err(Error::Capacity);
        }
        self.bytes += n;
        self.values.insert(r.clone(), v.clone());
        Ok(v)
    }
    fn is_type(&mut self, r: &ExactRef, wanted: &ExactRef, sub: bool) -> Result<bool, Error> {
        let mut todo = vec![r.clone()];
        let mut seen = BTreeSet::new();
        while let Some(r) = todo.pop() {
            if !seen.insert(r.clone()) {
                continue;
            }
            if seen.len() > 64 {
                return Err(Error::Capacity);
            }
            if &r == wanted {
                return Ok(true);
            }
            if !sub {
                return Ok(false);
            }
            let RecordData::Type(t) = self.get(&r)?.data else {
                return Err(Error::InvalidInput("target type hierarchy".into()));
            };
            todo.extend(t.parents);
        }
        Ok(false)
    }
}
#[derive(Clone)]
struct State {
    start: ExactRef,
    subject: ExactRef,
    position: usize,
    path: Vec<Neighbour>,
}
fn key(n: &Neighbour) -> (&ExactRef, &NeighbourSelector, &ExactRef, &NeighbourOrigin) {
    (&n.subject, &n.selector, &n.opposite, &n.origin)
}
pub(crate) fn evaluate(
    lease: &dyn BranchStorage,
    q: &ModelImpact,
    selection: Selection,
    prepare: bool,
) -> Result<ImpactPage, Error> {
    q.validate()?;
    let view = lease.model_view(&selection)?;
    let mut records = Records {
        lease,
        selection: &selection,
        values: BTreeMap::new(),
        bytes: 0,
        reads: 0,
    };
    let ontology = records.get(
        view.ontology
            .as_ref()
            .ok_or_else(|| Error::InvalidInput("pinned ontology required".into()))?,
    )?;
    let RecordData::Ontology(o) = &ontology.data else {
        return Err(Error::Integrity("ontology definition".into()));
    };
    let positions = match &q.plan {
        TraversalPlan::Repeated { alternatives } => vec![alternatives],
        TraversalPlan::Ordered { steps } => steps.iter().collect(),
    };
    for t in q.target.type_ref.iter().chain(
        positions
            .iter()
            .flat_map(|p| p.iter().flat_map(|s| s.opposite_type.iter())),
    ) {
        if !o.types.contains(t)
            || !matches!(records.get(t).map(|r| r.data), Ok(RecordData::Type(_)))
        {
            return Err(Error::InvalidInput(
                "unknown/external exact plan type".into(),
            ));
        }
    }
    for s in positions
        .iter()
        .flat_map(|p| p.iter())
        .flat_map(|s| s.selectors.iter())
    {
        let d = records.get(s.definition()).map_err(|e| {
            if e == Error::NotFound {
                Error::InvalidInput("unknown/external exact selector".into())
            } else {
                e
            }
        })?;
        if !o.types.contains(s.definition()) && !o.relationships.contains(s.definition())
            || !neighbours::facts::selector_supported(s, &d, Some(&ontology), &mut |r| {
                records.get(r)
            })?
        {
            return Err(Error::InvalidInput("selector outside pinned schema".into()));
        }
    }
    let mut page = ImpactPage {
        version: "knowledgehub-model-impact/1".into(),
        matching: match &q.starts {
            ImpactStarts::Text { selector } => Some(selector.mode),
            _ => None,
        },
        selection: selection.clone(),
        ontology: view.ontology,
        profile: view.profile,
        match_status: MatchStatus::Matched,
        starts: vec![],
        hits: vec![],
        dependencies: vec![],
        records: vec![],
        omitted_records: vec![],
        reached: BTreeSet::new(),
        exhausted: true,
        coverage: Coverage {
            meaning: "candidate_dependency".into(),
            domain_complete: false,
            no_witnessed_dependency: false,
            gaps: view.gaps,
            unresolved: vec![],
            dead_ends: vec![],
            omitted: view.gaps_omitted,
        },
        work: ReadWork::default(),
        continuation: None,
    };
    match &q.starts {
        ImpactStarts::Exact { references } => {
            for r in references.iter().collect::<BTreeSet<_>>() {
                if page.starts.len() == q.budgets.starts {
                    page.reached.insert(ReadLimit::Starts);
                    break;
                }
                match records.get(r) {
                    Ok(_) => page.starts.push(r.clone()),
                    Err(Error::NotFound) => page.coverage.unresolved.push(r.clone()),
                    Err(e) => return Err(e),
                }
            }
        }
        ImpactStarts::Text { selector } => {
            let result = lease.search_text(
                &SearchText {
                    selection: selection.clone(),
                    text: selector.clone(),
                    budgets: q.budgets.clone(),
                },
                prepare,
            )?;
            page.starts = result
                .hits
                .iter()
                .map(|h| h.record.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            page.hits = result.hits;
            page.reached = result.reached;
            page.work = result.work;
        }
    }
    if !page.coverage.unresolved.is_empty() || !page.reached.is_empty() {
        page.match_status = MatchStatus::Incomplete;
    } else if page.starts.is_empty() {
        page.match_status = MatchStatus::NoMatch;
    } else if q.match_policy == MatchPolicy::RequireOne && page.starts.len() != 1 {
        page.match_status = MatchStatus::Ambiguous;
    }
    let traverse = page.match_status == MatchStatus::Matched
        || q.match_policy == MatchPolicy::AllGrouped
            && page.match_status == MatchStatus::Incomplete;
    let mut frontier = if traverse {
        page.starts
            .iter()
            .map(|r| State {
                start: r.clone(),
                subject: r.clone(),
                position: 0,
                path: vec![],
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    };
    let mut visited: BTreeMap<(ExactRef, ExactRef, usize), (usize, usize)> = BTreeMap::new();
    let mut targets: BTreeMap<(ExactRef, ExactRef), Dependency> = BTreeMap::new();
    let mut retained_bytes = 0usize;
    let mut stopped = false;
    while !frontier.is_empty() && !stopped {
        frontier.sort_by(|a, b| {
            (&a.start, &a.subject, a.position).cmp(&(&b.start, &b.subject, b.position))
        });
        let mut active = Vec::new();
        for f in frontier {
            let state_key = (f.start.clone(), f.subject.clone(), f.position);
            let depth = f.path.len();
            if let Some((previous, n)) = visited.get_mut(&state_key) {
                if depth > *previous || *n >= q.budgets.witnesses {
                    page.reached.insert(ReadLimit::Witnesses);
                    continue;
                }
                *n += 1;
            } else {
                if visited.len() >= q.budgets.states {
                    page.reached.insert(ReadLimit::States);
                    stopped = true;
                    break;
                }
                visited.insert(state_key, (depth, 1));
                page.work.visited_states += 1;
            }
            let r = records.get(&f.subject)?;
            let is_target = (q.target.ids.is_empty() || q.target.ids.contains(&r.reference))
                && match &q.target.type_ref {
                    Some(t) => records.is_type(&r.type_ref, t, q.target.include_subtypes)?,
                    None => true,
                };
            if is_target && !f.path.is_empty() {
                let k = (f.start.clone(), f.subject.clone());
                if !targets.contains_key(&k) && targets.len() >= q.budgets.targets {
                    page.reached.insert(ReadLimit::Targets);
                    stopped = true;
                    break;
                }
                let d = targets.entry(k).or_insert_with(|| Dependency {
                    start: f.start.clone(),
                    target: f.subject.clone(),
                    witnesses: vec![],
                    omitted: false,
                });
                if !d.witnesses.contains(&f.path)
                    && d.witnesses.first().is_some_and(|p| p.len() < f.path.len())
                {
                    page.reached.insert(ReadLimit::Witnesses);
                } else if d.witnesses.len() < q.budgets.witnesses && !d.witnesses.contains(&f.path)
                {
                    retained_bytes += serde_json::to_vec(&f.path)
                        .map_err(crate::codec::encoding)?
                        .len();
                    d.witnesses.push(f.path.clone());
                } else if !d.witnesses.contains(&f.path) {
                    page.reached.insert(ReadLimit::Witnesses);
                }
            }
            if retained_bytes > 8 * 1024 * 1024 {
                page.reached.insert(ReadLimit::Bytes);
                stopped = true;
                break;
            }
            if matches!(&q.plan,TraversalPlan::Ordered{steps} if f.position==steps.len()) {
                continue;
            }
            if depth >= q.budgets.hops {
                page.reached.insert(ReadLimit::Hops);
                continue;
            }
            active.push(f);
        }
        let mut next = Vec::new();
        let mut frontier_bytes = 0;
        for position in active.iter().map(|f| f.position).collect::<BTreeSet<_>>() {
            let group = active
                .iter()
                .filter(|f| f.position == position)
                .collect::<Vec<_>>();
            let subjects = group
                .iter()
                .map(|f| f.subject.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let mut edges: BTreeMap<ExactRef, Vec<Neighbour>> = BTreeMap::new();
            for step in positions[position].iter() {
                for chunk in subjects.chunks(100) {
                    let remaining = q.budgets.examined.saturating_sub(
                        (page.work.examined_associations
                            + page.work.text_postings_examined
                            + page.work.expansion_attempts) as usize,
                    );
                    if remaining == 0 {
                        page.reached.insert(ReadLimit::Examined);
                        stopped = true;
                        break;
                    }
                    let mut n = Neighbours::new(selection.clone(), chunk.to_vec());
                    n.direction = step.direction;
                    n.selectors = step.selectors.clone();
                    n.categories = step.categories.clone();
                    n.opposite_type = step.opposite_type.clone();
                    n.include_subtypes = step.include_subtypes;
                    n.fact_scope = q.fact_scope;
                    n.preparation = q.budgets.preparation.clone();
                    n.page_size = 1000;
                    n.examined_limit = remaining;
                    n.byte_limit = 1_048_576;
                    let mut pos = neighbours::Position::default();
                    loop {
                        let s = match lease.neighbours(&n, &pos) {
                            Ok(s) => s,
                            Err(Error::IndexNotReady { .. }) if prepare => {
                                let w = lease.prepare_neighbours(&n, true)?;
                                page.work.prepared_owners += w.prepared_owners;
                                lease.neighbours(&n, &pos)?
                            }
                            Err(e) => return Err(e),
                        };
                        page.work.batch_lookups += 1;
                        page.work.examined_associations += s.page.work.examined_postings;
                        page.work.index_objects_read += s.page.work.index_objects_read;
                        page.work.index_bytes_read += s.page.work.index_bytes_read;
                        page.work.record_reads += s.page.work.record_reads;
                        page.work.hydrated_bytes += s.page.work.record_bytes;
                        for v in s.page.vertices {
                            if !v.available {
                                page.coverage.unresolved.push(v.reference);
                            }
                        }
                        for edge in s.page.neighbours {
                            edges.entry(edge.subject.clone()).or_default().push(edge);
                        }
                        if s.page.reached == Some(NeighbourLimit::ExaminedPostings) {
                            page.reached.insert(ReadLimit::Examined);
                            stopped = true;
                            break;
                        }
                        if let Some(p) = s.next {
                            if p == pos {
                                page.reached.insert(ReadLimit::Bytes);
                                stopped = true;
                                break;
                            }
                            pos = p;
                            n.examined_limit = q.budgets.examined.saturating_sub(
                                (page.work.examined_associations
                                    + page.work.text_postings_examined
                                    + page.work.expansion_attempts)
                                    as usize,
                            );
                            if n.examined_limit == 0 {
                                page.reached.insert(ReadLimit::Examined);
                                stopped = true;
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    if stopped {
                        break;
                    }
                }
                if stopped {
                    break;
                }
            }
            for list in edges.values_mut() {
                list.sort_by(|a, b| key(a).cmp(&key(b)));
                list.dedup();
            }
            for f in group {
                let es = edges.get(&f.subject).map(Vec::as_slice).unwrap_or(&[]);
                if es.is_empty() {
                    page.coverage.dead_ends.push(f.subject.clone());
                }
                for edge in es {
                    if page.work.expansion_attempts
                        + page.work.examined_associations
                        + page.work.text_postings_examined
                        >= q.budgets.examined as u64
                    {
                        page.reached.insert(ReadLimit::Examined);
                        stopped = true;
                        break;
                    }
                    page.work.expansion_attempts += 1;
                    if next.len() >= q.budgets.states * q.budgets.witnesses {
                        page.reached.insert(ReadLimit::States);
                        stopped = true;
                        break;
                    }
                    if page.coverage.unresolved.contains(&edge.opposite) {
                        continue;
                    }
                    let mut path = f.path.clone();
                    path.push(edge.clone());
                    frontier_bytes += serde_json::to_vec(&path)
                        .map_err(crate::codec::encoding)?
                        .len();
                    if frontier_bytes > 8 * 1024 * 1024 {
                        page.reached.insert(ReadLimit::Bytes);
                        stopped = true;
                        break;
                    }
                    let position = if matches!(&q.plan, TraversalPlan::Repeated { .. }) {
                        0
                    } else {
                        position + 1
                    };
                    next.push(State {
                        start: f.start.clone(),
                        subject: edge.opposite.clone(),
                        position,
                        path,
                    });
                }
            }
            if stopped {
                break;
            }
        }
        frontier = next;
    }
    page.dependencies = targets.into_values().collect();
    let mut wanted = BTreeSet::new();
    for d in &page.dependencies {
        if q.projection.target_records {
            wanted.insert(d.target.clone());
        }
        if q.projection.witness_records {
            wanted.extend(d.witnesses.iter().flatten().map(|n| n.origin.owner.clone()));
        }
    }
    let mut bytes = 0;
    for r in wanted {
        let record = records.get(&r)?;
        let n = serde_json::to_vec(&record)
            .map_err(crate::codec::encoding)?
            .len();
        if bytes + n > 8 * 1024 * 1024 || n > q.budgets.bytes / 2 {
            page.omitted_records.push(r);
        } else {
            bytes += n;
            page.records.push(record);
        }
    }
    if !q.projection.snippets {
        for h in &mut page.hits {
            h.snippet = None;
        }
    }
    page.coverage.dead_ends.sort();
    page.coverage.dead_ends.dedup();
    page.coverage.unresolved.sort();
    page.coverage.unresolved.dedup();
    page.coverage.no_witnessed_dependency = page.dependencies.is_empty() && traverse;
    page.work.record_reads += records.reads;
    page.work.hydrated_bytes += records.bytes as u64;
    page.exhausted = !page.reached.iter().any(|r| *r != ReadLimit::Witnesses);
    Ok(page)
}
