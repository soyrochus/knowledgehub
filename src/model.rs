//! Profile semantics and candidate validation have no filesystem or executor dependencies.
use crate::{codec, *};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) fn check_scope(scope: &Scope) -> Result<(), Error> {
    if [
        &scope.hub,
        &scope.model,
        &scope.branch,
        &scope.owner,
        &scope.profile.ontology.id,
    ]
    .iter()
    .any(|s| s.trim().is_empty())
    {
        return Err(Error::InvalidInput(
            "nonempty semantic scope and owner required".into(),
        ));
    }
    if scope.profile == Profile::meta() {
        return Ok(());
    }
    if !hash(&scope.profile.ontology.revision)
        || scope.profile != Profile::seed(scope.profile.ontology.clone())
    {
        return Err(Error::Incompatible(
            "unsupported exact ontology/profile definitions".into(),
        ));
    }
    Ok(())
}

pub(crate) fn hash(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn problem(out: &mut Vec<Violation>, rule: &str, subject: &str, message: &str) {
    out.push(Violation {
        rule: rule.into(),
        subject: subject.into(),
        message: message.into(),
    });
}

fn content(out: &mut Vec<Violation>, id: &str, c: &Content) {
    if ContentRef::of(&c.bytes) != c.reference {
        problem(
            out,
            "KH-DR-02",
            id,
            "exact original-byte digest or length mismatch",
        );
    }
    if std::str::from_utf8(&c.bytes).is_err() {
        problem(out, "KH-DR-05", id, "seed content must be UTF-8");
    }
}

fn range(bytes: &[u8], start: u64, end: u64) -> Option<&str> {
    let s = std::str::from_utf8(bytes).ok()?;
    let start = usize::try_from(start).ok()?;
    let end = usize::try_from(end).ok()?;
    if start >= end {
        return None;
    }
    s.get(start..end).filter(|s| !s.trim().is_empty())
}

fn revision<T: serde::Serialize>(out: &mut Vec<Violation>, reference: &RevisionRef, record: &T) {
    if codec::record_revision(record).ok().as_ref() != Some(&reference.revision) {
        problem(
            out,
            "KH-DR-02",
            &reference.id,
            "semantic revision differs from tagged canonical record",
        );
    }
}

fn interpretation(
    out: &mut Vec<Violation>,
    snapshot: &Snapshot,
    subject: &str,
    reference: &RevisionRef,
) {
    if !snapshot
        .interpretations
        .iter()
        .any(|r| &r.reference == reference)
    {
        problem(
            out,
            "KH-DR-05",
            subject,
            "required exact authored interpretation does not resolve",
        );
    }
}

fn authority(
    out: &mut Vec<Violation>,
    snapshot: &Snapshot,
    subject: &str,
    bindings: &[AuthorityBinding],
    narrative: &[&str],
    graph: &[&str],
) {
    let mut fields = BTreeSet::new();
    for binding in bindings {
        if !fields.insert(binding.field.as_str()) {
            problem(
                out,
                "KH-DR-06",
                subject,
                "competing normative owners for one field",
            );
        }
        if graph.contains(&binding.field.as_str()) {
            if binding.owner != Owner::Graph {
                problem(
                    out,
                    "KH-DR-06",
                    subject,
                    "semantic field must be graph-owned",
                );
            }
        } else if narrative.contains(&binding.field.as_str()) {
            match &binding.owner {
                Owner::Document(r) => {
                    let resolved = snapshot
                        .documents
                        .iter()
                        .find(|d| d.reference == r.document)
                        .and_then(|d| {
                            d.fragments
                                .iter()
                                .find(|f| f.id == r.fragment)
                                .and_then(|f| range(&d.content.bytes, f.start, f.end))
                        });
                    if resolved.is_none() {
                        problem(
                            out,
                            "KH-DR-06",
                            subject,
                            "nonempty exact document fragment must resolve",
                        );
                    }
                }
                Owner::Graph => problem(
                    out,
                    "KH-DR-06",
                    subject,
                    "narrative field must have one document owner",
                ),
            }
        } else {
            problem(out, "KH-DR-03", subject, "unknown normative property");
        }
    }
    for name in narrative.iter().chain(graph) {
        if !fields.contains(name) {
            problem(
                out,
                "KH-DR-06",
                subject,
                &format!("required field owner missing: {name}"),
            );
        }
    }
}

/// Validating a complete immutable snapshot is also part of recovery integrity verification.
pub(crate) fn validate_snapshot(snapshot: &Snapshot) -> Vec<Violation> {
    if let Some(state) = &snapshot.ontology {
        if state.scope.transport() != snapshot.scope
            || !snapshot.sources.is_empty()
            || !snapshot.documents.is_empty()
            || !snapshot.elements.is_empty()
            || !snapshot.assertions.is_empty()
            || !snapshot.interpretations.is_empty()
        {
            return vec![Violation {
                rule: "V-04".into(),
                subject: "snapshot".into(),
                message: "mixed scope or mixed snapshot formats".into(),
            }];
        }
        return crate::ontology::assess(state).violations;
    }
    let mut out = Vec::new();
    let mut ids = BTreeSet::new();
    let references: Vec<_> = snapshot
        .sources
        .iter()
        .map(|r| &r.reference)
        .chain(snapshot.documents.iter().map(|r| &r.reference))
        .chain(snapshot.interpretations.iter().map(|r| &r.reference))
        .chain(snapshot.elements.iter().map(|r| &r.reference))
        .chain(snapshot.assertions.iter().map(|r| &r.reference))
        .collect();
    for r in references {
        if r.id.trim().is_empty() || !ids.insert(&r.id) {
            problem(
                &mut out,
                "KH-DR-01",
                &r.id,
                "identity must be nonempty and unique in model scope",
            );
        }
        if !hash(&r.revision) {
            problem(
                &mut out,
                "KH-DR-02",
                &r.id,
                "exact revision must be a lowercase SHA-256 digest",
            );
        }
    }
    for s in &snapshot.sources {
        content(&mut out, &s.reference.id, &s.content);
        if s.reference.revision != s.content.reference.sha256 {
            problem(
                &mut out,
                "KH-DR-02",
                &s.reference.id,
                "source revision must bind original bytes",
            );
        }
        let mut anchors = BTreeSet::new();
        if s.anchors.is_empty() {
            problem(
                &mut out,
                "KH-DR-05",
                &s.reference.id,
                "source requires meaningful anchors",
            );
        }
        for a in &s.anchors {
            if a.id.trim().is_empty()
                || !anchors.insert(&a.id)
                || a.section.trim().is_empty()
                || range(&s.content.bytes, a.start, a.end) != Some(a.quote.as_str())
            {
                problem(
                    &mut out,
                    "KH-DR-05",
                    &s.reference.id,
                    "anchor ID, section, UTF-8 range and exact quotation must resolve",
                );
            }
        }
    }
    for d in &snapshot.documents {
        content(&mut out, &d.reference.id, &d.content);
        revision(&mut out, &d.reference, d);
        let mut fragments = BTreeSet::new();
        for f in &d.fragments {
            if f.id.trim().is_empty()
                || !fragments.insert(&f.id)
                || range(&d.content.bytes, f.start, f.end).is_none()
            {
                problem(
                    &mut out,
                    "KH-DR-06",
                    &d.reference.id,
                    "fragment requires a stable unique ID and nonempty UTF-8 range",
                );
            }
        }
    }
    for i in &snapshot.interpretations {
        revision(&mut out, &i.reference, i);
        let source = snapshot.sources.iter().find(|s| s.reference == i.source);
        if source
            .and_then(|s| s.anchors.iter().find(|a| a.id == i.anchor))
            .is_none()
            || i.text.trim().is_empty()
            || i.method.trim().is_empty()
            || !i.interpreter.contains(':')
        {
            problem(
                &mut out,
                "KH-DR-05",
                &i.reference.id,
                "authored interpretation requires exact source/anchor, text, method and qualified interpreter",
            );
        }
    }
    let p = &snapshot.scope.profile;
    let requirement = p.definition("Requirement").expect("validated profile");
    let use_case = p.definition("UseCase").expect("validated profile");
    for e in &snapshot.elements {
        revision(&mut out, &e.reference, e);
        let narrative: &[&str] = if e.element_type == requirement {
            &["title", "statement"]
        } else if e.element_type == use_case {
            &["title", "actor", "trigger", "outcome"]
        } else {
            problem(
                &mut out,
                "KH-DR-03",
                &e.reference.id,
                "unsupported exact element type revision",
            );
            &[]
        };
        authority(
            &mut out,
            snapshot,
            &e.reference.id,
            &e.authority,
            narrative,
            &["id", "revision", "type", "scope", "interpretation"],
        );
        interpretation(&mut out, snapshot, &e.reference.id, &e.interpretation);
    }
    for a in &snapshot.assertions {
        revision(&mut out, &a.reference, a);
        if a.relationship_type != p.definition("satisfies").expect("validated profile") {
            problem(
                &mut out,
                "KH-DR-03",
                &a.reference.id,
                "unsupported exact relationship type revision",
            );
        }
        let from = snapshot.elements.iter().find(|e| e.reference == a.from);
        let to = snapshot.elements.iter().find(|e| e.reference == a.to);
        if !from.is_some_and(|e| e.element_type == use_case)
            || !to.is_some_and(|e| e.element_type == requirement)
        {
            problem(
                &mut out,
                "KH-DR-03",
                &a.reference.id,
                "satisfies endpoints must resolve to exact UseCase → Requirement revisions in this snapshot",
            );
        }
        authority(
            &mut out,
            snapshot,
            &a.reference.id,
            &a.authority,
            &[],
            &[
                "id",
                "revision",
                "type",
                "scope",
                "from",
                "to",
                "interpretation",
            ],
        );
        interpretation(&mut out, snapshot, &a.reference.id, &a.interpretation);
    }
    if snapshot.sequence > 0 {
        if snapshot.assertions.is_empty() {
            problem(
                &mut out,
                "KH-DR-03",
                &snapshot.scope.model,
                "complete seed requires an identified satisfies assertion",
            );
        }
        if !snapshot.sources.iter().any(|s| {
            s.reference == p.ontology && s.anchors.iter().any(|a| a.section.contains("21.4"))
        }) {
            problem(
                &mut out,
                "KH-DR-05",
                &snapshot.scope.model,
                "complete seed requires the pinned ontology capture and section 21.4 anchor",
            );
        }
    } else if !ids.is_empty() {
        problem(
            &mut out,
            "KH-DR-08",
            &snapshot.scope.model,
            "genesis must be empty",
        );
    }
    out
}

pub(crate) struct ValidatedPlan {
    pub snapshot: Snapshot,
    pub receipt: Receipt,
}

pub(crate) fn candidate(
    base: &Snapshot,
    request: &ChangeSet,
) -> Result<(Snapshot, ValidationReport), Error> {
    check_scope(&request.scope)?;
    if base.scope != request.scope {
        return Err(Error::Incompatible(
            "request scope differs from opened model".into(),
        ));
    }
    if base.id != request.expected_head {
        return Err(Error::Conflict {
            expected: request.expected_head.clone(),
            actual: base.id.clone(),
        });
    }
    if request.operation_id.trim().is_empty() || request.operations.is_empty() {
        return Err(Error::InvalidInput(
            "operation identity and nonempty ordered operations required".into(),
        ));
    }
    let revisions = base.revisions();
    for guard in &request.guards {
        if guard.id.trim().is_empty() {
            return Err(Error::InvalidInput("empty guard identity".into()));
        }
        if revisions.get(&guard.id) != guard.expected.as_ref() {
            return Err(Error::GuardConflict {
                id: guard.id.clone(),
            });
        }
    }
    let mut snapshot = base.clone();
    snapshot.sequence = base
        .sequence
        .checked_add(1)
        .filter(|n| *n <= 9_007_199_254_740_991)
        .ok_or_else(|| Error::Unavailable("sequence exhausted".into()))?;
    let mut violations = Vec::new();
    for op in &request.operations {
        if revisions.contains_key(&op.reference().id) {
            problem(
                &mut violations,
                "KH-DR-02",
                &op.reference().id,
                "identity already accepted; successor authoring is deferred",
            );
        }
        match op {
            Operation::CaptureSource(s) => snapshot.sources.push(s.clone()),
            Operation::AddDocument(d) => snapshot.documents.push(d.clone()),
            Operation::Interpret(i) => snapshot.interpretations.push(i.clone()),
            Operation::Define(e) => snapshot.elements.push(e.clone()),
            Operation::Assert(a) => snapshot.assertions.push(a.clone()),
        }
    }
    violations.extend(validate_snapshot(&snapshot));
    Ok((
        snapshot,
        ValidationReport {
            base: base.id.clone(),
            violations,
        },
    ))
}

pub(crate) fn prepare(
    base: &Snapshot,
    request: &ChangeSet,
    actor: &str,
    id: SnapshotId,
) -> Result<ValidatedPlan, Error> {
    let (mut snapshot, report) = candidate(base, request)?;
    if !report.is_valid() {
        return Err(Error::Invalid(report.violations));
    }
    snapshot.id = id.clone();
    let receipt = Receipt {
        scope: request.scope.clone(),
        operation_id: request.operation_id.clone(),
        actor: actor.into(),
        fingerprint_version: FINGERPRINT_VERSION.into(),
        fingerprint: request.fingerprint(actor)?,
        previous_head: base.id.clone(),
        head: id,
        sequence: snapshot.sequence,
        qualified_affected: vec![],
        affected: request
            .operations
            .iter()
            .map(|o| o.reference().clone())
            .collect(),
    };
    Ok(ValidatedPlan { snapshot, receipt })
}

pub(crate) fn empty(scope: Scope, id: SnapshotId) -> Snapshot {
    Snapshot {
        id,
        scope,
        sequence: 0,
        sources: vec![],
        documents: vec![],
        interpretations: vec![],
        elements: vec![],
        assertions: vec![],
        ontology: None,
    }
}

pub(crate) fn verify_affected(parent: &Snapshot, next: &Snapshot, receipt: &Receipt) -> bool {
    if let (Some(a), Some(b)) = (&parent.ontology, &next.ontology) {
        let mut added = b
            .records
            .iter()
            .filter(|(k, _)| !a.records.contains_key(*k))
            .map(|(_, r)| r.reference.clone())
            .collect::<Vec<_>>();
        let mut affected = receipt.qualified_affected.clone();
        added.sort();
        affected.sort();
        return crate::ontology::history_valid(a, b) && added == affected;
    }
    if parent.ontology.is_some() || next.ontology.is_some() {
        return false;
    }
    let old = parent.revisions();
    let new = next.revisions();
    let added: BTreeMap<_, _> = new
        .iter()
        .filter(|(id, _)| !old.contains_key(*id))
        .map(|(id, r)| (id.clone(), r.clone()))
        .collect();
    let affected: BTreeMap<_, _> = receipt
        .affected
        .iter()
        .map(|r| (r.id.clone(), r.revision.clone()))
        .collect();
    old.iter().all(|(id, r)| new.get(id) == Some(r))
        && added == affected
        && affected.len() == receipt.affected.len()
}
