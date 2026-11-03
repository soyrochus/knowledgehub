use super::*;
use crate::Error;
use std::collections::{BTreeMap, BTreeSet};

/// One bounded matcher shared by in-memory validation and storage-bound reads.
/// The caller supplies exact immutable schema lookup in its selected snapshot.
pub(crate) fn matches_type(
    actual: &ExactRef,
    wanted: &ExactRef,
    inclusive: bool,
    mut lookup: impl FnMut(&ExactRef) -> Result<TypeDefinition, Error>,
) -> Result<bool, Error> {
    if !inclusive {
        return Ok(actual == wanted);
    }
    let mut pending = vec![actual.clone()];
    let mut seen = BTreeSet::new();
    while let Some(r) = pending.pop() {
        if !seen.insert(r.clone()) {
            continue;
        }
        if seen.len() > 1000 {
            return Err(Error::Capacity);
        }
        let t = lookup(&r)?;
        if r == *wanted {
            return Ok(true);
        }
        pending.extend(t.parents);
    }
    Ok(false)
}

pub(crate) fn applicable(state: &State, reference: &ExactRef) -> Option<Vec<ExactRef>> {
    fn visit(
        state: &State,
        r: &ExactRef,
        active: &mut BTreeSet<ExactRef>,
        done: &mut BTreeSet<ExactRef>,
        out: &mut Vec<ExactRef>,
    ) -> Option<()> {
        if active.contains(r) || active.len() > 64 || done.len() > 1000 {
            return None;
        }
        if done.contains(r) {
            return Some(());
        }
        let t = state.type_definition(r)?;
        active.insert(r.clone());
        for p in &t.parents {
            // Native payload kinds cannot be extended or acquire application parents.
            if t.native || state.type_definition(p)?.native {
                return None;
            }
            visit(state, p, active, done, out)?;
        }
        active.remove(r);
        done.insert(r.clone());
        out.push(r.clone());
        Some(())
    }
    let mut out = vec![];
    visit(
        state,
        reference,
        &mut BTreeSet::new(),
        &mut BTreeSet::new(),
        &mut out,
    )?;
    Some(out)
}

pub(crate) fn fields(state: &State, reference: &ExactRef) -> Option<BTreeMap<String, Property>> {
    Some(
        declared_fields(reference, |r| {
            state.type_definition(r).cloned().ok_or(Error::NotFound)
        })
        .ok()?
        .into_iter()
        .map(|(n, (_, p))| (n, p))
        .collect(),
    )
}

/// Resolve each field once by exact declaration origin, also for indexed reads.
pub(crate) fn declared_fields(
    reference: &ExactRef,
    mut lookup: impl FnMut(&ExactRef) -> Result<TypeDefinition, Error>,
) -> Result<BTreeMap<String, (ExactRef, Property)>, Error> {
    fn visit(
        r: &ExactRef,
        lookup: &mut dyn FnMut(&ExactRef) -> Result<TypeDefinition, Error>,
        active: &mut BTreeSet<ExactRef>,
        done: &mut BTreeSet<ExactRef>,
        out: &mut BTreeMap<String, (ExactRef, Property)>,
    ) -> Result<(), Error> {
        if active.contains(r) {
            return Err(Error::InvalidInput(format!(
                "cyclic type parent {}",
                r.key()
            )));
        }
        if done.contains(r) {
            return Ok(());
        }
        if active.len() > 64 || done.len() > 1000 {
            return Err(Error::Capacity);
        }
        let t = lookup(r)?;
        active.insert(r.clone());
        for p in &t.parents {
            if t.native || lookup(p)?.native {
                return Err(Error::InvalidInput(format!(
                    "native parent boundary {} -> {}",
                    r.key(),
                    p.key()
                )));
            }
            visit(p, lookup, active, done, out)?;
        }
        for (name, property) in t.properties {
            if let Some((origin, _)) = out.get(&name) {
                return Err(Error::InvalidInput(format!(
                    "ambiguous/shadowed field {name}: {} and {}",
                    origin.key(),
                    r.key()
                )));
            }
            out.insert(name, (r.clone(), property));
        }
        active.remove(r);
        done.insert(r.clone());
        Ok(())
    }
    let mut out = BTreeMap::new();
    visit(
        reference,
        &mut lookup,
        &mut BTreeSet::new(),
        &mut BTreeSet::new(),
        &mut out,
    )?;
    Ok(out)
}

pub(crate) fn accepts(state: &State, subject: &ExactRef, selectors: &[TypeSelector]) -> bool {
    let Some(record) = state.resolve(subject) else {
        return false;
    };
    selectors.iter().any(|selector| match selector {
        TypeSelector::Role(SubjectRole::AnySubject) => true,
        TypeSelector::Role(SubjectRole::ElementRevision) => !matches!(
            record.data,
            RecordData::Assertion(_) | RecordData::Identity { .. } | RecordData::Anchor(_)
        ),
        TypeSelector::Type(wanted) => matches_type(&record.type_ref, wanted, true, |r| {
            state.type_definition(r).cloned().ok_or(Error::NotFound)
        })
        .unwrap_or(false),
    })
}
