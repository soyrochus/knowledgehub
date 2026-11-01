#[path = "neighbour_support/mod.rs"]
mod fixture;
use knowledgehub::{Error, ontology::*};
use std::{collections::BTreeSet, path::PathBuf, time::Instant};
fn query() -> ModelImpact {
    ModelImpact {
        selection: None,
        starts: ImpactStarts::Text {
            selector: TextSelector {
                phrase: "increase the rent".into(),
                mode: LiteralMode::AsciiFold,
                fields: BTreeSet::from([TextField {
                    type_ref: fixture::node_type(),
                    path: "/name".into(),
                }]),
            },
        },
        match_policy: MatchPolicy::RequireOne,
        plan: TraversalPlan::Repeated {
            alternatives: vec![TraversalStep {
                direction: NeighbourDirection::Outgoing,
                selectors: BTreeSet::from([NeighbourSelector::Assertion {
                    definition: fixture::relation(),
                }]),
                categories: BTreeSet::from([NeighbourCategory::Association]),
                opposite_type: None,
                include_subtypes: false,
            }],
        },
        target: TargetPredicate {
            ids: BTreeSet::from([fixture::reference("v6")]),
            ..Default::default()
        },
        fact_scope: FactScope::Selected,
        projection: ImpactProjection {
            target_records: true,
            ..Default::default()
        },
        budgets: ReadBudgets::default(),
    }
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    let root = PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("absolute absent benchmark store directory"),
    );
    let (b, h) = fixture::create(&root).await?;
    let mut start = fixture::node("entry", "b", false);
    let RecordData::Element(e) = &mut start.data else {
        unreachable!()
    };
    e.properties.insert(
        "name".into(),
        Value::text("As a User I want to increase the rent"),
    );
    start.seal()?;
    let mut ops = vec![Operation::Put(start)];
    let mut from = "entry".to_owned();
    for i in 0..7 {
        let to = format!("v{i}");
        ops.push(Operation::Put(fixture::node(&to, "b", false)));
        ops.push(Operation::Put(fixture::edge(
            &format!("route:{i}"),
            &from,
            &to,
        )));
        from = to;
    }
    fixture::commit(&h, "route", ops).await?;
    let q = query();
    let mut rows = Vec::new();
    let mut previous = 0;
    for total in [0, 100, 1000] {
        if total > 0 {
            fixture::commit(
                &h,
                &format!("unrelated:{total}"),
                (previous..total)
                    .map(|i| {
                        Operation::Put(fixture::node(&format!("unrelated:{i:05}"), "b", false))
                    })
                    .collect(),
            )
            .await?;
        }
        // Cold/adapted preparation is measured independently from one ready public request.
        let prepare = h.model_impact(fixture::context(), q.clone()).await?;
        let before = b.file_work();
        let t = Instant::now();
        let p = h.model_impact(fixture::context(), q.clone()).await?;
        assert_eq!(p.dependencies.len(), 1);
        assert!(p.continuation.is_none());
        assert_eq!(
            b.file_work().state_materialisations,
            before.state_materialisations
        );
        rows.push(serde_json::json!({"unrelated":total,"public_requests":1,"ready_microseconds":t.elapsed().as_micros() as u64,"work":p.work,"preparation":prepare.work,"response_bytes":serde_json::to_vec(&p).unwrap().len(),"full_state_loads":b.file_work().state_materialisations-before.state_materialisations}));
        previous = total;
    }
    fixture::commit(
        &h,
        "fanout",
        (0..1000)
            .map(|i| Operation::Put(fixture::edge(&format!("fanout:{i:04}"), "entry", "b")))
            .collect(),
    )
    .await?;
    let mut fanout = q.clone();
    fanout.budgets.examined = 100;
    let TraversalPlan::Repeated { alternatives } = &mut fanout.plan else {
        unreachable!()
    };
    alternatives[0].opposite_type = Some(type_ref("SourceArtifact"));
    let p = h.model_impact(fixture::context(), fanout).await?;
    assert!(!p.exhausted);
    assert!(p.reached.contains(&ReadLimit::Examined));
    println!("{}",serde_json::to_string_pretty(&serde_json::json!({"rows":rows,"filtered_fanout":{"edges":1000,"work":p.work,"reached":p.reached,"exhausted":p.exhausted}})).unwrap());
    Ok(())
}
