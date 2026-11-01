use super::*;

struct Stream {
    subject: ExactRef,
    selector: NeighbourSelector,
    outgoing: Option<Group>,
    incoming: Option<Group>,
}
fn load_group(tree: &Tree<'_, '_>, v: Json) -> Result<Group, Error> {
    let g: Group =
        serde_json::from_value(v).map_err(|_| missing("invalid selector posting descriptor"))?;
    let n = number(&g.count).map_err(derived)?;
    if n == 0
        || n > 100_000
        || g.categories.is_empty()
        || tree.seq_count(&g.root, POSTINGS).map_err(derived)? != n
    {
        return Err(missing("posting descriptor count/category mismatch"));
    }
    Ok(g)
}
fn groups(
    tree: &Tree<'_, '_>,
    root: &str,
    subject: &ExactRef,
    q: &Neighbours,
) -> Result<BTreeMap<NeighbourSelector, Group>, Error> {
    let Some(inner) = tree
        .get(root, ENDPOINTS, &ref_key(subject))
        .map_err(derived)?
    else {
        return Ok(BTreeMap::new());
    };
    let inner = inner
        .as_str()
        .ok_or_else(|| missing("endpoint selector root is not an address"))?;
    let entries = if q.selectors.is_empty() {
        if tree.map_count(inner, SELECTORS).map_err(derived)? > 10000 {
            return Err(Error::Capacity);
        }
        tree.entries(inner, SELECTORS).map_err(derived)?
    } else {
        let mut entries = Vec::new();
        for s in &q.selectors {
            if let Some(v) = tree.get(inner, SELECTORS, &value!(s)).map_err(derived)? {
                entries.push((value!(s), v));
            }
        }
        entries
    };
    let mut result = BTreeMap::new();
    for (key, v) in entries {
        let selector: NeighbourSelector =
            serde_json::from_value(key).map_err(|_| missing("selector grammar differs"))?;
        let summary: Group =
            serde_json::from_value(v.clone()).map_err(|_| missing("selector summary"))?;
        if !summary.categories.is_disjoint(&q.categories) {
            result.insert(selector, load_group(tree, v)?);
        }
    }
    Ok(result)
}
fn streams(tree: &Tree<'_, '_>, m: &Manifest, q: &Neighbours) -> Result<Vec<Stream>, Error> {
    let mut result = Vec::new();
    let mut starts = q.starts.clone();
    starts.sort();
    starts.dedup();
    for subject in starts {
        let mut out = if q.direction != NeighbourDirection::Incoming {
            groups(tree, &m.outgoing, &subject, q)?
        } else {
            BTreeMap::new()
        };
        let mut incoming = if q.direction != NeighbourDirection::Outgoing {
            groups(tree, &m.incoming, &subject, q)?
        } else {
            BTreeMap::new()
        };
        let selectors = out
            .keys()
            .chain(incoming.keys())
            .cloned()
            .collect::<BTreeSet<_>>();
        for selector in selectors {
            result.push(Stream {
                subject: subject.clone(),
                outgoing: out.remove(&selector),
                incoming: incoming.remove(&selector),
                selector,
            });
            if result.len() > 10000 {
                return Err(Error::Capacity);
            }
        }
    }
    Ok(result)
}
fn length(g: &Option<Group>) -> Result<u64, Error> {
    g.as_ref()
        .map(|g| number(&g.count))
        .transpose()
        .map(|n| n.unwrap_or(0))
        .map_err(derived)
}
fn advance(p: &mut Position, streams: &[Stream]) -> Result<(), Error> {
    while let Some(s) = streams.get(p.group) {
        let (out, inc) = (length(&s.outgoing)?, length(&s.incoming)?);
        if p.outgoing > out || p.incoming > inc {
            return Err(Error::InvalidCursor);
        }
        if p.outgoing != out || p.incoming != inc {
            break;
        }
        p.group += 1;
        p.outgoing = 0;
        p.incoming = 0;
    }
    Ok(())
}
fn posting(
    tree: &Tree<'_, '_>,
    g: &Option<Group>,
    at: u64,
    work: &mut NeighbourWork,
) -> Result<Option<Posting>, Error> {
    if at >= length(g)? {
        return Ok(None);
    }
    let g = g.as_ref().unwrap();
    let v = tree
        .seq_range(&g.root, POSTINGS, at, 1)
        .map_err(derived)?
        .pop()
        .ok_or_else(|| missing("missing posting item"))?;
    let p: Posting = serde_json::from_value(v).map_err(|_| missing("invalid posting item"))?;
    if !p.valid() {
        return Err(missing("invalid posting references"));
    }
    work.postings_read += 1;
    Ok(Some(p))
}
fn vertex(records: &mut Records<'_, '_>, r: &ExactRef) -> Result<NeighbourVertex, Error> {
    match records.get(r) {
        Ok(v) => {
            let selected = Tree {
                objects: records.objects,
            }
            .get(&records.view.selected, "selection/1", &identity_key(r))?;
            Ok(NeighbourVertex {
                reference: r.clone(),
                available: true,
                selected: if r.revision == "@identity" {
                    selected.is_some()
                } else {
                    selected == Some(value!(r))
                },
                type_ref: Some(v.type_ref),
            })
        }
        Err(Error::NotFound) => Ok(NeighbourVertex {
            reference: r.clone(),
            available: false,
            selected: false,
            type_ref: None,
        }),
        Err(e) => Err(e),
    }
}
fn type_match(
    records: &mut Records<'_, '_>,
    actual: &ExactRef,
    wanted: &ExactRef,
    subtypes: bool,
) -> Result<bool, Error> {
    if actual == wanted {
        return Ok(true);
    }
    if !subtypes {
        return Ok(false);
    }
    let mut todo = vec![(actual.clone(), 0)];
    let mut seen = BTreeSet::new();
    while let Some((r, depth)) = todo.pop() {
        if !seen.insert(r.clone()) {
            continue;
        }
        if depth > 64 || seen.len() > 1000 {
            return Err(Error::Capacity);
        }
        if r == *wanted {
            return Ok(true);
        }
        let r = records.get(&r)?;
        let RecordData::Type(t) = r.data else {
            return Err(integrity("opposite type hierarchy"));
        };
        for p in t.parents {
            todo.push((p, depth + 1));
        }
    }
    Ok(false)
}
fn validate_filters(q: &Neighbours, records: &mut Records<'_, '_>) -> Result<(), Error> {
    let ontology = records
        .view
        .g
        .ontology
        .as_ref()
        .map(|r| records.get(r))
        .transpose()?;
    if let Some(t) = &q.opposite_type {
        match records.get(t) {
            Ok(r) if matches!(r.data, RecordData::Type(_)) => {}
            Ok(_) | Err(Error::NotFound) => {
                return Err(Error::InvalidInput(
                    "unknown exact opposite type filter".into(),
                ));
            }
            Err(e) => return Err(e),
        }
    }
    for selector in &q.selectors {
        let record = records.get(selector.definition()).map_err(|e| {
            if e == Error::NotFound {
                Error::InvalidInput("unknown exact selector definition".into())
            } else {
                e
            }
        })?;
        let valid = facts::selector_supported(selector, &record, ontology.as_ref(), &mut |r| {
            records.get(r)
        })?;
        if !valid {
            return Err(Error::InvalidInput(
                "selector definition, schema field or native mode is unsupported".into(),
            ));
        }
    }
    Ok(())
}
fn verify_fact(
    records: &mut Records<'_, '_>,
    n: &Neighbour,
    ontology: Option<&Record>,
    known: &mut BTreeMap<ExactRef, BTreeSet<Fact>>,
) -> Result<(), Error> {
    if !known.contains_key(&n.origin.owner) {
        let owner = records.get(&n.origin.owner).map_err(|e| {
            if e == Error::NotFound {
                missing("posting owner is unavailable in this view")
            } else {
                e
            }
        })?;
        let extracted = facts::extract(&owner, ontology, &mut |r| records.get(r))?;
        known.insert(n.origin.owner.clone(), extracted.into_iter().collect());
    }
    for direction in &n.directions {
        let (from, to) = if *direction == NeighbourDirection::Outgoing {
            (&n.subject, &n.opposite)
        } else {
            (&n.opposite, &n.subject)
        };
        let expected = Fact {
            from: from.clone(),
            selector: n.selector.clone(),
            to: to.clone(),
            origin: n.origin.clone(),
            category: n.category,
        };
        if !known[&n.origin.owner].contains(&expected) {
            return Err(missing(
                "posting does not match its exact authoritative origin",
            ));
        }
    }
    Ok(())
}

