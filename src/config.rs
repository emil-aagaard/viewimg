use crate::{
    constants::{DEFAULT_INTERVAL, DEFAULT_OUTPUT_FILE_NAME},
    error::Error,
};

pub struct Config {
    /// Name of output file.
    pub output_file_name: String,
    /// Optional path of the output directory. If `None`, the [`std::env::temp_dir`] will be used.
    pub output_directory: Option<String>,
    /// Update interval (ms).
    pub interval: u64,
    /// Optional open command. If `None`, then the command will be chosen by [`crate::browser::open_browser`] function.
    pub open_command: Option<String>,
}

impl Config {
    pub fn from_iter<'a, I>(mut iter: I) -> Result<Self, Error>
    where
        I: Iterator<Item = &'a String>,
    {
        let mut config = Self::default();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--output" => {
                    if let Some(output_file_name) = iter.next() {
                        config.output_file_name = output_file_name.to_string()
                    } else {
                        return Err(Error::MissingOutputFileName);
                    }
                }
                "--output-dir" => {
                    if let Some(output_directory) = iter.next() {
                        config.output_directory = Some(output_directory.to_string())
                    } else {
                        return Err(Error::MissingOutputDirectory);
                    }
                }
                "--interval" => {
                    if let Some(interval) = iter.next() {
                        config.interval = interval
                            .parse()
                            .map_err(|_| Error::UnableToParseInterval(interval.to_string()))?;
                    } else {
                        return Err(Error::MissingInterval);
                    }
                }
                "--open" => {
                    if let Some(open_command) = iter.next() {
                        config.open_command = Some(open_command.to_string());
                    } else {
                        return Err(Error::MissingOpenCommand);
                    }
                }
                _ => return Err(Error::InvalidArg(arg.to_string())),
            }
        }
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_file_name: String::from(DEFAULT_OUTPUT_FILE_NAME),
            output_directory: None,
            interval: DEFAULT_INTERVAL,
            open_command: None,
        }
    }
}
