//! Glue from the front door to `gemini-webapi-rs`.
//!
//! `gemini-webapi-rs` is the ported Gemini client core and a library dependency of the front
//! door. It does not itself handle a CLI command, so the `cli` route (no subcommand, `--help`,
//! unknown commands) falls back to the original. Command-specific glue lives in the per-command
//! unit files, which call `gemini_webapi_rs::port` once the client core is ported.
use std::ffi::OsString;
use std::process::ExitCode;

use clap::ArgMatches;

pub fn run(_command: &str, matches: &ArgMatches) -> ExitCode {
    let args: Vec<OsString> = matches
        .get_many::<OsString>("args")
        .into_iter()
        .flatten()
        .cloned()
        .collect();
    crate::original::run(&args)
}
