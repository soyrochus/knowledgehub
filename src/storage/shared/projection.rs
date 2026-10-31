//! Lossless physical projections: semantic record/2 digests never change.
use super::*;
use super::{
    content::{self, Binding},
    objects::Objects,
    trees::{Tree, exact_key, identity_key, ref_key},
};
use crate::ontology::{
    self, AnchorData, BaselineData, ExactRef, Gap, Locator, LocatorManifest, Record, RecordData,
    State,
};
use serde_json::{Value, json as value};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoredRecord {
    schema: String,
    semantic_codec: String,
    reference: ExactRef,
    semantic_sha256: String,
    type_ref: ExactRef,
    predecessor: Option<ExactRef>,
    provenance: Vec<ExactRef>,
    data_oid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Data {
    schema: String,
    value: RecordData,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Baseline {
    pub schema: String,
    pub ontology: ExactRef,
    pub profile: ExactRef,
    pub selected_root: String,
    pub supporting_root: String,
    pub artifacts_root: String,
    pub selected_order: Option<String>,
    pub supporting_order: Option<String>,
    pub parents: Vec<ExactRef>,
    pub gaps_root: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoredAnchor {
    schema: String,
    source: ExactRef,
    label: String,
    locator: Value,
    quotation: Option<Quote>,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase", deny_unknown_fields)]
enum Quote {
    Literal {
        text: String,
    },
    Blob {
        content_oid: String,
        sha256: String,
        length: String,
    },
    Span {
        source_ref: ExactRef,
        content_oid: String,
        start: String,
        end: String,
        sha256: String,
    },
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoredGap {
    schema: String,
    value: Gap,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoredReceipt {
    schema: String,
    fields: Value,
    affected_root: String,
    qualified_affected_root: String,
}

pub(super) fn encode_record(
    objects: &Objects<'_>,
    r: &Record,
    state: &State,
    artifacts_root: &str,
) -> Result<String, Error> {
    if !r.reference.is_valid() || r.digest()? != r.sha256 {
        return Err(integrity("semantic record digest"));
    }
    let tree = Tree { objects };
    let data_oid = match &r.data {
        RecordData::Baseline(b) => {
            let selected_root = tree.map(
                "selection/1",
                b.selected.iter().map(|r| (identity_key(r), value!(r))),
            )?;
            let mut supporting = BTreeMap::new();
            for r in &b.supporting {
                supporting.insert(json(&ref_key(r))?, r);
            }
            let supporting_root = tree.map(
                "exact-reference-set/1",
                supporting.into_values().map(|r| (ref_key(r), Value::Null)),
            )?;
            let selected_order = order(&tree, &selected_root, "selection/1", &b.selected)?;
            let supporting_order = order(
                &tree,
                &supporting_root,
                "exact-reference-set/1",
                &b.supporting,
            )?;
            let gaps_root = put_gaps(objects, None, &b.gaps)?;
            objects.put_json(
                "baseline-data",
                &Baseline {
                    schema: "knowledgehub-baseline-data/1".into(),
                    ontology: b.ontology.clone(),
                    profile: b.profile.clone(),
                    selected_root,
                    supporting_root,
                    artifacts_root: artifacts_root.into(),
                    selected_order,
                    supporting_order,
                    parents: b.parents.clone(),
                    gaps_root,
                },
            )?
        }
        RecordData::Anchor(a) => {
            let mut locator = serde_json::to_value(&a.locator).map_err(codec::encoding)?;
            locator
                .as_object_mut()
                .ok_or_else(|| integrity("anchor locator"))?
                .remove("quote");
            let descriptor = match &a.locator {
                Locator::ByteRange { start, end, quote } => {
                    Some(span(objects, state, &a.source, *start, *end, quote)?)
                }
                Locator::Manifest {
                    manifest,
                    entry,
                    quote,
                    ..
                } => match quote {
                    None => None,
                    Some(quote) => {
                        let bytes = state
                            .artifact(manifest)
                            .ok_or_else(|| integrity("locator manifest source"))?;
                        let m: LocatorManifest = parse(&bytes.bytes)?;
                        let e = m
                            .entries
                            .get(entry)
                            .ok_or_else(|| integrity("locator manifest entry"))?;
                        Some(span(objects, state, &e.derived, e.start, e.end, quote)?)
                    }
                },
            };
            objects.put_json(
                "anchor-data",
                &StoredAnchor {
                    schema: "knowledgehub-anchor-data/1".into(),
                    source: a.source.clone(),
                    label: a.label.clone(),
                    locator,
                    quotation: descriptor,
                },
            )?
        }
        _ => objects.put_json(
            "record-data",
            &Data {
                schema: "knowledgehub-record-data/1".into(),
                value: r.data.clone(),
            },
        )?,
    };
    objects.put_json(
        "record",
        &StoredRecord {
            schema: "knowledgehub-stored-record/1".into(),
            semantic_codec: ontology::RECORD_FORMAT.into(),
            reference: r.reference.clone(),
            semantic_sha256: r.sha256.clone(),
            type_ref: r.type_ref.clone(),
            predecessor: r.predecessor.clone(),
            provenance: r.provenance.clone(),
            data_oid,
        },
    )
}

fn span(
    objects: &Objects<'_>,
    state: &State,
    source: &ExactRef,
    start: u64,
    end: u64,
    quote: &str,
) -> Result<Quote, Error> {
    let original = state
        .artifact(source)
        .ok_or_else(|| integrity("quote original bytes"))?;
    let text = std::str::from_utf8(&original.bytes).map_err(|_| integrity("quote UTF-8"))?;
    let start_index = usize::try_from(start).map_err(|_| Error::Capacity)?;
    let end_index = usize::try_from(end).map_err(|_| Error::Capacity)?;
    if text.get(start_index..end_index) != Some(quote) {
        return Err(integrity("exact quote differs from span"));
    }
    let binding = content::put(objects, original)?;
    Ok(Quote::Span {
        source_ref: source.clone(),
        content_oid: binding.content_oid,
        start: start.to_string(),
        end: end.to_string(),
        sha256: codec::digest(quote.as_bytes()),
    })
}

fn order(
    tree: &Tree<'_, '_>,
    root: &str,
    space: &str,
    refs: &[ExactRef],
) -> Result<Option<String>, Error> {
    let natural = members(tree, root, space)?;
    if natural == refs {
        Ok(None)
    } else {
        Ok(Some(tree.sequence(
            "exact-refs/1",
            &refs.iter().map(|r| value!(r)).collect::<Vec<_>>(),
        )?))
    }
}
fn members(tree: &Tree<'_, '_>, root: &str, space: &str) -> Result<Vec<ExactRef>, Error> {
    tree.entries(root, space)?
        .into_iter()
        .map(|(key, value)| {
            if space == "selection/1" {
                let r: ExactRef = serde_json::from_value(value)
                    .map_err(|_| integrity("selected exact reference"))?;
                if identity_key(&r) != key || !r.is_valid() {
                    return Err(integrity("selected identity differs"));
                }
                Ok(r)
            } else {
                if value != Value::Null {
                    return Err(integrity("reference set value"));
                }
                exact_key(&key)
            }
        })
        .collect()
}
fn ordered(
    tree: &Tree<'_, '_>,
    root: &str,
    space: &str,
    order: &Option<String>,
) -> Result<Vec<ExactRef>, Error> {
    let natural = members(tree, root, space)?;
    let Some(id) = order else {
        return Ok(natural);
    };
    let refs = tree
        .items(id, "exact-refs/1")?
        .into_iter()
        .map(|v| serde_json::from_value::<ExactRef>(v).map_err(|_| integrity("ordered reference")))
        .collect::<Result<Vec<_>, _>>()?;
    if natural.iter().collect::<BTreeSet<_>>() != refs.iter().collect::<BTreeSet<_>>()
        || refs.iter().any(|r| !r.is_valid())
    {
        return Err(integrity("ordered baseline membership differs"));
    }
    Ok(refs)
}

pub(super) fn decode_record(objects: &Objects<'_>, id: &str) -> Result<Record, Error> {
    let r: StoredRecord = objects.get_json(id, "record")?;
    if r.schema != "knowledgehub-stored-record/1" || r.semantic_codec != ontology::RECORD_FORMAT {
        return Err(Error::Incompatible("stored semantic record version".into()));
    }
    let data = objects.get(&r.data_oid)?;
    let tree = Tree { objects };
    let data = match data.kind.as_str() {
        "record-data" => {
            let d: Data = parse(&data.bytes)?;
            if d.schema != "knowledgehub-record-data/1" {
                return Err(integrity("record data schema"));
            }
            d.value
        }
        "baseline-data" => {
            let b: Baseline = parse(&data.bytes)?;
            if b.schema != "knowledgehub-baseline-data/1" {
                return Err(integrity("baseline data schema"));
            }
            RecordData::Baseline(BaselineData {
                ontology: b.ontology,
                profile: b.profile,
                selected: ordered(&tree, &b.selected_root, "selection/1", &b.selected_order)?,
                supporting: ordered(
                    &tree,
                    &b.supporting_root,
                    "exact-reference-set/1",
                    &b.supporting_order,
                )?,
                parents: b.parents,
                gaps: read_gaps(objects, &b.gaps_root)?,
            })
        }
        "anchor-data" => {
            let a: StoredAnchor = parse(&data.bytes)?;
            if a.schema != "knowledgehub-anchor-data/1" {
                return Err(integrity("anchor data schema"));
            }
            let quote = match a.quotation {
                None => None,
                Some(Quote::Literal { text }) => Some(text),
                Some(Quote::Blob {
                    content_oid,
                    sha256,
                    length,
                }) => {
                    let b = Binding {
                        content_oid,
                        reference: ContentRef {
                            sha256,
                            length: number(&length)?,
                        },
                    };
                    Some(
                        String::from_utf8(content::read(objects, &b)?.bytes)
                            .map_err(|_| integrity("literal quote UTF-8"))?,
                    )
                }
                Some(Quote::Span {
                    source_ref,
                    content_oid,
                    start,
                    end,
                    sha256,
                }) => {
                    if !source_ref.is_valid() {
                        return Err(integrity("quote source reference"));
                    }
                    let start = number(&start)?;
                    let end = number(&end)?;
                    if end < start {
                        return Err(integrity("quote range"));
                    }
                    let content = objects.get(&content_oid)?;
                    let reference = if content.kind == "blob" {
                        ContentRef::of(&content.bytes)
                    } else {
                        let m: Value = parse(&content.bytes)?;
                        ContentRef {
                            sha256: m["sha256"]
                                .as_str()
                                .ok_or_else(|| integrity("quote manifest digest"))?
                                .into(),
                            length: number(
                                m["length"]
                                    .as_str()
                                    .ok_or_else(|| integrity("quote manifest length"))?,
                            )?,
                        }
                    };
                    let bytes = content::range(
                        objects,
                        &Binding {
                            content_oid,
                            reference,
                        },
                        start,
                        end - start,
                    )?
                    .0;
                    if codec::digest(&bytes) != sha256 {
                        return Err(integrity("quote content hash"));
                    }
                    Some(String::from_utf8(bytes).map_err(|_| integrity("quote boundary UTF-8"))?)
                }
            };
            let mut locator = a.locator;
            locator
                .as_object_mut()
                .ok_or_else(|| integrity("stored locator"))?
                .insert(
                    "quote".into(),
                    quote.map(Value::String).unwrap_or(Value::Null),
                );
            RecordData::Anchor(AnchorData {
                source: a.source,
                label: a.label,
                locator: serde_json::from_value(locator)
                    .map_err(|_| integrity("anchor locator reconstruction"))?,
            })
        }
        _ => return Err(integrity("unsupported record data kind")),
    };
    let result = Record {
        reference: r.reference,
        type_ref: r.type_ref,
        predecessor: r.predecessor,
        provenance: r.provenance,
        data,
        sha256: r.semantic_sha256,
    };
    if !result.reference.is_valid() || result.digest()? != result.sha256 {
        return Err(integrity("reconstructed semantic digest differs"));
    }
    Ok(result)
}

pub(super) fn baseline(
    objects: &Objects<'_>,
    record_oid: &str,
    reference: &ExactRef,
) -> Result<Baseline, Error> {
    let r: StoredRecord = objects.get_json(record_oid, "record")?;
    if r.schema != "knowledgehub-stored-record/1"
        || r.semantic_codec != ontology::RECORD_FORMAT
        || r.reference != *reference
    {
        return Err(integrity("baseline record envelope"));
    }
    let b: Baseline = objects.get_json(&r.data_oid, "baseline-data")?;
    if b.schema != "knowledgehub-baseline-data/1" {
        return Err(integrity("baseline schema"));
    }
    Ok(b)
}

pub(super) fn put_gaps(
    objects: &Objects<'_>,
    previous: Option<&str>,
    gaps: &[Gap],
) -> Result<String, Error> {
    let ids = gaps
        .iter()
        .map(|g| {
            Ok(value!(objects.put_json(
                "gap",
                &StoredGap {
                    schema: "knowledgehub-gap/1".into(),
                    value: g.clone()
                }
            )?))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    Tree { objects }.reconcile(previous, "gap-oids/1", &ids)
}
pub(super) fn read_gaps(objects: &Objects<'_>, root: &str) -> Result<Vec<Gap>, Error> {
    Tree { objects }
        .items(root, "gap-oids/1")?
        .into_iter()
        .map(|v| {
            let id = v.as_str().ok_or_else(|| integrity("gap OID"))?;
            let g: StoredGap = objects.get_json(id, "gap")?;
            if g.schema != "knowledgehub-gap/1" {
                return Err(integrity("gap schema"));
            }
            Ok(g.value)
        })
        .collect()
}
pub(super) fn encode_receipt(objects: &Objects<'_>, receipt: &Receipt) -> Result<String, Error> {
    let mut fields = serde_json::to_value(receipt).map_err(codec::encoding)?;
    let m = fields.as_object_mut().unwrap();
    m.remove("affected");
    m.remove("qualified_affected");
    let tree = Tree { objects };
    objects.put_json(
        "receipt",
        &StoredReceipt {
            schema: "knowledgehub-stored-receipt/1".into(),
            fields,
            affected_root: tree.sequence(
                "legacy-refs/1",
                &receipt
                    .affected
                    .iter()
                    .map(|r| value!(r))
                    .collect::<Vec<_>>(),
            )?,
            qualified_affected_root: tree.sequence(
                "exact-refs/1",
                &receipt
                    .qualified_affected
                    .iter()
                    .map(|r| value!(r))
                    .collect::<Vec<_>>(),
            )?,
        },
    )
}
pub(super) fn decode_receipt(objects: &Objects<'_>, id: &str) -> Result<Receipt, Error> {
    let mut r: StoredReceipt = objects.get_json(id, "receipt")?;
    if r.schema != "knowledgehub-stored-receipt/1" {
        return Err(integrity("receipt schema"));
    }
    let tree = Tree { objects };
    let m = r
        .fields
        .as_object_mut()
        .ok_or_else(|| integrity("receipt fields"))?;
    m.insert(
        "affected".into(),
        Value::Array(tree.items(&r.affected_root, "legacy-refs/1")?),
    );
    let qualified = tree.items(&r.qualified_affected_root, "exact-refs/1")?;
    if !qualified.is_empty() {
        m.insert("qualified_affected".into(), Value::Array(qualified));
    }
    serde_json::from_value(r.fields).map_err(|_| integrity("receipt projection"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::shared::specimens;
    #[test]
    fn original_kernel_digests_spans_order_nulls_and_cycles_round_trip() {
        let (_s, b) = super::super::object_backend("projections");
        let o = Objects::new(&b.store);
        let mut state = specimens::specimen_state();
        let artifacts = Tree { objects: &o }
            .map(
                "artifacts/1",
                state.artifacts.iter().map(|(key, c)| {
                    let r = state.records.get(key).unwrap();
                    (ref_key(&r.reference), value!(content::put(&o, c).unwrap()))
                }),
            )
            .unwrap();
        for r in state.records.values() {
            let id = encode_record(&o, r, &state, &artifacts).unwrap();
            let got = decode_record(&Objects::new(&b.store), &id).unwrap();
            assert_eq!(&got, r);
            assert_eq!(got.digest().unwrap(), r.sha256);
        }
        let mut baseline = state.baseline_data().unwrap();
        baseline.selected.reverse();
        baseline.supporting.reverse();
        baseline.supporting.push(baseline.supporting[0].clone());
        let record = Record::new(
            ExactRef::new("test", "ordered-baseline", "1"),
            ontology::type_ref("ModelBaseline"),
            RecordData::Baseline(baseline),
            vec![],
        )
        .unwrap();
        let id = encode_record(&o, &record, &state, &artifacts).unwrap();
        assert_eq!(decode_record(&o, &id).unwrap(), record);
        let source = specimens::document();
        let r = Record::new(
            ExactRef::new("test", "empty-span", "1"),
            ontology::type_ref("SourceAnchor"),
            RecordData::Anchor(AnchorData {
                source: source.clone(),
                label: "empty".into(),
                locator: Locator::ByteRange {
                    start: 0,
                    end: 0,
                    quote: String::new(),
                },
            }),
            vec![],
        )
        .unwrap();
        let id = encode_record(&o, &r, &state, &artifacts).unwrap();
        assert_eq!(decode_record(&o, &id).unwrap(), r);
        let r = Record::new(
            ExactRef::new("test", "null-quote", "1"),
            ontology::type_ref("SourceAnchor"),
            RecordData::Anchor(AnchorData {
                source,
                label: "null".into(),
                locator: Locator::Manifest {
                    locator: ontology::LocatorKind::PdfPage,
                    manifest: specimens::document(),
                    entry: "page".into(),
                    quote: None,
                },
            }),
            vec![],
        )
        .unwrap();
        let id = encode_record(&o, &r, &state, &artifacts).unwrap();
        assert_eq!(decode_record(&o, &id).unwrap(), r);
        // Logical cycles require no cyclic object hashes: exact references are semantic tokens.
        for (a, z) in [("a", "b"), ("b", "a")] {
            let r = Record::new(
                ExactRef::new("cycle", a, "1"),
                ontology::type_ref("Requirement"),
                RecordData::Element(ontology::ElementData::new(BTreeMap::from([(
                    "reference".into(),
                    ontology::Value::Ref(ExactRef::new("cycle", z, "1")),
                )]))),
                vec![],
            )
            .unwrap();
            state.records.insert(r.reference.key(), r.clone());
            let id = encode_record(&o, &r, &state, &artifacts).unwrap();
            assert_eq!(decode_record(&o, &id).unwrap(), r);
        }
        let mut invalid = state
            .records
            .get(&specimens::anchor().key())
            .unwrap()
            .clone();
        if let RecordData::Anchor(a) = &mut invalid.data {
            if let Locator::ByteRange { quote, .. } = &mut a.locator {
                quote.push('!');
            }
        }
        invalid.seal().unwrap();
        assert!(encode_record(&o, &invalid, &state, &artifacts).is_err());
    }
}
