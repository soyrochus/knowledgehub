use super::*;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn selector_supported(
    selector: &NeighbourSelector,
    definition: &Record,
    ontology: Option<&Record>,
    lookup: &mut dyn FnMut(&ExactRef) -> Result<Record, Error>,
) -> Result<bool, Error> {
    match (selector, &definition.data) {
        (NeighbourSelector::Assertion { .. }, RecordData::Relationship(_)) => Ok(true),
        (NeighbourSelector::Native { path, .. }, RecordData::Type(t)) => {
            if ["/type_ref", "/predecessor", "/provenance/*"].contains(&path.as_str()) {
                return Ok(true);
            }
            // Exact Type selectors are native metadata references, including
            // nested property/qualifier/structure schemas. Roles contain no ref.
            fn value_schema(parts: &[&str]) -> bool {
                if parts == ["schema", "*", "selection"] {
                    return true;
                }
                if parts.len() > 2 && parts[0] == "schema" {
                    if parts[1] == "item" || parts[1] == "*" {
                        return value_schema(&parts[2..]);
                    }
                    if parts.len() > 3 && parts[2] == "value" {
                        return value_schema(&parts[3..]);
                    }
                }
                false
            }
            let parts = path.trim_start_matches('/').split('/').collect::<Vec<_>>();
            let selector_metadata = match t.name.as_str() {
                "RelationshipTypeDefinition" => {
                    matches!(path.as_str(), "/from/*/selection" | "/to/*/selection")
                        || (parts.len() > 3
                            && parts[0] == "qualifiers"
                            && parts[2] == "value"
                            && value_schema(&parts[3..]))
                }
                "ElementTypeDefinition" => {
                    parts.len() > 3
                        && parts[0] == "properties"
                        && parts[2] == "value"
                        && value_schema(&parts[3..])
                }
                "OntologyDefinition" => {
                    parts.len() > 2 && parts[0] == "structures" && value_schema(&parts[2..])
                }
                _ => false,
            };
            if t.native && selector_metadata {
                return Ok(true);
            }
            let paths: &[&str] = match t.name.as_str() {
                "SystemModel" => &["/ontology", "/profile"],
                "OntologyDefinition" => &[
                    "/types/*",
                    "/relationships/*",
                    "/constraints/*",
                    "/profiles/*",
                ],
                "ElementTypeDefinition" => &["/parents/*", "/constraints/*"],
                "ProfileDefinition" => &["/ontology", "/vocabulary/*"],
                "ElementIdentity" => &["/subject", "/declared_type"],
                "RelationshipAssertion" => &["/signature", "/from", "/to"],
                "SourceRevision" => &["/source", "/derived_from"],
                "SourceAnchor" => &["/source", "/locator/manifest"],
                "Interpretation" => &["/subject", "/anchor", "/review"],
                "RepresentationBinding" => &["/subject", "/anchor"],
                "ModelBaseline" => &[
                    "/ontology",
                    "/profile",
                    "/parents/*",
                    "/gaps/*/subject",
                    "/gaps/*/provenance/*",
                ],
                "ReviewDecision" => &["/targets/*"],
                _ => &[],
            };
            Ok(t.native && paths.contains(&path.as_str()))
        }
        (NeighbourSelector::Property { path, .. }, data) => {
            let mut parts = path
                .strip_prefix('/')
                .unwrap_or("")
                .split('/')
                .map(|s| s.replace("~1", "/").replace("~0", "~"))
                .collect::<Vec<_>>();
            let properties = match data {
                RecordData::Type(t) if !t.native => fields(&definition.reference, lookup)?,
                RecordData::Relationship(d) if parts.first().is_some_and(|p| p == "qualifiers") => {
                    parts.remove(0);
                    d.qualifiers.clone()
                }
                _ => return Ok(false),
            };
            let Some((first, rest)) = parts.split_first() else {
                return Ok(false);
            };
            let Some(p) = properties.get(first) else {
                return Ok(false);
            };
            let structures = match ontology.map(|r| &r.data) {
                Some(RecordData::Ontology(o)) => &o.structures,
                _ => return Ok(false),
            };
            fn contains(
                k: &ValueKind,
                path: &[String],
                structures: &BTreeMap<String, ValueKind>,
                depth: usize,
            ) -> bool {
                if depth > 64 {
                    return false;
                }
                if path.is_empty() {
                    return true;
                }
                match k {
                    ValueKind::Structured(n) => structures
                        .get(n)
                        .is_some_and(|k| contains(k, path, structures, depth + 1)),
                    ValueKind::Union(ks) => {
                        ks.iter().any(|k| contains(k, path, structures, depth + 1))
                    }
                    ValueKind::Record(m) => m
                        .get(&path[0])
                        .is_some_and(|p| contains(&p.value, &path[1..], structures, depth + 1)),
                    ValueKind::List { item, .. } | ValueKind::Set { item, .. } => {
                        path[0] == "*" && contains(item, &path[1..], structures, depth + 1)
                    }
                    _ => false,
                }
            }
            Ok(contains(&p.value, rest, structures, 0))
        }
        _ => Ok(false),
    }
}

