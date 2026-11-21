//! Glue from the front door to `cli-rs::port::cmd_list`.

use std::ffi::OsString;
use std::process::ExitCode;

use clap::ArgMatches;

pub fn run(_command: &str, matches: &ArgMatches) -> ExitCode {
    let args: Vec<&OsString> = matches
        .get_many::<OsString>("args")
        .into_iter()
        .flatten()
        .collect();

    let mut json_args = serde_json::Map::new();
    json_args.insert("cookies_json".to_string(), serde_json::Value::Null);
    json_args.insert("verbose".to_string(), serde_json::Value::Bool(false));

    for arg in args.iter().skip(1) {
        let s = arg.to_string_lossy();
        if let Some(v) = s.strip_prefix("--cookies-json=") {
            json_args.insert("cookies_json".to_string(), serde_json::Value::String(v.to_string()));
        }
        if s == "--verbose" {
            json_args.insert("verbose".to_string(), serde_json::Value::Bool(true));
        }
    }

    let args_value = serde_json::Value::Object(json_args);

    match cli_rs::port::cmd_list(args_value) {
        Ok(code) => {
            if let serde_json::Value::Number(n) = code {
                ExitCode::from(n.as_u64().unwrap_or(0) as u8)
            } else {
                ExitCode::from(0)
            }
        }
        Err(e) => {
            eprintln!("front door: list failed: {e}");
            ExitCode::from(1)
        }
    }
}
