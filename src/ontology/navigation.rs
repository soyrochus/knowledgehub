use super::*;
use crate::{Action, Context, Error, codec};
use std::{
    collections::BTreeMap,
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
    pub(crate) impact: Option<super::impact::ImpactCursor>,
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
        || q.depth > 32
        || q.subject_limit == 0
        || q.subject_limit > 10000
        || q.edge_limit == 0
        || q.edge_limit > 10000
        || q.starts.len() > 1000
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
    unreachable!("traversal is handled by the current graph engine")
}
fn expand(
    h: &KnowledgeHub,
    context: &Context,
    q: &Query,
) -> Result<(Vec<Record>, Option<String>), Error> {
    query_bounds(q)?;
    let first = {
        let lease = h.store.acquire_read(&h.scope)?;
        super::impact::evaluate_query(lease.as_ref(), q, false)
    };
    match first {
        Err(Error::IndexNotReady { .. }) if q.preparation.allow_rebuild => {
            let lease = h.store.acquire(&h.scope)?;
            context.check(&h.scope, Action::Read)?;
            super::impact::evaluate_query(lease.as_ref(), q, true)
        }
        other => other,
    }
}
impl KnowledgeHub {
    pub async fn query(&self, context: Context, query: Query) -> Result<Page, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope, Action::Read)?;
                let (items, truncated) = if !query.starts.is_empty() {
                    expand(&h, &context, &query)?
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
                    impact: None,
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
                context.check(&h.scope, Action::Read)?;
                let now = h.cursors.now();
                let mut map = h.cursors.entries.lock().unwrap();
                h.cursors.expire(&mut map, now);
                let c = map.get_mut(&continuation).ok_or(Error::InvalidCursor)?;
                if c.neighbour.is_some()
                    || c.impact.is_some()
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
                context.check(&h.scope, Action::Read)?;
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
                impact: None,
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
