//! Shared public-contract fixture. The model is always submitted through ordinary operations.
#![allow(dead_code)]
use knowledgehub::*;
use std::sync::Arc;

pub struct Allow;
impl Policy for Allow {
    fn allows(&self, _: &str, _: &Scope, _: Action) -> Result<bool, PolicyFailure> {
        Ok(true)
    }
}
pub fn context() -> Context {
    Context::new("host:maintainer", Arc::new(Allow))
}
pub const ONTOLOGY: &[u8] = include_bytes!("../../source/system-definition-ontology-proposal.md");

pub fn scope() -> Scope {
    Scope {
        hub: "urn:knowledgehub:self-hub".into(),
        model: "urn:knowledgehub:library".into(),
        branch: "main".into(),
        owner: "host:maintainer".into(),
        profile: Profile::seed(RevisionRef {
            id: "urn:source:ontology".into(),
            revision: ContentRef::of(ONTOLOGY).sha256,
        }),
    }
}

pub fn seed(scope: Scope, head: SnapshotId, operation_id: &str) -> ChangeSet {
    let text = std::str::from_utf8(ONTOLOGY).unwrap();
    let start = text.find("### 21.4 Change sets").unwrap();
    let end = text.find("### 21.5 Default").unwrap();
    let source = Source::capture(
        scope.profile.ontology.id.clone(),
        ONTOLOGY.to_vec(),
        vec![Anchor {
            id: "atomic-change".into(),
            start: start as u64,
            end: end as u64,
            quote: text[start..end].into(),
            section: "21.4 Change sets and consistency guarantees".into(),
        }],
    );
    let mut bytes = b"# KnowledgeHub first self-definition\n\n".to_vec();
    let mut fragments = Vec::new();
    for (id, value) in [
        ("requirement-title", "Atomic model changes"),
        (
            "requirement-statement",
            "An admitted change commits definitions, provenance, head and original receipt together, preserving exact revisions and guarded intent.",
        ),
        ("use-case-title", "Commit Change Set"),
        ("actor", "Authorised host maintainer"),
        (
            "trigger",
            "Submit a coherent sourced definition against an expected head",
        ),
        (
            "outcome",
            "A durable, attributable receipt identifies the complete accepted snapshot; uncertain callers resolve the original operation.",
        ),
    ] {
        bytes.extend_from_slice(format!("## {id}\n\n").as_bytes());
        let start = bytes.len();
        bytes.extend_from_slice(value.as_bytes());
        fragments.push(Fragment {
            id: id.into(),
            start: start as u64,
            end: bytes.len() as u64,
        });
        bytes.extend_from_slice(b"\n\n");
    }
    let document = Document::new("urn:document:knowledgehub-seed", bytes, fragments).unwrap();
    let interpretation = Interpretation { reference: RevisionRef { id: "urn:interpretation:atomic-change".into(), revision: String::new() },
        source: source.reference.clone(), anchor: "atomic-change".into(), text: "The proposed commit use case satisfies the atomic-change requirement. This is authored intent, without review approval or execution evidence.".into(),
        interpreter: "host:maintainer".into(), method: "manual source interpretation".into(), status: InterpretationStatus::Authored }.seal().unwrap();
    let narrative = |field: &str, fragment: &str| AuthorityBinding {
        field: field.into(),
        owner: Owner::Document(FragmentRef {
            document: document.reference.clone(),
            fragment: fragment.into(),
        }),
    };
    let graph = |fields: &[&str]| -> Vec<_> {
        fields
            .iter()
            .map(|field| AuthorityBinding {
                field: (*field).into(),
                owner: Owner::Graph,
            })
            .collect()
    };
    let mut req_owners = graph(&["id", "revision", "type", "scope", "interpretation"]);
    req_owners.extend([
        narrative("title", "requirement-title"),
        narrative("statement", "requirement-statement"),
    ]);
    let requirement = Element {
        reference: RevisionRef {
            id: "urn:requirement:atomic-change".into(),
            revision: String::new(),
        },
        element_type: scope.profile.definition("Requirement").unwrap(),
        interpretation: interpretation.reference.clone(),
        authority: req_owners,
    }
    .seal()
    .unwrap();
    let mut uc_owners = graph(&["id", "revision", "type", "scope", "interpretation"]);
    uc_owners.extend([
        narrative("title", "use-case-title"),
        narrative("actor", "actor"),
        narrative("trigger", "trigger"),
        narrative("outcome", "outcome"),
    ]);
    let use_case = Element {
        reference: RevisionRef {
            id: "urn:use-case:commit-change-set".into(),
            revision: String::new(),
        },
        element_type: scope.profile.definition("UseCase").unwrap(),
        interpretation: interpretation.reference.clone(),
        authority: uc_owners,
    }
    .seal()
    .unwrap();
    let assertion = Assertion {
        reference: RevisionRef {
            id: "urn:assertion:commit-satisfies-atomic-change".into(),
            revision: String::new(),
        },
        relationship_type: scope.profile.definition("satisfies").unwrap(),
        from: use_case.reference.clone(),
        to: requirement.reference.clone(),
        interpretation: interpretation.reference.clone(),
        authority: graph(&[
            "id",
            "revision",
            "type",
            "scope",
            "from",
            "to",
            "interpretation",
        ]),
    }
    .seal()
    .unwrap();
    ChangeSet {
        scope,
        operation_id: operation_id.into(),
        expected_head: head,
        guards: vec![RevisionGuard {
            id: requirement.reference.id.clone(),
            expected: None,
        }],
        operations: vec![
            Operation::CaptureSource(source),
            Operation::AddDocument(document),
            Operation::Interpret(interpretation),
            Operation::Define(requirement),
            Operation::Define(use_case),
            Operation::Assert(assertion),
        ],
    }
}

/// A later additive change exercises head movement and receipt retention without successor authoring.
pub fn additional_document(scope: Scope, head: SnapshotId, n: usize) -> ChangeSet {
    let document = Document::new(
        format!("urn:document:extra:{n}"),
        format!("# Additional authored material {n}\n").into_bytes(),
        vec![],
    )
    .unwrap();
    ChangeSet {
        scope,
        expected_head: head,
        operation_id: format!("additional:{n}"),
        guards: vec![],
        operations: vec![Operation::AddDocument(document)],
    }
}
