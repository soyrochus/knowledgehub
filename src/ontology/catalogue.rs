use super::*;
use crate::{Content, Error, codec};
use serde::Deserialize;
use std::collections::BTreeMap;

pub fn type_ref(name: &str) -> ExactRef {
    ExactRef::new("sdo", format!("sdo:type:{name}"), "catalogue-1")
}
pub fn relationship_ref(source: &str, verb: &str) -> ExactRef {
    ExactRef::new(
        "sdo",
        format!("sdo:relationship:{source}.{verb}"),
        "catalogue-1",
    )
}
pub fn ontology_ref() -> ExactRef {
    ExactRef::new("sdo", "sdo:ontology:system-definition", "catalogue-1")
}
pub fn profile_ref() -> ExactRef {
    ExactRef::new("sdo", "sdo:profile:system-definition", "catalogue-1")
}
pub fn self_profile_ref() -> ExactRef {
    ExactRef::new(
        "sdo",
        "sdo:profile:knowledgehub-self-description",
        "catalogue-1",
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
    let anchor = ExactRef::new("sdo", "sdo:anchor:catalogue", "catalogue-1");
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
        let source = ExactRef::new("sdo", id, "catalogue-1");
        let capture = ExactRef::new("sdo", format!("{id}:capture"), "catalogue-1");
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
    let source = ExactRef::new("sdo", "urn:source:ontology:capture", "catalogue-1");
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
    let mut types = vec![];
    for t in data.types {
        let r = type_ref(&t.name);
        types.push(r.clone());
        out.records.push(Record::new(
            r,
            type_ref("ElementTypeDefinition"),
            RecordData::Type(t),
            vec![anchor.clone()],
        )?);
    }
    let mut relationships = vec![];
    for d in data.relationships {
        let r = relationship_ref(&d.from[0], &d.name);
        relationships.push(r.clone());
        out.records.push(Record::new(
            r,
            type_ref("RelationshipTypeDefinition"),
            RecordData::Relationship(d),
            vec![anchor.clone()],
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
        let r = ExactRef::new("sdo", format!("sdo:constraint:V-{n:02}"), "catalogue-1");
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
            name: "system-definition-ontology/1".into(),
            ontology: ontology_ref(),
            vocabulary: types.clone(),
            allow_gaps: true,
            required_types: vec![],
            inapplicable: BTreeMap::new(),
            citation: citation.clone(),
        }),
        vec![anchor.clone()],
    )?);
    out.records.push(Record::new(self_profile_ref(),type_ref("ProfileDefinition"),RecordData::Profile(ProfileDefinition{name:"knowledgehub-self-description/1".into(),ontology:ontology_ref(),vocabulary:types.clone(),allow_gaps:true,required_types:vec!["SystemModel","BusinessGoal","Actor","BoundedContext","AggregateDefinition","Requirement","UseCase","FunctionDefinition","SoftwareComponent","Module","AuthorizationPolicy","DeliverySlice","TestCase","TestRun","EvidenceArtifact"].into_iter().map(str::to_owned).collect(),inapplicable:[("Interaction and visual design".into(),"Embedded library exposes no product screens; supported vocabulary is exercised by isolated conformance models.".into()),("AI participation".into(),"The library has no runtime AI engine; development assistance is not a runtime agent.".into())].into(),citation:citation.clone()}),vec![anchor.clone()])?);
    out.records.push(Record::new(
        ontology_ref(),
        type_ref("OntologyDefinition"),
        RecordData::Ontology(OntologyDefinition {
            name: "System Definition Ontology".into(),
            version: "system-definition-ontology/1".into(),
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

pub(crate) fn meta_profile() -> crate::Profile {
    crate::Profile {
        version: "knowledgehub-meta/1".into(),
        ontology: crate::RevisionRef {
            id: "knowledgehub:meta-schema".into(),
            revision: codec::digest(
                b"knowledgehub-meta/1:closed-native-records-and-schema-descriptors",
            ),
        },
        definitions: vec![],
    }
}
pub(crate) fn supports_role(state: &State, r: &ExactRef, allowed: &[String]) -> bool {
    let Some(kind) = state.kind(r) else {
        return false;
    };
    allowed.iter().any(|a| {
        a == kind
            || a == "AnySubject"
            || ((a == "AnyElementRevision" || a == "ElementRevision")
                && state
                    .get(r)
                    .is_some_and(|r| !matches!(r.data, RecordData::Assertion(_))))
    })
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
        }
        RecordData::Type(t) => {
            out.extend(t.parents.clone());
            out.extend(t.constraints.clone());
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
