use std::{fs::write, path::PathBuf, process::Command};

use crate::error::Error;

pub fn create_html(path: &PathBuf, output_path: &PathBuf, interval: u64) -> Result<(), Error> {
    let path = path.to_string_lossy();
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
</head>
<body>
    <img id="plot" src="{path}" style="max-width: 100%">
        <script>
            setInterval(() => {{
                document.getElementById('plot').src = '{path}?t=' + Date.now();
            }}, {interval});
        </script>
    </body>
</html>"#
    );
    write(output_path, html.as_bytes()).map_err(|error| Error::UnableToWriteToOutputFile(error))
}

pub fn open_browser(output_path: &PathBuf, open_command: &Option<String>) -> Result<(), Error> {
    if let Some(open_command) = open_command {
        Command::new(open_command).arg(output_path).spawn()
    } else {
        if cfg!(target_os = "macos") {
            Command::new("open").arg(output_path).spawn()
        } else if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "start", format!("{:?}", output_path).as_str()])
                .spawn()
        } else {
            Command::new("xdg-open").arg(output_path).spawn()
        }
    }
    .map_err(|_| Error::UnableToOpenBrowser)?;
    Ok(())
}
