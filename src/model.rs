//! Current full-model publication invariants; no file or executor dependencies.
use crate::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Snapshot {
    pub id: SnapshotId,
    pub scope: ModelScope,
    pub sequence: u64,
    pub state: State,
}
pub(crate) struct ValidatedPlan {
    pub snapshot: Snapshot,
    pub receipt: Receipt,
}
pub(crate) fn check_scope(scope: &ModelScope) -> Result<(), Error> {
    if [&scope.hub, &scope.model, &scope.branch, &scope.owner]
        .iter()
        .any(|s| s.trim().is_empty() || s.contains('\0'))
    {
        return Err(Error::InvalidInput(
            "nonempty qualified model scope required".into(),
        ));
    }
    Ok(())
}
pub(crate) fn hash(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub(crate) fn empty(scope: ModelScope, id: SnapshotId) -> Snapshot {
    Snapshot {
        state: State::empty(scope.clone()),
        scope,
        id,
        sequence: 0,
    }
}
pub(crate) fn validate_snapshot(snapshot: &Snapshot) -> Vec<Violation> {
    let mut out = assess(&snapshot.state).violations;
    if snapshot.scope != snapshot.state.scope {
        out.push(Violation {
            rule: "KH-DR-04".into(),
            subject: snapshot.id.0.clone(),
            message: "snapshot scope differs from state".into(),
        });
    }
    out
}
pub(crate) fn verify_affected(parent: &Snapshot, next: &Snapshot, receipt: &Receipt) -> bool {
    let mut added = next
        .state
        .records
        .iter()
        .filter(|(key, _)| !parent.state.records.contains_key(*key))
        .map(|(_, r)| r.reference.clone())
        .collect::<Vec<_>>();
    let mut affected = receipt.affected.clone();
    added.sort();
    affected.sort();
    crate::ontology::history_valid(&parent.state, &next.state) && added == affected
}
