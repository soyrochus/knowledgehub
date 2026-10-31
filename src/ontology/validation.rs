use super::catalogue::{references, supports_role};
use super::types::pointer_segment;
use super::*;
use crate::{ContentRef, Error, Violation, codec};
use std::collections::{BTreeMap, BTreeSet};

fn issue(out: &mut Vec<Violation>, rule: &str, r: &ExactRef, path: &str, msg: &str) {
    out.push(Violation {
        rule: rule.into(),
        subject: r.key(),
        message: format!("{path}: {msg}"),
    })
}
fn nonempty(s: &str) -> bool {
    !s.trim().is_empty()
}
pub(crate) fn integer(s: &str) -> bool {
    let n = s.strip_prefix('-').unwrap_or(s);
    !n.is_empty()
        && n.bytes().all(|b| b.is_ascii_digit())
        && (n == "0" || !n.starts_with('0'))
        && s != "-0"
}
fn nonnegative(s: &str) -> bool {
    integer(s) && !s.starts_with('-')
}
pub(crate) fn timestamp(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() < 20 {
        return false;
    }
    let num = |a: usize, z: usize| -> Option<u32> {
        let v = b.get(a..z)?;
        if !v.iter().all(u8::is_ascii_digit) {
            return None;
        }
        std::str::from_utf8(v).ok()?.parse().ok()
    };
    if b.get(4) != Some(&b'-')
        || b.get(7) != Some(&b'-')
        || !matches!(b.get(10), Some(b'T' | b't'))
        || b.get(13) != Some(&b':')
        || b.get(16) != Some(&b':')
    {
        return false;
    }
    let (Some(y), Some(m), Some(d), Some(h), Some(mi), Some(se)) = (
        num(0, 4),
        num(5, 7),
        num(8, 10),
        num(11, 13),
        num(14, 16),
        num(17, 19),
    ) else {
        return false;
    };
    let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
    let days = match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if leap {
                29
            } else {
                28
            }
        }
        _ => return false,
    };
    if d == 0 || d > days || h > 23 || mi > 59 || se > 60 {
        return false;
    }
    let mut i = 19;
    if b.get(i) == Some(&b'.') {
        i += 1;
        let start = i;
        while b.get(i).is_some_and(u8::is_ascii_digit) {
            i += 1
        }
        if i == start {
            return false;
        }
    }
    if matches!(b.get(i), Some(b'Z' | b'z')) {
        return i + 1 == b.len();
    }
    if !matches!(b.get(i), Some(b'+' | b'-')) || i + 6 != b.len() || b.get(i + 3) != Some(&b':') {
        return false;
    }
    num(i + 1, i + 3).is_some_and(|n| n <= 23) && num(i + 4, i + 6).is_some_and(|n| n <= 59)
}
fn ordinal_int(a: &str, b: &str) -> std::cmp::Ordering {
    let (an, bn) = (a.starts_with('-'), b.starts_with('-'));
    match (an, bn) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => {
            let aa = a.trim_start_matches('-');
            let bb = b.trim_start_matches('-');
            let order = aa.len().cmp(&bb.len()).then_with(|| aa.cmp(bb));
            if an { order.reverse() } else { order }
        }
    }
}
fn text_range(s: &[u8], a: u64, b: u64) -> Option<&str> {
    if a >= b {
        return None;
    }
    std::str::from_utf8(s)
        .ok()?
        .get(usize::try_from(a).ok()?..usize::try_from(b).ok()?)
}

fn location_valid(kind: &LocatorKind, location: &SourceLocation) -> bool {
    match (kind, location) {
        (LocatorKind::Section, SourceLocation::Section { path }) => {
            !path.is_empty() && path.iter().all(|s| nonempty(s))
        }
        (LocatorKind::RequirementId, SourceLocation::RequirementId { id }) => nonempty(id),
        (LocatorKind::TableRow, SourceLocation::TableRow { table, row }) => {
            nonempty(table) && nonempty(row)
        }
        (LocatorKind::JsonPointer, SourceLocation::JsonPointer { pointer }) => {
            (pointer.is_empty() || pointer.starts_with('/'))
                && pointer
                    .split('~')
                    .skip(1)
                    .all(|s| s.starts_with('0') || s.starts_with('1'))
        }
        (LocatorKind::CodeSymbol, SourceLocation::CodeSymbol { language, symbol }) => {
            nonempty(language) && nonempty(symbol)
        }
        (LocatorKind::UiElement, SourceLocation::UiElement { screen, element }) => {
            nonempty(screen) && nonempty(element)
        }
        (
            LocatorKind::ImageRegion,
            SourceLocation::ImageRegion {
                x,
                y,
                width,
                height,
                image_width,
                image_height,
            },
        ) => {
            *width > 0
                && *height > 0
                && x.checked_add(*width).is_some_and(|end| end <= *image_width)
                && y.checked_add(*height)
                    .is_some_and(|end| end <= *image_height)
        }
        (
            LocatorKind::TimeRange,
            SourceLocation::TimeRange {
                start_ms,
                end_ms,
                duration_ms,
            },
        ) => start_ms < end_ms && end_ms <= duration_ms,
        (LocatorKind::PdfPage, SourceLocation::PdfPage { page, pages }) => {
            *page > 0 && page <= pages
        }
        _ => false,
    }
}
pub(crate) fn anchor_text(state: &State, r: &ExactRef) -> Option<String> {
    let RecordData::Anchor(a) = &state.get(r)?.data else {
        return None;
    };
    let original = state.artifact(&a.source)?;
    match &a.locator {
        Locator::ByteRange { start, end, quote } => {
            let t = text_range(&original.bytes, *start, *end)?;
            if t != quote || !nonempty(t) {
                return None;
            }
            Some(t.into())
        }
        Locator::Manifest {
            locator,
            manifest,
            entry,
            quote,
        } => {
            let m: LocatorManifest = codec::decode(&state.artifact(manifest)?.bytes).ok()?;
            if m.source != a.source
                || m.original != original.reference
                || !nonempty(&m.tool)
                || !nonempty(&m.version)
            {
                return None;
            }
            let e = m.entries.get(entry)?;
            if &e.kind != locator || !nonempty(&e.label) || !location_valid(locator, &e.location) {
                return None;
            }
            let RecordData::SourceRevision(derived) = &state.get(&e.derived)?.data else {
                return None;
            };
            if derived.derived_from.as_ref() != Some(&a.source)
                || derived.derivation.as_ref().is_none_or(|s| !nonempty(s))
            {
                return None;
            }
            let t = text_range(&state.artifact(&e.derived)?.bytes, e.start, e.end)?;
            if !nonempty(t) || quote.as_ref().is_some_and(|q| q != t) {
                return None;
            }
            Some(t.into())
        }
    }
}

