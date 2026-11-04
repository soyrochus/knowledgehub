use knowledgehub::{Action, Context, Error, ExactRef, ModelScope, Policy, PolicyFailure};
use serde::Deserialize;
use std::{
    collections::BTreeSet,
    fs,
    io::Read,
    os::unix::fs::MetadataExt,
    path::{Component, Path, PathBuf},
    sync::Arc,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub alias: String,
    pub name: String,
    pub directory: PathBuf,
    pub scope: ModelScope,
    pub actor: String,
    pub grants: BTreeSet<String>,
    #[serde(default)]
    pub known_baselines: Vec<ExactRef>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Workspace {
    pub format: String,
    pub default: String,
    pub models: Vec<Entry>,
}
pub fn invalid(message: &str) -> Error {
    Error::InvalidInput(message.into())
}
pub fn load(path: &Path) -> Result<Workspace, Error> {
    let f = fs::File::open(path).map_err(|_| invalid("workspace unavailable"))?;
    if f.metadata()
        .map_err(|_| invalid("workspace metadata"))?
        .len()
        > 1_048_576
    {
        return Err(Error::Capacity);
    }
    let mut bytes = vec![];
    f.take(1_048_577)
        .read_to_end(&mut bytes)
        .map_err(|_| invalid("workspace read"))?;
    if bytes.len() > 1_048_576 {
        return Err(Error::Capacity);
    }
    let w: Workspace =
        serde_json::from_slice(&bytes).map_err(|_| invalid("invalid workspace fields/JSON"))?;
    if w.format != "knowledgehub-client-workspace/1" || w.models.is_empty() || w.models.len() > 64 {
        return Err(invalid("unsupported workspace or model count"));
    }
    let mut aliases = BTreeSet::new();
    for e in &w.models {
        if e.alias.is_empty()
            || e.alias.len() > 100
            || !e
                .alias
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"-_".contains(&c))
            || !aliases.insert(&e.alias)
            || e.name.trim().is_empty()
            || !e.actor.contains(':')
            || e.actor.chars().any(char::is_control)
            || e.known_baselines.len() > 1000
            || e.known_baselines.iter().any(|r| !r.is_valid())
            || e.grants
                .iter()
                .any(|g| !matches!(g.as_str(), "Open" | "Read" | "ReadHistory"))
        {
            return Err(invalid(
                "invalid alias, actor, grants or handover references",
            ));
        }
    }
    if !aliases.contains(&w.default) {
        return Err(invalid("unknown default alias"));
    }
    Ok(w)
}
pub fn ordinary(path: &Path) -> Result<PathBuf, Error> {
    if path
        .components()
        .any(|c| matches!(c, Component::ParentDir | Component::CurDir))
    {
        return Err(invalid("path traversal is not allowed"));
    }
    let full = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|_| invalid("cwd"))?
            .join(path)
    };
    let canonical = full
        .canonicalize()
        .map_err(|_| invalid("configured path does not exist"))?;
    if canonical != full {
        return Err(invalid("symlink or noncanonical configured path"));
    }
    Ok(canonical)
}
pub fn directory(workspace: &Path, entry: &Entry) -> Result<PathBuf, Error> {
    let p = if entry.directory.is_absolute() {
        entry.directory.clone()
    } else {
        workspace
            .parent()
            .ok_or_else(|| invalid("workspace parent"))?
            .join(&entry.directory)
    };
    let p = ordinary(&p)?;
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"));
    if [repo.join("hub"), repo.join("old_hub")]
        .iter()
        .any(|r| p.starts_with(r))
    {
        return Err(invalid("protected historical store"));
    }
    if !p.is_dir() {
        return Err(invalid("configured model is not a directory"));
    }
    Ok(p)
}
pub fn identity(path: &Path) -> Result<(u64, u64), Error> {
    let m = fs::symlink_metadata(path)
        .map_err(|_| Error::Unavailable("model directory unavailable".into()))?;
    if !m.is_dir() {
        return Err(Error::Unavailable("model directory replaced".into()));
    }
    Ok((m.dev(), m.ino()))
}
#[derive(Clone)]
pub struct Authority {
    pub file: PathBuf,
    pub entry: Entry,
    pub directory: PathBuf,
    pub identity: (u64, u64),
}
impl Authority {
    pub fn current(&self) -> Result<Entry, Error> {
        let w = load(&self.file)?;
        let now = w
            .models
            .into_iter()
            .find(|e| e.alias == self.entry.alias)
            .ok_or_else(|| Error::Unavailable("model admission removed".into()))?;
        if now.actor != self.entry.actor
            || now.scope != self.entry.scope
            || directory(&self.file, &now)? != self.directory
            || identity(&self.directory)? != self.identity
        {
            return Err(Error::Unavailable(
                "admitted model identity changed; reopen explicitly".into(),
            ));
        }
        Ok(now)
    }
    pub fn check(&self, action: Action) -> Result<(), Error> {
        match self.allows(&self.entry.actor, &self.entry.scope, action) {
            Ok(true) => Ok(()),
            Ok(false) => Err(Error::Denied { action }),
            Err(_) => Err(Error::Unavailable("current host policy unavailable".into())),
        }
    }
    pub fn context(&self) -> Context {
        Context::new(self.entry.actor.clone(), Arc::new(self.clone()))
    }
}
impl Policy for Authority {
    fn allows(
        &self,
        actor: &str,
        scope: &ModelScope,
        action: Action,
    ) -> Result<bool, PolicyFailure> {
        if actor != self.entry.actor || scope != &self.entry.scope {
            return Ok(false);
        }
        let grant = match action {
            Action::Open => "Open",
            Action::Read => "Read",
            Action::ReadHistory => "ReadHistory",
            Action::Create
            | Action::RegisterOntology
            | Action::Validate
            | Action::Commit
            | Action::OperationResult => return Ok(false),
        };
        Ok(self
            .current()
            .map_err(|_| PolicyFailure)?
            .grants
            .contains(grant))
    }
}
