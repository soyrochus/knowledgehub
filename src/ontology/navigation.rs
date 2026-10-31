use super::*;
use crate::{Action, Context, Error, codec};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::Mutex,
    time::{Duration, Instant},
};

pub(crate) struct Cursors {
    pub(crate) entries: Mutex<BTreeMap<String, Cursor>>,
    origin: Instant,
    #[cfg(test)]
    pub(crate) offset: std::sync::atomic::AtomicU64,
}
pub(crate) struct Cursor {
    pub(crate) actor: String,
    pub(crate) scope: ModelScope,
    pub(crate) fingerprint: String,
    pub(crate) query: Query,
    pub(crate) items: Vec<Record>,
    pub(crate) index: usize,
    pub(crate) truncated: Option<String>,
    pub(crate) touched: Duration,
    pub(crate) neighbour: Option<super::neighbours::NeighbourCursor>,
}
impl Cursors {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(BTreeMap::new()),
            origin: Instant::now(),
            #[cfg(test)]
            offset: std::sync::atomic::AtomicU64::new(0),
        }
    }
    pub(crate) fn now(&self) -> Duration {
        let n = self.origin.elapsed();
        #[cfg(test)]
        {
            n + Duration::from_secs(self.offset.load(std::sync::atomic::Ordering::SeqCst))
        }
        #[cfg(not(test))]
        {
            n
        }
    }
    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }
    pub(crate) fn expire(&self, map: &mut BTreeMap<String, Cursor>, now: Duration) {
        map.retain(|_, c| now.saturating_sub(c.touched) < Duration::from_secs(1800))
    }
}
fn fingerprint(q: &Query, scope: &ModelScope, actor: &str) -> Result<String, Error> {
    codec::hash_value("knowledgehub-query/1", &(scope, actor, q))
}
fn page(c: &mut Cursor, id: Option<String>) -> Page {
    let end = (c.index + c.query.page_size).min(c.items.len());
    let values = c.items[c.index..end].to_vec();
    c.index = end;
    let (records, assertions) = values
        .into_iter()
        .partition(|r| !matches!(r.data, RecordData::Assertion(_)));
    let more = end < c.items.len();
    Page {
        selection: c.query.selection.clone(),
        records,
        assertions,
        continuation: if more { id } else { None },
        truncated: if more { None } else { c.truncated.clone() },
        exhausted: !more && c.truncated.is_none(),
    }
}
fn materialise(state: &State, q: &Query) -> Result<(Vec<Record>, Option<String>), Error> {
    query_bounds(q)?;
    if let Some(t) = &q.type_ref {
        if state.type_definition(t).is_none() {
            return Err(Error::InvalidInput("unknown exact query type".into()));
        }
    }
    for s in &q.signatures {
        if !state
            .get(s)
            .is_some_and(|r| matches!(r.data, RecordData::Relationship(_)))
        {
            return Err(Error::InvalidInput(
                "unknown exact relationship signature".into(),
            ));
        }
    }
    scan(state, q)
}
fn query_bounds(q: &Query) -> Result<(), Error> {
    if q.page_size == 0
        || q.page_size > 1000
        || q.depth > 4
        || q.subject_limit == 0
        || q.subject_limit > 10000
        || q.edge_limit == 0
        || q.edge_limit > 10000
        || q.starts.len() > 10000
    {
        return Err(Error::InvalidInput(
            "query exceeds finite page/depth/subject/edge bounds".into(),
        ));
    }
    Ok(())
}
fn scan(state: &State, q: &Query) -> Result<(Vec<Record>, Option<String>), Error> {
    let matches = |r: &Record| {
        (q.type_ref.as_ref().is_none_or(|t| r.type_ref == *t))
            && (q.ids.is_empty() || q.ids.contains(&r.reference.id))
    };
    if q.starts.is_empty() {
        if !q.signatures.is_empty() {
            return Err(Error::InvalidInput(
                "relationship traversal needs starting subjects".into(),
            ));
        }
        let mut all = state.selected_records().collect::<Vec<_>>();
        all.sort_by(|a, b| a.reference.cmp(&b.reference));
        let truncated = (all.len() > q.subject_limit).then(|| "subject budget".into());
        let items = all
            .into_iter()
            .take(q.subject_limit)
            .filter(|r| matches(r))
            .cloned()
            .collect();
        return Ok((items, truncated));
    }
    let mut starts = q.starts.clone();
    starts.sort();
    starts.dedup();
    let mut queue = VecDeque::new();
    let mut visited = BTreeSet::new();
    let mut seen_edges = BTreeSet::new();
    let mut items = vec![];
    let mut truncated = None;
    for r in starts {
        let node = state.resolve(&r).ok_or(Error::NotFound)?;
        if visited.len() == q.subject_limit {
            truncated = Some("subject budget".into());
            break;
        }
        visited.insert(r.clone());
        queue.push_back((r, 0));
        if matches(&node) {
            items.push(node)
        }
    }
    let mut edges = state
        .selected_records()
        .filter(|r| matches!(r.data, RecordData::Assertion(_)))
        .collect::<Vec<_>>();
    edges.sort_by(|a, b| {
        let sig = |r: &Record| {
            if let RecordData::Assertion(a) = &r.data {
                a.signature.clone()
            } else {
                unreachable!()
            }
        };
        sig(a)
            .cmp(&sig(b))
            .then_with(|| a.reference.cmp(&b.reference))
    });
    while let Some((subject, depth)) = queue.pop_front() {
        for edge in &edges {
            let RecordData::Assertion(a) = &edge.data else {
                unreachable!()
            };
            if !q.signatures.is_empty() && !q.signatures.contains(&a.signature) {
                continue;
            }
            let target = match q.direction {
                Direction::Forward if a.from == subject => &a.to,
                Direction::Reverse if a.to == subject => &a.from,
                _ => continue,
            };
            if seen_edges.contains(&edge.reference) {
                continue;
            }
            if depth == q.depth {
                if truncated.is_none() {
                    truncated = Some("depth bound".into())
                }
                continue;
            }
            if seen_edges.len() == q.edge_limit {
                truncated = Some("edge budget".into());
                return Ok((items, truncated));
            }
            if !visited.contains(target) && visited.len() == q.subject_limit {
                truncated = Some("subject budget".into());
                return Ok((items, truncated));
            }
            seen_edges.insert(edge.reference.clone());
            items.push((*edge).clone());
            if visited.insert(target.clone()) {
                let node = state.resolve(target).ok_or(Error::NotFound)?;
                queue.push_back((target.clone(), depth + 1));
                if matches(&node) {
                    items.push(node)
                }
            }
        }
    }
    Ok((items, truncated))
}
fn indexed(
    h: &OntologyHub,
    context: &Context,
    q: &Query,
) -> Result<(Vec<Record>, Option<String>), Error> {
    query_bounds(q)?;
    {
        let lease = h.store.acquire_read(&h.scope.transport())?;
        if let Some(t) = &q.type_ref {
            match lease.record(&q.selection, t) {
                Ok(r) if matches!(r.data, RecordData::Type(_)) => {}
                Ok(_) | Err(Error::NotFound) => {
                    return Err(Error::InvalidInput("unknown exact query type".into()));
                }
                Err(e) => return Err(e),
            }
        }
        for s in &q.signatures {
            match lease.record(&q.selection, s) {
                Ok(r) if matches!(r.data, RecordData::Relationship(_)) => {}
                Ok(_) | Err(Error::NotFound) => {
                    return Err(Error::InvalidInput(
                        "unknown exact relationship signature".into(),
                    ));
                }
                Err(e) => return Err(e),
            }
        }
    }
    let mut starts = q.starts.clone();
    starts.sort();
    starts.dedup();
    let request = |r: ExactRef| {
        let mut n = Neighbours::new(q.selection.clone(), vec![r]);
        n.direction = match q.direction {
            Direction::Forward => NeighbourDirection::Outgoing,
            Direction::Reverse => NeighbourDirection::Incoming,
        };
        n.categories = BTreeSet::from([NeighbourCategory::Association]);
        n.selectors = q
            .signatures
            .iter()
            .map(|r| NeighbourSelector::Assertion {
                definition: r.clone(),
            })
            .collect();
        n.page_size = 1000;
        n.byte_limit = 1_048_576;
        n
    };
    // Prepare before holding the traversal's shared lease, then pin all remaining reads against maintenance.
    let first_request = request(starts[0].clone());
    super::neighbours::read_slice(
        h,
        context,
        &first_request,
        &super::neighbours::Position::default(),
    )?;
    let lease = h.store.acquire_read(&h.scope.transport())?;
    let matches = |r: &Record| {
        q.type_ref.as_ref().is_none_or(|t| r.type_ref == *t)
            && (q.ids.is_empty() || q.ids.contains(&r.reference.id))
    };
    let mut queue = VecDeque::new();
    let mut visited = BTreeSet::new();
    let mut seen_edges = BTreeSet::new();
    let mut items = Vec::new();
    let mut truncated = None;
    for r in starts {
        let node = lease.record(&q.selection, &r)?;
        if visited.len() == q.subject_limit {
            truncated = Some("subject budget".into());
            break;
        }
        visited.insert(r.clone());
        queue.push_back((r, 0));
        if matches(&node) {
            items.push(node);
        }
    }
    while let Some((subject, depth)) = queue.pop_front() {
        let n = request(subject);
        let mut position = super::neighbours::Position::default();
        // Legacy order is signature then assertion reference (the additive operation orders opposite first).
        let mut edges: BTreeMap<(ExactRef, ExactRef), ExactRef> = BTreeMap::new();
        loop {
            let slice = lease.neighbours(&n, &position)?;
            let mut at_depth = false;
            for fact in slice.page.neighbours {
                if seen_edges.contains(&fact.origin.owner) {
                    continue;
                }
                if depth == q.depth {
                    at_depth = true;
                    break;
                }
                let NeighbourSelector::Assertion { definition } = fact.selector else {
                    return Err(Error::Integrity(
                        "legacy adjacency contains a non-assertion".into(),
                    ));
                };
                edges.insert((definition, fact.origin.owner), fact.opposite);
                let needed = q.edge_limit - seen_edges.len() + 1;
                if edges.len() > needed {
                    edges.pop_last();
                }
            }
            if at_depth {
                if truncated.is_none() {
                    truncated = Some("depth bound".into());
                }
                break;
            }
            let Some(next) = slice.next else {
                break;
            };
            position = next;
        }
        for ((_, reference), target) in edges {
            if seen_edges.len() == q.edge_limit {
                return Ok((items, Some("edge budget".into())));
            }
            if !visited.contains(&target) && visited.len() == q.subject_limit {
                return Ok((items, Some("subject budget".into())));
            }
            seen_edges.insert(reference.clone());
            items.push(lease.record(&q.selection, &reference)?);
            if visited.insert(target.clone()) {
                let node = lease.record(&q.selection, &target)?;
                queue.push_back((target, depth + 1));
                if matches(&node) {
                    items.push(node);
                }
            }
        }
    }
    Ok((items, truncated))
}
impl OntologyHub {
    pub async fn query(&self, context: Context, query: Query) -> Result<Page, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope.transport(), Action::Read)?;
                let (items, truncated) =
                    if h.store.version() == SHARED_FILE_FORMAT && !query.starts.is_empty() {
                        indexed(&h, &context, &query)?
                    } else {
                        let (_, state) = h.select(&query.selection)?;
                        materialise(&state, &query)?
                    };
                let now = h.cursors.now();
                let fp = fingerprint(&query, &h.scope, &context.actor)?;
                let mut cursor = Cursor {
                    actor: context.actor,
                    scope: h.scope.clone(),
                    fingerprint: fp,
                    query,
                    items,
                    index: 0,
                    truncated,
                    touched: now,
                    neighbour: None,
                };
                if cursor.items.len() <= cursor.query.page_size {
                    return Ok(page(&mut cursor, None));
                }
                let mut map = h.cursors.entries.lock().unwrap();
                h.cursors.expire(&mut map, now);
                if map.len() >= 64 {
                    return Err(Error::Capacity);
                }
                let id = h.store.allocate_id()?.0;
                let result = page(&mut cursor, Some(id.clone()));
                map.insert(id, cursor);
                Ok(result)
            })
            .await
    }
    pub async fn continue_query(
        &self,
        context: Context,
        query: Query,
        continuation: String,
    ) -> Result<Page, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope.transport(), Action::Read)?;
                let now = h.cursors.now();
                let mut map = h.cursors.entries.lock().unwrap();
                h.cursors.expire(&mut map, now);
                let c = map.get_mut(&continuation).ok_or(Error::InvalidCursor)?;
                if c.neighbour.is_some()
                    || c.actor != context.actor
                    || c.scope != h.scope
                    || c.fingerprint != fingerprint(&query, &h.scope, &context.actor)?
                {
                    return Err(Error::InvalidCursor);
                }
                c.touched = now;
                let p = page(c, Some(continuation.clone()));
                if p.continuation.is_none() {
                    map.remove(&continuation);
                }
                Ok(p)
            })
            .await
    }
    pub async fn release_cursor(
        &self,
        context: Context,
        continuation: String,
    ) -> Result<(), Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope.transport(), Action::Read)?;
                let mut map = h.cursors.entries.lock().unwrap();
                if map
                    .get(&continuation)
                    .is_none_or(|c| c.actor != context.actor || c.scope != h.scope)
                {
                    return Err(Error::InvalidCursor);
                }
                map.remove(&continuation);
                Ok(())
            })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn controlled_clock_expires_at_thirty_idle_minutes() {
        let cursors = Cursors::new();
        let scope = ModelScope {
            hub: "h".into(),
            model: "m".into(),
            branch: "b".into(),
            owner: "owner:test".into(),
        };
        let q = Query::select(Selection::Snapshot(crate::SnapshotId("snapshot".into())));
        let mut map = cursors.entries.lock().unwrap();
        map.insert(
            "opaque".into(),
            Cursor {
                actor: "actor:test".into(),
                scope,
                fingerprint: "binding".into(),
                query: q,
                items: vec![],
                index: 0,
                truncated: None,
                touched: cursors.now(),
                neighbour: None,
            },
        );
        cursors
            .offset
            .store(1799, std::sync::atomic::Ordering::SeqCst);
        cursors.expire(&mut map, cursors.now());
        assert_eq!(map.len(), 1);
        cursors
            .offset
            .store(1800, std::sync::atomic::Ordering::SeqCst);
        cursors.expire(&mut map, cursors.now());
        assert!(map.is_empty());
    }
}