fn value_valid(state: &State, v: &Value, k: &ValueKind, depth: usize) -> bool {
    if depth > 64 {
        return false;
    }
    match (v, k) {
        (Value::Text(t), ValueKind::Text) => nonempty(t),
        (Value::Bool(_), ValueKind::Bool) => true,
        (Value::Integer(n), ValueKind::Integer) => integer(n),
        (
            Value::Decimal {
                coefficient,
                exponent,
            },
            ValueKind::Decimal,
        ) => {
            integer(coefficient)
                && integer(exponent)
                && if coefficient == "0" {
                    exponent == "0"
                } else {
                    !coefficient.ends_with('0')
                }
        }
        (Value::Text(t), ValueKind::Timestamp) => timestamp(t),
        (Value::Enum(e), ValueKind::Enum(es)) => es.contains(e),
        (Value::Ref(r), ValueKind::Reference(types)) => supports_role(state, r, types),
        (Value::Artifact(r), ValueKind::Artifact) => state.kind(r) == Some("SourceRevision"),
        (Value::Narrative { document, fragment }, ValueKind::Narrative) => {
            state
                .get(fragment)
                .is_some_and(|r| matches!(&r.data,RecordData::Anchor(a) if a.source==*document))
                && anchor_text(state, fragment).is_some()
        }
        (Value::List(a), ValueKind::List { item, min }) => {
            a.len() >= *min && a.iter().all(|v| value_valid(state, v, item, depth + 1))
        }
        (Value::Set(a), ValueKind::Set { item, min }) => {
            a.len() >= *min
                && a.iter().all(|v| value_valid(state, v, item, depth + 1))
                && a.windows(2).all(|a| {
                    codec::canonical(&serde_json::to_value(&a[0]).unwrap()).unwrap()
                        < codec::canonical(&serde_json::to_value(&a[1]).unwrap()).unwrap()
                })
        }
        (Value::Record(a), ValueKind::Record(fields)) => {
            properties_valid(state, a, fields, depth + 1)
        }
        (_, ValueKind::Structured(name)) => state
            .structures()
            .and_then(|s| s.get(name))
            .is_some_and(|k| value_valid(state, v, k, depth + 1)),
        (_, ValueKind::Union(ks)) => ks.iter().any(|k| value_valid(state, v, k, depth + 1)),
        (_, ValueKind::Any) => match v {
            Value::Null => true,
            Value::Text(t) => nonempty(t),
            Value::Enum(t) => nonempty(t),
            Value::Bool(_) => true,
            Value::Integer(_) => value_valid(state, v, &ValueKind::Integer, depth + 1),
            Value::Decimal { .. } => value_valid(state, v, &ValueKind::Decimal, depth + 1),
            Value::Artifact(_) => value_valid(state, v, &ValueKind::Artifact, depth + 1),
            Value::Ref(_) => value_valid(
                state,
                v,
                &ValueKind::Reference(vec!["AnySubject".into()]),
                depth + 1,
            ),
            Value::Narrative { .. } => value_valid(state, v, &ValueKind::Narrative, depth + 1),
            Value::List(a) => a.iter().all(|v| value_valid(state, v, k, depth + 1)),
            Value::Set(_) => value_valid(
                state,
                v,
                &ValueKind::Set {
                    item: Box::new(ValueKind::Any),
                    min: 0,
                },
                depth + 1,
            ),
            Value::Record(a) => a.values().all(|v| value_valid(state, v, k, depth + 1)),
        },
        _ => false,
    }
}
fn properties_valid(
    state: &State,
    values: &BTreeMap<String, Value>,
    schema: &BTreeMap<String, Property>,
    depth: usize,
) -> bool {
    values.iter().all(|(n, v)| {
        schema.get(n).is_some_and(|p| {
            if *v == Value::Null {
                p.nullable
            } else {
                value_valid(state, v, &p.value, depth)
            }
        })
    }) && schema
        .iter()
        .all(|(n, p)| !p.required || values.contains_key(n))
}
fn fields_for(
    state: &State,
    t: &TypeDefinition,
    seen: &mut BTreeSet<String>,
) -> Option<BTreeMap<String, Property>> {
    if !seen.insert(t.name.clone()) {
        return None;
    }
    let mut fields = BTreeMap::new();
    for p in &t.parents {
        let t = state.type_definition(p)?;
        for (k, v) in fields_for(state, t, seen)? {
            if fields.insert(k, v).is_some() {
                return None;
            }
        }
    }
    for (k, v) in &t.properties {
        if fields.insert(k.clone(), v.clone()).is_some() {
            return None;
        }
    }
    seen.remove(&t.name);
    Some(fields)
}
fn schema_valid(state: &State, k: &ValueKind, depth: usize) -> bool {
    if depth > 64 {
        return false;
    }
    match k {
        ValueKind::Enum(v) => {
            !v.is_empty()
                && v.iter().all(|s| nonempty(s))
                && v.iter().collect::<BTreeSet<_>>().len() == v.len()
        }
        ValueKind::Reference(v) => {
            !v.is_empty()
                && v.iter().all(|n| {
                    matches!(
                        n.as_str(),
                        "AnySubject" | "AnyElementRevision" | "ElementRevision"
                    ) || state
                        .records
                        .values()
                        .any(|r| matches!(&r.data,RecordData::Type(t) if &t.name==n))
                })
        }
        ValueKind::List { item, min } | ValueKind::Set { item, min } => {
            *min <= 10000 && schema_valid(state, item, depth + 1)
        }
        ValueKind::Record(fields) => fields
            .iter()
            .all(|(n, p)| nonempty(n) && schema_valid(state, &p.value, depth + 1)),
        ValueKind::Structured(n) => state.structures().is_some_and(|s| s.contains_key(n)),
        ValueKind::Union(v) => !v.is_empty() && v.iter().all(|k| schema_valid(state, k, depth + 1)),
        _ => true,
    }
}
fn gap(
    state: &State,
    r: &ExactRef,
    path: &str,
    out: &mut Vec<Violation>,
    missing: &mut Vec<Gap>,
    msg: &str,
) {
    if let Some(g) = state
        .gaps
        .iter()
        .find(|g| g.subject == *r && g.path == path)
    {
        if state.profile_definition().is_some_and(|p| p.allow_gaps)
            && nonempty(&g.reason)
            && !g.provenance.is_empty()
            && g.provenance.iter().all(|p| state.get(p).is_some())
        {
            missing.push(g.clone());
            return;
        }
    }
    issue(out, "V-17", r, path, msg)
}
fn refs(v: Option<&Value>) -> Vec<ExactRef> {
    match v {
        Some(Value::Ref(r)) => vec![r.clone()],
        Some(Value::List(a) | Value::Set(a)) => {
            a.iter().filter_map(Value::as_ref).cloned().collect()
        }
        _ => vec![],
    }
}
fn prop<'a>(state: &'a State, r: &ExactRef, p: &str) -> Option<&'a Value> {
    state.get(r)?.element()?.get(p)
}
fn outgoing<'a>(state: &'a State, r: &ExactRef, verb: &str) -> Vec<&'a AssertionData> {
    state
        .selected_records()
        .filter_map(|e| {
            if let RecordData::Assertion(a) = &e.data {
                let d = state.get(&a.signature)?;
                if matches!(&d.data,RecordData::Relationship(d) if d.name==verb) && a.from == *r {
                    Some(a)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}
fn cycle(graph: &BTreeMap<ExactRef, Vec<ExactRef>>) -> bool {
    fn visit(
        n: &ExactRef,
        g: &BTreeMap<ExactRef, Vec<ExactRef>>,
        open: &mut BTreeSet<ExactRef>,
        done: &mut BTreeSet<ExactRef>,
    ) -> bool {
        if done.contains(n) {
            return false;
        }
        if !open.insert(n.clone()) {
            return true;
        }
        if g.get(n)
            .is_some_and(|v| v.iter().any(|v| visit(v, g, open, done)))
        {
            return true;
        }
        open.remove(n);
        done.insert(n.clone());
        false
    }
    let mut done = BTreeSet::new();
    graph
        .keys()
        .any(|n| visit(n, graph, &mut BTreeSet::new(), &mut done))
}
fn native_valid(state: &State, r: &Record, out: &mut Vec<Violation>) {
    let p = &r.reference;
    let need = |k: &ExactRef, names: &[&str]| {
        supports_role(
            state,
            k,
            &names.iter().map(|s| (*s).into()).collect::<Vec<_>>(),
        )
    };
    let valid = match &r.data {
        RecordData::Hub {
            owner,
            participants,
            policy_references,
        } => {
            owner == &state.scope.owner
                && !participants.is_empty()
                && participants.iter().all(|s| nonempty(s))
                && policy_references.iter().all(|s| nonempty(s))
        }
        RecordData::Model(m) => {
            m.owner == state.scope.owner
                && nonempty(&m.purpose)
                && need(&m.ontology, &["OntologyDefinition"])
                && need(&m.profile, &["ProfileDefinition"])
        }
        // Identity projections are derived by the resolver; admitting an editable copy would create a second owner.
        RecordData::Identity { .. } => false,
        RecordData::SourceArtifact(s) => {
            nonempty(&s.origin)
                && nonempty(&s.media_type)
                && nonempty(&s.owner)
                && [
                    "observation",
                    "hypothesis",
                    "authored-requirement",
                    "design-decision",
                    "implementation-observation",
                    "validation-result",
                ]
                .contains(&s.status.as_str())
        }
        RecordData::SourceRevision(s) => {
            need(&s.source, &["SourceArtifact"])
                && nonempty(&s.media_type)
                && state.artifact(p).is_some_and(|c| {
                    c.reference == s.original && ContentRef::of(&c.bytes) == s.original
                })
                && s.derived_from.as_ref().is_none_or(|r| {
                    need(r, &["SourceRevision"])
                        && s.derivation.as_ref().is_some_and(|s| nonempty(s))
                })
        }
        RecordData::Anchor(a) => {
            nonempty(&a.label)
                && need(&a.source, &["SourceRevision"])
                && anchor_text(state, p).is_some()
        }
        RecordData::Interpretation(i) => {
            need(&i.subject, &["AnySubject"])
                && need(&i.anchor, &["SourceAnchor"])
                && nonempty(&i.text)
                && i.interpreter.contains(':')
                && nonempty(&i.method)
                && i.review
                    .as_ref()
                    .is_none_or(|r| need(r, &["ReviewDecision"]))
        }
        RecordData::Representation(v) => {
            need(&v.subject, &["AnyElementRevision"]) && need(&v.anchor, &["SourceAnchor"])
        }
        RecordData::Review(v) => {
            v.assessor.contains(':')
                && !v.targets.is_empty()
                && v.targets.iter().all(|r| need(r, &["AnySubject"]))
                && nonempty(&v.rationale)
                && [
                    "accepted",
                    "conditional",
                    "rejected",
                    "proposed",
                    "approved",
                    "disputed",
                ]
                .contains(&v.decision.as_str())
                && timestamp(&v.assessed_at)
        }
        RecordData::Type(t) => {
            nonempty(&t.name)
                && (!KERNEL_KINDS.contains(&t.name.as_str()) || (t.native && r.reference==type_ref(&t.name)))
                && (!t.native || r.reference == type_ref(&t.name))
                && nonempty(&t.family)
                && !t.coverage.is_empty()
                && t.properties
                    .iter()
                    .all(|(n, p)| nonempty(n) && schema_valid(state, &p.value, 0))
                && t.parents
                    .iter()
                    .all(|p| need(p, &["ElementTypeDefinition"]))
                && t.constraints
                    .iter()
                    .all(|p| need(p, &["ConstraintDefinition"]))
                && fields_for(state, t, &mut BTreeSet::new()).is_some()
        }
        RecordData::Relationship(d) => {
            nonempty(&d.name)
                && !d.from.is_empty()
                && !d.to.is_empty()
                && schema_valid(state, &ValueKind::Reference(d.from.clone()), 0)
                && schema_valid(state, &ValueKind::Reference(d.to.clone()), 0)
                && d.max.is_none_or(|m| m >= d.min_complete)
                && d.qualifiers
                    .values()
                    .all(|p| schema_valid(state, &p.value, 0))
        }
        RecordData::Constraint(d) => nonempty(&d.name) && constraint_definition_valid(&d.rule, 0),
        RecordData::Profile(d) => {
            need(&d.ontology, &["OntologyDefinition"])
                && !d.vocabulary.is_empty()
                && state.get(&d.ontology).is_some_and(|r|matches!(&r.data,RecordData::Ontology(o) if d.vocabulary.iter().all(|t|o.types.contains(t))))
                && d.vocabulary
                    .iter()
                    .all(|r| need(r, &["ElementTypeDefinition"]))
                && d.inapplicable
                    .iter()
                    .all(|(k, v)| nonempty(k) && nonempty(v))
        }
        RecordData::Ontology(d) => {
            nonempty(&d.name)
                && nonempty(&d.version)
                && !d.types.is_empty()
                && d.types.iter().map(ExactRef::identity).collect::<BTreeSet<_>>().len()==d.types.len()
                && d.relationships.iter().map(ExactRef::identity).collect::<BTreeSet<_>>().len()==d.relationships.len()
                && d.constraints.iter().map(ExactRef::identity).collect::<BTreeSet<_>>().len()==d.constraints.len()
                && d.profiles.iter().map(ExactRef::identity).collect::<BTreeSet<_>>().len()==d.profiles.len()
                && d.types.iter().all(|r| need(r, &["ElementTypeDefinition"]))
                && d.relationships
                    .iter()
                    .all(|r| need(r, &["RelationshipTypeDefinition"]))
                && d.constraints
                    .iter()
                    .all(|r| need(r, &["ConstraintDefinition"]))
                && d.profiles.iter().all(|r| need(r, &["ProfileDefinition"]))
                && d.structures.values().all(|s| schema_valid(state, s, 0))
        }
        RecordData::Baseline(b) => baseline_valid(state, b),
        RecordData::Assertion(_) | RecordData::Element(_) => true,
    };
    if !valid {
        issue(
            out,
            "V-02",
            p,
            "/data",
            "native kernel/schema contract does not resolve or conform",
        )
    }
    let citation = match &r.data {
        RecordData::Type(d) => Some(&d.citation),
        RecordData::Relationship(d) => Some(&d.citation),
        RecordData::Constraint(d) => Some(&d.citation),
        RecordData::Profile(d) => Some(&d.citation),
        RecordData::Ontology(d) => Some(&d.citation),
        _ => None,
    };
    if let Some(c) = citation {
        let cited=r.provenance.iter().any(|p|{
            let anchor=match state.get(p).map(|r|&r.data){Some(RecordData::Anchor(_))=>Some(p),Some(RecordData::Interpretation(i))=>Some(&i.anchor),_=>None};
            anchor.and_then(|a|state.get(a)).is_some_and(|r|matches!(&r.data,RecordData::Anchor(a) if state.artifact(&a.source).is_some_and(|original|original.reference==c.original)))
        });
        if !cited
            || !nonempty(&c.section)
            || !nonempty(&c.source_status)
            || c.refinement.as_ref().is_some_and(|s| !nonempty(s))
        {
            issue(
                out,
                "V-05",
                p,
                "/citation",
                "definition citation must bind its exact retained source and explicit claim/refinement status",
            );
        }
    }
    if let Some(kind) = r.data.native_kind() {
        if state
            .type_definition(&r.type_ref)
            .is_none_or(|t| !t.native || t.name != kind || r.type_ref != type_ref(kind))
        {
            issue(
                out,
                "V-02",
                p,
                "/type_ref",
                "native payload does not match its exact declared kind",
            )
        }
    }
}
fn constraint_definition_valid(c: &Constraint, d: usize) -> bool {
    if d > 64 {
        return false;
    }
    match c {
        Constraint::Builtin(n) => (1..=17).any(|i| *n == format!("V-{i:02}")),
        Constraint::All(cs) | Constraint::Any(cs) => {
            !cs.is_empty() && cs.iter().all(|c| constraint_definition_valid(c, d + 1))
        }
        Constraint::Required(v) => !v.is_empty() && v.iter().all(|s| nonempty(s)),
        Constraint::Unique { property } | Constraint::Acyclic { property } => nonempty(property),
        Constraint::SameOwner { property, owner } => nonempty(property) && nonempty(owner),
        Constraint::Cardinality { property, min, max } => {
            nonempty(property) && max.is_none_or(|m| m >= *min)
        }
        Constraint::Range { property, min, max } => {
            nonempty(property) && integer(min) && integer(max) && !ordinal_int(min, max).is_gt()
        }
    }
}
fn baseline_valid(state: &State, b: &BaselineData) -> bool {
    if state.kind(&b.ontology) != Some("OntologyDefinition")
        || state.kind(&b.profile) != Some("ProfileDefinition")
    {
        return false;
    }
    let selected: BTreeMap<_, _> = b.selected.iter().map(|r| (r.identity(), r)).collect();
    if b.selected.is_empty() || selected.len() != b.selected.len() {
        return false;
    }
    let closure: BTreeSet<_> = b.selected.iter().chain(&b.supporting).cloned().collect();
    if closure.iter().any(|r| state.get(r).is_none())
        || !closure.contains(&b.ontology)
        || !closure.contains(&b.profile)
    {
        return false;
    }
    for r in &closure {
        let Some(record) = state.get(r) else {
            return false;
        };
        if references(record).iter().any(|r| {
            !closure.contains(r)
                && !(r.revision == "@identity"
                    && closure.iter().any(|v| v.identity() == r.identity()))
        }) {
            return false;
        }
    }
    b.parents
        .iter()
        .all(|p| state.kind(p) == Some("ModelBaseline"))
        && state
            .get(&b.profile)
            .is_some_and(|p| matches!(&p.data,RecordData::Profile(p) if p.ontology==b.ontology))
}
fn selector_exists(state: &State, subject: &ExactRef, path: &str) -> bool {
    let Some(record) = state.get(subject) else {
        return false;
    };
    let field = path.strip_prefix('/').unwrap_or(path);
    if state.kind(subject) == Some("DataSchema")
        && refs(record.element().and_then(|p| p.get("fields")))
            .iter()
            .any(|r| prop(state, r, "name").and_then(Value::as_text) == Some(field))
    {
        return true;
    }
    if matches!(&record.data,RecordData::Type(t) if t.properties.contains_key(field)) {
        return true;
    }
    let parts = field
        .split('/')
        .map(|p| p.replace("~1", "/").replace("~0", "~"))
        .collect::<Vec<_>>();
    if parts.len() > 64 {
        return false;
    }
    if let Some(properties) = record.element() {
        let Some(mut value) = properties.get(&parts[0]) else {
            return false;
        };
        for part in &parts[1..] {
            value = match value {
                Value::Record(m) => match m.get(part) {
                    Some(v) => v,
                    None => return false,
                },
                Value::List(v) | Value::Set(v) => {
                    match part.parse::<usize>().ok().and_then(|i| v.get(i)) {
                        Some(v) => v,
                        None => return false,
                    }
                }
                _ => return false,
            };
        }
        return true;
    }
    let Ok(mut payload) = serde_json::to_value(&record.data) else {
        return false;
    };
    let Some(mut value) = payload.get_mut("data") else {
        return false;
    };
    for part in parts {
        value = match value {
            serde_json::Value::Object(m) => match m.get_mut(&part) {
                Some(v) => v,
                None => return false,
            },
            serde_json::Value::Array(v) => {
                match part.parse::<usize>().ok().and_then(|i| v.get_mut(i)) {
                    Some(v) => v,
                    None => return false,
                }
            }
            _ => return false,
        };
    }
    true
}
fn datatype_matches(kind: &str, value: &Value) -> bool {
    matches!(
        (kind, value),
        ("text", Value::Text(_))
            | ("boolean", Value::Bool(_))
            | ("integer", Value::Integer(_))
            | ("decimal", Value::Decimal { .. })
            | ("enumeration", Value::Enum(_))
            | ("reference", Value::Ref(_))
            | ("record", Value::Record(_))
            | ("list", Value::List(_))
            | ("set", Value::Set(_))
            | ("binary", Value::Artifact(_))
    )
}
fn semantic_constraints(
    state: &State,
    record: &Record,
    out: &mut Vec<Violation>,
    missing: &mut Vec<Gap>,
) {
    let Some(p) = record.element() else { return };
    let r = &record.reference;
    let kind = state.kind(r).unwrap_or("");
    let owns = |target: &ExactRef, owner: &str| refs(prop(state, target, owner)).contains(r);
    match kind {
        "AggregateDefinition" => {
            let roots = outgoing(state, r, "hasRoot");
            if roots.len() > 1 {
                issue(out, "V-08", r, "/hasRoot", "more than one aggregate root")
            };
            if roots.is_empty() {
                gap(
                    state,
                    r,
                    "/hasRoot",
                    out,
                    missing,
                    "complete aggregate requires one root",
                )
            };
            let members = refs(p.get("members"));
            if roots.iter().any(|a| !members.contains(&a.to)) {
                issue(
                    out,
                    "V-08",
                    r,
                    "/members",
                    "root must belong to aggregate membership",
                )
            };
            if outgoing(state, r, "protects").is_empty() {
                gap(
                    state,
                    r,
                    "/protects",
                    out,
                    missing,
                    "complete aggregate declares its invariants",
                )
            }
        }
        "CommandDefinition" => {
            if outgoing(state, r, "handledBy").is_empty() {
                gap(
                    state,
                    r,
                    "/handledBy",
                    out,
                    missing,
                    "complete command requires its logical handler",
                )
            }
        }
        "Transition" => {
            let owner = refs(p.get("owner"));
            for field in ["from", "to"] {
                for s in refs(p.get(field)) {
                    if refs(prop(state, &s, "owner")) != owner {
                        issue(
                            out,
                            "V-10",
                            r,
                            field,
                            "state belongs to another lifecycle/process",
                        )
                    }
                }
            }
            if outgoing(state, r, "triggeredBy").is_empty() {
                gap(
                    state,
                    r,
                    "/triggeredBy",
                    out,
                    missing,
                    "complete transition requires a trigger",
                )
            }
        }
        "LifecycleDefinition" => {
            let states = refs(p.get("states"));
            for f in ["initial_state", "terminal_states"] {
                if refs(p.get(f)).iter().any(|s| !states.contains(s)) {
                    issue(
                        out,
                        "V-10",
                        r,
                        f,
                        "selected state is not a lifecycle member",
                    )
                }
            }
            for s in &states {
                if !owns(s, "owner") {
                    issue(
                        out,
                        "V-10",
                        r,
                        "/states",
                        "state owner differs from lifecycle",
                    )
                }
            }
            for s in refs(p.get("transitions")) {
                if !owns(&s, "owner") {
                    issue(
                        out,
                        "V-10",
                        r,
                        "/transitions",
                        "transition owner differs from lifecycle",
                    )
                }
            }
        }
        "ProcessDefinition" => {
            let steps = refs(p.get("steps"));
            if refs(p.get("initial_steps"))
                .iter()
                .any(|s| !steps.contains(s))
            {
                issue(
                    out,
                    "V-10",
                    r,
                    "/initial_steps",
                    "initial step absent from process",
                )
            };
            for s in &steps {
                if !owns(s, "process") {
                    issue(out, "V-10", r, "/steps", "step belongs to another process")
                }
            }
            let states = refs(p.get("states"));
            for s in &states {
                if !owns(s, "owner") {
                    issue(
                        out,
                        "V-10",
                        r,
                        "/states",
                        "state belongs to another process",
                    )
                }
            }
            let allowed = refs(p.get("allowed_loops"));
            let transitions = refs(p.get("transitions"));
            if allowed.iter().any(|t| !transitions.contains(t)) {
                issue(
                    out,
                    "V-10",
                    r,
                    "/allowed_loops",
                    "allowed transition is not a process member",
                )
            };
            let mut graph = BTreeMap::new();
            for t in transitions {
                if !owns(&t, "owner") {
                    issue(
                        out,
                        "V-10",
                        r,
                        "/transitions",
                        "transition belongs to another process",
                    )
                };
                if !allowed.contains(&t) {
                    if let (Some(a), Some(b)) = (
                        refs(prop(state, &t, "from")).first(),
                        refs(prop(state, &t, "to")).first(),
                    ) {
                        graph
                            .entry(a.clone())
                            .or_insert_with(Vec::new)
                            .push(b.clone())
                    }
                }
            }
            if cycle(&graph) {
                issue(
                    out,
                    "V-10",
                    r,
                    "/transitions",
                    "cycle lacks an explicit allowed-loop declaration",
                )
            }
        }
        "ProcessStep" => {
            let owner = refs(p.get("process"));
            for s in refs(p.get("next_steps")) {
                if refs(prop(state, &s, "process")) != owner {
                    issue(
                        out,
                        "V-10",
                        r,
                        "/next_steps",
                        "next step belongs to another process",
                    )
                }
            }
        }
        "EventHandler" => {
            for a in outgoing(state, r, "consumes") {
                if refs(prop(state, &a.to, "schema")) != refs(p.get("input_schema")) {
                    issue(
                        out,
                        "V-11",
                        r,
                        "/input_schema",
                        "event and handler schema revisions differ",
                    )
                }
            }
        }
        "EventDefinition" => {
            if let Some(Value::Record(d)) = p.get("delivery") {
                if refs(d.get("schema")) != refs(p.get("schema")) {
                    issue(
                        out,
                        "V-11",
                        r,
                        "/delivery/schema",
                        "delivery schema differs from event schema",
                    );
                }
            }
            for handler in refs(p.get("consumers")) {
                if refs(prop(state, &handler, "input_schema")) != refs(p.get("schema")) {
                    issue(
                        out,
                        "V-11",
                        r,
                        "/consumers",
                        "consumer input differs from event schema",
                    );
                }
            }
        }
        "ProcessExecution" => {
            if let Some(def) = refs(p.get("definition")).first() {
                if let Some(Value::List(steps)) = p.get("admitted_steps") {
                    for step in steps {
                        if let Value::Record(step) = step {
                            if refs(step.get("step"))
                                .iter()
                                .any(|s| !refs(prop(state, def, "steps")).contains(s))
                            {
                                issue(
                                    out,
                                    "V-10",
                                    r,
                                    "/admitted_steps",
                                    "execution step is absent from exact process definition",
                                );
                            }
                        }
                    }
                }
            }
        }
        "JourneyStep" => {
            let owner = refs(p.get("journey"));
            for step in refs(p.get("next_steps"))
                .into_iter()
                .chain(refs(p.get("alternatives")))
            {
                if refs(prop(state, &step, "journey")) != owner {
                    issue(
                        out,
                        "V-10",
                        r,
                        "/next_steps",
                        "journey step belongs to another journey",
                    );
                }
            }
        }
        "StoryMap" => {
            let owner = refs(p.get("journey"));
            for a in refs(p.get("activities")) {
                if refs(prop(state, &a, "journey")) != owner {
                    issue(
                        out,
                        "V-10",
                        r,
                        "/activities",
                        "activity belongs to another journey",
                    );
                }
            }
            for cut in refs(p.get("cuts")) {
                if !owns(&cut, "map") {
                    issue(
                        out,
                        "V-10",
                        r,
                        "/cuts",
                        "outcome cut belongs to another map",
                    );
                }
            }
        }
        "BoundedContext" => {
            for term in refs(p.get("language")) {
                if !owns(&term, "context") {
                    issue(
                        out,
                        "V-07",
                        r,
                        "/language",
                        "term is owned by another context",
                    );
                }
            }
        }
        "DataSchema" => {
            let fields = refs(p.get("fields"));
            if refs(p.get("identity_fields"))
                .iter()
                .any(|v| !fields.contains(v))
            {
                issue(
                    out,
                    "V-12",
                    r,
                    "/identity_fields",
                    "identity field must belong to schema",
                )
            };
            for f in fields {
                if !owns(&f, "schema") {
                    issue(
                        out,
                        "V-12",
                        r,
                        "/fields",
                        "field declares another owner schema",
                    )
                }
            }
        }
        "TransformationDefinition" => {
            for f in ["hasInputContract", "hasOutputContract"] {
                if outgoing(state, r, f).is_empty() {
                    gap(
                        state,
                        r,
                        &format!("/{f}"),
                        out,
                        missing,
                        "complete transformation requires an exact contract",
                    )
                }
            }
        }
        "TransformationRun" => {
            if let Some(d) = refs(p.get("definition")).first() {
                let input = outgoing(state, d, "hasInputContract");
                let output = outgoing(state, d, "hasOutputContract");
                if input.is_empty() || output.is_empty() {
                    issue(
                        out,
                        "V-12",
                        r,
                        "/definition",
                        "transformation run requires resolved definition contracts",
                    )
                }
            }
        }
        "RunLimit" => {
            if !["calls", "duration_ms", "concurrency"].iter().any(|f| {
                p.get(*f)
                    .is_some_and(|v| matches!(v,Value::Integer(n) if nonnegative(n)))
            }) {
                gap(
                    state,
                    r,
                    "/limits",
                    out,
                    missing,
                    "complete run limit needs at least one finite applicable bound",
                )
            }
        }
        "ComponentPlacement" => {
            for a in outgoing(state, r, "contains") {
                if refs(p.get("screen")) != refs(prop(state, &a.to, "screen")) {
                    issue(
                        out,
                        "V-13",
                        r,
                        "/contains",
                        "nested placement belongs to another screen",
                    );
                }
            }
            let edits = outgoing(state, r, "edits");
            for a in outgoing(state, r, "uses") {
                let inputs = outgoing(state, &a.to, "hasInputContract");
                let outputs = outgoing(state, &a.to, "hasOutputContract");
                if !edits.is_empty()
                    && (!edits.iter().all(|e| inputs.iter().any(|i| i.to == e.to))
                        || outputs.is_empty())
                {
                    issue(
                        out,
                        "V-13",
                        r,
                        "/uses",
                        "transformation contracts differ from the edited schema",
                    );
                }
            }
            for a in outgoing(state, r, "issues") {
                let bindings = refs(p.get("bindings"));
                if !bindings
                    .iter()
                    .any(|b| refs(prop(state, b, "command")).contains(&a.to))
                {
                    gap(
                        state,
                        r,
                        "/bindings",
                        out,
                        missing,
                        "issued command needs an explicit action binding",
                    );
                }
            }
        }
        "DesignToken" => {
            if let (Some(datatype), Some(value)) = (refs(p.get("datatype")).first(), p.get("value"))
            {
                if let Some(Value::Enum(kind)) = prop(state, datatype, "value_kind") {
                    if !datatype_matches(kind, value) {
                        issue(
                            out,
                            "V-13",
                            r,
                            "/value",
                            "token value differs from its declared datatype",
                        );
                    }
                }
            }
        }
        "ScreenDefinition" => {
            for v in refs(p.get("states")) {
                if !owns(&v, "screen") {
                    issue(
                        out,
                        "V-13",
                        r,
                        "/states",
                        "view state belongs to another screen",
                    );
                }
            }
            for a in outgoing(state, r, "contains") {
                if !owns(&a.to, "screen") {
                    issue(
                        out,
                        "V-13",
                        r,
                        "/contains",
                        "placement belongs to another screen",
                    );
                }
            }
        }
        "DataBinding" => {
            let reads = outgoing(state, r, "reads");
            if reads.is_empty() {
                gap(
                    state,
                    r,
                    "/reads",
                    out,
                    missing,
                    "complete data binding requires an exact query",
                );
            }
            if let Some(Value::Record(selector)) = p.get("target") {
                if let Some(target) = refs(selector.get("subject")).first() {
                    if let Some(schema) = refs(prop(state, target, "schema")).first() {
                        for read in &reads {
                            if !refs(prop(state, &read.to, "result")).contains(schema)
                                && !p.contains_key("mapping")
                            {
                                issue(
                                    out,
                                    "V-13",
                                    r,
                                    "/target",
                                    "query result and target schema require an explicit mapping",
                                );
                            }
                        }
                    }
                }
            }
        }
        "ActionBinding" => {
            if let (Some(interaction), Some(command)) = (
                refs(p.get("interaction")).first(),
                refs(p.get("command")).first(),
            ) {
                if !outgoing(state, interaction, "issues")
                    .iter()
                    .any(|a| &a.to == command)
                {
                    issue(
                        out,
                        "V-13",
                        r,
                        "/command",
                        "interaction does not issue the bound exact command",
                    );
                }
                let fields = refs(prop(state, command, "input"))
                    .first()
                    .map(|s| refs(prop(state, s, "fields")))
                    .unwrap_or_default();
                if let Some(Value::List(params)) = p.get("parameters") {
                    for b in params {
                        if let Value::Record(b) = b {
                            if refs(b.get("parameter")).iter().any(|f| !fields.contains(f)) {
                                issue(
                                    out,
                                    "V-13",
                                    r,
                                    "/parameters",
                                    "bound parameter is outside command input schema",
                                );
                            }
                        }
                    }
                }
            }
        }
        "Module" => {
            if let Some(component) = refs(p.get("component")).first() {
                if !refs(prop(state, component, "modules")).contains(r) {
                    gap(
                        state,
                        r,
                        "/component/modules",
                        out,
                        missing,
                        "component does not select the owned module",
                    );
                }
            }
        }
        "TestRun" => {
            for field in ["build", "evidence"] {
                for artifact in p.get(field).into_iter().flat_map(Value::refs) {
                    if let Some(Record {
                        data: RecordData::SourceRevision(source),
                        ..
                    }) = state.get(artifact)
                    {
                        let permitted=state.get(&source.source).is_some_and(|r|matches!(&r.data,RecordData::SourceArtifact(s) if if field=="build" {["observation","implementation-observation"].contains(&s.status.as_str())}else{["observation","implementation-observation","validation-result"].contains(&s.status.as_str())}));
                        if !permitted {
                            issue(
                                out,
                                "V-15",
                                r,
                                &format!("/{field}"),
                                "actual run requires an observed build/evidence capture, not a planned definition",
                            );
                        }
                    }
                }
            }
            if let Some(b) = refs(p.get("baseline"))
                .first()
                .and_then(|b| state.baseline(b))
            {
                for a in outgoing(state, r, "executes") {
                    if !b.selected.contains(&a.to) {
                        issue(
                            out,
                            "V-15",
                            r,
                            "/executes",
                            "observed case must be selected in exact run baseline",
                        );
                    }
                }
            }
            if outgoing(state, r, "executes").len() != 1 {
                gap(
                    state,
                    r,
                    "/executes",
                    out,
                    missing,
                    "actual test run identifies exactly one exact test case",
                )
            }
        }
        "AcceptanceRecord" => {
            if !p
                .get("assessor")
                .and_then(Value::as_text)
                .is_some_and(|s| s.contains(':'))
            {
                issue(
                    out,
                    "V-16",
                    r,
                    "/assessor",
                    "qualified actual assessor required",
                )
            }
            if let Some(b) = refs(p.get("baseline"))
                .first()
                .and_then(|v| state.baseline(v))
            {
                if refs(p.get("assessed_scope"))
                    .iter()
                    .any(|r| !b.selected.contains(r))
                {
                    issue(
                        out,
                        "V-16",
                        r,
                        "/assessed_scope",
                        "assessment must identify exact selected baseline members",
                    );
                }
            }
        }
        _ => {}
    }
    // Validate reusable structured contracts wherever nested, independent of the enclosing family.
    fn structure(state: &State, r: &ExactRef, path: &str, v: &Value, out: &mut Vec<Violation>) {
        match v {
            Value::Record(m) => {
                if let (Some(Value::Integer(min)), Some(Value::Bool(unbounded))) =
                    (m.get("min"), m.get("unbounded"))
                {
                    let valid = nonnegative(min)
                        && if *unbounded {
                            !m.contains_key("max")
                        } else {
                            m.get("max").is_some_and(|v|matches!(v,Value::Integer(max) if nonnegative(max)&&!ordinal_int(min,max).is_gt()))
                        };
                    if !valid {
                        issue(out, "V-02", r, path, "invalid cardinality")
                    }
                }
                if let Some(Value::Enum(mode)) = m.get("applicability") {
                    let limits = [
                        "timeout_ms",
                        "active_jobs",
                        "waiting_jobs",
                        "max_results",
                        "max_depth",
                        "max_steps",
                    ];
                    let bounded = limits.iter().filter_map(|k| m.get(*k)).collect::<Vec<_>>();
                    if (mode == "bounded"
                        && (bounded.is_empty()
                            || bounded
                                .iter()
                                .any(|v| !matches!(v,Value::Integer(n) if nonnegative(n)))))
                        || (mode == "not-applicable"
                            && !matches!(m.get("reason"), Some(Value::Narrative { .. })))
                    {
                        issue(
                            out,
                            "V-09",
                            r,
                            path,
                            "limits require finite bounds or a reasoned non-applicability claim",
                        )
                    }
                }
                if let Some(Value::Enum(policy)) = m.get("policy") {
                    if policy == "mapped" && !matches!(m.get("mapping"), Some(Value::Ref(_))) {
                        issue(
                            out,
                            "V-11",
                            r,
                            path,
                            "mapped compatibility needs an exact mapping",
                        )
                    }
                }
                if let (Some(Value::Ref(parameter)), Some(value)) =
                    (m.get("parameter"), m.get("value"))
                {
                    if let Some(datatype) = refs(prop(state, parameter, "datatype")).first() {
                        if let Some(Value::Enum(kind)) = prop(state, datatype, "value_kind") {
                            let valid = match (kind.as_str(), value) {
                                ("text", Value::Text(_))
                                | ("boolean", Value::Bool(_))
                                | ("integer", Value::Integer(_))
                                | ("decimal", Value::Decimal { .. })
                                | ("enumeration", Value::Enum(_))
                                | ("reference", Value::Ref(_))
                                | ("record", Value::Record(_))
                                | ("list", Value::List(_))
                                | ("set", Value::Set(_))
                                | ("binary", Value::Artifact(_)) => true,
                                (_, Value::Null) => matches!(
                                    prop(state, parameter, "nullable"),
                                    Some(Value::Bool(true))
                                ),
                                _ => false,
                            };
                            if !valid {
                                issue(
                                    out,
                                    "V-12",
                                    r,
                                    path,
                                    "bound value differs from parameter datatype/nullability",
                                );
                            }
                        }
                    }
                }
                if let Some(Value::Ref(subject)) = m.get("subject") {
                    if let Some(Value::Text(field)) = m.get("path") {
                        if !selector_exists(state, subject, field) {
                            issue(
                                out,
                                "V-12",
                                r,
                                path,
                                "selector field does not resolve against the exact subject/schema",
                            );
                        }
                    }
                }
                for (k, v) in m {
                    structure(state, r, &format!("{path}/{k}"), v, out)
                }
            }
            Value::List(v) | Value::Set(v) => {
                for (i, v) in v.iter().enumerate() {
                    structure(state, r, &format!("{path}/{i}"), v, out)
                }
            }
            _ => {}
        }
    }
    for (k, v) in p {
        structure(state, r, &format!("/{k}"), v, out)
    }
}

fn constraint_check(
    state: &State,
    kind: &str,
    c: &Constraint,
    records: &[&Record],
    out: &mut Vec<Violation>,
) {
    match c {
        Constraint::Builtin(_) => {}
        Constraint::All(cs) => {
            for c in cs {
                constraint_check(state, kind, c, records, out)
            }
        }
        Constraint::Any(cs) => {
            let mut failures = vec![];
            for c in cs {
                let mut sub = vec![];
                constraint_check(state, kind, c, records, &mut sub);
                if sub.is_empty() {
                    return;
                }
                failures.extend(sub)
            }
            out.extend(failures)
        }
        Constraint::Unique { property } => {
            let mut seen = BTreeSet::new();
            for r in records {
                if let Some(v) = r.element().and_then(|p| p.get(property)) {
                    let key = codec::hash_value("constraint-value/1", v).unwrap();
                    if !seen.insert(key) {
                        issue(
                            out,
                            "V-02",
                            &r.reference,
                            property,
                            "declared unique value is duplicated",
                        )
                    }
                }
            }
        }
        Constraint::Acyclic { property } => {
            let g = records
                .iter()
                .map(|r| {
                    (
                        r.reference.clone(),
                        refs(r.element().and_then(|p| p.get(property))),
                    )
                })
                .collect();
            if cycle(&g) {
                for r in records {
                    issue(
                        out,
                        "V-03",
                        &r.reference,
                        property,
                        "declared acyclic property has a cycle",
                    )
                }
            }
        }
        _ => {
            for r in records {
                let p = r.element();
                let valid = match c {
                    Constraint::Required(names) => {
                        names.iter().all(|n| p.is_some_and(|p| p.contains_key(n)))
                    }
                    Constraint::SameOwner { property, owner } => {
                        let expected = refs(p.and_then(|p| p.get(owner)));
                        refs(p.and_then(|p| p.get(property)))
                            .iter()
                            .all(|v| refs(prop(state, v, owner)) == expected)
                    }
                    Constraint::Cardinality { property, min, max } => {
                        let len = match p.and_then(|p| p.get(property)) {
                            Some(Value::List(a) | Value::Set(a)) => a.len(),
                            Some(Value::Null) | None => 0,
                            Some(_) => 1,
                        };
                        len >= *min && max.is_none_or(|m| len <= m)
                    }
                    Constraint::Range { property, min, max } => p
                        .and_then(|p| p.get(property))
                        .and_then(Value::as_text)
                        .is_some_and(|n| {
                            integer(n)
                                && !ordinal_int(n, min).is_lt()
                                && !ordinal_int(n, max).is_gt()
                        }),
                    _ => true,
                };
                if !valid {
                    issue(
                        out,
                        "V-02",
                        &r.reference,
                        &match c {
                            Constraint::Required(names) => format!(
                                "/{}",
                                names
                                    .iter()
                                    .find(|n| p.is_none_or(|p| !p.contains_key(*n)))
                                    .cloned()
                                    .unwrap_or_default()
                            ),
                            Constraint::SameOwner { property, .. }
                            | Constraint::Cardinality { property, .. }
                            | Constraint::Range { property, .. } => format!("/{property}"),
                            _ => format!("/{kind}"),
                        },
                        "declared constraint failed",
                    )
                }
            }
        }
    }
}
/// Structural and purpose assessment never commits or executes modelled algorithms.
pub fn assess(state: &State) -> Assessment {
    assess_inner(state, true)
}
fn assess_inner(state: &State, check_baselines: bool) -> Assessment {
    let mut out = vec![];
    let mut gaps = vec![];
    if [
        &state.scope.hub,
        &state.scope.model,
        &state.scope.branch,
        &state.scope.owner,
    ]
    .iter()
    .any(|s| !nonempty(s) || s.contains('\0'))
    {
        out.push(Violation {
            rule: "V-01".into(),
            subject: "scope".into(),
            message: "nonempty qualified scope required".into(),
        })
    }
    if state.records.is_empty() {
        if state.ontology.is_some()
            || state.profile.is_some()
            || !state.selected.is_empty()
            || !state.artifacts.is_empty()
        {
            out.push(Violation {
                rule: "V-04".into(),
                subject: "scope".into(),
                message: "empty bootstrap contains unresolved selections".into(),
            })
        }
        return Assessment {
            violations: out,
            gaps,
        };
    }
    let active = state
        .ontology
        .as_ref()
        .and_then(|r| state.get(r))
        .and_then(|r| {
            if let RecordData::Ontology(o) = &r.data {
                Some(o)
            } else {
                None
            }
        });
    if active.is_none()
        || state
            .profile_definition()
            .is_none_or(|p| Some(&p.ontology) != state.ontology.as_ref())
    {
        out.push(Violation {
            rule: "V-04".into(),
            subject: "scope".into(),
            message: "exact ontology/profile selection required".into(),
        })
    }
    let mut identities = BTreeMap::new();
    static TRUSTED: std::sync::OnceLock<BTreeMap<String, Record>> = std::sync::OnceLock::new();
    let trusted = TRUSTED.get_or_init(|| {
        builtin_catalogue()
            .expect("embedded catalogue")
            .records
            .into_iter()
            .map(|r| (r.reference.key(), r))
            .collect()
    });
    for (key, r) in &state.records {
        if trusted.get(key).is_some_and(|expected| expected != r) {
            issue(
                &mut out,
                "V-02",
                &r.reference,
                "/data",
                "trusted catalogue token has different content",
            );
        }
        if check_baselines {
            if let RecordData::Baseline(b) = &r.data {
                let report = assess_inner(&state.view(b), false);
                if !report.is_valid() {
                    issue(
                        &mut out,
                        "V-04",
                        &r.reference,
                        "/selected",
                        "baseline closure does not form a coherent conforming selection",
                    );
                }
            }
        }
        if key != &r.reference.key()
            || !r.reference.is_valid()
            || r.reference.revision == "@identity"
        {
            issue(
                &mut out,
                "V-01",
                &r.reference,
                "/reference",
                "invalid identity/revision/key",
            )
        }
        if r.digest().ok().as_ref() != Some(&r.sha256) {
            issue(
                &mut out,
                "V-04",
                &r.reference,
                "/sha256",
                "immutable record digest mismatch",
            )
        }
        if let Some(t) = identities.insert(r.reference.identity(), &r.type_ref) {
            if t.identity() != r.type_ref.identity() {
                issue(
                    &mut out,
                    "V-01",
                    &r.reference,
                    "/type_ref",
                    "one identity cannot change declared type",
                )
            }
        }
        if let Some(p) = &r.predecessor {
            if p.identity() != r.reference.identity()
                || p == &r.reference
                || state
                    .get(p)
                    .is_none_or(|v| v.type_ref.identity() != r.type_ref.identity())
            {
                issue(
                    &mut out,
                    "V-04",
                    &r.reference,
                    "/predecessor",
                    "predecessor does not resolve to the same identity/type",
                )
            }
        }
        for reference in references(r) {
            if state.resolve(&reference).is_none() {
                issue(
                    &mut out,
                    "V-04",
                    &r.reference,
                    "/references",
                    &format!("unresolved exact reference {}", reference.key()),
                )
            }
        }
        let original = matches!(
            r.data,
            RecordData::SourceArtifact(_)
                | RecordData::SourceRevision(_)
                | RecordData::Anchor(_)
                | RecordData::Hub { .. }
        );
        if !original
            && (r.provenance.is_empty()
                || r.provenance
                    .iter()
                    .any(|p| !matches!(state.kind(p), Some("SourceAnchor" | "Interpretation"))))
        {
            issue(
                &mut out,
                "V-05",
                &r.reference,
                "/provenance",
                "exact source anchor or authored interpretation required",
            )
        }
        native_valid(state, r, &mut out);
        if let RecordData::Element(e) = &r.data {
            let Some(t) = state.type_definition(&r.type_ref) else {
                issue(
                    &mut out,
                    "V-02",
                    &r.reference,
                    "/type_ref",
                    "type definition missing",
                );
                continue;
            };
            if t.native {
                issue(
                    &mut out,
                    "V-02",
                    &r.reference,
                    "/data",
                    "native kind requires its governed native payload",
                );
                continue;
            }
            let Some(fields) = fields_for(state, t, &mut BTreeSet::new()) else {
                issue(
                    &mut out,
                    "V-02",
                    &r.reference,
                    "/type_ref",
                    "inheritance is cyclic or ambiguous",
                );
                continue;
            };
            let selected = state.selected.get(&r.reference.identity()) == Some(&r.reference);
            for (name, v) in &e.properties {
                match fields.get(name) {
                    Some(p) if *v == Value::Null && p.nullable => {}
                    Some(p) if *v != Value::Null && value_valid(state, v, &p.value, 0) => {}
                    Some(_) => issue(
                        &mut out,
                        "V-02",
                        &r.reference,
                        &format!("/{}", pointer_segment(name)),
                        "value violates pinned type/nullability/reference schema",
                    ),
                    None => issue(
                        &mut out,
                        "V-02",
                        &r.reference,
                        &format!("/{}", pointer_segment(name)),
                        "unknown normative property",
                    ),
                }
            }
            for (name, p) in &fields {
                if p.required && !e.properties.contains_key(name) {
                    if selected {
                        gap(
                            state,
                            &r.reference,
                            &format!("/{}", pointer_segment(name)),
                            &mut out,
                            &mut gaps,
                            "required property missing",
                        )
                    } else if !state.gaps.iter().any(|g| {
                        g.subject == r.reference && g.path == format!("/{}", pointer_segment(name))
                    }) {
                        issue(
                            &mut out,
                            "V-02",
                            &r.reference,
                            &format!("/{}", pointer_segment(name)),
                            "required property missing in historical record",
                        )
                    }
                }
            }
            let expected = ElementData::new(e.properties.clone()).owners;
            let mut owners = BTreeMap::new();
            for o in &e.owners {
                if owners.insert(&o.path, o.owner).is_some() {
                    issue(
                        &mut out,
                        "V-06",
                        &r.reference,
                        &o.path,
                        "competing normative owners",
                    )
                }
            }
            for o in &expected {
                if owners.get(&o.path) != Some(&o.owner) {
                    issue(
                        &mut out,
                        "V-06",
                        &r.reference,
                        &o.path,
                        "required field owner missing or inconsistent with representation",
                    )
                }
            }
            if owners.len() != expected.len() {
                issue(
                    &mut out,
                    "V-06",
                    &r.reference,
                    "/owners",
                    "unknown or duplicate owner path",
                )
            }
            for (name, p) in &fields {
                if let Some(v) = e.properties.get(name) {
                    if *v != Value::Null
                        && matches!(p.value, ValueKind::Narrative)
                        && p.owner != PropertyOwner::Document
                    {
                        issue(
                            &mut out,
                            "V-06",
                            &r.reference,
                            name,
                            "narrative schema requires exact document ownership",
                        )
                    }
                }
            }
            if selected
                && state
                    .profile_definition()
                    .is_some_and(|p| !p.vocabulary.contains(&r.type_ref))
            {
                issue(
                    &mut out,
                    "V-02",
                    &r.reference,
                    "/type_ref",
                    "type revision absent from selected profile",
                )
            }
        }
        if let RecordData::Assertion(a) = &r.data {
            let Some(d) = state.get(&a.signature).and_then(|r| {
                if let RecordData::Relationship(d) = &r.data {
                    Some(d)
                } else {
                    None
                }
            }) else {
                issue(
                    &mut out,
                    "V-03",
                    &r.reference,
                    "/signature",
                    "exact relationship declaration absent",
                );
                continue;
            };
            if d.name == "represents"
                && state.get(&a.from).is_some_and(
                    |r| matches!(&r.data,RecordData::Representation(b) if b.subject != a.to),
                )
            {
                issue(
                    &mut out,
                    "V-06",
                    &r.reference,
                    "/endpoints",
                    "representation payload and assertion must pin the same subject",
                );
            }
            if !supports_role(state, &a.from, &d.from) || !supports_role(state, &a.to, &d.to) {
                issue(
                    &mut out,
                    "V-03",
                    &r.reference,
                    "/endpoints",
                    "endpoint role/type or direction mismatch",
                )
            }
            if !properties_valid(state, &a.qualifiers, &d.qualifiers, 0) {
                issue(
                    &mut out,
                    "V-03",
                    &r.reference,
                    "/qualifiers",
                    "unknown or invalid typed qualifier",
                )
            }
            if state.selected.get(&r.reference.identity()) == Some(&r.reference) {
                for endpoint in [&a.from, &a.to] {
                    if endpoint.revision != "@identity"
                        && state.selected.get(&endpoint.identity()) != Some(endpoint)
                    {
                        issue(
                            &mut out,
                            "V-04",
                            &r.reference,
                            "/endpoints",
                            "selected assertion requires the selected exact endpoint revision",
                        )
                    }
                }
                if active.is_some_and(|o| !o.relationships.contains(&a.signature)) {
                    issue(
                        &mut out,
                        "V-03",
                        &r.reference,
                        "/signature",
                        "signature is not in selected exact ontology",
                    )
                }
            }
        }
    }
    for (k, r) in &state.selected {
        if &r.identity() != k || state.get(r).is_none() {
            issue(
                &mut out,
                "V-04",
                r,
                "/selected",
                "selected identity/revision does not resolve",
            )
        }
    }
    let lineage = state
        .records
        .values()
        .map(|r| (r.reference.clone(), r.predecessor.iter().cloned().collect()))
        .collect();
    if cycle(&lineage) {
        out.push(Violation {
            rule: "V-04".into(),
            subject: "lineage".into(),
            message: "revision lineage cycle".into(),
        })
    }
    for (key, c) in &state.artifacts {
        if ContentRef::of(&c.bytes) != c.reference
            || !state.records.get(key).is_some_and(
                |r| matches!(&r.data,RecordData::SourceRevision(s) if s.original==c.reference),
            )
        {
            out.push(Violation {
                rule: "V-05".into(),
                subject: key.clone(),
                message: "orphan artifact or original digest mismatch".into(),
            })
        }
    }
    for r in state.selected_records() {
        semantic_constraints(state, r, &mut out, &mut gaps);
    }
    if let Some(o) = active {
        let mut type_names = BTreeSet::new();
        for t in &o.types {
            if let Some(d) = state.type_definition(t) {
                if !type_names.insert(&d.name) {
                    issue(
                        &mut out,
                        "V-02",
                        t,
                        "/name",
                        "ontology selects ambiguous same-name type definitions",
                    )
                }
                let records = state
                    .selected_records()
                    .filter(|r| r.type_ref == *t)
                    .collect::<Vec<_>>();
                for c in &d.constraints {
                    if let Some(Record {
                        data: RecordData::Constraint(c),
                        ..
                    }) = state.get(c)
                    {
                        constraint_check(state, &d.name, &c.rule, &records, &mut out)
                    }
                }
            }
        }
        for signature in &o.relationships {
            let Some(Record {
                data: RecordData::Relationship(d),
                ..
            }) = state.get(signature)
            else {
                continue;
            };
            let mut graph: BTreeMap<ExactRef, Vec<ExactRef>> = BTreeMap::new();
            for r in state.selected_records() {
                if let RecordData::Assertion(a) = &r.data {
                    if &a.signature == signature {
                        graph.entry(a.from.clone()).or_default().push(a.to.clone());
                    }
                }
            }
            for (from, to) in &graph {
                if d.max.is_some_and(|m| to.len() > m) {
                    issue(
                        &mut out,
                        "V-03",
                        from,
                        "/relationships",
                        "outgoing multiplicity exceeded",
                    )
                }
            }
            if d.acyclic && cycle(&graph) {
                issue(
                    &mut out,
                    "V-03",
                    signature,
                    "/relationships",
                    "declared acyclic relationship has a containment cycle",
                )
            }
        }
    }
    if let Some(p) = state.profile_definition() {
        for t in &p.required_types {
            if !state
                .selected_records()
                .any(|r| state.kind(&r.reference) == Some(t))
            {
                out.push(Violation {
                    rule: "V-17".into(),
                    subject: p.name.clone(),
                    message: format!("required self-model group/type absent: {t}"),
                })
            }
        }
    }
    for g in &state.gaps {
        if !gaps.contains(g) {
            if !nonempty(&g.reason)
                || g.provenance.is_empty()
                || g.provenance.iter().any(|r| state.get(r).is_none())
                || state.get(&g.subject).is_none()
                || state.profile_definition().is_none_or(|p| !p.allow_gaps)
            {
                issue(
                    &mut out,
                    "V-17",
                    &g.subject,
                    &g.path,
                    "invalid or disallowed gap claim",
                )
            } else {
                if state.selected.get(&g.subject.identity()) == Some(&g.subject) {
                    gaps.push(g.clone());
                }
            }
        }
    }
    Assessment {
        violations: out,
        gaps,
    }
}

pub(crate) fn candidate(base: &State, change: &Change) -> Result<(State, Assessment), Error> {
    if base.scope != change.scope {
        return Err(Error::Incompatible("model scope differs".into()));
    }
    if change.expected_ontology != base.ontology || change.expected_profile != base.profile {
        return Err(Error::GuardConflict {
            id: "exact ontology/profile selection".into(),
        });
    }
    if !nonempty(&change.operation_id) || change.operations.is_empty() {
        return Err(Error::InvalidInput(
            "operation identity and ordered operations required".into(),
        ));
    }
    for g in &change.guards {
        if base.selected.get(&format!("{}\0{}", g.model, g.id)) != g.expected.as_ref() {
            return Err(Error::GuardConflict { id: g.id.clone() });
        }
    }
    let mut next = base.clone();
    let mut extra = vec![];
    for op in &change.operations {
        match op {
            Operation::Put(r) | Operation::Capture { record: r, .. } => {
                if next.records.contains_key(&r.reference.key()) {
                    issue(
                        &mut extra,
                        "V-04",
                        &r.reference,
                        "/reference",
                        "published token cannot be reused",
                    )
                }
                if let Some(old) = next.selected.get(&r.reference.identity()) {
                    if r.predecessor.as_ref() != Some(old) {
                        issue(
                            &mut extra,
                            "V-04",
                            &r.reference,
                            "/predecessor",
                            "successor must guard exact selected predecessor",
                        )
                    }
                } else if next
                    .records
                    .values()
                    .any(|old| old.reference.identity() == r.reference.identity())
                {
                    issue(
                        &mut extra,
                        "V-04",
                        &r.reference,
                        "/predecessor",
                        "retained identity already has a lineage; explicitly select its exact predecessor before revising",
                    );
                } else if r.predecessor.is_some() {
                    issue(
                        &mut extra,
                        "V-04",
                        &r.reference,
                        "/predecessor",
                        "new identity has no accepted predecessor",
                    )
                }
                next.records.insert(r.reference.key(), r.clone());
                next.selected
                    .insert(r.reference.identity(), r.reference.clone());
                if let Operation::Capture { content, .. } = op {
                    next.artifacts.insert(r.reference.key(), content.clone());
                }
            }
            Operation::Select(reference) => {
                if next.get(reference).is_none() {
                    return Err(Error::NotFound);
                }
                next.selected
                    .insert(reference.identity(), reference.clone());
            }
            Operation::Unselect(reference) => {
                if next.selected.get(&reference.identity()) != Some(reference) {
                    return Err(Error::GuardConflict {
                        id: reference.id.clone(),
                    });
                }
                next.selected.remove(&reference.identity());
            }
            Operation::SelectProfile { ontology, profile } => {
                next.ontology = Some(ontology.clone());
                next.profile = Some(profile.clone())
            }
            Operation::Gaps(g) => {
                let historical = next
                    .gaps
                    .iter()
                    .filter(|g| next.selected.get(&g.subject.identity()) != Some(&g.subject))
                    .cloned()
                    .collect::<Vec<_>>();
                next.gaps = g.clone();
                for old in historical {
                    if !next.gaps.contains(&old) {
                        next.gaps.push(old);
                    }
                }
            }
        }
    }
    let mut result = assess(&next);
    result.violations.extend(extra);
    Ok((next, result))
}
pub(crate) fn history_valid(before: &State, after: &State) -> bool {
    before.scope == after.scope
        && before
            .records
            .iter()
            .all(|(k, v)| after.records.get(k) == Some(v))
        && before
            .artifacts
            .iter()
            .all(|(k, v)| after.artifacts.get(k) == Some(v))
}

/// Coverage is assessed against exact built-in schemas, not a count of arbitrary labels.
pub fn catalogue_coverage(state: &State) -> Vec<String> {
    let Ok(builtin) = builtin_catalogue() else {
        return vec!["builtin catalogue encoding".into()];
    };
    let active = state
        .ontology
        .as_ref()
        .and_then(|r| state.get(r))
        .and_then(|r| {
            if let RecordData::Ontology(o) = &r.data {
                Some(o)
            } else {
                None
            }
        });
    builtin
        .records
        .iter()
        .filter(|r| {
            matches!(
                r.data,
                RecordData::Type(_)
                    | RecordData::Relationship(_)
                    | RecordData::Constraint(_)
                    | RecordData::Ontology(_)
                    | RecordData::Profile(_)
            )
        })
        .filter(|r| {
            state.get(&r.reference) != Some(*r)
                || match &r.data {
                    RecordData::Type(_) => active.is_none_or(|o| !o.types.contains(&r.reference)),
                    RecordData::Relationship(_) => {
                        active.is_none_or(|o| !o.relationships.contains(&r.reference))
                    }
                    RecordData::Constraint(_) => {
                        active.is_none_or(|o| !o.constraints.contains(&r.reference))
                    }
                    _ => false,
                }
        })
        .map(|r| r.reference.id.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn closed_constraint_primitives_have_independent_positive_and_negative_cases() {
        let mut state = State::empty(ModelScope {
            hub: "h".into(),
            model: "m".into(),
            branch: "b".into(),
            owner: "owner:test".into(),
        });
        let a = ExactRef::new("m", "a", "1");
        let child = ExactRef::new("m", "child", "1");
        let owner = ExactRef::new("m", "owner", "1");
        let values = BTreeMap::from([
            ("present".into(), Value::Bool(true)),
            ("number".into(), Value::Integer("2".into())),
            (
                "items".into(),
                Value::List(vec![Value::text("a"), Value::text("b")]),
            ),
            ("link".into(), Value::Ref(a.clone())),
            ("owner".into(), Value::Ref(owner)),
            (
                "members".into(),
                Value::List(vec![Value::Ref(child.clone())]),
            ),
        ]);
        let record = Record::new(
            a,
            type_ref("Requirement"),
            RecordData::Element(ElementData::new(values)),
            vec![],
        )
        .unwrap();
        let mut other = record.clone();
        other.reference = child.clone();
        if let RecordData::Element(e) = &mut other.data {
            e.properties.insert("owner".into(), Value::Ref(child));
        }
        other.seal().unwrap();
        state.records.insert(other.reference.key(), other.clone());
        let check = |rule: &Constraint, records: &[&Record]| {
            let mut violations = vec![];
            assert!(constraint_definition_valid(rule, 0));
            constraint_check(&state, "test", rule, records, &mut violations);
            violations
        };
        for rule in [
            Constraint::Required(vec!["missing".into()]),
            Constraint::Cardinality {
                property: "items".into(),
                min: 0,
                max: Some(1),
            },
            Constraint::Range {
                property: "number".into(),
                min: "0".into(),
                max: "1".into(),
            },
            Constraint::Acyclic {
                property: "link".into(),
            },
            Constraint::SameOwner {
                property: "members".into(),
                owner: "owner".into(),
            },
        ] {
            assert!(!check(&rule, &[&record]).is_empty(), "{rule:?}");
        }
        assert!(
            !check(
                &Constraint::Unique {
                    property: "present".into()
                },
                &[&record, &other]
            )
            .is_empty()
        );
        for rule in [
            Constraint::Required(vec!["present".into()]),
            Constraint::Cardinality {
                property: "items".into(),
                min: 2,
                max: Some(2),
            },
            Constraint::Range {
                property: "number".into(),
                min: "2".into(),
                max: "99999999999999999999999999999".into(),
            },
            Constraint::All(vec![Constraint::Required(vec!["present".into()])]),
            Constraint::Any(vec![
                Constraint::Required(vec!["missing".into()]),
                Constraint::Required(vec!["present".into()]),
            ]),
        ] {
            assert!(check(&rule, &[&record]).is_empty(), "{rule:?}");
        }
        assert!(!constraint_definition_valid(
            &Constraint::Builtin("execute-script".into()),
            0
        ));
        assert!(!constraint_definition_valid(
            &Constraint::Cardinality {
                property: "items".into(),
                min: 2,
                max: Some(1)
            },
            0
        ));
    }
}

#[cfg(test)]
mod locator_tests {
    use super::*;
    #[test]
    fn original_coordinates_are_typed_bounded_and_independent_of_derived_text() {
        assert!(location_valid(
            &LocatorKind::TimeRange,
            &SourceLocation::TimeRange {
                start_ms: 3000,
                end_ms: 6000,
                duration_ms: 9000
            }
        ));
        assert!(!location_valid(
            &LocatorKind::TimeRange,
            &SourceLocation::TimeRange {
                start_ms: 6000,
                end_ms: 3000,
                duration_ms: 9000
            }
        ));
        assert!(!location_valid(
            &LocatorKind::TimeRange,
            &SourceLocation::TimeRange {
                start_ms: 3000,
                end_ms: 10000,
                duration_ms: 9000
            }
        ));
        assert!(!location_valid(
            &LocatorKind::ImageRegion,
            &SourceLocation::ImageRegion {
                x: 80,
                y: 0,
                width: 64,
                height: 64,
                image_width: 128,
                image_height: 128
            }
        ));
        assert!(!location_valid(
            &LocatorKind::PdfPage,
            &SourceLocation::PdfPage { page: 0, pages: 26 }
        ));
        assert!(!location_valid(
            &LocatorKind::PdfPage,
            &SourceLocation::PdfPage {
                page: 27,
                pages: 26
            }
        ));
        assert!(!location_valid(
            &LocatorKind::PdfPage,
            &SourceLocation::RequirementId {
                id: "different role".into()
            }
        ));
        assert!(location_valid(
            &LocatorKind::JsonPointer,
            &SourceLocation::JsonPointer {
                pointer: "/a~1b/c~0d".into()
            }
        ));
        assert!(!location_valid(
            &LocatorKind::JsonPointer,
            &SourceLocation::JsonPointer {
                pointer: "/a~2b".into()
            }
        ));
    }
}
