use super::*;
use crate::ontology::navigation::Cursor;
use crate::{Action, Context, codec};

#[derive(Clone)]
pub(crate) struct NeighbourCursor {
    pub request: Neighbours,
    pub position: Position,
}
fn supported(h: &OntologyHub, q: &Neighbours) -> Result<(), Error> {
    validate(q)?;
    if h.store.version() != SHARED_FILE_FORMAT {
        return Err(Error::Incompatible(
            "neighbours require explicit files/3 capability".into(),
        ));
    }
    Ok(())
}
fn fingerprint(h: &OntologyHub, q: &Neighbours, actor: &str) -> Result<String, Error> {
    codec::hash_value("knowledgehub-neighbours/1", &(&h.scope, actor, q))
}
pub(crate) fn read_slice(
    h: &OntologyHub,
    context: &Context,
    q: &Neighbours,
    p: &Position,
) -> Result<Slice, Error> {
    supported(h, q)?;
    context.check(&h.scope.transport(), Action::Read)?;
    let first = h.store.acquire_read(&h.scope.transport())?.neighbours(q, p);
    let reason = match first {
        Ok(slice) => return Ok(slice),
        Err(Error::IndexNotReady { reason }) => reason,
        Err(e) => return Err(e),
    };
    if !q.preparation.allow_rebuild {
        return Err(Error::IndexNotReady { reason });
    }
    let prepared = {
        let lease = h.store.acquire(&h.scope.transport())?;
        context.check(&h.scope.transport(), Action::Read)?;
        lease.prepare_neighbours(q, true)?
    };
    context.check(&h.scope.transport(), Action::Read)?;
    let mut slice = h
        .store
        .acquire_read(&h.scope.transport())?
        .neighbours(q, p)?;
    slice.page.work.prepared_owners = prepared.prepared_owners;
    slice.page.work.prepared_facts = prepared.prepared_facts;
    slice.page.work.index_objects_read += prepared.index_objects_read;
    slice.page.work.index_bytes_read += prepared.index_bytes_read;
    slice.page.work.record_reads += prepared.record_reads;
    slice.page.work.index_rebuilt = true;
    slice.page.work.rebuild_reason = Some(reason);
    Ok(slice)
}
impl OntologyHub {
    /// Prepare a disposable exact-view index within the request's finite cold-build budget.
    pub async fn prepare_neighbours(
        &self,
        context: Context,
        request: Neighbours,
    ) -> Result<NeighbourWork, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                supported(&h, &request)?;
                context.check(&h.scope.transport(), Action::Read)?;
                let lease = h.store.acquire(&h.scope.transport())?;
                context.check(&h.scope.transport(), Action::Read)?;
                lease.prepare_neighbours(&request, false)
            })
            .await
    }
    pub async fn neighbours(
        &self,
        context: Context,
        request: Neighbours,
    ) -> Result<NeighbourPage, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope.transport(), Action::Read)?;
                let mut slice = read_slice(&h, &context, &request, &Position::default())?;
                let Some(position) = slice.next else {
                    return Ok(slice.page);
                };
                let now = h.cursors.now();
                let mut entries = h.cursors.entries.lock().unwrap();
                h.cursors.expire(&mut entries, now);
                if entries.len() >= 64 {
                    return Err(Error::Capacity);
                }
                let id = h.store.allocate_id()?.0;
                let fp = fingerprint(&h, &request, &context.actor)?;
                entries.insert(
                    id.clone(),
                    Cursor {
                        actor: context.actor,
                        scope: h.scope.clone(),
                        fingerprint: fp,
                        query: Query::select(request.selection.clone()),
                        items: Vec::new(),
                        index: 0,
                        truncated: None,
                        touched: now,
                        neighbour: Some(NeighbourCursor { request, position }),
                    },
                );
                slice.page.continuation = Some(id);
                Ok(slice.page)
            })
            .await
    }
    pub async fn continue_neighbours(
        &self,
        context: Context,
        request: Neighbours,
        continuation: String,
    ) -> Result<NeighbourPage, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                supported(&h, &request)?;
                context.check(&h.scope.transport(), Action::Read)?;
                let now = h.cursors.now();
                let fp = fingerprint(&h, &request, &context.actor)?;
                let (position, epoch) = {
                    let mut entries = h.cursors.entries.lock().unwrap();
                    h.cursors.expire(&mut entries, now);
                    let c = entries.get_mut(&continuation).ok_or(Error::InvalidCursor)?;
                    if c.actor != context.actor || c.scope != h.scope || c.fingerprint != fp {
                        return Err(Error::InvalidCursor);
                    }
                    let n = c.neighbour.as_ref().ok_or(Error::InvalidCursor)?;
                    if n.request != request {
                        return Err(Error::InvalidCursor);
                    }
                    c.touched = now;
                    (n.position.clone(), c.index)
                };
                // Do disk work outside the cursor mutex. Concurrent use of one token is refused by its epoch.
                let mut slice = read_slice(&h, &context, &request, &position)?;
                let mut entries = h.cursors.entries.lock().unwrap();
                let c = entries.get_mut(&continuation).ok_or(Error::InvalidCursor)?;
                if c.index != epoch {
                    return Err(Error::Busy);
                }
                c.index += 1;
                c.touched = h.cursors.now();
                if let Some(position) = slice.next {
                    c.neighbour = Some(NeighbourCursor { request, position });
                    slice.page.continuation = Some(continuation);
                } else {
                    entries.remove(&continuation);
                }
                Ok(slice.page)
            })
            .await
    }
}
