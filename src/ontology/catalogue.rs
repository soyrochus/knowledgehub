use super::*;
use crate::{Content, Error, codec};
use serde::Deserialize;
use std::collections::BTreeMap;

pub fn type_ref(name: &str) -> ExactRef {
    ExactRef::new("sdo", format!("sdo:type:{name}"), "catalogue-3")
}
pub fn relationship_ref(source: &str, verb: &str) -> ExactRef {
    ExactRef::new(
        "sdo",
        format!("sdo:relationship:{source}.{verb}"),
        "catalogue-3",
    )
}
pub fn ontology_ref() -> ExactRef {
    ExactRef::new("sdo", "sdo:ontology:system-definition", "catalogue-3")
}
pub fn profile_ref() -> ExactRef {
    ExactRef::new("sdo", "sdo:profile:system-definition", "catalogue-3")
}
pub fn self_profile_ref() -> ExactRef {
    ExactRef::new(
        "sdo",
        "sdo:profile:knowledgehub-self-description",
        "catalogue-3",
    )
}
#[derive(Clone, Debug)]
pub struct Catalogue {
    pub records: Vec<Record>,
    pub artifacts: BTreeMap<String, Content>,
}
impl Catalogue {
    pub fn operations(&self) -> Vec<Operation> {
        self.records
            .iter()
            .map(|r| {
                if let Some(c) = self.artifacts.get(&r.reference.key()) {
                    Operation::Capture {
                        record: r.clone(),
                        content: c.clone(),
                    }
                } else {
                    Operation::Put(r.clone())
                }
            })
            .chain([Operation::SelectProfile {
                ontology: ontology_ref(),
                profile: profile_ref(),
            }])
            .collect()
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Data {
    types: Vec<TypeDefinition>,
    relationships: Vec<RelationshipDefinition>,
    structures: BTreeMap<String, ValueKind>,
}
/// The versioned built-in data has fixed exact definition IDs across stores.
pub fn builtin_catalogue() -> Result<Catalogue, Error> {
    let data: Data = codec::decode(include_bytes!("catalogue.json"))?;
    let md = include_bytes!("../../source/system-definition-ontology-proposal.md");
    let pdf = include_bytes!("../../source/system-definition-ontology-proposal.pdf");
    let mut out = Catalogue {
        records: vec![],
        artifacts: BTreeMap::new(),
    };
    let anchor = ExactRef::new("sdo", "sdo:anchor:catalogue", "catalogue-3");
    for (id, bytes, media, origin) in [
        (
            "urn:source:ontology",
            md.as_slice(),
            "text/markdown",
            "source/system-definition-ontology-proposal.md",
        ),
        (
            "sdo:source:ontology-pdf",
            pdf.as_slice(),
            "application/pdf",
            "source/system-definition-ontology-proposal.pdf",
        ),
    ] {
        let source = ExactRef::new("sdo", id, "catalogue-3");
        let capture = ExactRef::new("sdo", format!("{id}:capture"), "catalogue-3");
        out.records.push(Record::new(
            source.clone(),
            type_ref("SourceArtifact"),
            RecordData::SourceArtifact(SourceArtifactData {
                origin: origin.into(),
                media_type: media.into(),
                owner: "source:author".into(),
                status: "authored-requirement".into(),
            }),
            vec![],
        )?);
        let content = Content::capture(bytes.to_vec());
        out.records.push(Record::new(
            capture.clone(),
            type_ref("SourceRevision"),
            RecordData::SourceRevision(SourceRevisionData {
                source,
                original: content.reference.clone(),
                media_type: media.into(),
                derived_from: None,
                derivation: None,
            }),
            vec![],
        )?);
        out.artifacts.insert(capture.key(), content);
    }
    let source = ExactRef::new("sdo", "urn:source:ontology:capture", "catalogue-3");
    out.records.push(Record::new(
        anchor.clone(),
        type_ref("SourceAnchor"),
        RecordData::Anchor(AnchorData {
            source,
            label: "Source draft 0.2, complete exact Markdown; definitions in §§5–17".into(),
            locator: Locator::ByteRange {
                start: 0,
                end: md.len() as u64,
                quote: String::from_utf8(md.to_vec()).unwrap(),
            },
        }),
        vec![],
    )?);
    let decision_anchor = ExactRef::new("sdo", "sdo:anchor:experience-contract", "catalogue-3");
    let logical_anchor = ExactRef::new("sdo", "sdo:anchor:logical-design-contract", "catalogue-3");
    for (name, decision) in [
        (
            "experience-contract",
            include_bytes!("../../design/experience-contract.md").as_slice(),
        ),
        (
            "logical-design-contract",
            include_bytes!("../../design/logical-design-contract.md").as_slice(),
        ),
    ] {
        let decision_source = ExactRef::new("sdo", format!("sdo:source:{name}"), "catalogue-3");
        let decision_capture =
            ExactRef::new("sdo", format!("sdo:source:{name}:capture"), "catalogue-3");
        let decision_anchor = ExactRef::new("sdo", format!("sdo:anchor:{name}"), "catalogue-3");
        out.records.push(Record::new(
            decision_source.clone(),
            type_ref("SourceArtifact"),
            RecordData::SourceArtifact(SourceArtifactData {
                origin: format!("design/{name}.md"),
                media_type: "text/markdown".into(),
                owner: "repository:knowledgehub".into(),
                status: "design-decision".into(),
            }),
            vec![],
        )?);
        let content = Content::capture(decision.to_vec());
        out.records.push(Record::new(
            decision_capture.clone(),
            type_ref("SourceRevision"),
            RecordData::SourceRevision(SourceRevisionData {
                source: decision_source,
                original: content.reference.clone(),
                media_type: "text/markdown".into(),
                derived_from: None,
                derivation: None,
            }),
            vec![],
        )?);
        out.artifacts.insert(decision_capture.key(), content);
        out.records.push(Record::new(
            decision_anchor.clone(),
            type_ref("SourceAnchor"),
            RecordData::Anchor(AnchorData {
                source: decision_capture,
                label: format!("Authored {name} refinement"),
                locator: Locator::ByteRange {
                    start: 0,
                    end: decision.len() as u64,
                    quote: String::from_utf8(decision.to_vec()).unwrap(),
                },
            }),
            vec![],
        )?);
    }
    let mut types = vec![];
    for t in data.types {
        let refined = matches!(
            t.name.as_str(),
            "StoryMap" | "UserActivity" | "OutcomeCut" | "StoryPlacement"
        );
        let r = type_ref(&t.name);
        types.push(r.clone());
        out.records.push(Record::new(
            r,
            type_ref("ElementTypeDefinition"),
            RecordData::Type(t),
            if refined {
                vec![
                    anchor.clone(),
                    decision_anchor.clone(),
                    logical_anchor.clone(),
                ]
            } else {
                vec![anchor.clone(), logical_anchor.clone()]
            },
        )?);
    }
    let mut relationships = vec![];
    for d in data.relationships {
        let refined = d.coverage.starts_with('E');
        let source = match &d.from[0] {
            TypeSelector::Type(r) => r.id.strip_prefix("sdo:type:").unwrap(),
            TypeSelector::Role(SubjectRole::ElementRevision) => "ElementRevision",
            TypeSelector::Role(SubjectRole::AnySubject) => "AnySubject",
        };
        let r = relationship_ref(source, &d.name);
        relationships.push(r.clone());
        out.records.push(Record::new(
            r,
            type_ref("RelationshipTypeDefinition"),
            RecordData::Relationship(d),
            if refined {
                vec![
                    anchor.clone(),
                    decision_anchor.clone(),
                    logical_anchor.clone(),
                ]
            } else {
                vec![anchor.clone(), logical_anchor.clone()]
            },
        )?);
    }
    let citation = match &out
        .records
        .iter()
        .find(|r| matches!(r.data, RecordData::Type(_)))
        .unwrap()
        .data
    {
        RecordData::Type(t) => t.citation.clone(),
        _ => unreachable!(),
    };
    let mut constraints = vec![];
    for n in 1..=17 {
        let r = ExactRef::new("sdo", format!("sdo:constraint:V-{n:02}"), "catalogue-3");
        constraints.push(r.clone());
        out.records.push(Record::new(
            r,
            type_ref("ConstraintDefinition"),
            RecordData::Constraint(ConstraintDefinition {
                name: format!("V-{n:02}"),
                coverage: format!("V-{n:02}"),
                rule: Constraint::Builtin(format!("V-{n:02}")),
                citation: citation.clone(),
            }),
            vec![anchor.clone()],
        )?);
    }
    out.records.push(Record::new(
        profile_ref(),
        type_ref("ProfileDefinition"),
        RecordData::Profile(ProfileDefinition {
            name: "system-definition-ontology/3".into(),
            ontology: ontology_ref(),
            vocabulary: types.clone(),
            allow_gaps: true,
            required_types: vec![],
            inapplicable: BTreeMap::new(),
            citation: citation.clone(),
        }),
        vec![anchor.clone()],
    )?);
    out.records.push(Record::new(self_profile_ref(),type_ref("ProfileDefinition"),RecordData::Profile(ProfileDefinition{name:"knowledgehub-self-description/3".into(),ontology:ontology_ref(),vocabulary:types.clone(),allow_gaps:true,required_types:vec!["SystemModel","BusinessGoal","Actor","BoundedContext","AggregateDefinition","Requirement","UseCase","FunctionDefinition","SoftwareComponent","Module","AuthorizationPolicy","DeliverySlice","TestCase","TestRun","EvidenceArtifact"].into_iter().map(str::to_owned).collect(),inapplicable:[("Interaction and visual design".into(),"Embedded library exposes no product screens; supported vocabulary is exercised by isolated conformance models.".into()),("AI participation".into(),"The library has no runtime AI engine; development assistance is not a runtime agent.".into())].into(),citation:citation.clone()}),vec![anchor.clone()])?);
    out.records.push(Record::new(
        ontology_ref(),
        type_ref("OntologyDefinition"),
        RecordData::Ontology(OntologyDefinition {
            name: "System Definition Ontology".into(),
            version: "system-definition-ontology/3".into(),
            types,
            relationships,
            constraints,
            profiles: vec![profile_ref(), self_profile_ref()],
            structures: data.structures,
            citation,
        }),
        vec![anchor],
    )?);
    Ok(out)
}

pub(crate) fn supports_role(state: &State, r: &ExactRef, allowed: &[TypeSelector]) -> bool {
    super::schema::accepts(state, r, allowed)
}
fn schema_references(kind: &ValueKind, out: &mut Vec<ExactRef>) {
    match kind {
        ValueKind::Reference(selectors) => out.extend(selectors.iter().filter_map(|s| {
            if let TypeSelector::Type(r) = s {
                Some(r.clone())
            } else {
                None
            }
        })),
        ValueKind::List { item, .. } | ValueKind::Set { item, .. } => schema_references(item, out),
        ValueKind::Record(fields) => fields
            .values()
            .for_each(|p| schema_references(&p.value, out)),
        ValueKind::Union(kinds) => kinds.iter().for_each(|k| schema_references(k, out)),
        _ => {}
    }
}
pub(crate) fn references(record: &Record) -> Vec<ExactRef> {
    let mut out = vec![record.type_ref.clone()];
    out.extend(record.predecessor.clone());
    out.extend(record.provenance.clone());
    match &record.data {
        RecordData::Element(e) => {
            for v in e.properties.values() {
                out.extend(v.refs().into_iter().cloned())
            }
        }
        RecordData::Assertion(a) => {
            out.extend([a.signature.clone(), a.from.clone(), a.to.clone()]);
            for v in a.qualifiers.values() {
                out.extend(v.refs().into_iter().cloned());
            }
        }
        RecordData::Ontology(o) => {
            out.extend(o.types.clone());
            out.extend(o.relationships.clone());
            out.extend(o.constraints.clone());
            out.extend(o.profiles.clone());
            o.structures
                .values()
                .for_each(|k| schema_references(k, &mut out));
        }
        RecordData::Type(t) => {
            out.extend(t.parents.clone());
            out.extend(t.constraints.clone());
            t.properties
                .values()
                .for_each(|p| schema_references(&p.value, &mut out));
        }
        RecordData::Relationship(d) => {
            schema_references(&ValueKind::Reference(d.from.clone()), &mut out);
            schema_references(&ValueKind::Reference(d.to.clone()), &mut out);
            d.qualifiers
                .values()
                .for_each(|p| schema_references(&p.value, &mut out));
        }
        RecordData::Profile(p) => {
            out.push(p.ontology.clone());
            out.extend(p.vocabulary.clone());
        }
        RecordData::Model(m) => out.extend([m.ontology.clone(), m.profile.clone()]),
        RecordData::Identity {
            subject,
            declared_type,
        } => out.extend([subject.clone(), declared_type.clone()]),
        RecordData::SourceRevision(s) => {
            out.push(s.source.clone());
            out.extend(s.derived_from.clone());
        }
        RecordData::Anchor(a) => {
            out.push(a.source.clone());
            if let Locator::Manifest { manifest, .. } = &a.locator {
                out.push(manifest.clone())
            }
        }
        RecordData::Interpretation(i) => {
            out.extend([i.subject.clone(), i.anchor.clone()]);
            out.extend(i.review.clone())
        }
        RecordData::Representation(r) => out.extend([r.subject.clone(), r.anchor.clone()]),
        RecordData::Baseline(b) => {
            out.extend([b.ontology.clone(), b.profile.clone()]);
            out.extend(b.selected.clone());
            out.extend(b.supporting.clone());
            out.extend(b.parents.clone());
        }
        RecordData::Review(r) => out.extend(r.targets.clone()),
        _ => {}
    }
    out
}
