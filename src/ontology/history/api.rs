use super::*;
use crate::{
    Action, Context, Error, KnowledgeHub, Query, Selection, codec, ontology::navigation::Cursor,
};

fn authority(h: &KnowledgeHub, context: &Context) -> Result<(), Error> {
    context.check(&h.scope, Action::Read)?;
    context.check(&h.scope, Action::ReadHistory)
}
fn fingerprint(h: &KnowledgeHub, context: &Context, q: &HistoryRequest) -> Result<String, Error> {
    codec::hash_value(HISTORY_FORMAT, &(&h.scope, &context.actor, q))
}
impl KnowledgeHub {
    /// Resolve original publication metadata at an exact accepted snapshot.
    /// Requires current `Read` and `ReadHistory`; never impersonates the original actor.
    pub async fn publication(
        &self,
        context: Context,
        request: Publication,
    ) -> Result<PublicationResult, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                authority(&h, &context)?;
                types::validate_anchor(&request.anchor)?;
                types::validate_id(&request.operation_id)?;
                request.limits.validate()?;
                let result = h.store.publication(&h.scope, &request)?;
                authority(&h, &context)?;
                Ok(result)
            })
            .await
    }
    /// Discover accepted publications newest first, pinned to the request's anchor.
    pub async fn publications(
        &self,
        context: Context,
        request: Publications,
    ) -> Result<HistoryPage, Error> {
        self.start_history(context, HistoryRequest::Publications(request))
            .await
    }
    /// Page the exact original affected vector, retaining order and repetitions.
    pub async fn publication_affected(
        &self,
        context: Context,
        request: PublicationAffected,
    ) -> Result<HistoryPage, Error> {
        self.start_history(context, HistoryRequest::Affected(request))
            .await
    }
    async fn start_history(
        &self,
        context: Context,
        request: HistoryRequest,
    ) -> Result<HistoryPage, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                authority(&h, &context)?;
                request.validate()?;
                let mut slice = h
                    .store
                    .history_page(&h.scope, &request, &Position::default())?;
                authority(&h, &context)?;
                if let Some(position) = slice.next {
                    let now = h.cursors.now();
                    let mut entries = h.cursors.entries.lock().unwrap();
                    h.cursors.expire(&mut entries, now);
                    if entries.len() >= 64 {
                        return Err(Error::Capacity);
                    }
                    let id = h.store.allocate_id()?.0;
                    let fp = fingerprint(&h, &context, &request)?;
                    entries.insert(
                        id.clone(),
                        Cursor {
                            actor: context.actor,
                            scope: h.scope.clone(),
                            fingerprint: fp,
                            query: Query::select(Selection::Snapshot(request.anchor().clone())),
                            items: vec![],
                            index: 0,
                            truncated: None,
                            touched: now,
                            neighbour: None,
                            impact: None,
                            history: Some(HistoryCursor { position }),
                        },
                    );
                    slice.page.continuation = Some(id);
                }
                Ok(slice.page)
            })
            .await
    }
    /// Resume only with identical intent. Limits, actor, scope, anchor and kind are bound.
    /// Tokens are process-local, share the 64-entry pool, and expire after 30 idle minutes.
    pub async fn continue_history(
        &self,
        context: Context,
        request: HistoryRequest,
        continuation: String,
    ) -> Result<HistoryPage, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                authority(&h, &context)?;
                request.validate()?;
                let fp = fingerprint(&h, &context, &request)?;
                let (position, epoch) = {
                    let mut entries = h.cursors.entries.lock().unwrap();
                    h.cursors.expire(&mut entries, h.cursors.now());
                    let c = entries.get(&continuation).ok_or(Error::InvalidCursor)?;
                    let cursor = c.history.as_ref().ok_or(Error::InvalidCursor)?;
                    if c.actor != context.actor || c.scope != h.scope || c.fingerprint != fp {
                        return Err(Error::InvalidCursor);
                    }
                    (cursor.position.clone(), c.index)
                };
                let mut slice = h.store.history_page(&h.scope, &request, &position)?;
                authority(&h, &context)?;
                let mut entries = h.cursors.entries.lock().unwrap();
                h.cursors.expire(&mut entries, h.cursors.now());
                let c = entries.get_mut(&continuation).ok_or(Error::InvalidCursor)?;
                if c.index != epoch {
                    return Err(Error::Busy);
                }
                c.index = c.index.checked_add(1).ok_or(Error::Capacity)?;
                c.touched = h.cursors.now();
                if let Some(position) = slice.next {
                    c.history = Some(HistoryCursor { position });
                    slice.page.continuation = Some(continuation);
                } else {
                    entries.remove(&continuation);
                }
                Ok(slice.page)
            })
            .await
    }
    /// Release a history token under current scoped authority and the same reader actor.
    pub async fn release_history(
        &self,
        context: Context,
        continuation: String,
    ) -> Result<(), Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                authority(&h, &context)?;
                let mut entries = h.cursors.entries.lock().unwrap();
                h.cursors.expire(&mut entries, h.cursors.now());
                let c = entries.get(&continuation).ok_or(Error::InvalidCursor)?;
                if c.actor != context.actor || c.scope != h.scope || c.history.is_none() {
                    return Err(Error::InvalidCursor);
                }
                entries.remove(&continuation);
                Ok(())
            })
            .await
    }
}
