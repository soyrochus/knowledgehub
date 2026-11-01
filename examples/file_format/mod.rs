//! Owned host command arguments for the sole current backend.
#![allow(dead_code)]
use std::path::PathBuf;
pub struct Arguments {
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
        let mut out = Self {
            positional: vec![],
            result: None,
            validate_input: false,
            verify: false,
            compact: false,
        };
        let mut args = args;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--result" => {
                    out.result = Some(args.next().ok_or("--result requires a path")?.into())
                }
                "--validate-input" => out.validate_input = true,
                "--verify" => out.verify = true,
                "--compact" => out.compact = true,
                _ if arg.starts_with('-') => {
                    return Err(format!("unknown host argument {arg}").into());
                }
                _ => out.positional.push(arg),
            }
        }
        Ok(out)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordinary_commands_need_no_format_selection() {
        let args = Arguments::from(
            ["input.json", "/absolute/store", "--verify"]
                .into_iter()
                .map(String::from),
        )
        .unwrap();
        assert_eq!(args.positional, vec!["input.json", "/absolute/store"]);
        assert!(args.verify);
    }
    #[test]
    fn obsolete_format_flags_and_missing_values_are_refused() {
        for args in [
            vec!["--format"],
            vec!["--format", "files2"],
            vec!["--format", "files3"],
            vec!["--unknown"],
            vec!["--result"],
        ] {
            assert!(Arguments::from(args.into_iter().map(String::from)).is_err());
        }
    }
}
