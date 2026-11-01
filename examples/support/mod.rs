//! Trusted host fixture shared by current full-model examples and tests.
#![allow(dead_code)]
use knowledgehub::*;
use std::sync::Arc;
pub struct Allow;
impl Policy for Allow {
    fn allows(&self, _: &str, _: &ModelScope, _: Action) -> Result<bool, PolicyFailure> {
        Ok(true)
    }
}
pub fn context() -> Context {
    Context::new("host:maintainer", Arc::new(Allow))
}
pub const ONTOLOGY: &[u8] = include_bytes!("../../source/system-definition-ontology-proposal.md");
pub fn scope() -> ModelScope {
    ModelScope {
        hub: "urn:knowledgehub:self-hub".into(),
        model: "urn:knowledgehub:library".into(),
        branch: "main".into(),
        owner: "host:maintainer".into(),
    }
}
