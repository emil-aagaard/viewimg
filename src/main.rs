//! # viewimg
//!
//! This is a simple CLI written in Rust to open images in browsers.
//! Images are updated at each interval (every 500 ms by default),
//! which enables users to view their images while altering them.
//!
//! ## Usage
//!
//! ```bash
//! viewimg [file]
//! ```
use std::{
    env, error,
    fmt::{self, Display},
    fs, io,
    path::PathBuf,
    process,
};

/// Help message.
const HELP_MESSAGE: &str = r#"Usage:

viewimg [FILE_PATH|COMMAND]

Commands:
    help"#;
/// Name of output file.
const OUTPUT_FILE: &str = ".viewimg.html";

/// Main function.
fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        process::exit(1);
    }
}

/// Main run flow.
fn run() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("help") => {
            println!("{}", HELP_MESSAGE);
            Ok(())
        }
        Some(relative_path_str) => {
            let path_buf = get_path_buf(relative_path_str)?;
            let output_file_path_buf = get_output_file_path_buf()?;
            create_html(path_buf, output_file_path_buf.clone())?;
            open_browser(output_file_path_buf)
        }
        None => Err(Error::NoArgs),
    }
}

/// Getting the path for the output file.
fn get_output_file_path_buf() -> Result<PathBuf, Error> {
    let home_path_string = env::var("HOME").map_err(|_| Error::UnableToFindHome)?;
    Ok(PathBuf::from(home_path_string).join(OUTPUT_FILE))
}

/// Taking the `relative_path_str` and appending the full path.
fn get_path_buf(relative_path_str: &str) -> Result<PathBuf, Error> {
    fs::canonicalize(relative_path_str)
        .map_err(|_| Error::PathInvalid(relative_path_str.to_string()))
}

/// Creating the HTML file.
fn create_html(path_buf: PathBuf, output_file_path_buf: PathBuf) -> Result<(), Error> {
    let path_string = path_buf.to_string_lossy();
    let html_string = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <body>
            <img id="plot" src="{path_string}" style="max-width: 100%">
            <script>
                setInterval(() => {{
                    document.getElementById('plot').src = '{path_string}?t=' + Date.now();
                }}, 500);
            </script>
        </body>
        </html>
    "#
    );
    fs::write(output_file_path_buf, html_string)
        .map_err(|error| Error::UnableToWriteToOutputFile(error))
}

/// Opens the default browser using the `open` command.
fn open_browser(output_file_path_buf: PathBuf) -> Result<(), Error> {
    process::Command::new("open")
        .arg(output_file_path_buf)
        .spawn()
        .map_err(|_| Error::UnableToOpenBrowser)?;
    Ok(())
}

/// Errors for `viewimg`.
#[derive(Debug)]
enum Error {
    NoArgs,
    PathInvalid(String),
    UnableToFindHome,
    UnableToWriteToOutputFile(io::Error),
    UnableToOpenBrowser,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoArgs => {
                write!(f, "No arguments given.\n\n{}", HELP_MESSAGE)
            }
            Self::PathInvalid(relative_path_string) => {
                write!(f, "Path not found: {relative_path_string}.")
            }
            Self::UnableToFindHome => write!(f, "Unable to find the HOME environment variable."),
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
            Self::NoArgs => None,
            Self::PathInvalid(_) => None,
            Self::UnableToFindHome => None,
            Self::UnableToWriteToOutputFile(error) => Some(error),
            Self::UnableToOpenBrowser => None,
        }
    }
}
