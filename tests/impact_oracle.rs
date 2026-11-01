//! Tiny independent assertion-path oracle. Never uses production extraction or indexes.
#![allow(dead_code)]
use knowledgehub::ontology::*;
use std::collections::{BTreeMap, BTreeSet};
pub fn paths(
    state: &State,
    starts: &[ExactRef],
    steps: &[NeighbourDirection],
    signature: &ExactRef,
) -> BTreeMap<ExactRef, Vec<Vec<ExactRef>>> {
    let mut frontier = starts
        .iter()
        .map(|s| (s.clone(), Vec::<ExactRef>::new()))
        .collect::<Vec<_>>();
    let mut result = BTreeMap::<ExactRef, Vec<Vec<ExactRef>>>::new();
    for direction in steps {
        let mut next = Vec::new();
        for (subject, path) in frontier {
            for r in state.selected_records() {
                if let RecordData::Assertion(a) = &r.data {
                    if &a.signature != signature {
                        continue;
                    }
                    let opposite =
                        if a.from == subject && *direction != NeighbourDirection::Incoming {
                            Some(&a.to)
                        } else if a.to == subject && *direction != NeighbourDirection::Outgoing {
                            Some(&a.from)
                        } else {
                            None
                        };
                    if let Some(to) = opposite {
                        let mut p = path.clone();
                        p.push(r.reference.clone());
                        next.push((to.clone(), p.clone()));
                        let found = result.entry(to.clone()).or_default();
                        if found.first().is_none_or(|old| old.len() == p.len()) {
                            found.push(p);
                        }
                    }
                }
            }
        }
        frontier = next;
    }
    for v in result.values_mut() {
        v.sort();
        v.dedup();
    }
    result
}
pub fn memberships(p: &ImpactPage) -> BTreeSet<ExactRef> {
    p.dependencies.iter().map(|d| d.target.clone()).collect()
}
