use super::*;
use crate::{
    Action, Context, DenyAll, Error, Policy, PolicyFailure, Scope, neighbour_test_support as f,
    ontology::*, test_support::Sandbox,
};
use std::{
    collections::BTreeSet,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
fn request() -> ModelImpact {
    ModelImpact {
        selection: None,
        starts: ImpactStarts::Exact {
            references: vec![f::reference("a")],
        },
        match_policy: MatchPolicy::AllGrouped,
        plan: TraversalPlan::Repeated {
            alternatives: vec![TraversalStep {
                direction: NeighbourDirection::Outgoing,
                selectors: BTreeSet::from([NeighbourSelector::Assertion {
                    definition: f::relation(),
                }]),
                categories: BTreeSet::from([NeighbourCategory::Association]),
                opposite_type: None,
                include_subtypes: false,
            }],
        },
        target: TargetPredicate {
            type_ref: Some(f::node_type()),
            ..Default::default()
        },
        fact_scope: FactScope::Selected,
        projection: ImpactProjection::default(),
        budgets: ReadBudgets {
            page_size: 1,
            ..Default::default()
        },
    }
}
struct Switch(AtomicBool);
impl Policy for Switch {
    fn allows(&self, _: &str, _: &Scope, _: Action) -> Result<bool, PolicyFailure> {
        Ok(self.0.load(Ordering::SeqCst))
    }
}
#[tokio::test]
async fn composite_cursors_share_capacity_expire_and_recheck_disclosure() {
    let s = Sandbox::new();
    let (b, h) = f::create(&s.store).await.unwrap();
    f::commit(
        &h,
        "edges",
        vec![
            Operation::Put(f::edge("ab", "a", "b")),
            Operation::Put(f::edge("ac", "a", "c")),
        ],
    )
    .await
    .unwrap();
    let q = request();
    let token = h
        .model_impact(f::context(), q.clone())
        .await
        .unwrap()
        .continuation
        .unwrap();
    assert!(matches!(
        h.continue_model_impact(
            Context::new("fixture:neighbour-author", Arc::new(DenyAll)),
            q.clone(),
            token.clone()
        )
        .await,
        Err(Error::Denied { .. })
    ));
    assert_eq!(
        h.continue_model_impact(
            Context::new("someone:else", Arc::new(Switch(AtomicBool::new(true)))),
            q.clone(),
            token.clone()
        )
        .await
        .unwrap_err(),
        Error::InvalidCursor
    );
    // Neither legacy cursor API can consume a composite token.
    let head = h.head(f::context()).await.unwrap();
    assert_eq!(
        h.continue_query(
            f::context(),
            Query::select(Selection::Snapshot(head)),
            token.clone()
        )
        .await
        .unwrap_err(),
        Error::InvalidCursor
    );
    h.cursors.offset.store(1799, Ordering::SeqCst);
    assert!(
        h.continue_model_impact(f::context(), q.clone(), token)
            .await
            .is_ok()
    );
    let token = h
        .model_impact(f::context(), q.clone())
        .await
        .unwrap()
        .continuation
        .unwrap();
    h.cursors.offset.store(3599, Ordering::SeqCst);
    assert_eq!(
        h.continue_model_impact(f::context(), q.clone(), token)
            .await
            .unwrap_err(),
        Error::InvalidCursor
    );
    let mut tokens = Vec::new();
    for _ in 0..64 {
        tokens.push(
            h.model_impact(f::context(), q.clone())
                .await
                .unwrap()
                .continuation
                .unwrap(),
        );
    }
    assert_eq!(
        h.model_impact(f::context(), q.clone()).await.unwrap_err(),
        Error::Capacity
    );
    let mut n = Neighbours::new(
        Selection::Snapshot(h.head(f::context()).await.unwrap()),
        vec![f::reference("a")],
    );
    n.page_size = 1;
    assert_eq!(
        h.neighbours(f::context(), n).await.unwrap_err(),
        Error::Capacity
    );
    for token in tokens {
        h.release_cursor(f::context(), token).await.unwrap();
    }
    let switch = Arc::new(Switch(AtomicBool::new(true)));
    let context = Context::new("fixture:neighbour-author", switch.clone());
    let token = h
        .model_impact(context.clone(), q.clone())
        .await
        .unwrap()
        .continuation
        .unwrap();
    switch.0.store(false, Ordering::SeqCst);
    assert!(matches!(
        h.continue_model_impact(context, q.clone(), token).await,
        Err(Error::Denied { .. })
    ));
    b.drain().await;
}
#[tokio::test]
async fn limits_count_filtered_and_duplicate_work_and_bound_every_response() {
    let s = Sandbox::new();
    let (_, h) = f::create(&s.store).await.unwrap();
    let mut ops = vec![
        Operation::Put(f::edge("ab", "a", "b")),
        Operation::Put(f::edge("bc", "b", "c")),
        Operation::Put(f::edge("ba", "b", "a")),
    ];
    ops.extend((0..20).map(|i| Operation::Put(f::edge(&format!("parallel:{i}"), "a", "b"))));
    f::commit(&h, "graph", ops).await.unwrap();
    for (limit, mut q) in [
        (ReadLimit::States, request()),
        (ReadLimit::Hops, request()),
        (ReadLimit::Targets, request()),
    ] {
        q.budgets.page_size = 100;
        match limit {
            ReadLimit::States => q.budgets.states = 1,
            ReadLimit::Hops => q.budgets.hops = 1,
            ReadLimit::Targets => q.budgets.targets = 1,
            _ => unreachable!(),
        }
        let p = h.model_impact(f::context(), q).await.unwrap();
        assert!(p.reached.contains(&limit), "{limit:?}: {:?}", p.reached);
        assert!(!p.exhausted);
    }
    let mut q = request();
    q.budgets.examined = 10;
    let p = h.model_impact(f::context(), q).await.unwrap();
    assert!(p.reached.contains(&ReadLimit::Examined));
    assert!(p.work.examined_associations + p.work.expansion_attempts <= 10);
    let mut q = request();
    q.plan = TraversalPlan::Ordered {
        steps: vec![
            vec![match q.plan {
                TraversalPlan::Repeated { mut alternatives } => alternatives.remove(0),
                _ => unreachable!(),
            }];
            3
        ],
    };
    q.target.ids = BTreeSet::from([f::reference("c")]);
    let p = h.model_impact(f::context(), q).await.unwrap();
    assert!(p.dependencies.iter().any(|d| d.witnesses[0].len() == 2));
}
