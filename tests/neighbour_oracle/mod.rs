//! Independent oracle: flatten actual tagged values/native JSON first, then resolve schema paths.
//! It does not import the production extractor, index structures or private storage port.
use knowledgehub::ontology::*;
use std::collections::{BTreeMap, BTreeSet};
type Raw = (
    ExactRef,
    NeighbourSelector,
    ExactRef,
    NeighbourOrigin,
    NeighbourCategory,
);
fn pointer(s: &str) -> String {
    s.replace('~', "~0").replace('/', "~1")
}
fn property(state: &State, t: &ExactRef, key: &str) -> Option<Property> {
    let t = state.type_definition(t)?;
    t.properties
        .get(key)
        .cloned()
        .or_else(|| t.parents.iter().find_map(|p| property(state, p, key)))
}
fn schema_path(
    k: &ValueKind,
    parts: &[String],
    prefix: String,
    state: &State,
    depth: usize,
) -> Option<String> {
    if depth > 64 {
        return None;
    }
    if parts.is_empty() {
        return Some(prefix);
    }
    match k {
        ValueKind::Any | ValueKind::Narrative => Some(prefix),
        ValueKind::Structured(n) => {
            schema_path(state.structures()?.get(n)?, parts, prefix, state, depth + 1)
        }
        ValueKind::Union(choices) => choices
            .iter()
            .find_map(|k| schema_path(k, parts, prefix.clone(), state, depth + 1)),
        ValueKind::Record(fields) => schema_path(
            &fields.get(&parts[0])?.value,
            &parts[1..],
            format!("{prefix}/{}", pointer(&parts[0])),
            state,
            depth + 1,
        ),
        ValueKind::List { item, .. } | ValueKind::Set { item, .. } => {
            schema_path(item, &parts[1..], format!("{prefix}/*"), state, depth + 1)
        }
        _ => None,
    }
}
fn values(v: &Value, path: Vec<String>, out: &mut Vec<(ExactRef, Vec<String>, NeighbourCategory)>) {
    match v {
        Value::Ref(r) => out.push((r.clone(), path, NeighbourCategory::Property)),
        Value::Artifact(r) => out.push((r.clone(), path, NeighbourCategory::Representation)),
        Value::Narrative { document, fragment } => {
            for (role, r) in [("document", document), ("fragment", fragment)] {
                let mut p = path.clone();
                p.push(role.into());
                out.push((r.clone(), p, NeighbourCategory::Representation));
            }
        }
        Value::List(a) | Value::Set(a) => {
            for (i, v) in a.iter().enumerate() {
                let mut p = path.clone();
                p.push(i.to_string());
                values(v, p, out);
            }
        }
        Value::Record(m) => {
            for (k, v) in m {
                let mut p = path.clone();
                p.push(k.clone());
                values(v, p, out);
            }
        }
        _ => {}
    }
}
fn native(
    v: &serde_json::Value,
    path: String,
    schema: String,
    out: &mut Vec<(ExactRef, String, String)>,
) {
    // Serde also accepts positional struct arrays; three role names are not a reference object.
    if let Ok(r) = if v.is_object() {
        serde_json::from_value::<ExactRef>(v.clone())
    } else {
        Err(serde::de::Error::custom("not a reference object"))
    } {
        out.push((r, path, schema));
        return;
    }
    match v {
        serde_json::Value::Array(a) => {
            for (i, v) in a.iter().enumerate() {
                native(v, format!("{path}/{i}"), format!("{schema}/*"), out);
            }
        }
        serde_json::Value::Object(m) => {
            for (k, v) in m {
                native(
                    v,
                    format!("{path}/{}", pointer(k)),
                    format!("{schema}/{}", pointer(k)),
                    out,
                );
            }
        }
        _ => {}
    }
}
fn facts(state: &State, scope: FactScope) -> Vec<Raw> {
    let mut out = Vec::new();
    let owners: Vec<&Record> = match scope {
        FactScope::Selected => state.selected_records().collect(),
        FactScope::Closure => state.records.values().collect(),
    };
    for r in owners {
        let origin = |path: String| NeighbourOrigin {
            owner: r.reference.clone(),
            owner_type: r.type_ref.clone(),
            occurrence: path,
        };
        let selector = |path: String| NeighbourSelector::Native {
            definition: r.type_ref.clone(),
            path,
        };
        out.push((
            r.reference.clone(),
            selector("/type_ref".into()),
            r.type_ref.clone(),
            origin("/type_ref".into()),
            NeighbourCategory::Administrative,
        ));
        if let Some(p) = &r.predecessor {
            out.push((
                r.reference.clone(),
                selector("/predecessor".into()),
                p.clone(),
                origin("/predecessor".into()),
                NeighbourCategory::Provenance,
            ));
        }
        for (i, p) in r.provenance.iter().enumerate() {
            out.push((
                r.reference.clone(),
                selector("/provenance/*".into()),
                p.clone(),
                origin(format!("/provenance/{i}")),
                NeighbourCategory::Provenance,
            ));
        }
        if let RecordData::Assertion(a) = &r.data {
            out.push((
                a.from.clone(),
                NeighbourSelector::Assertion {
                    definition: a.signature.clone(),
                },
                a.to.clone(),
                origin(String::new()),
                NeighbourCategory::Association,
            ));
        }
        let fields: Vec<(String, &Value, Property, ExactRef)> = match &r.data {
            RecordData::Element(e) => e
                .properties
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v,
                        property(state, &r.type_ref, k).unwrap(),
                        r.type_ref.clone(),
                    )
                })
                .collect(),
            RecordData::Assertion(a) => {
                let RecordData::Relationship(d) = &state.get(&a.signature).unwrap().data else {
                    unreachable!()
                };
                a.qualifiers
                    .iter()
                    .map(|(k, v)| {
                        (
                            format!("qualifiers/{k}"),
                            v,
                            d.qualifiers[k].clone(),
                            a.signature.clone(),
                        )
                    })
                    .collect()
            }
            _ => vec![],
        };
        for (key, v, p, definition) in fields {
            let prefix = if key.starts_with("qualifiers/") {
                format!(
                    "/qualifiers/{}",
                    pointer(key.strip_prefix("qualifiers/").unwrap())
                )
            } else {
                format!("/{}", pointer(&key))
            };
            let mut leaves = Vec::new();
            values(v, vec![], &mut leaves);
            for (target, path, category) in leaves {
                let schema = schema_path(&p.value, &path, prefix.clone(), state, 0).unwrap();
                let occurrence = format!(
                    "{prefix}{}",
                    path.iter()
                        .map(|p| format!("/{}", pointer(p)))
                        .collect::<String>()
                );
                out.push((
                    r.reference.clone(),
                    NeighbourSelector::Property {
                        definition: definition.clone(),
                        path: schema,
                    },
                    target,
                    origin(occurrence),
                    category,
                ));
            }
        }
        if matches!(r.data, RecordData::Element(_)) {
            continue;
        }
        let data = serde_json::to_value(&r.data).unwrap();
        let mut body = data["data"].clone();
        if let Some(m) = body.as_object_mut() {
            if matches!(r.data, RecordData::Baseline(_)) {
                m.remove("selected");
                m.remove("supporting");
            }
            if matches!(r.data, RecordData::Assertion(_)) {
                m.remove("qualifiers");
            }
        }
        let mut links = Vec::new();
        native(&body, String::new(), String::new(), &mut links);
        for (target, path, schema) in links {
            use NeighbourCategory::{Administrative as A, Provenance as P, Representation as R};
            let category = match &r.data {
                RecordData::SourceRevision(_) | RecordData::Review(_) => P,
                RecordData::Anchor(_) => {
                    if schema == "/locator/manifest" {
                        P
                    } else {
                        R
                    }
                }
                RecordData::Interpretation(_) => {
                    if schema == "/review" {
                        P
                    } else {
                        R
                    }
                }
                RecordData::Representation(_) => R,
                RecordData::Baseline(_) => {
                    if schema.starts_with("/parents/") || schema.contains("/provenance/") {
                        P
                    } else {
                        A
                    }
                }
                _ => A,
            };
            out.push((
                r.reference.clone(),
                selector(schema),
                target,
                origin(path),
                category,
            ));
        }
    }
    out
}
fn is_type(state: &State, actual: &ExactRef, wanted: &ExactRef, subtypes: bool) -> bool {
    let mut todo = vec![actual.clone()];
    let mut visited = BTreeSet::new();
    while let Some(r) = todo.pop() {
        if !visited.insert(r.clone()) {
            continue;
        }
        if r == *wanted {
            return true;
        }
        if subtypes {
            todo.extend(state.type_definition(&r).unwrap().parents.clone());
        }
    }
    false
}
pub fn neighbours(state: &State, q: &Neighbours) -> Vec<Neighbour> {
    let mut all = BTreeMap::new();
    for (from, selector, to, origin, category) in facts(state, q.fact_scope) {
        if !q.categories.contains(&category)
            || (!q.selectors.is_empty() && !q.selectors.contains(&selector))
        {
            continue;
        }
        for subject in q.starts.iter().collect::<BTreeSet<_>>() {
            let mut directions = BTreeSet::new();
            if q.direction != NeighbourDirection::Incoming && *subject == from {
                directions.insert(NeighbourDirection::Outgoing);
            }
            if q.direction != NeighbourDirection::Outgoing && *subject == to {
                directions.insert(NeighbourDirection::Incoming);
            }
            if directions.is_empty() {
                continue;
            }
            let opposite = if *subject == from {
                to.clone()
            } else {
                from.clone()
            };
            if let Some(t) = &q.opposite_type {
                if !is_type(
                    state,
                    &state.resolve(&opposite).unwrap().type_ref,
                    t,
                    q.include_subtypes,
                ) {
                    continue;
                }
            }
            let key = (
                subject.clone(),
                selector.clone(),
                opposite.clone(),
                origin.clone(),
                category,
            );
            all.insert(
                key,
                Neighbour {
                    subject: subject.clone(),
                    selector: selector.clone(),
                    opposite,
                    origin: origin.clone(),
                    category,
                    directions,
                },
            );
        }
    }
    all.into_values().collect()
}

