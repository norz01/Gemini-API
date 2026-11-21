//! Glue from the front door to `crate::port::cmd_read`.

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
    json_args.insert("chat_id".to_string(), serde_json::Value::Null);
    json_args.insert("max_turns".to_string(), serde_json::Value::Number(30.into()));
    json_args.insert("output".to_string(), serde_json::Value::Null);
    json_args.insert("cookies_json".to_string(), serde_json::Value::Null);
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
        } else if let Some(v) = s.strip_prefix("--output=") {
            json_args.insert("output".to_string(), serde_json::Value::String(v.to_string()));
        } else if s == "--output" {
            if let Some(v) = iter.next() {
                json_args.insert("output".to_string(), serde_json::Value::String(v.to_string_lossy().into_owned()));
            }
        } else if let Some(v) = s.strip_prefix("--max-turns=") {
            if let Ok(n) = v.parse::<i64>() {
                json_args.insert("max_turns".to_string(), serde_json::Value::Number(n.into()));
            }
        } else if s == "--max-turns" {
            if let Some(v) = iter.next() {
                if let Ok(n) = v.to_string_lossy().parse::<i64>() {
                    json_args.insert("max_turns".to_string(), serde_json::Value::Number(n.into()));
                }
            }
        } else if s == "--verbose" {
            json_args.insert("verbose".to_string(), serde_json::Value::Bool(true));
        } else if !s.starts_with('-') {
            if json_args.get("chat_id").and_then(|v| v.as_str()).is_none() {
                json_args.insert("chat_id".to_string(), serde_json::Value::String(s.to_string()));
            }
        }
    }

    let args_value = serde_json::Value::Object(json_args);

    match crate::port::cmd_read(args_value) {
        Ok(code) => crate::port::exit_code_from_value(&code),
        Err(e) => {
            eprintln!("front door: read failed: {e}");
            ExitCode::from(1)
        }
    }
}
