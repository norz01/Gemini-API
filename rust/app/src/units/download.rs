//! Glue from the front door to `cli-rs::port::cmd_download`.

use std::ffi::OsString;
use std::process::ExitCode;

use clap::ArgMatches;

pub fn run(_command: &str, matches: &ArgMatches) -> ExitCode {
    let _args: Vec<&OsString> = matches
        .get_many::<OsString>("args")
        .into_iter()
        .flatten()
        .collect();

    match cli_rs::port::cmd_download(serde_json::Value::Null) {
        Ok(code) => {
            if let serde_json::Value::Number(n) = code {
                ExitCode::from(n.as_u64().unwrap_or(0) as u8)
            } else {
                ExitCode::from(0)
            }
        }
        Err(e) => {
            eprintln!("front door: download failed: {e}");
            ExitCode::from(1)
        }
    }
}