/// The callback supplies exact schema records, never latest definitions or inferred identifiers.
pub(crate) fn extract(
    r: &Record,
    ontology: Option<&Record>,
    lookup: &mut dyn FnMut(&ExactRef) -> Result<Record, Error>,
) -> Result<Vec<Fact>, Error> {
    let mut facts = BTreeSet::new();
    let mut emit = |selector: NeighbourSelector,
                    to: ExactRef,
                    category: NeighbourCategory,
                    occurrence: String| {
        facts.insert(Fact {
            from: r.reference.clone(),
            selector,
            to,
            origin: NeighbourOrigin {
                owner: r.reference.clone(),
                owner_type: r.type_ref.clone(),
                occurrence,
            },
            category,
        });
    };
    let native = |path: &str| NeighbourSelector::Native {
        definition: r.type_ref.clone(),
        path: path.into(),
    };
    emit(
        native("/type_ref"),
        r.type_ref.clone(),
        NeighbourCategory::Administrative,
        "/type_ref".into(),
    );
    if let Some(p) = &r.predecessor {
        emit(
            native("/predecessor"),
            p.clone(),
            NeighbourCategory::Provenance,
            "/predecessor".into(),
        );
    }
    for (i, p) in r.provenance.iter().enumerate() {
        emit(
            native("/provenance/*"),
            p.clone(),
            NeighbourCategory::Provenance,
            format!("/provenance/{i}"),
        );
    }
    let structures = match ontology.map(|o| &o.data) {
        Some(RecordData::Ontology(o)) => o.structures.clone(),
        None => BTreeMap::new(),
        _ => {
            return Err(Error::Integrity(
                "index ontology is not a definition".into(),
            ));
        }
    };
    match &r.data {
        RecordData::Element(e) => {
            let fields = fields(&r.type_ref, lookup)?;
            for (k, v) in &e.properties {
                let p = fields.get(k).ok_or_else(|| {
                    Error::Integrity("reference owner schema field missing".into())
                })?;
                let path = format!("/{}", pointer_segment(k));
                walk(
                    v,
                    &p.value,
                    &r.type_ref,
                    &path,
                    &path,
                    &structures,
                    0,
                    &mut emit,
                )?;
            }
        }
        RecordData::Assertion(a) => {
            facts.insert(Fact {
                from: a.from.clone(),
                selector: NeighbourSelector::Assertion {
                    definition: a.signature.clone(),
                },
                to: a.to.clone(),
                origin: NeighbourOrigin {
                    owner: r.reference.clone(),
                    owner_type: r.type_ref.clone(),
                    occurrence: String::new(),
                },
                category: NeighbourCategory::Association,
            });
            // Assertion endpoints/signature are native administrative links, distinct from its model association.
            let refs = [
                ("/from", a.from.clone(), NeighbourCategory::Administrative),
                ("/to", a.to.clone(), NeighbourCategory::Administrative),
                (
                    "/signature",
                    a.signature.clone(),
                    NeighbourCategory::Administrative,
                ),
            ];
            for (path, to, category) in refs {
                facts.insert(Fact {
                    from: r.reference.clone(),
                    selector: native(path),
                    to,
                    origin: NeighbourOrigin {
                        owner: r.reference.clone(),
                        owner_type: r.type_ref.clone(),
                        occurrence: path.into(),
                    },
                    category,
                });
            }
            let signature = lookup(&a.signature)?;
            let RecordData::Relationship(d) = signature.data else {
                return Err(Error::Integrity(
                    "assertion schema definition missing".into(),
                ));
            };
            let mut emit = |selector, to, category, occurrence| {
                facts.insert(Fact {
                    from: r.reference.clone(),
                    selector,
                    to,
                    origin: NeighbourOrigin {
                        owner: r.reference.clone(),
                        owner_type: r.type_ref.clone(),
                        occurrence,
                    },
                    category,
                });
            };
            for (k, v) in &a.qualifiers {
                let p = d
                    .qualifiers
                    .get(k)
                    .ok_or_else(|| Error::Integrity("qualifier descriptor missing".into()))?;
                let path = format!("/qualifiers/{}", pointer_segment(k));
                walk(
                    v,
                    &p.value,
                    &a.signature,
                    &path,
                    &path,
                    &structures,
                    0,
                    &mut emit,
                )?;
            }
        }
        _ => {
            let mut links = Vec::new();
            let mut one = |path: &str, to: &ExactRef, c| {
                links.push((path.to_string(), path.to_string(), to.clone(), c))
            };
            use NeighbourCategory::{Administrative as A, Provenance as P, Representation as R};
            match &r.data {
                RecordData::Model(m) => {
                    one("/ontology", &m.ontology, A);
                    one("/profile", &m.profile, A);
                }
                RecordData::Profile(p) => {
                    one("/ontology", &p.ontology, A);
                    for (i, t) in p.vocabulary.iter().enumerate() {
                        links.push((
                            "/vocabulary/*".into(),
                            format!("/vocabulary/{i}"),
                            t.clone(),
                            A,
                        ));
                    }
                }
                RecordData::Type(_) | RecordData::Relationship(_) | RecordData::Ontology(_) => {
                    fn metadata(
                        v: &serde_json::Value,
                        path: String,
                        schema: String,
                        out: &mut Vec<(String, String, ExactRef, NeighbourCategory)>,
                    ) {
                        if v.is_object() {
                            if let Ok(r) = serde_json::from_value::<ExactRef>(v.clone()) {
                                out.push((schema, path, r, NeighbourCategory::Administrative));
                                return;
                            }
                        }
                        match v {
                            serde_json::Value::Object(m) => {
                                for (k, v) in m {
                                    let k = pointer_segment(k);
                                    metadata(
                                        v,
                                        format!("{path}/{k}"),
                                        format!("{schema}/{k}"),
                                        out,
                                    );
                                }
                            }
                            serde_json::Value::Array(a) => {
                                for (i, v) in a.iter().enumerate() {
                                    metadata(v, format!("{path}/{i}"), format!("{schema}/*"), out);
                                }
                            }
                            _ => {}
                        }
                    }
                    let data = serde_json::to_value(&r.data).map_err(crate::codec::encoding)?;
                    metadata(&data["data"], String::new(), String::new(), &mut links);
                }
                RecordData::Identity {
                    subject,
                    declared_type,
                } => {
                    one("/subject", subject, A);
                    one("/declared_type", declared_type, A);
                }
                RecordData::SourceRevision(s) => {
                    one("/source", &s.source, P);
                    if let Some(p) = &s.derived_from {
                        one("/derived_from", p, P);
                    }
                }
                RecordData::Anchor(a) => {
                    one("/source", &a.source, R);
                    if let Locator::Manifest { manifest, .. } = &a.locator {
                        one("/locator/manifest", manifest, P);
                    }
                }
                RecordData::Interpretation(i) => {
                    one("/subject", &i.subject, R);
                    one("/anchor", &i.anchor, R);
                    if let Some(review) = &i.review {
                        one("/review", review, P);
                    }
                }
                RecordData::Representation(b) => {
                    one("/subject", &b.subject, R);
                    one("/anchor", &b.anchor, R);
                }
                RecordData::Baseline(b) => {
                    one("/ontology", &b.ontology, A);
                    one("/profile", &b.profile, A);
                    for (i, p) in b.parents.iter().enumerate() {
                        links.push(("/parents/*".into(), format!("/parents/{i}"), p.clone(), P));
                    }
                    // Membership stays in baseline maps; it is not eagerly duplicated as adjacency.
                    for (i, g) in b.gaps.iter().enumerate() {
                        links.push((
                            "/gaps/*/subject".into(),
                            format!("/gaps/{i}/subject"),
                            g.subject.clone(),
                            A,
                        ));
                        for (j, p) in g.provenance.iter().enumerate() {
                            links.push((
                                "/gaps/*/provenance/*".into(),
                                format!("/gaps/{i}/provenance/{j}"),
                                p.clone(),
                                P,
                            ));
                        }
                    }
                }
                RecordData::Review(d) => {
                    for (i, t) in d.targets.iter().enumerate() {
                        links.push(("/targets/*".into(), format!("/targets/{i}"), t.clone(), P));
                    }
                }
                _ => {}
            }
            for (path, occurrence, target, category) in links {
                emit(native(&path), target, category, occurrence);
            }
        }
    }
    Ok(facts.into_iter().collect())
}

