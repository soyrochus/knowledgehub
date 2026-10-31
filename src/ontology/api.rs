use super::*;
use crate::{
    Action, Context, Error, OperationOutcome, Receipt, SnapshotId, backends::files::FileBackend,
    jobs::Jobs, model::ValidatedPlan, storage::Storage,
};
use std::sync::Arc;

fn representation_bindings(record: Option<&Record>) -> Vec<(String, serde_json::Value)> {
    fn walk(path: String, value: &Value, out: &mut Vec<(String, serde_json::Value)>) {
        match value {
            Value::Narrative { .. } => {
                out.push((path, serde_json::to_value(value).expect("owned narrative")))
            }
            Value::Record(fields) => {
                for (k, v) in fields {
                    walk(format!("{path}/{k}"), v, out)
                }
            }
            Value::List(values) | Value::Set(values) => {
                for (i, v) in values.iter().enumerate() {
                    walk(format!("{path}/{i}"), v, out)
                }
            }
            _ => {}
        }
    }
    let mut out = vec![];
    if let Some(record) = record {
        match &record.data {
            RecordData::Element(e) => {
                for (k, v) in &e.properties {
                    walk(format!("/{k}"), v, &mut out)
                }
            }
            RecordData::Representation(b) => out.push((
                "/binding".into(),
                serde_json::to_value(b).expect("owned binding"),
            )),
            RecordData::Anchor(a) => out.push((
                "/anchor".into(),
                serde_json::to_value(a).expect("owned anchor"),
            )),
            _ => {}
        }
    }
    out
}

