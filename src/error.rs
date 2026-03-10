use std::{
    error,
    fmt::{self, Display, Formatter},
    io,
};

use crate::constants::HELP_MESSAGE;

/// Errors for `viewimg`.
#[derive(Debug)]
pub enum Error {
    NoArgs,
    InvalidArg(String),
    UnableToParseInterval(String),
    MissingOutputFileName,
    MissingOutputDirectory,
    MissingInterval,
    MissingOpenCommand,
    PathInvalid(String),
    MissingHomeEnvVar(String),
    UnableToWriteToOutputFile(io::Error),
    UnableToOpenBrowser,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoArgs => {
                write!(f, "No arguments given.\n\n{}", HELP_MESSAGE)
            }
            Self::InvalidArg(arg) => write!(f, "Invalid arg: {arg}"),
            Self::UnableToParseInterval(interval) => {
                write!(f, "Unable to parse interval as integer: {interval}")
            }
            Self::MissingOutputFileName => {
                write!(f, "Missing output file name after the `--output` flag.")
            }
            Self::MissingOutputDirectory => {
                write!(f, "Missing output directory after the `--output-dir` flag.")
            }
            Self::MissingInterval => write!(f, "Missing interval after the `--interval` flag."),
            Self::MissingOpenCommand => write!(f, "Missing open command after the `--open` flag."),
            Self::PathInvalid(relative_path_string) => {
                write!(f, "Path not found: {relative_path_string}.")
            }
            Self::MissingHomeEnvVar(home_env_var_name) => write!(
                f,
                "Missing environment variable: {home_env_var_name}. Consider specifying the output directory using the `--output-dir` flag."
            ),
            Self::UnableToWriteToOutputFile(error) => {
                write!(f, "Unable to write to output file: {error}.")
            }
            Self::UnableToOpenBrowser => write!(f, "Unable to open browser."),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::UnableToWriteToOutputFile(error) => Some(error),
            _ => None,
        }
    }
}
