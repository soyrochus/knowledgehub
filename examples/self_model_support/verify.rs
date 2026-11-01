//! Actual library-model indexed reads; fictional domain fixtures are separate.
use super::*;
use knowledgehub::{Context, backends::files::FileBackend};
use serde_json::json;
use std::{collections::BTreeSet, time::Instant};

pub async fn indexed(
    h: &OntologyHub,
    backend: &FileBackend,
    context: Context,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let selection = Selection::Baseline(r("kh:baseline:B"));
    let subject = ExactRef::new("knowledgehub", "urn:requirement:atomic-change", "self-2");
    let mut neighbours = Neighbours::new(selection.clone(), vec![subject.clone()]);
    neighbours.direction = NeighbourDirection::Incoming;
    neighbours.selectors = BTreeSet::from([NeighbourSelector::Assertion {
        definition: relationship_ref("UseCase", "satisfies"),
    }]);
    neighbours.preparation.owner_limit = 100000;
    neighbours.preparation.allow_rebuild = false;
    let cold = match h.neighbours(context.clone(), neighbours.clone()).await {
        Ok(p) => json!({"ready":true,"work":p.work}),
        Err(knowledgehub::Error::IndexNotReady { reason }) => {
            json!({"ready":false,"reason":reason})
        }
        Err(e) => return Err(e.into()),
    };
    neighbours.preparation.allow_rebuild = true;
    let t = Instant::now();
    let prepared = h
        .prepare_neighbours(context.clone(), neighbours.clone())
        .await?;
    let preparation_us = t.elapsed().as_micros() as u64;
    neighbours.preparation.allow_rebuild = false;
    let t = Instant::now();
    let warm = h.neighbours(context.clone(), neighbours.clone()).await?;
    let warm_us = t.elapsed().as_micros() as u64;
    assert!(
        warm.neighbours
            .iter()
            .any(|n| n.opposite == r("urn:use-case:commit-change-set"))
    );
    let text = TextSelector {
        phrase: "checks current authority".into(),
        mode: LiteralMode::AsciiFold,
        fields: BTreeSet::from([TextField {
            type_ref: type_ref("Requirement"),
            path: "/statement".into(),
        }]),
    };
    let mut budgets = ReadBudgets::default();
    budgets.preparation.owner_limit = 100000;
    let search = h
        .search_text(
            context.clone(),
            SearchText {
                selection: selection.clone(),
                text: text.clone(),
                budgets: budgets.clone(),
            },
        )
        .await?;
    assert!(search.hits.iter().any(|s| s.record == subject));
    budgets.preparation.allow_rebuild = false;
    let step = |source: &str, verb: &str, direction| TraversalStep {
        direction,
        selectors: BTreeSet::from([NeighbourSelector::Assertion {
            definition: relationship_ref(source, verb),
        }]),
        categories: BTreeSet::from([NeighbourCategory::Association]),
        opposite_type: None,
        include_subtypes: false,
    };
    let q = ModelImpact {
        selection: Some(selection),
        starts: ImpactStarts::Text { selector: text },
        match_policy: MatchPolicy::AllGrouped,
        plan: TraversalPlan::Ordered {
            steps: vec![
                vec![step("UseCase", "satisfies", NeighbourDirection::Incoming)],
                vec![step("UseCase", "invokes", NeighbourDirection::Outgoing)],
                vec![step(
                    "FunctionDefinition",
                    "operatesOn",
                    NeighbourDirection::Outgoing,
                )],
            ],
        },
        target: TargetPredicate {
            type_ref: Some(type_ref("AggregateDefinition")),
            ids: BTreeSet::from([r("kh:aggregate:model-branch")]),
            include_subtypes: false,
        },
        fact_scope: FactScope::Selected,
        projection: ImpactProjection {
            target_records: true,
            witness_records: true,
            snippets: true,
        },
        budgets,
    };
    let before = backend.file_work();
    let t = Instant::now();
    let composite = h.model_impact(context.clone(), q.clone()).await?;
    let composite_us = t.elapsed().as_micros() as u64;
    assert!(composite.dependencies.iter().any(|d| d.start == subject
        && d.target == r("kh:aggregate:model-branch")
        && d.witnesses[0].len() == 3));
    assert!(composite.exhausted);
    let again = h.model_impact(context, q).await?;
    assert_eq!(composite.dependencies, again.dependencies);
    assert_eq!(composite.hits, again.hits);
    assert_eq!(composite.coverage, again.coverage);
    let full_loads = backend.file_work().state_materialisations - before.state_materialisations;
    assert_eq!(full_loads, 0);
    Ok(
        json!({"cold_neighbours":cold,"preparation":prepared,"preparation_microseconds":preparation_us,"warm_neighbours":warm,"warm_microseconds":warm_us,"search":search,"composite":composite,"composite_microseconds":composite_us,"repeated_composite_work":again.work,"repeated_exact_result":true,"composite_full_state_loads":full_loads,"adapter_work":backend.file_work(),"meaning":"candidate_dependency; actual library definitions; no fictional rent concepts"}),
    )
}