pub(in crate::storage::shared) fn read(
    store: &FileStore,
    scope: &ModelScope,
    q: &Neighbours,
    position: &Position,
) -> Result<Slice, Error> {
    let objects = Objects::new(store);
    let (id, g) = generation::root(store, scope, &objects)?;
    let view = generation::view(&objects, &id, &g, &q.selection)?;
    let mut records = Records {
        objects: &objects,
        view: &view,
        values: BTreeMap::new(),
        reads: 0,
        bytes: 0,
        limit: 64 * 1024 * 1024,
    };
    validate_filters(q, &mut records)?;
    let c = cache(store);
    let m = manifest(&c, &Binding::view(&view, q.fact_scope))?;
    let index = Objects::new(&c);
    let tree = Tree { objects: &index };
    let streams = streams(&tree, &m, q)?;
    let mut p = position.clone();
    if p.group > streams.len() {
        return Err(Error::InvalidCursor);
    }
    advance(&mut p, &streams)?;
    let mut page = NeighbourPage {
        selection: q.selection.clone(),
        ontology: view.g.ontology.clone(),
        profile: view.g.profile.clone(),
        fact_scope: q.fact_scope,
        neighbours: Vec::new(),
        vertices: Vec::new(),
        records: Vec::new(),
        continuation: None,
        reached: None,
        exhausted: false,
        work: NeighbourWork {
            indexed_owners: m.owners,
            indexed_facts: m.facts,
            indexed_postings: 2 * m.facts,
            ..NeighbourWork::default()
        },
    };
    let mut vertices = BTreeMap::new();
    let mut projected = BTreeMap::new();
    let mut bytes = 8;
    for r in q.starts.iter().collect::<BTreeSet<_>>() {
        let v = vertex(&mut records, r)?;
        bytes += json(&v)?.len() + 1;
        if q.projection == NeighbourProjection::Records && v.available {
            let record = records.get(r)?;
            bytes += json(&record)?.len() + 1;
            projected.insert(r.clone(), record);
        }
        vertices.insert(r.clone(), v);
    }
    if bytes > q.byte_limit {
        return Err(Error::Capacity);
    }
    let ontology = view
        .g
        .ontology
        .as_ref()
        .map(|r| records.get(r))
        .transpose()?;
    let mut known = BTreeMap::new();
    while p.group < streams.len() {
        if page.neighbours.len() == q.page_size {
            page.reached = Some(NeighbourLimit::Page);
            break;
        }
        if page.work.examined_postings == q.examined_limit as u64 {
            page.reached = Some(NeighbourLimit::ExaminedPostings);
            break;
        }
        let before = p.clone();
        let s = &streams[p.group];
        let out = posting(&tree, &s.outgoing, p.outgoing, &mut page.work)?;
        let inc = posting(&tree, &s.incoming, p.incoming, &mut page.work)?;
        let (posting, directions) = match (out, inc) {
            (Some(a), Some(b)) if a == b && a.opposite == s.subject => {
                p.outgoing += 1;
                p.incoming += 1;
                (
                    a,
                    BTreeSet::from([NeighbourDirection::Outgoing, NeighbourDirection::Incoming]),
                )
            }
            (Some(a), Some(b)) if a <= b => {
                p.outgoing += 1;
                (a, BTreeSet::from([NeighbourDirection::Outgoing]))
            }
            (_, Some(b)) => {
                p.incoming += 1;
                (b, BTreeSet::from([NeighbourDirection::Incoming]))
            }
            (Some(a), None) => {
                p.outgoing += 1;
                (a, BTreeSet::from([NeighbourDirection::Outgoing]))
            }
            (None, None) => return Err(missing("posting cursor exceeds its descriptor")),
        };
        page.work.examined_postings += 1;
        advance(&mut p, &streams)?;
        if !q.categories.contains(&posting.category) {
            continue;
        }
        if q.fact_scope == FactScope::Selected
            && (Tree { objects: &objects }).get(
                &view.selected,
                "selection/1",
                &identity_key(&posting.origin.owner),
            )? != Some(value!(posting.origin.owner))
        {
            return Err(missing("unselected origin leaked into selected facts"));
        }
        let opposite = vertex(&mut records, &posting.opposite)?;
        if !opposite.available {
            return Err(missing("posting target is unavailable in this view"));
        }
        if let Some(t) = &q.opposite_type {
            if !type_match(
                &mut records,
                opposite.type_ref.as_ref().unwrap(),
                t,
                q.include_subtypes,
            )? {
                continue;
            }
        }
        let neighbour = Neighbour {
            subject: s.subject.clone(),
            selector: s.selector.clone(),
            opposite: posting.opposite,
            origin: posting.origin,
            category: posting.category,
            directions,
        };
        verify_fact(&mut records, &neighbour, ontology.as_ref(), &mut known)?;
        let mut extra = json(&neighbour)?.len() + 1;
        if !vertices.contains_key(&opposite.reference) {
            extra += json(&opposite)?.len() + 1;
        }
        let mut additional = Vec::new();
        if q.projection == NeighbourProjection::Records {
            for r in [&neighbour.opposite, &neighbour.origin.owner] {
                if !projected.contains_key(r)
                    && !additional.iter().any(|v: &Record| v.reference == *r)
                {
                    let record = records.get(r)?;
                    extra += json(&record)?.len() + 1;
                    additional.push(record);
                }
            }
        }
        if bytes + extra > q.byte_limit {
            if page.neighbours.is_empty() {
                return Err(Error::Capacity);
            }
            p = before;
            page.reached = Some(NeighbourLimit::ProjectedBytes);
            break;
        }
        bytes += extra;
        vertices.insert(opposite.reference.clone(), opposite);
        for record in additional {
            projected.insert(record.reference.clone(), record);
        }
        page.neighbours.push(neighbour);
    }
    page.exhausted = p.group == streams.len();
    if page.exhausted {
        page.reached = None;
    }
    page.vertices = vertices.into_values().collect();
    page.records = projected.into_values().collect();
    page.work.record_reads = records.reads;
    page.work.record_bytes = records.bytes as u64;
    let work = c.work.snapshot();
    page.work.index_objects_read = work.object_reads;
    page.work.index_bytes_read = work.encoded_bytes_read;
    Ok(Slice {
        page,
        next: (p.group != streams.len()).then_some(p),
    })
}
