use super::*;
use crate::{Action, Context, Error, codec, ontology::navigation::Cursor, ontology::*};
use std::collections::BTreeSet;
#[derive(Clone)]
pub(crate) struct ImpactCursor {
    request: ModelImpact,
    result: ImpactPage,
    position: usize,
}
fn fp(h: &KnowledgeHub, q: &ModelImpact, actor: &str) -> Result<String, Error> {
    codec::hash_value("knowledgehub-model-impact-cursor/1", &(&h.scope, actor, q))
}
fn size(v: &impl serde::Serialize) -> Result<usize, Error> {
    Ok(serde_json::to_vec(v).map_err(codec::encoding)?.len())
}
fn cut(c: &mut ImpactCursor) -> Result<ImpactPage, Error> {
    let mut p = c.result.clone();
    p.dependencies.clear();
    p.records.clear();
    p.continuation = None;
    let cap = c.request.budgets.bytes;
    // Bounded repeated metadata, with explicit omissions. Never produce a non-progressing cursor.
    while size(&p)? > cap / 2 {
        p.reached.insert(ReadLimit::Bytes);
        p.coverage.omitted = true;
        if p.hits.pop().is_some()
            || p.coverage.gaps.pop().is_some()
            || p.coverage.dead_ends.pop().is_some()
            || p.coverage.unresolved.pop().is_some()
            || p.omitted_records.pop().is_some()
            || p.starts.pop().is_some()
        {
            continue;
        }
        return Err(Error::Capacity);
    }
    let mut wanted = BTreeSet::new();
    while let Some(d) = c.result.dependencies.get(c.position) {
        if p.dependencies.len() == c.request.budgets.page_size {
            break;
        }
        let mut d = d.clone();
        p.dependencies.push(d.clone());
        if size(&p)? + 128 > cap {
            p.dependencies.pop();
            if !p.dependencies.is_empty() {
                break;
            }
            d.witnesses.clear();
            d.omitted = true;
            p.reached.insert(ReadLimit::Bytes);
            p.dependencies.push(d.clone());
            if size(&p)? + 128 > cap {
                return Err(Error::Capacity);
            }
        }
        if c.request.projection.target_records {
            wanted.insert(d.target.clone());
        }
        if c.request.projection.witness_records {
            wanted.extend(d.witnesses.iter().flatten().map(|n| n.origin.owner.clone()));
        }
        c.position += 1;
    }
    for r in &c.result.records {
        if !wanted.contains(&r.reference) {
            continue;
        }
        p.records.push(r.clone());
        if size(&p)? + 128 > cap {
            p.records.pop();
            p.omitted_records.push(r.reference.clone());
            p.reached.insert(ReadLimit::Bytes);
        }
    }
    while size(&p)? + 128 > cap {
        p.coverage.omitted = true;
        p.reached.insert(ReadLimit::Bytes);
        if p.omitted_records.pop().is_none() {
            return Err(Error::Capacity);
        }
    }
    p.exhausted = p.exhausted && c.position == c.result.dependencies.len();
    Ok(p)
}
impl KnowledgeHub {
    /// Literal entry on explicitly selected Text/Narrative fields. No relevance ranking.
    pub async fn search_text(
        &self,
        context: Context,
        request: SearchText,
    ) -> Result<SearchResult, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                request.text.validate()?;
                request.budgets.validate()?;
                context.check(&h.scope, Action::Read)?;
                let first = h.store.acquire_read(&h.scope)?.search_text(&request, false);
                match first {
                    Ok(v) => Ok(v),
                    Err(Error::IndexNotReady { .. })
                        if request.budgets.preparation.allow_rebuild =>
                    {
                        let lease = h.store.acquire(&h.scope)?;
                        context.check(&h.scope, Action::Read)?;
                        lease.search_text(&request, true)
                    }
                    Err(e) => Err(e),
                }
            })
            .await
    }
    /// One pinned read: literal/exact starts, closed typed policy, compact candidate dependencies.
    pub async fn model_impact(
        &self,
        context: Context,
        request: ModelImpact,
    ) -> Result<ImpactPage, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                request.validate()?;
                context.check(&h.scope, Action::Read)?;
                let (selection, first) = {
                    let lease = h.store.acquire_read(&h.scope)?;
                    let selection = match &request.selection {
                        Some(v) => v.clone(),
                        None => Selection::Snapshot(lease.head()?.snapshot),
                    };
                    let first =
                        engine::evaluate(lease.as_ref(), &request, selection.clone(), false);
                    (selection, first)
                };
                let result = match first {
                    Ok(v) => v,
                    Err(Error::IndexNotReady { .. })
                        if request.budgets.preparation.allow_rebuild =>
                    {
                        let lease = h.store.acquire(&h.scope)?;
                        context.check(&h.scope, Action::Read)?;
                        engine::evaluate(lease.as_ref(), &request, selection, true)?
                    }
                    Err(e) => return Err(e),
                };
                // The evaluated frontier and evidence are finite. Bound retained cursor data independently.
                if size(&result)? > 16 * 1024 * 1024 {
                    return Err(Error::Capacity);
                }
                let mut c = ImpactCursor {
                    request: request.clone(),
                    result,
                    position: 0,
                };
                let mut p = cut(&mut c)?;
                context.check(&h.scope, Action::Read)?;
                if c.position < c.result.dependencies.len() {
                    let now = h.cursors.now();
                    let mut entries = h.cursors.entries.lock().unwrap();
                    h.cursors.expire(&mut entries, now);
                    if entries.len() >= 64 {
                        return Err(Error::Capacity);
                    }
                    let id = h.store.allocate_id()?.0;
                    entries.insert(
                        id.clone(),
                        Cursor {
                            actor: context.actor.clone(),
                            scope: h.scope.clone(),
                            fingerprint: fp(&h, &request, &context.actor)?,
                            query: Query::select(p.selection.clone()),
                            items: vec![],
                            index: 0,
                            truncated: None,
                            touched: now,
                            neighbour: None,
                            impact: Some(c),
                        },
                    );
                    p.continuation = Some(id);
                }
                Ok(p)
            })
            .await
    }
    pub async fn continue_model_impact(
        &self,
        context: Context,
        request: ModelImpact,
        continuation: String,
    ) -> Result<ImpactPage, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                request.validate()?;
                context.check(&h.scope, Action::Read)?;
                let now = h.cursors.now();
                let mut entries = h.cursors.entries.lock().unwrap();
                h.cursors.expire(&mut entries, now);
                let c = entries.get_mut(&continuation).ok_or(Error::InvalidCursor)?;
                if c.actor != context.actor
                    || c.scope != h.scope
                    || c.fingerprint != fp(&h, &request, &context.actor)?
                {
                    return Err(Error::InvalidCursor);
                }
                let impact = c.impact.as_mut().ok_or(Error::InvalidCursor)?;
                if impact.request != request {
                    return Err(Error::InvalidCursor);
                }
                let mut p = cut(impact)?;
                c.touched = now;
                if impact.position < impact.result.dependencies.len() {
                    p.continuation = Some(continuation);
                } else {
                    entries.remove(&continuation);
                }
                Ok(p)
            })
            .await
    }
    pub async fn traverse(&self, context: Context, request: Traverse) -> Result<ImpactPage, Error> {
        self.model_impact(context, request.composite()).await
    }
    pub async fn continue_traverse(
        &self,
        context: Context,
        request: Traverse,
        continuation: String,
    ) -> Result<ImpactPage, Error> {
        self.continue_model_impact(context, request.composite(), continuation)
            .await
    }
}
