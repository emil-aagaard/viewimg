use std::{env::var, fs::canonicalize, path::PathBuf};

use crate::error::Error;

pub fn get_output_path(
    output_directory: &Option<String>,
    output_file_name: &String,
) -> Result<PathBuf, Error> {
    let output_directory = if let Some(output_directory) = output_directory {
        output_directory
    } else {
        let home_env_var_name = if cfg!(target_os = "windows") {
            "USERPROFILE"
        } else {
            "HOME"
        };
        &var(home_env_var_name)
            .map_err(|_| Error::MissingHomeEnvVar(home_env_var_name.to_string()))?
    };
    Ok(PathBuf::from(output_directory).join(output_file_name))
}

pub fn get_path(relative_path: &str) -> Result<PathBuf, Error> {
    canonicalize(relative_path).map_err(|_| Error::PathInvalid(relative_path.to_string()))
}