/// Independent assertion graph BFS over the selected in-memory records.
/// No production traversal, extractor, neighbour API or index is called.
pub fn query(state: &State, q: &Query) -> (Vec<Record>, Vec<Record>, Option<String>, bool) {
    let mut frontier = q
        .starts
        .iter()
        .map(|r| (r.clone(), r.clone()))
        .collect::<Vec<_>>();
    let mut seen = BTreeSet::new();
    let mut owners = BTreeSet::new();
    let mut records = Vec::new();
    let mut assertions = Vec::new();
    let mut bounded = false;
    for depth in 0..=q.depth {
        frontier.sort();
        let mut active = Vec::new();
        for (start, subject) in frontier {
            if !seen.insert(subject.clone()) {
                continue;
            }
            let record = state.get(&subject).unwrap();
            if q.type_ref.as_ref().is_none_or(|t| *t == record.type_ref)
                && (q.ids.is_empty() || q.ids.contains(&record.reference.id))
            {
                records.push(record.clone());
            }
            if depth == q.depth {
                bounded = true;
            } else {
                active.push((start, subject));
            }
        }
        let mut next = Vec::new();
        for (start, subject) in active {
            let mut adjacent = state
                .selected_records()
                .filter_map(|record| {
                    let RecordData::Assertion(a) = &record.data else {
                        return None;
                    };
                    if !q.signatures.is_empty() && !q.signatures.contains(&a.signature) {
                        return None;
                    }
                    let (from, to) = match q.direction {
                        Direction::Forward => (&a.from, &a.to),
                        Direction::Reverse => (&a.to, &a.from),
                    };
                    (from == &subject).then_some((a.signature.clone(), to.clone(), record))
                })
                .collect::<Vec<_>>();
            adjacent
                .sort_by(|a, b| (&a.0, &a.1, &a.2.reference).cmp(&(&b.0, &b.1, &b.2.reference)));
            for (_, target, record) in adjacent {
                if owners.insert(record.reference.clone()) {
                    assertions.push(record.clone());
                }
                next.push((start.clone(), target));
            }
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }
    (
        records,
        assertions,
        bounded.then(|| "depth bound".into()),
        !bounded,
    )
}
