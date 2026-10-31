use super::*;
use crate::{Action, Context, Error, codec};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    sync::Mutex,
    time::{Duration, Instant},
};

pub(crate) struct Cursors {
    entries: Mutex<BTreeMap<String, Cursor>>,
    origin: Instant,
    #[cfg(test)]
    offset: std::sync::atomic::AtomicU64,
}
struct Cursor {
    actor: String,
    scope: ModelScope,
    fingerprint: String,
    query: Query,
    items: Vec<Record>,
    index: usize,
    truncated: Option<String>,
    touched: Duration,
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
    fn now(&self) -> Duration {
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
    fn expire(&self, map: &mut BTreeMap<String, Cursor>, now: Duration) {
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
impl OntologyHub {
    pub async fn query(&self, context: Context, query: Query) -> Result<Page, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope.transport(), Action::Read)?;
                let (_, state) = h.select(&query.selection)?;
                let (items, truncated) = materialise(&state, &query)?;
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
                if c.actor != context.actor
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