#[derive(Clone)]
pub struct OntologyHub {
    pub(crate) scope: ModelScope,
    pub(crate) store: Arc<dyn Storage>,
    pub(crate) jobs: Arc<Jobs>,
    pub(crate) cursors: Arc<super::navigation::Cursors>,
}
impl OntologyHub {
    pub async fn create(
        backend: FileBackend,
        context: Context,
        scope: ModelScope,
    ) -> Result<Self, Error> {
        if !matches!(backend.store.version(), FILE_FORMAT | SHARED_FILE_FORMAT) {
            return Err(Error::Incompatible(
                "explicit files/2 or files/3 backend required".into(),
            ));
        }
        let h = Self {
            scope,
            store: backend.store,
            jobs: backend.jobs,
            cursors: backend.cursors,
        };
        let job = h.clone();
        h.jobs
            .run(move || {
                context.check(&job.scope.transport(), Action::Create)?;
                let mut s = crate::model::empty(job.scope.transport(), job.store.allocate_id()?);
                s.ontology = Some(Box::new(State::empty(job.scope.clone())));
                job.store.create(s)
            })
            .await?;
        Ok(h)
    }
    pub async fn open(
        backend: FileBackend,
        context: Context,
        scope: ModelScope,
    ) -> Result<Self, Error> {
        if !matches!(backend.store.version(), FILE_FORMAT | SHARED_FILE_FORMAT) {
            return Err(Error::Incompatible(
                "explicit files/2 or files/3 backend required".into(),
            ));
        }
        let h = Self {
            scope,
            store: backend.store,
            jobs: backend.jobs,
            cursors: backend.cursors,
        };
        let job = h.clone();
        h.jobs
            .run(move || {
                context.check(&job.scope.transport(), Action::Open)?;
                job.store.acquire_read(&job.scope.transport())?.head()?;
                Ok(())
            })
            .await?;
        Ok(h)
    }
    pub fn scope(&self) -> ModelScope {
        self.scope.clone()
    }
    pub async fn snapshot(&self, context: Context) -> Result<(SnapshotId, State), Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope.transport(), Action::Read)?;
                let current = h.store.acquire_read(&h.scope.transport())?.current()?;
                let s = &current.snapshot;
                Ok((
                    s.id.clone(),
                    s.ontology
                        .as_deref()
                        .ok_or_else(|| Error::Incompatible("not a typed ontology snapshot".into()))?
                        .clone(),
                ))
            })
            .await
    }
    pub async fn head(&self, context: Context) -> Result<SnapshotId, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope.transport(), Action::Read)?;
                Ok(h.store.acquire_read(&h.scope.transport())?.head()?.snapshot)
            })
            .await
    }
    pub(crate) fn select(&self, selection: &Selection) -> Result<(Selection, State), Error> {
        Ok((
            selection.clone(),
            self.store
                .acquire_read(&self.scope.transport())?
                .state(selection)?,
        ))
    }
    pub async fn read_state(&self, context: Context, selection: Selection) -> Result<State, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope.transport(), Action::Read)?;
                Ok(h.select(&selection)?.1)
            })
            .await
    }
    pub async fn read(
        &self,
        context: Context,
        selection: Selection,
        reference: ExactRef,
    ) -> Result<Record, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope.transport(), Action::Read)?;
                h.store
                    .acquire_read(&h.scope.transport())?
                    .record(&selection, &reference)
            })
            .await
    }
    pub async fn read_artifact(
        &self,
        context: Context,
        selection: Selection,
        reference: ExactRef,
    ) -> Result<crate::Content, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope.transport(), Action::Read)?;
                h.store
                    .acquire_read(&h.scope.transport())?
                    .artifact(&selection, &reference)
            })
            .await
    }
    /// Read an owned exact original-content range, without claiming a full hash check for partial chunk reads.
    pub async fn read_artifact_range(
        &self,
        context: Context,
        selection: Selection,
        reference: ExactRef,
        offset: u64,
        length: u64,
    ) -> Result<ArtifactRange, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope.transport(), Action::Read)?;
                h.store
                    .acquire_read(&h.scope.transport())?
                    .artifact_range(&selection, &reference, offset, length)
            })
            .await
    }
    pub async fn validate(&self, context: Context, change: Change) -> Result<Assessment, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope.transport(), Action::Validate)?;
                let (_, state) = h.select(&Selection::Snapshot(change.expected_head.clone()))?;
                Ok(super::validation::candidate(&state, &change)?.1)
            })
            .await
    }
    pub async fn commit(&self, context: Context, change: Change) -> Result<Receipt, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                let scope = h.scope.transport();
                context.check(&scope, Action::Commit)?;
                context.check(&scope, Action::OperationResult)?;
                if change.scope != h.scope {
                    return Err(Error::Incompatible("scope differs".into()));
                }
                let lease = h.store.acquire(&scope)?;
                context.check(&scope, Action::Commit)?;
                context.check(&scope, Action::OperationResult)?;
                let shared = h.store.version() == SHARED_FILE_FORMAT;
                let history = if shared { None } else { Some(lease.recover()?) };
                let current_head = match &history {
                    Some(hist) => hist.current().reference.clone(),
                    None => lease.head()?,
                };
                let definition = |r: &Record| {
                    matches!(
                        r.data,
                        RecordData::Ontology(_)
                            | RecordData::Type(_)
                            | RecordData::Relationship(_)
                            | RecordData::Constraint(_)
                            | RecordData::Profile(_)
                    )
                };
                let mut register = false;
                for o in &change.operations {
                    let target = if let Operation::Select(reference) = o {
                        match &history {
                            Some(hist) => hist
                                .current()
                                .snapshot
                                .ontology
                                .as_deref()
                                .and_then(|s| s.get(reference))
                                .cloned(),
                            None => match lease.record(
                                &Selection::Snapshot(current_head.snapshot.clone()),
                                reference,
                            ) {
                                Ok(r) => Some(r),
                                Err(Error::NotFound) => None,
                                Err(error) => return Err(error),
                            },
                        }
                    } else {
                        None
                    };
                    register |= target.as_ref().is_some_and(definition)
                        || o.record().is_some_and(definition)
                        || matches!(o, Operation::SelectProfile { .. });
                }
                if register {
                    context.check(&scope, Action::RegisterOntology)?
                }
                let fp = change.fingerprint(&context.actor)?;
                let original = match &history {
                    Some(hist) => hist.operation(&change.operation_id).cloned(),
                    None => lease.operation(&change.operation_id)?,
                };
                if let Some(r) = original {
                    if r.actor != context.actor || r.fingerprint != fp {
                        return Err(Error::OperationReuse);
                    }
                    return Ok(r);
                }
                let current = match history {
                    Some(mut hist) => hist.entries.pop().expect("accepted genesis"),
                    None => lease.current()?,
                };
                if current.snapshot.id != change.expected_head {
                    return Err(Error::Conflict {
                        expected: change.expected_head.clone(),
                        actual: current.snapshot.id.clone(),
                    });
                }
                let base = current
                    .snapshot
                    .ontology
                    .as_deref()
                    .ok_or_else(|| Error::Incompatible("ontology snapshot absent".into()))?;
                let (next, report) = super::validation::candidate(base, &change)?;
                if !report.is_valid() {
                    return Err(Error::Invalid(report.violations));
                }
                let id = h.store.allocate_id()?;
                let mut snapshot = crate::model::empty(scope.clone(), id.clone());
                snapshot.sequence = current
                    .snapshot
                    .sequence
                    .checked_add(1)
                    .filter(|n| *n <= 9_007_199_254_740_991)
                    .ok_or_else(|| Error::Unavailable("sequence exhausted".into()))?;
                snapshot.ontology = Some(Box::new(next));
                let receipt = Receipt {
                    scope,
                    operation_id: change.operation_id.clone(),
                    actor: context.actor.clone(),
                    fingerprint_version: REQUEST_FORMAT.into(),
                    fingerprint: fp,
                    previous_head: change.expected_head,
                    head: id,
                    sequence: snapshot.sequence,
                    affected: change
                        .operations
                        .iter()
                        .filter_map(Operation::record)
                        .map(|r| crate::RevisionRef {
                            id: r.reference.id.clone(),
                            revision: r.reference.revision.clone(),
                        })
                        .collect(),
                    qualified_affected: change
                        .operations
                        .iter()
                        .filter_map(Operation::record)
                        .map(|r| r.reference.clone())
                        .collect(),
                };
                lease.publish(
                    ValidatedPlan { snapshot, receipt },
                    current.reference.clone(),
                )
            })
            .await
    }
    pub async fn operation_result(
        &self,
        context: Context,
        id: String,
    ) -> Result<OperationOutcome, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope.transport(), Action::OperationResult)?;
                if id.trim().is_empty() {
                    return Err(Error::InvalidInput("operation ID required".into()));
                }
                if h.store.version() == SHARED_FILE_FORMAT {
                    let lease = h.store.acquire_read(&h.scope.transport())?;
                    return match lease.operation(&id)? {
                        Some(r) if r.actor == context.actor => {
                            Ok(OperationOutcome::Committed(Box::new(r)))
                        }
                        Some(_) => Err(Error::OperationReuse),
                        None => Ok(OperationOutcome::Absent {
                            recovered_head: lease.head()?.snapshot,
                        }),
                    };
                }
                let hist = h.store.acquire(&h.scope.transport())?.recover()?;
                match hist.operation(&id) {
                    Some(r) if r.actor == context.actor => {
                        Ok(OperationOutcome::Committed(Box::new(r.clone())))
                    }
                    Some(_) => Err(Error::OperationReuse),
                    None => Ok(OperationOutcome::Absent {
                        recovered_head: hist.current().snapshot.id.clone(),
                    }),
                }
            })
            .await
    }
    pub async fn compare(
        &self,
        context: Context,
        before: ExactRef,
        after: ExactRef,
    ) -> Result<Vec<Difference>, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope.transport(), Action::Read)?;
                let a = h.select(&Selection::Baseline(before))?.1;
                let b = h.select(&Selection::Baseline(after))?.1;
                let mut ids = a
                    .selected
                    .keys()
                    .chain(b.selected.keys())
                    .cloned()
                    .collect::<Vec<_>>();
                ids.sort();
                ids.dedup();
                Ok(ids
                    .into_iter()
                    .filter_map(|id| {
                        let ar = a.selected.get(&id);
                        let br = b.selected.get(&id);
                        if ar == br {
                            return None;
                        }
                        let ra = ar.and_then(|r| a.get(r));
                        let rb = br.and_then(|r| b.get(r));
                        Some(Difference {
                            identity: id,
                            before: ar.cloned(),
                            after: br.cloned(),
                            properties_changed: ra.map(|r| &r.data) != rb.map(|r| &r.data),
                            provenance_changed: ra.map(|r| &r.provenance)
                                != rb.map(|r| &r.provenance),
                            representation_changed: representation_bindings(ra)
                                != representation_bindings(rb),
                        })
                    })
                    .collect())
            })
            .await
    }
    pub async fn drain(&self) {
        self.cursors.clear();
        self.jobs.drain().await;
    }
}

