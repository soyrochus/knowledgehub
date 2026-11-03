use super::*;
use crate::{Error, codec, ontology::*};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FieldRef {
    pub record: ExactRef,
    pub field: String,
}
impl FieldRef {
    pub(crate) fn valid(&self) -> bool {
        self.record.is_valid() && self.field.starts_with('/') && self.field.len() <= 4096
    }
}

pub(crate) fn grams(mode: LiteralMode, text: &str) -> Result<BTreeSet<String>, Error> {
    let mut result = BTreeSet::new();
    for g in mode.bytes(text).windows(3) {
        result.insert(format!("{:02x}{:02x}{:02x}", g[0], g[1], g[2]));
        if result.len() > 100000 {
            return Err(Error::IndexNotReady {
                reason: "text distinct-gram preparation budget".into(),
            });
        }
    }
    Ok(result)
}

pub(crate) fn fields(
    t: &ExactRef,
    get: &mut dyn FnMut(&ExactRef) -> Result<Record, Error>,
    _seen: &mut BTreeSet<ExactRef>,
) -> Result<BTreeMap<String, Property>, Error> {
    Ok(
        crate::ontology::schema::declared_fields(t, |r| match get(r)?.data {
            RecordData::Type(t) => Ok(t),
            _ => Err(Error::InvalidInput("unknown exact type".into())),
        })?
        .into_iter()
        .map(|(n, (_, p))| (n, p))
        .collect(),
    )
}
pub(crate) fn validate_fields(
    fs: &BTreeSet<TextField>,
    ontology: &Record,
    get: &mut dyn FnMut(&ExactRef) -> Result<Record, Error>,
) -> Result<(), Error> {
    let RecordData::Ontology(o) = &ontology.data else {
        return Err(Error::Integrity("missing ontology".into()));
    };
    fn text_kind(
        k: &ValueKind,
        parts: &[String],
        s: &BTreeMap<String, ValueKind>,
        d: usize,
    ) -> bool {
        if d > 64 {
            return false;
        }
        match k {
            ValueKind::Structured(n) => s.get(n).is_some_and(|k| text_kind(k, parts, s, d + 1)),
            ValueKind::Union(ks) => ks.iter().any(|k| text_kind(k, parts, s, d + 1)),
            ValueKind::Text | ValueKind::Narrative => parts.is_empty(),
            ValueKind::Record(m) => parts
                .split_first()
                .is_some_and(|(p, r)| m.get(p).is_some_and(|k| text_kind(&k.value, r, s, d + 1))),
            ValueKind::List { item, .. } | ValueKind::Set { item, .. } => parts
                .split_first()
                .is_some_and(|(p, r)| p == "*" && text_kind(item, r, s, d + 1)),
            _ => false,
        }
    }
    for f in fs {
        if !o.types.contains(&f.type_ref) {
            return Err(Error::InvalidInput(
                "text type outside pinned ontology".into(),
            ));
        }
        let props = fields(&f.type_ref, get, &mut BTreeSet::new())?;
        let parts = parts(&f.path)?;
        if !parts.split_first().is_some_and(|(p, r)| {
            props
                .get(p)
                .is_some_and(|k| text_kind(&k.value, r, &o.structures, 0))
        }) {
            return Err(Error::InvalidInput(
                "field does not select declared Text/Narrative".into(),
            ));
        }
    }
    Ok(())
}
fn parts(path: &str) -> Result<Vec<String>, Error> {
    if !path.starts_with('/')
        || path
            .bytes()
            .enumerate()
            .any(|(i, b)| b == b'~' && !matches!(path.as_bytes().get(i + 1), Some(b'0' | b'1')))
    {
        return Err(Error::InvalidInput("invalid field pointer".into()));
    }
    Ok(path[1..]
        .split('/')
        .map(|p| p.replace("~1", "/").replace("~0", "~"))
        .collect())
}
pub(crate) fn values(
    record: &Record,
    requested: &BTreeSet<TextField>,
) -> Result<Vec<(String, Value)>, Error> {
    fn walk(v: &Value, parts: &[String], path: String, out: &mut Vec<(String, Value)>) {
        if parts.is_empty() {
            if matches!(v, Value::Text(_) | Value::Narrative { .. }) {
                out.push((path, v.clone()));
            }
            return;
        }
        let p = &parts[0];
        let rest = &parts[1..];
        match v {
            Value::Record(m) => {
                if let Some(v) = m.get(p) {
                    walk(
                        v,
                        rest,
                        format!("{path}/{}", p.replace('~', "~0").replace('/', "~1")),
                        out,
                    )
                }
            }
            Value::List(vs) | Value::Set(vs) if p == "*" => {
                for (i, v) in vs.iter().enumerate() {
                    walk(v, rest, format!("{path}/{i}"), out)
                }
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    if let RecordData::Element(e) = &record.data {
        let root = Value::Record(e.properties.clone());
        for f in requested.iter().filter(|f| f.type_ref == record.type_ref) {
            walk(&root, &parts(&f.path)?, String::new(), &mut out);
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out.dedup();
    Ok(out)
}
type ReadSpan<'a> = dyn FnMut(&ExactRef, u64, u64) -> Result<Vec<u8>, Error> + 'a;
pub(crate) fn fragment(
    value: &Value,
    get: &mut dyn FnMut(&ExactRef) -> Result<Record, Error>,
    bytes: &mut ReadSpan<'_>,
) -> Result<(String, Option<SourceSpan>), Error> {
    let Value::Narrative { document, fragment } = value else {
        return match value {
            Value::Text(t) => Ok((t.clone(), None)),
            _ => Err(Error::InvalidInput("nontext projection".into())),
        };
    };
    let RecordData::Anchor(a) = get(fragment)?.data else {
        return Err(Error::Integrity("narrative anchor".into()));
    };
    if a.source != *document {
        return Err(Error::Integrity("narrative document binding".into()));
    }
    let (span_document, start, end, location, quote) = match a.locator {
        Locator::ByteRange { start, end, quote } => {
            (document.clone(), start, end, None, Some(quote))
        }
        Locator::Manifest {
            manifest,
            entry,
            quote,
            ..
        } => {
            let RecordData::SourceRevision(m) = get(&manifest)?.data else {
                return Err(Error::Integrity("locator manifest capture".into()));
            };
            if m.original.length > 64 * 1024 * 1024 {
                return Err(Error::Capacity);
            }
            let m: LocatorManifest = codec::decode(&bytes(&manifest, 0, m.original.length)?)?;
            let e = m
                .entries
                .get(&entry)
                .ok_or_else(|| Error::Integrity("locator entry".into()))?;
            (
                e.derived.clone(),
                e.start,
                e.end,
                Some(e.location.clone()),
                quote,
            )
        }
    };
    let RecordData::SourceRevision(s) = get(&span_document)?.data else {
        return Err(Error::Integrity("narrative capture".into()));
    };
    let data = bytes(
        &span_document,
        start,
        end.checked_sub(start)
            .ok_or_else(|| Error::Integrity("inverted narrative span".into()))?,
    )?;
    let text = String::from_utf8(data).map_err(|_| Error::Integrity("narrative UTF-8".into()))?;
    if quote.as_ref().is_some_and(|q| q != &text) {
        return Err(Error::Integrity("narrative exact quotation".into()));
    }
    Ok((
        text,
        Some(SourceSpan {
            document: document.clone(),
            anchor: fragment.clone(),
            span_document,
            content: s.original,
            start,
            end,
            original_location: location,
        }),
    ))
}