fn fields(
    reference: &ExactRef,
    lookup: &mut dyn FnMut(&ExactRef) -> Result<Record, Error>,
) -> Result<BTreeMap<String, Property>, Error> {
    Ok(
        crate::ontology::schema::declared_fields(reference, |r| match lookup(r)?.data {
            RecordData::Type(t) => Ok(t),
            _ => Err(Error::Integrity(
                "reference type declaration missing".into(),
            )),
        })?
        .into_iter()
        .map(|(n, (_, p))| (n, p))
        .collect(),
    )
}

#[allow(clippy::too_many_arguments)]
fn walk(
    v: &Value,
    k: &ValueKind,
    definition: &ExactRef,
    schema: &str,
    path: &str,
    structures: &BTreeMap<String, ValueKind>,
    depth: usize,
    emit: &mut dyn FnMut(NeighbourSelector, ExactRef, NeighbourCategory, String),
) -> Result<(), Error> {
    if depth > 64 {
        return Err(Error::Capacity);
    }
    let selector = || NeighbourSelector::Property {
        definition: definition.clone(),
        path: schema.into(),
    };
    match (v, k) {
        (_, ValueKind::Structured(name)) => walk(
            v,
            structures
                .get(name)
                .ok_or_else(|| Error::Integrity("structured reference schema missing".into()))?,
            definition,
            schema,
            path,
            structures,
            depth + 1,
            emit,
        )?,
        (_, ValueKind::Union(ks)) => {
            for k in ks {
                walk(v, k, definition, schema, path, structures, depth + 1, emit)?;
            }
        }
        (Value::Ref(r), ValueKind::Reference(_) | ValueKind::Any) => emit(
            selector(),
            r.clone(),
            NeighbourCategory::Property,
            path.into(),
        ),
        (Value::Artifact(r), ValueKind::Artifact | ValueKind::Any) => emit(
            selector(),
            r.clone(),
            NeighbourCategory::Representation,
            path.into(),
        ),
        (Value::Narrative { document, fragment }, ValueKind::Narrative | ValueKind::Any) => {
            emit(
                selector(),
                document.clone(),
                NeighbourCategory::Representation,
                format!("{path}/document"),
            );
            emit(
                selector(),
                fragment.clone(),
                NeighbourCategory::Representation,
                format!("{path}/fragment"),
            );
        }
        (Value::Record(values), ValueKind::Record(fields)) => {
            for (key, v) in values {
                if let Some(p) = fields.get(key) {
                    let segment = pointer_segment(key);
                    walk(
                        v,
                        &p.value,
                        definition,
                        &format!("{schema}/{segment}"),
                        &format!("{path}/{segment}"),
                        structures,
                        depth + 1,
                        emit,
                    )?;
                }
            }
        }
        (Value::List(values), ValueKind::List { item, .. })
        | (Value::Set(values), ValueKind::Set { item, .. }) => {
            for (i, v) in values.iter().enumerate() {
                walk(
                    v,
                    item,
                    definition,
                    &format!("{schema}/*"),
                    &format!("{path}/{i}"),
                    structures,
                    depth + 1,
                    emit,
                )?;
            }
        }
        (Value::Record(values), ValueKind::Any) => {
            for (key, v) in values {
                walk(
                    v,
                    k,
                    definition,
                    schema,
                    &format!("{path}/{}", pointer_segment(key)),
                    structures,
                    depth + 1,
                    emit,
                )?;
            }
        }
        (Value::List(values) | Value::Set(values), ValueKind::Any) => {
            for (i, v) in values.iter().enumerate() {
                walk(
                    v,
                    k,
                    definition,
                    schema,
                    &format!("{path}/{i}"),
                    structures,
                    depth + 1,
                    emit,
                )?;
            }
        }
        _ => {}
    }
    Ok(())
}
