//! # vimage
//! Simple CLI to open images in the default browser.
//! The images are updated at a given interval, so that
//! the user can edit the images keeping track of their
//! changes.
//!
//! ## Usage
//!    `vimage` <FILE> [OPTIONS]
//!    `vimage` [COMMAND]
//!
//! ### Options
//!    `--output` <FILENAME>   File name of the output file.
//!                            [default: `.vimage.html`]
//!    `--output-dir` <DIR>    Directory of the output file.
//!                            [default (mac, linux): `HOME` environment variable.]
//!                            [default (windows): `USERPROFILE` environment variable.]
//!    `--interval` <INT>      Update interval of output image (ms).
//!                            [default: 500]
//!    `--open` <CMD>          Command to open output file.
//!                            [default (mac): `open`]
//!                            [default (windows): `start`]
//!                            [default (linux): `xdg-open`]
//!
//! ### Commands
//!    `help`    Prints this message.
pub mod browser;
pub mod config;
pub mod constants;
pub mod error;
pub mod path;
