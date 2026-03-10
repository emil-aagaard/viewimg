/// Help message.
pub const HELP_MESSAGE: &str = "\
Usage:
    viewimg <FILE> [OPTIONS]
    viewimg [COMMAND]

Options:
    --output <FILENAME> File name of the output file.
                        [default: `.viewimg.html`]
    --output-dir <DIR>  Directory of the output file.
                        [default (mac, linux): `HOME` environment variable.]
                        [default (windows): `USERPROFILE` environment variable.]
    --interval <INT>    Update interval of output image (ms).
                        [default: 500]
    --open <CMD>        Command to open output file.
                        [default (mac): `open`]
                        [default (windows): `start`]
                        [default (linux): `xdg-open`]

Commands:
    help    Prints this message.";
/// Default name of output file.
pub const DEFAULT_OUTPUT_FILE_NAME: &str = ".viewimg.html";
/// Default update interval (ms).
pub const DEFAULT_INTERVAL: u64 = 500;
