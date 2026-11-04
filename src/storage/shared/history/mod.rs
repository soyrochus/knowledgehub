use super::{
    generation::{self, Generation},
    objects::Objects,
    projection,
    trees::Tree,
    *,
};
use crate::ontology::history::{Position, Slice};
use crate::{
    HISTORY_FORMAT, HistoryItems, HistoryLimits, HistoryPage, HistoryRequest, HistoryResource,
    HistoryWork, Publication, PublicationHeader, PublicationLookup, PublicationResult,
};
use serde_json::json as value;
pub(in crate::storage::shared) mod budget;
#[cfg(test)]
mod tests;

fn anchor(
    o: &Objects<'_>,
    scope: &ModelScope,
    a: &SnapshotId,
) -> Result<(String, Generation), Error> {
    let (id, head) = generation::root(o.store, scope, o)?;
    generation::ancestor(o, &id, &head, a).map_err(|e| match e {
        Error::NotFound => Error::HistoryAnchorUnavailable { anchor: a.clone() },
        other => other,
    })
}
fn verify(
    o: &Objects<'_>,
    g: &Generation,
    oid: &str,
    h: &PublicationHeader,
) -> Result<Generation, Error> {
    if h.scope != g.scope
        || h.head != g.snapshot_id
        || h.sequence != number(&g.sequence)?
        || h.fingerprint_version != ontology::REQUEST_FORMAT
        || !model::hash(&h.fingerprint)
        || h.actor.trim().is_empty()
        || h.operation_id.trim().is_empty()
        || g.receipt.as_deref() != Some(oid)
    {
        return Err(integrity("history original receipt binding"));
    }
    let parent = generation::load_generation(
        o,
        g.parent
            .as_deref()
            .ok_or_else(|| integrity("history receipt parent"))?,
        &g.scope,
    )?;
    if h.previous_head != parent.snapshot_id
        || number(&parent.sequence)?.checked_add(1) != Some(h.sequence)
    {
        return Err(integrity("history original receipt parent binding"));
    }
    Ok(parent)
}
fn known(
    o: &Objects<'_>,
    id: &str,
    a: &Generation,
    operation: &str,
) -> Result<Option<(PublicationHeader, String)>, Error> {
    let Some(v) =
        (Tree { objects: o }).get(&a.operations_root, "operations/1", &value!(operation))?
    else {
        return Ok(None);
    };
    let oid = v
        .as_str()
        .ok_or_else(|| integrity("history operation receipt address"))?;
    let (h, affected) = projection::receipt_header(o, oid)?;
    if h.operation_id != operation {
        return Err(integrity("history operation key binding"));
    }
    let (_, g) = generation::ancestor(o, id, a, &h.head).map_err(|e| match e {
        Error::NotFound => integrity("history receipt references unaccepted ancestry"),
        other => other,
    })?;
    verify(o, &g, oid, &h)?;
    Ok(Some((h, affected)))
}
fn limit(work: &HistoryWork, limits: &HistoryLimits) -> Error {
    Error::HistoryLimit {
        resource: HistoryResource::OutputBytes,
        limit: limits.output_bytes,
        work: work.clone(),
    }
}
fn seal_lookup(r: &mut PublicationResult, limits: &HistoryLimits) -> Result<(), Error> {
    loop {
        let n = ontology::history::output(r)?;
        if n > limits.output_bytes {
            return Err(limit(&r.work, limits));
        }
        if n == r.work.output_bytes {
            return Ok(());
        }
        r.work.output_bytes = n;
    }
}
fn seal(r: &mut HistoryPage, limits: &HistoryLimits) -> Result<(), Error> {
    loop {
        let n = ontology::history::output(r)?;
        if n > limits.output_bytes {
            return Err(limit(&r.work, limits));
        }
        if n == r.work.output_bytes {
            return Ok(());
        }
        r.work.output_bytes = n;
    }
}
fn page_base(scope: &ModelScope, q: &HistoryRequest, items: HistoryItems) -> HistoryPage {
    HistoryPage {
        format: HISTORY_FORMAT.into(),
        scope: scope.clone(),
        anchor: q.anchor().clone(),
        items,
        complete: false,
        initial_boundary: None,
        stopped_by: Some(HistoryResource::Items),
        continuation: Some("0".repeat(64)),
        work: HistoryWork::default(),
    }
}
fn resource(e: &Error) -> Option<HistoryResource> {
    if let Error::HistoryLimit { resource, .. } = e {
        Some(*resource)
    } else {
        None
    }
}

