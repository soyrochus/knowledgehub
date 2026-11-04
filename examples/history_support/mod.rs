//! Two synthetic domain fixtures with exact source-bound original and successor definitions.
#![allow(dead_code)]
use knowledgehub::*;
use std::{collections::BTreeMap, sync::Arc};

pub struct Reader(pub ModelScope);
impl Policy for Reader {
    fn allows(
        &self,
        actor: &str,
        scope: &ModelScope,
        action: Action,
    ) -> Result<bool, PolicyFailure> {
        Ok(actor == "host:history-reader"
            && scope == &self.0
            && matches!(action, Action::Open | Action::Read | Action::ReadHistory))
    }
}
pub fn reader(scope: ModelScope) -> Context {
    Context::new("host:history-reader", Arc::new(Reader(scope)))
}
pub struct Fixture {
    pub initial: SnapshotId,
    pub receipts: Vec<Receipt>,
    pub retry: Change,
    pub baselines: [ExactRef; 2],
    pub definitions: [Record; 2],
    pub sources: [ExactRef; 2],
    pub bytes: [Vec<u8>; 2],
}
fn install(s: &mut State, r: &Record) {
    s.records.insert(r.reference.key(), r.clone());
    s.selected
        .insert(r.reference.identity(), r.reference.clone());
}
pub async fn populate(
    h: &KnowledgeHub,
    a: Context,
    domain: &str,
    later: usize,
) -> Result<Fixture, Error> {
    let (initial, state) = h.snapshot(a.clone()).await?;
    let mut register = Change::at(&state, initial.clone(), "history:catalogue");
    register.operations = builtin_catalogue()?.operations();
    let mut receipts = vec![h.commit(a.clone(), register).await?];
    let sources = [
        ExactRef::new(domain, "policy:source", "1"),
        ExactRef::new(domain, "policy:source", "2"),
    ];
    let baselines = [
        ExactRef::new(domain, "baseline:A", "1"),
        ExactRef::new(domain, "baseline:B", "1"),
    ];
    let bytes = [
        format!("{domain}: preserve the original policy.\n").into_bytes(),
        format!("{domain}: preserve the original policy and its replacement.\n").into_bytes(),
    ];
    let mut definitions: Vec<Record> = vec![];
    let mut retry = None;
    for i in 0..2 {
        let (head, state) = h.snapshot(a.clone()).await?;
        let source_artifact = ExactRef::new(domain, "policy:origin", "1");
        let anchor = ExactRef::new(domain, "policy:anchor", (i + 1).to_string());
        let content = Content::capture(bytes[i].clone());
        let mut source = Record::new(
            sources[i].clone(),
            type_ref("SourceRevision"),
            RecordData::SourceRevision(SourceRevisionData {
                source: source_artifact.clone(),
                original: content.reference.clone(),
                media_type: "text/markdown".into(),
                derived_from: None,
                derivation: None,
            }),
            vec![],
        )?;
        if i == 1 {
            source.predecessor = Some(sources[0].clone());
            source.seal()?;
        }
        let mut anchor_record = Record::new(
            anchor.clone(),
            type_ref("SourceAnchor"),
            RecordData::Anchor(AnchorData {
                source: sources[i].clone(),
                label: "whole exact policy".into(),
                locator: Locator::ByteRange {
                    start: 0,
                    end: bytes[i].len() as u64,
                    quote: String::from_utf8(bytes[i].clone()).unwrap(),
                },
            }),
            vec![],
        )?;
        if i == 1 {
            anchor_record.predecessor = Some(ExactRef::new(domain, "policy:anchor", "1"));
            anchor_record.seal()?;
        }
        let narrative = Value::Narrative {
            document: sources[i].clone(),
            fragment: anchor.clone(),
        };
        let mut definition = Record::new(
            ExactRef::new(domain, "requirement:retain-policy", (i + 1).to_string()),
            type_ref("Requirement"),
            RecordData::Element(ElementData::new(BTreeMap::from([
                (
                    "name".into(),
                    Value::text(format!("{domain} policy {}", i + 1)),
                ),
                ("description".into(), narrative.clone()),
                ("statement".into(), narrative),
                ("rationale".into(), Value::List(vec![])),
                (
                    "claim_kind".into(),
                    Value::enumeration("authored-requirement"),
                ),
            ]))),
            vec![anchor.clone()],
        )?;
        if i == 1 {
            definition.predecessor = Some(definitions[0].reference.clone());
            definition.seal()?;
        }
        let mut c = Change::at(
            &state,
            head,
            if i == 0 {
                "history:original-A"
            } else {
                "history:successor-B"
            },
        );
        if i == 0 {
            c.operations.push(Operation::Put(Record::new(
                source_artifact,
                type_ref("SourceArtifact"),
                RecordData::SourceArtifact(SourceArtifactData {
                    origin: format!("synthetic {domain} policy"),
                    media_type: "text/markdown".into(),
                    owner: a.actor.clone(),
                    status: "hypothesis".into(),
                }),
                vec![],
            )?));
        }
        c.operations.extend([
            Operation::Capture {
                record: source,
                content,
            },
            Operation::Put(anchor_record),
            Operation::Put(definition.clone()),
        ]);
        let mut candidate = state.clone();
        for op in &c.operations {
            match op {
                Operation::Put(r) | Operation::Capture { record: r, .. } => {
                    install(&mut candidate, r)
                }
                _ => (),
            }
        }
        // baseline_data derives immutable artifact pins from SourceRevision metadata too.
        for op in &c.operations {
            if let Operation::Capture { record, content } = op {
                candidate
                    .artifacts
                    .insert(record.reference.key(), content.clone());
            }
        }
        let mut baseline = candidate.baseline_data()?;
        if i == 1 {
            baseline.parents = vec![baselines[0].clone()];
        }
        c.operations.push(Operation::Put(Record::new(
            baselines[i].clone(),
            type_ref("ModelBaseline"),
            RecordData::Baseline(baseline),
            vec![anchor],
        )?));
        let r = h.commit(a.clone(), c.clone()).await?;
        if i == 0 {
            retry = Some(c);
        }
        receipts.push(r);
        definitions.push(definition);
    }
    for i in 0..later {
        receipts.push(append(h, a.clone(), &format!("history:later:{i}")).await?);
    }
    Ok(Fixture {
        initial,
        receipts,
        retry: retry.unwrap(),
        baselines,
        definitions: definitions.try_into().unwrap(),
        sources,
        bytes,
    })
}
pub async fn append(h: &KnowledgeHub, a: Context, id: &str) -> Result<Receipt, Error> {
    let (head, state) = h.snapshot(a.clone()).await?;
    let mut c = Change::at(&state, head, id);
    c.operations.push(Operation::Put(Record::new(
        ExactRef::new("history-extra", id, "1"),
        type_ref("SourceArtifact"),
        RecordData::SourceArtifact(SourceArtifactData {
            origin: "unrelated synthetic input".into(),
            media_type: "text/plain".into(),
            owner: a.actor.clone(),
            status: "hypothesis".into(),
        }),
        vec![],
    )?));
    h.commit(a, c).await
}
pub async fn headers(
    h: &KnowledgeHub,
    b: Context,
    q: Publications,
) -> Result<Vec<PublicationHeader>, Error> {
    let mut page = h.publications(b.clone(), q.clone()).await?;
    let mut out = vec![];
    loop {
        if let HistoryItems::Publications(headers) = page.items {
            out.extend(headers);
        }
        match page.continuation {
            Some(t) => {
                page = h
                    .continue_history(b.clone(), HistoryRequest::Publications(q.clone()), t)
                    .await?
            }
            None => {
                assert!(page.complete);
                break;
            }
        }
    }
    Ok(out)
}
pub async fn affected(
    h: &KnowledgeHub,
    b: Context,
    q: PublicationAffected,
) -> Result<Vec<ExactRef>, Error> {
    let mut page = h.publication_affected(b.clone(), q.clone()).await?;
    let mut out = vec![];
    loop {
        if let HistoryItems::Affected {
            references, offset, ..
        } = page.items
        {
            assert_eq!(offset, out.len() as u64);
            out.extend(references);
        }
        match page.continuation {
            Some(t) => {
                page = h
                    .continue_history(b.clone(), HistoryRequest::Affected(q.clone()), t)
                    .await?
            }
            None => {
                assert!(page.complete);
                break;
            }
        }
    }
    Ok(out)
}
pub async fn meaning(h: &KnowledgeHub, b: Context, f: &Fixture) -> Result<(), Error> {
    for i in 0..2 {
        let selection = Selection::Baseline(f.baselines[i].clone());
        assert_eq!(
            h.read(
                b.clone(),
                selection.clone(),
                f.definitions[i].reference.clone()
            )
            .await?,
            f.definitions[i]
        );
        assert_eq!(
            h.read_artifact(b.clone(), selection, f.sources[i].clone())
                .await?,
            Content::capture(f.bytes[i].clone())
        );
    }
    let b_state = h
        .read_state(b.clone(), Selection::Baseline(f.baselines[1].clone()))
        .await?;
    assert_eq!(
        b_state
            .current(
                &f.definitions[0].reference.model,
                &f.definitions[0].reference.id
            )
            .unwrap(),
        &f.definitions[1]
    );
    assert_eq!(
        f.definitions[1].predecessor.as_ref(),
        Some(&f.definitions[0].reference)
    );
    let diff = h
        .compare(b, f.baselines[0].clone(), f.baselines[1].clone())
        .await?;
    assert!(
        diff.iter()
            .any(|d| d.before.as_ref() == Some(&f.definitions[0].reference)
                && d.after.as_ref() == Some(&f.definitions[1].reference))
    );
    Ok(())
}
