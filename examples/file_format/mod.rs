//! Host-only format selection. Existing positional invocations keep files/2.
#![allow(dead_code)]
use knowledgehub::{Error, backends::files::*};
use std::path::{Path, PathBuf};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Files2,
    Files3,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_positionals_keep_files2_and_explicit_files3_is_additive() {
        let old = Arguments::from(
            ["input.json", "/absolute/store"]
                .into_iter()
                .map(String::from),
        )
        .unwrap();
        assert_eq!(old.format, Format::Files2);
        assert_eq!(old.positional, vec!["input.json", "/absolute/store"]);
        let new = Arguments::from(
            ["--format", "files3", "/absolute/store", "--verify"]
                .into_iter()
                .map(String::from),
        )
        .unwrap();
        assert_eq!(new.format, Format::Files3);
        assert!(new.verify);
        assert_eq!(new.positional, vec!["/absolute/store"]);
    }
    #[test]
    fn unknown_formats_flags_and_missing_values_are_refused() {
        for args in [
            vec!["--format"],
            vec!["--format", "files4"],
            vec!["--unknown"],
            vec!["--result"],
        ] {
            assert!(Arguments::from(args.into_iter().map(String::from)).is_err());
        }
    }
}
impl Format {
    pub fn name(self) -> &'static str {
        match self {
            Self::Files2 => "knowledgehub-files/2",
            Self::Files3 => "knowledgehub-files/3",
        }
    }
    pub fn backend(self, path: &Path) -> Result<FileBackend, Error> {
        match self {
            Self::Files2 => FileBackend::new_ontology(path, FileOptions::local_ext4()),
            Self::Files3 => FileBackend::new_shared_ontology(path, FileOptions::local_ext4()),
        }
    }
}
pub struct Arguments {
    pub format: Format,
    pub positional: Vec<String>,
    pub result: Option<PathBuf>,
    pub validate_input: bool,
    pub verify: bool,
    pub compact: bool,
}
impl Arguments {
    pub fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        Self::from(std::env::args().skip(1))
    }
    pub fn from(args: impl Iterator<Item = String>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut result = Self {
            format: Format::Files2,
            positional: vec![],
            result: None,
            validate_input: false,
            verify: false,
            compact: false,
        };
        let mut args = args;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--format" => {
                    result.format = match args.next().as_deref() {
                        Some("files2") => Format::Files2,
                        Some("files3") => Format::Files3,
                        _ => return Err("--format requires files2 or files3".into()),
                    }
                }
                "--result" => {
                    result.result = Some(args.next().ok_or("--result requires a path")?.into())
                }
                "--validate-input" => result.validate_input = true,
                "--verify" => result.verify = true,
                "--compact" => result.compact = true,
                _ if arg.starts_with('-') => {
                    return Err(format!("unknown host argument {arg}").into());
                }
                _ => result.positional.push(arg),
            }
        }
        Ok(result)
    }
}