impl State {
    /// Native identity projection has one owner: the actual record's immutable header/lineage.
    pub fn identity_reference(&self, r: &ExactRef) -> Option<ExactRef> {
        self.get(r)
            .map(|_| ExactRef::new(&r.model, &r.id, "@identity"))
    }
    pub fn resolve(&self, r: &ExactRef) -> Option<Record> {
        if let Some(v) = self.get(r) {
            return Some(v.clone());
        }
        if r.revision != "@identity" {
            return None;
        }
        let original = self
            .records
            .values()
            .find(|v| v.reference.identity() == r.identity() && v.predecessor.is_none())?;
        Record::new(
            r.clone(),
            type_ref("ElementIdentity"),
            RecordData::Identity {
                subject: original.reference.clone(),
                declared_type: original.type_ref.clone(),
            },
            original.provenance.clone(),
        )
        .ok()
    }
    /// Explicit exact closure for a new baseline; callers retain this complete intent before submission.
    pub fn baseline_data(&self) -> Result<BaselineData, Error> {
        Ok(BaselineData {
            ontology: self.ontology.clone().ok_or(Error::NotFound)?,
            profile: self.profile.clone().ok_or(Error::NotFound)?,
            selected: self.selected.values().cloned().collect(),
            supporting: self.records.values().map(|r| r.reference.clone()).collect(),
            parents: vec![],
            gaps: self.gaps.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn comparison_detects_nested_narratives_and_native_representation_changes() {
        let a = ExactRef::new("m", "anchor", "1");
        let b = ExactRef::new("m", "anchor", "2");
        let document = ExactRef::new("m", "document", "1");
        let data = |fragment: ExactRef| {
            RecordData::Element(ElementData::new(std::collections::BTreeMap::from([(
                "body".into(),
                Value::Record(std::collections::BTreeMap::from([(
                    "description".into(),
                    Value::Narrative {
                        document: document.clone(),
                        fragment,
                    },
                )])),
            )])))
        };
        let record = |data| {
            Record::new(
                ExactRef::new("m", "definition", "1"),
                type_ref("FunctionDefinition"),
                data,
                vec![],
            )
            .unwrap()
        };
        assert_ne!(
            representation_bindings(Some(&record(data(a.clone())))),
            representation_bindings(Some(&record(data(b.clone()))))
        );
        let binding = |anchor| {
            record(RecordData::Representation(RepresentationData {
                subject: ExactRef::new("m", "subject", "1"),
                anchor,
                role: RepresentationRole::Derived,
            }))
        };
        assert_ne!(
            representation_bindings(Some(&binding(a))),
            representation_bindings(Some(&binding(b)))
        );
        let plain = record(RecordData::Element(ElementData::new(
            std::collections::BTreeMap::new(),
        )));
        assert_eq!(
            representation_bindings(Some(&plain)),
            representation_bindings(None)
        );
    }
}
