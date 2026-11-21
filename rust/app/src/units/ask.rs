//! Glue from the front door to `crate::port::cmd_ask`.

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
    json_args.insert("prompt".to_string(), serde_json::Value::String(String::new()));
    json_args.insert("no_stream".to_string(), serde_json::Value::Bool(true));
    json_args.insert("cookies_json".to_string(), serde_json::Value::Null);
    json_args.insert("model".to_string(), serde_json::Value::Null);
    json_args.insert("verbose".to_string(), serde_json::Value::Bool(false));

    let mut iter = args.iter().skip(1);
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
            if json_args.get("prompt").and_then(|v| v.as_str()).map_or(true, |v| v.is_empty()) {
                json_args.insert("prompt".to_string(), serde_json::Value::String(s.to_string()));
            }
        }
    }

    let args_value = serde_json::Value::Object(json_args);

    match crate::port::cmd_ask(args_value) {
        Ok(code) => crate::port::exit_code_from_value(&code),
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }
    }
}
