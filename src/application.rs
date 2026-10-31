use crate::{backends::files::FileBackend, jobs::Jobs, model, storage::Storage, *};
use std::sync::Arc;

/// Owned application handle. Every new operation checks the host's current scoped policy.
#[derive(Clone)]
pub struct KnowledgeHub {
    scope: Scope,
    store: Arc<dyn Storage>,
    jobs: Arc<Jobs>,
}

impl KnowledgeHub {
    pub async fn create(
        backend: FileBackend,
        context: Context,
        scope: Scope,
    ) -> Result<Self, Error> {
        if backend.store.version() != FILE_VERSION || scope.profile.version != PROFILE_VERSION {
            return Err(Error::Incompatible(
                "seed API requires files/1 and seed profile".into(),
            ));
        }
        let hub = Self {
            scope,
            store: backend.store,
            jobs: backend.jobs,
        };
        let h = hub.clone();
        hub.jobs
            .run(move || {
                model::check_scope(&h.scope)?;
                context.check(&h.scope, Action::Create)?;
                h.store
                    .create(model::empty(h.scope.clone(), h.store.allocate_id()?))
            })
            .await?;
        Ok(hub)
    }
    pub async fn open(backend: FileBackend, context: Context, scope: Scope) -> Result<Self, Error> {
        if backend.store.version() != FILE_VERSION || scope.profile.version != PROFILE_VERSION {
            return Err(Error::Incompatible(
                "seed API requires files/1 and seed profile".into(),
            ));
        }
        let hub = Self {
            scope,
            store: backend.store,
            jobs: backend.jobs,
        };
        let h = hub.clone();
        hub.jobs
            .run(move || {
                model::check_scope(&h.scope)?;
                context.check(&h.scope, Action::Open)?;
                h.store.acquire(&h.scope)?.recover()?;
                Ok(())
            })
            .await?;
        Ok(hub)
    }
    pub fn scope(&self) -> Scope {
        self.scope.clone()
    }

    pub async fn head(&self, context: Context) -> Result<SnapshotId, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope, Action::Read)?;
                Ok(h.store
                    .acquire(&h.scope)?
                    .recover()?
                    .current()
                    .snapshot
                    .id
                    .clone())
            })
            .await
    }
    pub async fn read_snapshot(
        &self,
        context: Context,
        snapshot: SnapshotId,
    ) -> Result<Snapshot, Error> {
        self.select_snapshot(context, snapshot, Ok).await
    }
    async fn select_snapshot<T, F>(
        &self,
        context: Context,
        snapshot: SnapshotId,
        select: F,
    ) -> Result<T, Error>
    where
        T: Send + 'static,
        F: FnOnce(Snapshot) -> Result<T, Error> + Send + 'static,
    {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope, Action::Read)?;
                let history = h.store.acquire(&h.scope)?.recover()?;
                let snapshot = history
                    .entries
                    .into_iter()
                    .find(|s| s.snapshot.id == snapshot)
                    .map(|s| s.snapshot)
                    .ok_or(Error::NotFound)?;
                select(snapshot)
            })
            .await
    }
    pub async fn read_element(
        &self,
        context: Context,
        snapshot: SnapshotId,
        reference: RevisionRef,
    ) -> Result<Element, Error> {
        self.select_snapshot(context, snapshot, move |s| {
            s.elements
                .into_iter()
                .find(|e| e.reference == reference)
                .ok_or(Error::NotFound)
        })
        .await
    }
    pub async fn read_assertion(
        &self,
        context: Context,
        snapshot: SnapshotId,
        reference: RevisionRef,
    ) -> Result<Assertion, Error> {
        self.select_snapshot(context, snapshot, move |s| {
            s.assertions
                .into_iter()
                .find(|e| e.reference == reference)
                .ok_or(Error::NotFound)
        })
        .await
    }
    pub async fn read_source(
        &self,
        context: Context,
        snapshot: SnapshotId,
        reference: RevisionRef,
    ) -> Result<Source, Error> {
        self.select_snapshot(context, snapshot, move |s| {
            s.sources
                .into_iter()
                .find(|e| e.reference == reference)
                .ok_or(Error::NotFound)
        })
        .await
    }
    pub async fn read_document(
        &self,
        context: Context,
        snapshot: SnapshotId,
        reference: RevisionRef,
    ) -> Result<Document, Error> {
        self.select_snapshot(context, snapshot, move |s| {
            s.documents
                .into_iter()
                .find(|e| e.reference == reference)
                .ok_or(Error::NotFound)
        })
        .await
    }
    /// A report is bound to this request's exact base. It is never an admission token.
    pub async fn validate(
        &self,
        context: Context,
        request: ChangeSet,
    ) -> Result<ValidationReport, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope, Action::Validate)?;
                if request.scope != h.scope {
                    return Err(Error::Incompatible("request scope differs".into()));
                }
                let history = h.store.acquire(&h.scope)?.recover()?;
                let base = history
                    .entries
                    .iter()
                    .find(|s| s.snapshot.id == request.expected_head)
                    .ok_or(Error::NotFound)?;
                Ok(model::candidate(&base.snapshot, &request)?.1)
            })
            .await
    }
    pub async fn commit(&self, context: Context, request: ChangeSet) -> Result<Receipt, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                // Before lookup, bytes are hashed only after current disclosure/admission checks.
                context.check(&h.scope, Action::Commit)?;
                context.check(&h.scope, Action::OperationResult)?;
                if request.scope != h.scope {
                    return Err(Error::Incompatible("request scope differs".into()));
                }
                let lease = h.store.acquire(&h.scope)?;
                // This current scoped check is the new publication's authorisation point.
                context.check(&h.scope, Action::Commit)?;
                context.check(&h.scope, Action::OperationResult)?;
                let history = lease.recover()?;
                let fingerprint = request.fingerprint(&context.actor)?;
                if let Some(original) = history.operation(&request.operation_id) {
                    if original.actor != context.actor || original.fingerprint != fingerprint {
                        return Err(Error::OperationReuse);
                    }
                    return Ok(original.clone());
                }
                let current = history.current();
                let plan = model::prepare(
                    &current.snapshot,
                    &request,
                    &context.actor,
                    h.store.allocate_id()?,
                )?;
                lease.publish(plan, current.reference.clone())
            })
            .await
    }
    pub async fn operation_result(
        &self,
        context: Context,
        operation_id: String,
    ) -> Result<OperationOutcome, Error> {
        let h = self.clone();
        self.jobs
            .run(move || {
                context.check(&h.scope, Action::OperationResult)?;
                if operation_id.trim().is_empty() {
                    return Err(Error::InvalidInput("operation identity required".into()));
                }
                let history = h.store.acquire(&h.scope)?.recover()?;
                match history.operation(&operation_id) {
                    Some(r) if r.actor == context.actor => {
                        Ok(OperationOutcome::Committed(Box::new(r.clone())))
                    }
                    Some(_) => Err(Error::OperationReuse),
                    None => Ok(OperationOutcome::Absent {
                        recovered_head: history.current().snapshot.id.clone(),
                    }),
                }
            })
            .await
    }
    /// Stops admission across clones and drains queued and running work. The host owns runtime shutdown.
    pub async fn drain(&self) {
        self.jobs.drain().await;
    }
}
