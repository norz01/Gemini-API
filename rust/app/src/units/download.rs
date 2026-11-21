//! Glue from the front door to `crate::port::cmd_download`.

use std::process::ExitCode;

pub fn run(_command: &str, _matches: &clap::ArgMatches) -> ExitCode {
    match crate::port::cmd_download(serde_json::Value::Null) {
        Ok(code) => crate::port::exit_code_from_value(&code),
        Err(e) => {
            eprintln!("front door: download failed: {e}");
            ExitCode::from(1)
        }
    }
}
