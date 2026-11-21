//! Glue from the front door to `cli-rs::port::cmd_inspect`.

use std::process::ExitCode;

pub fn run(_command: &str, _matches: &clap::ArgMatches) -> ExitCode {
    match cli_rs::port::cmd_inspect(serde_json::Value::Null) {
        Ok(code) => {
            if let serde_json::Value::Number(n) = code {
                ExitCode::from(n.as_u64().unwrap_or(0) as u8)
            } else {
                ExitCode::from(0)
            }
        }
        Err(e) => {
            eprintln!("front door: inspect failed: {e}");
            ExitCode::from(1)
        }
    }
}