pub(in crate::storage) fn publication(
    store: &FileStore,
    scope: &ModelScope,
    q: &Publication,
) -> Result<PublicationResult, Error> {
    let budget = budget::Budget::start(&q.limits);
    let _lease = generation::acquire(store, scope, true)?;
    let o = Objects::new(store);
    let (id, a) = anchor(&o, scope, &q.anchor)?;
    let outcome = match known(&o, &id, &a, &q.operation_id)? {
        Some((h, _)) => PublicationLookup::Published(Box::new(h)),
        None => PublicationLookup::NotPublishedAt,
    };
    let mut r = PublicationResult {
        format: HISTORY_FORMAT.into(),
        scope: scope.clone(),
        anchor: q.anchor.clone(),
        operation_id: q.operation_id.clone(),
        outcome,
        work: budget.work(),
    };
    seal_lookup(&mut r, &q.limits)?;
    Ok(r)
}

pub(in crate::storage) fn page(
    store: &FileStore,
    scope: &ModelScope,
    q: &HistoryRequest,
    position: &Position,
) -> Result<Slice, Error> {
    let limits = q.limits();
    let budget = budget::Budget::start(limits);
    let _lease = generation::acquire(store, scope, true)?;
    let o = Objects::new(store);
    let (id, a) = anchor(&o, scope, q.anchor())?;
    match q {
        HistoryRequest::Publications(_) => {
            let mut at = position.next.clone().unwrap_or_else(|| q.anchor().clone());
            let mut page = page_base(scope, q, HistoryItems::Publications(vec![]));
            loop {
                let read = || -> Result<(PublicationHeader, Generation), Error> {
                    let (_, g) = generation::ancestor(&o, &id, &a, &at)?;
                    let oid = g
                        .receipt
                        .as_deref()
                        .ok_or_else(|| integrity("publication receipt missing"))?;
                    let (h, _) = projection::receipt_header(&o, oid)?;
                    let mapped = (Tree { objects: &o }).get(
                        &a.operations_root,
                        "operations/1",
                        &value!(h.operation_id),
                    )?;
                    if mapped.as_ref().and_then(serde_json::Value::as_str) != Some(oid) {
                        return Err(integrity("history generation operation binding"));
                    }
                    let parent = verify(&o, &g, oid, &h)?;
                    Ok((h, parent))
                };
                // The initial accepted generation has no receipt. It is not a publication.
                if a.sequence == "0" && at == a.snapshot_id {
                    page.complete = true;
                    page.initial_boundary = Some(at.clone());
                    break;
                }
                let (header, parent) = match read() {
                    Ok(v) => v,
                    Err(e) => {
                        let has_items =
                            matches!(&page.items, HistoryItems::Publications(v) if !v.is_empty());
                        if let Some(r) = resource(&e).filter(|_| has_items) {
                            page.stopped_by = Some(r);
                            break;
                        }
                        return Err(e);
                    }
                };
                let HistoryItems::Publications(headers) = &mut page.items else {
                    unreachable!()
                };
                headers.push(header);
                page.work = budget.work();
                // Size the actual boundary shape: a complete page needs no token.
                page.complete = parent.sequence == "0";
                page.initial_boundary = page.complete.then(|| parent.snapshot_id.clone());
                page.continuation = (!page.complete).then(|| "0".repeat(64));
                page.stopped_by = (!page.complete).then_some(HistoryResource::Items);
                if let Err(e) = seal(&mut page, limits) {
                    let HistoryItems::Publications(headers) = &mut page.items else {
                        unreachable!()
                    };
                    headers.pop();
                    if headers.is_empty() {
                        return Err(e);
                    }
                    page.complete = false;
                    page.initial_boundary = None;
                    page.continuation = Some("0".repeat(64));
                    page.stopped_by = Some(HistoryResource::OutputBytes);
                    break;
                }
                at = parent.snapshot_id.clone();
                if page.complete {
                    break;
                }
                let HistoryItems::Publications(headers) = &page.items else {
                    unreachable!()
                };
                if headers.len() as u64 >= limits.items {
                    page.stopped_by = Some(HistoryResource::Items);
                    break;
                }
            }
            page.work = budget.work();
            if page.complete {
                page.continuation = None;
                page.stopped_by = None;
            }
            // A later attempted verification can increase envelope digits; trim with a logical rewind.
            while let Err(e) = seal(&mut page, limits) {
                let HistoryItems::Publications(headers) = &mut page.items else {
                    unreachable!()
                };
                let h = headers.pop().ok_or(e.clone())?;
                if headers.is_empty() {
                    return Err(e);
                }
                at = h.head;
                page.complete = false;
                page.initial_boundary = None;
                page.continuation = Some("0".repeat(64));
                page.stopped_by = Some(HistoryResource::OutputBytes);
            }
            let next = (!page.complete).then_some(Position {
                next: Some(at),
                offset: 0,
            });
            Ok(Slice { page, next })
        }
        HistoryRequest::Affected(q) => {
            let found = known(&o, &id, &a, &q.operation_id)?;
            let (published, total, root) = match found {
                Some((h, root)) => (true, h.affected_count, root),
                None => (false, 0, String::new()),
            };
            if position.offset > total {
                return Err(Error::InvalidCursor);
            }
            let mut page = page_base(
                scope,
                &HistoryRequest::Affected(q.clone()),
                HistoryItems::Affected {
                    operation_id: q.operation_id.clone(),
                    published,
                    total,
                    offset: position.offset,
                    references: vec![],
                },
            );
            let mut at = position.offset;
            while at < total && at - position.offset < limits.items {
                let read = || -> Result<ExactRef, Error> {
                    let v = (Tree { objects: &o }).seq_range(&root, "exact-refs/1", at, 1)?;
                    let r: ExactRef = serde_json::from_value(
                        v.into_iter()
                            .next()
                            .ok_or_else(|| integrity("history affected range"))?,
                    )
                    .map_err(|_| integrity("history affected reference"))?;
                    if !r.is_valid() {
                        return Err(integrity("history invalid affected reference"));
                    }
                    Ok(r)
                };
                let r = match read() {
                    Ok(r) => r,
                    Err(e) => {
                        if let Some(r) = resource(&e).filter(|_| at > position.offset) {
                            page.stopped_by = Some(r);
                            break;
                        }
                        return Err(e);
                    }
                };
                let HistoryItems::Affected { references, .. } = &mut page.items else {
                    unreachable!()
                };
                references.push(r);
                page.work = budget.work();
                page.complete = at.checked_add(1) == Some(total);
                page.continuation = (!page.complete).then(|| "0".repeat(64));
                page.stopped_by = (!page.complete).then_some(HistoryResource::Items);
                if let Err(e) = seal(&mut page, limits) {
                    let HistoryItems::Affected { references, .. } = &mut page.items else {
                        unreachable!()
                    };
                    references.pop();
                    if at == position.offset {
                        return Err(e);
                    }
                    page.complete = false;
                    page.continuation = Some("0".repeat(64));
                    page.stopped_by = Some(HistoryResource::OutputBytes);
                    break;
                }
                at = at.checked_add(1).ok_or(Error::Capacity)?;
            }
            page.work = budget.work();
            page.complete = at == total;
            if page.complete {
                page.continuation = None;
                page.stopped_by = None;
            }
            while let Err(e) = seal(&mut page, limits) {
                let HistoryItems::Affected { references, .. } = &mut page.items else {
                    unreachable!()
                };
                references.pop().ok_or(e.clone())?;
                at = at.checked_sub(1).ok_or(Error::Capacity)?;
                if at == position.offset {
                    return Err(e);
                }
                page.complete = false;
                page.continuation = Some("0".repeat(64));
                page.stopped_by = Some(HistoryResource::OutputBytes);
            }
            let next = (!page.complete).then_some(Position {
                next: None,
                offset: at,
            });
            Ok(Slice { page, next })
        }
    }
}
