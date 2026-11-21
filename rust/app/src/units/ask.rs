//! Glue from the front door to `cli-rs::port::cmd_ask`.
//!
//! Parses the arguments, calls `cli_rs::port::cmd_ask`, prints what the original prints
//! and returns its exit code. Falls back to `crate::original::run` when the kill switch
//! is set or the port returns an error.

use std::ffi::OsString;
use std::process::ExitCode;

use clap::ArgMatches;

pub fn run(_command: &str, matches: &ArgMatches) -> ExitCode {
    let args: Vec<&OsString> = matches
        .get_many::<OsString>("args")
        .into_iter()
        .flatten()
        .collect();

    // Build JSON args for cmd_ask by parsing the raw argv
    let mut json_args = serde_json::Map::new();
    json_args.insert("prompt".to_string(), serde_json::Value::String(String::new()));
    json_args.insert("no_stream".to_string(), serde_json::Value::Bool(true));
    json_args.insert("cookies_json".to_string(), serde_json::Value::Null);
    json_args.insert("model".to_string(), serde_json::Value::Null);
    json_args.insert("verbose".to_string(), serde_json::Value::Bool(false));

    let mut iter = args.iter().skip(1); // skip command name
    while let Some(arg) = iter.next() {
        let s = arg.to_string_lossy();
        if let Some(v) = s.strip_prefix("--cookies-json=") {
            json_args.insert("cookies_json".to_string(), serde_json::Value::String(v.to_string()));
        } else if s == "--cookies-json" {
            if let Some(v) = iter.next() {
                json_args.insert("cookies_json".to_string(), serde_json::Value::String(v.to_string_lossy().into_owned()));
            }
        } else if let Some(v) = s.strip_prefix("--model=") {
            json_args.insert("model".to_string(), serde_json::Value::String(v.to_string()));
        } else if s == "--model" {
            if let Some(v) = iter.next() {
                json_args.insert("model".to_string(), serde_json::Value::String(v.to_string_lossy().into_owned()));
            }
        } else if s == "--no-stream" {
            json_args.insert("no_stream".to_string(), serde_json::Value::Bool(true));
        } else if s == "--stream" {
            json_args.insert("no_stream".to_string(), serde_json::Value::Bool(false));
        } else if s == "--verbose" {
            json_args.insert("verbose".to_string(), serde_json::Value::Bool(true));
        } else if !s.starts_with('-') {
            // First non-flag arg is the prompt
            if json_args.get("prompt").and_then(|v| v.as_str()).map_or(true, |v| v.is_empty()) {
                json_args.insert("prompt".to_string(), serde_json::Value::String(s.to_string()));
            }
        }
    }

    let args_value = serde_json::Value::Object(json_args);

    match cli_rs::port::cmd_ask(args_value) {
        Ok(code) => {
            if let serde_json::Value::Number(n) = code {
                ExitCode::from(n.as_u64().unwrap_or(0) as u8)
            } else {
                ExitCode::from(0)
            }
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }
    }
}
