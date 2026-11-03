//! Ordinary native evolution of a generated fixture, independently of source import.
use knowledgehub::{backends::files::*, ontology::*, *};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};
#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Host {
    actor: String,
    grants: BTreeSet<String>,
}
struct CurrentPolicy {
    host: Host,
    scope: ModelScope,
}
impl Policy for CurrentPolicy {
    fn allows(
        &self,
        actor: &str,
        scope: &ModelScope,
        action: Action,
    ) -> Result<bool, PolicyFailure> {
        Ok(actor == self.host.actor
            && scope == &self.scope
            && self.host.grants.contains(&format!("{action:?}")))
    }
}

fn rewrite(v: &mut Value, m: &BTreeMap<ExactRef, ExactRef>) {
    match v {
        Value::Ref(r) => {
            if let Some(next) = m.get(r) {
                *r = next.clone();
            }
        }
        Value::List(v) | Value::Set(v) => v.iter_mut().for_each(|v| rewrite(v, m)),
        Value::Record(p) => p.values_mut().for_each(|v| rewrite(v, m)),
        _ => {}
    }
}
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 4 {
        return Err(
            "usage: model_successor GENERATION-MANIFEST LOGICAL-KEY HOST.json OUTPUT.json".into(),
        );
    }
    let manifest: serde_json::Value = serde_json::from_slice(&std::fs::read(&args[0])?)?;
    let scope: ModelScope = serde_json::from_value(manifest["scope"].clone())?;
    let host: Host = serde_json::from_slice(&std::fs::read(&args[2])?)?;
    let ctx = Context::new(
        host.actor.clone(),
        std::sync::Arc::new(CurrentPolicy {
            host,
            scope: scope.clone(),
        }),
    );
    let directory = PathBuf::from(manifest["destination"].as_str().ok_or("destination")?);
    let a: ExactRef = serde_json::from_value(manifest["baseline"].clone())?;
    let reference: ExactRef = serde_json::from_value(manifest["key_map"][&args[1]].clone())?;
    let backend = || FileBackend::new(&directory, FileOptions::native_local().unwrap()).unwrap();
    let h = KnowledgeHub::open(backend(), ctx.clone(), scope.clone()).await?;
    let (head, state) = h.snapshot(ctx.clone()).await?;
    let original = state.get(&reference).ok_or("selected definition missing")?;
    let mut changed = original.clone();
    changed.reference.revision = "native-successor-2".into();
    changed.predecessor = Some(reference.clone());
    let RecordData::Element(e) = &mut changed.data else {
        return Err("choose an element definition".into());
    };
    let label = e
        .properties
        .get("name")
        .and_then(Value::as_text)
        .ok_or("name")?
        .to_owned();
    e.properties.insert(
        "name".into(),
        Value::text(format!("{label} — authored native successor")),
    );
    e.owners = ElementData::new(e.properties.clone()).owners;
    let anchor = original
        .provenance
        .iter()
        .find(|r| state.kind(r) == Some("SourceAnchor"))
        .cloned()
        .ok_or("source anchor required")?;
    let interpretation_ref = ExactRef::new(
        scope.model.clone(),
        "model:interpretation:ordinary-native-successor",
        "1",
    );
    let interpretation=Record::new(interpretation_ref.clone(),type_ref("Interpretation"),RecordData::Interpretation(InterpretationData{
        subject:changed.reference.clone(),property:Some("name".into()),anchor:anchor.clone(),text:"Authored native follow-up label. Original sources and baseline remain exact; this is not a source refresh, implementation claim or human acceptance.".into(),interpreter:ctx.actor.clone(),method:"ordinary native API evolution demonstration".into(),review:None}),vec![anchor])?;
    changed.provenance.push(interpretation_ref);
    changed.seal()?;
    let mut replacements = BTreeMap::from([(reference.clone(), changed)]);
    loop {
        let mapping = replacements
            .iter()
            .map(|(old, new)| (old.clone(), new.reference.clone()))
            .collect::<BTreeMap<_, _>>();
        let mut added = false;
        for record in state.selected_records() {
            if replacements.contains_key(&record.reference) {
                continue;
            }
            let affected = match &record.data {
                RecordData::Assertion(a) => {
                    mapping.contains_key(&a.from) || mapping.contains_key(&a.to)
                }
                RecordData::Representation(b) => mapping.contains_key(&b.subject),
                RecordData::Element(e) => e
                    .properties
                    .values()
                    .flat_map(Value::refs)
                    .any(|r| mapping.contains_key(r)),
                _ => false,
            };
            if affected {
                let mut next = record.clone();
                next.reference.revision = "native-successor-2".into();
                next.predecessor = Some(record.reference.clone());
                replacements.insert(record.reference.clone(), next);
                added = true;
            }
        }
        if !added {
            break;
        }
    }
    let mapping = replacements
        .iter()
        .map(|(old, new)| (old.clone(), new.reference.clone()))
        .collect::<BTreeMap<_, _>>();
    for record in replacements.values_mut() {
        match &mut record.data {
            RecordData::Element(e) => {
                e.properties.values_mut().for_each(|v| rewrite(v, &mapping));
                e.owners = ElementData::new(e.properties.clone()).owners;
            }
            RecordData::Assertion(a) => {
                if let Some(r) = mapping.get(&a.from) {
                    a.from = r.clone();
                }
                if let Some(r) = mapping.get(&a.to) {
                    a.to = r.clone();
                }
            }
            RecordData::Representation(b) => {
                if let Some(r) = mapping.get(&b.subject) {
                    b.subject = r.clone();
                }
            }
            _ => {}
        }
        record.seal()?;
    }
    let mut gaps = state.gaps.clone();
    for gap in &state.gaps {
        if let Some(r) = mapping.get(&gap.subject) {
            let mut next = gap.clone();
            next.subject = r.clone();
            gaps.push(next);
        }
    }
    let mut candidate = state.clone();
    candidate.gaps = gaps.clone();
    candidate
        .records
        .insert(interpretation.reference.key(), interpretation.clone());
    candidate.selected.insert(
        interpretation.reference.identity(),
        interpretation.reference.clone(),
    );
    for record in replacements.values() {
        candidate
            .records
            .insert(record.reference.key(), record.clone());
        candidate
            .selected
            .insert(record.reference.identity(), record.reference.clone());
    }
    let mut b_data = candidate.baseline_data()?;
    b_data.parents = vec![a.clone()];
    let b = ExactRef::new(scope.model.clone(), "model:baseline:native-B", "1");
    let mut c = Change::at(&state, head.clone(), "fixture:ordinary-native-successor");
    c.operations = replacements.values().cloned().map(Operation::Put).collect();
    c.operations.push(Operation::Put(interpretation));
    c.operations.push(Operation::Gaps(gaps));
    c.operations.push(Operation::Put(Record::new(
        b.clone(),
        type_ref("ModelBaseline"),
        RecordData::Baseline(b_data),
        original.provenance.clone(),
    )?));
    let report = h.validate(ctx.clone(), c.clone()).await?;
    if !report.is_valid() {
        return Err(format!("{report:#?}").into());
    }
    let receipt = h.commit(ctx.clone(), c.clone()).await?;
    assert_eq!(receipt, h.commit(ctx.clone(), c).await?);
    h.drain().await;
    let h = KnowledgeHub::open(backend(), ctx.clone(), scope.clone()).await?;
    for (old, new) in &replacements {
        assert_eq!(
            h.read(ctx.clone(), Selection::Baseline(a.clone()), old.clone())
                .await?,
            *state.get(old).unwrap()
        );
        assert_eq!(
            h.read(
                ctx.clone(),
                Selection::Baseline(b.clone()),
                new.reference.clone()
            )
            .await?,
            *new
        );
    }
    let mut bytes = 0;
    for (key, value) in manifest["key_map"].as_object().ok_or("key map")? {
        if !key.starts_with("capture:") {
            continue;
        }
        let r: ExactRef = serde_json::from_value(value.clone())?;
        let original = h
            .read_artifact(ctx.clone(), Selection::Baseline(a.clone()), r.clone())
            .await?;
        let current = h
            .read_artifact(ctx.clone(), Selection::Baseline(b.clone()), r)
            .await?;
        assert_eq!(original, current);
        bytes += original.bytes.len();
    }
    for dispatch in manifest["known_operations"]
        .as_array()
        .ok_or("known operations")?
    {
        let id = dispatch["request"]["operation_id"]
            .as_str()
            .ok_or("operation id")?;
        let expected: Receipt = serde_json::from_value(dispatch["receipt"].clone())?;
        assert_eq!(
            h.operation_result(ctx.clone(), id.into()).await?,
            OperationOutcome::Committed(Box::new(expected))
        );
    }
    assert_eq!(h.head(ctx.clone()).await?, receipt.head);
    h.drain().await;
    let changed = mapping.iter().collect::<Vec<_>>();
    std::fs::write(
        &args[3],
        serde_json::to_vec_pretty(
            &serde_json::json!({"scope":scope,"original_head":head,"current_head":receipt.head,"baseline_A":a,"baseline_B":b,"changed":changed,"original_bytes_equal":bytes,"known_generation_receipts":"exact","native_validity":report.is_valid(),"native_completeness":report.is_complete(),"human_acceptance":"pending"}),
        )?,
    )?;
    Ok(())
}
