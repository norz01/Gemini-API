//! Glue from the front door to `crate::port::cmd_models`.

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

    match crate::port::cmd_models(args_value) {
        Ok(code) => crate::port::exit_code_from_value(&code),
        Err(e) => {
            eprintln!("front door: models failed: {e}");
            ExitCode::from(1)
        }
    }
}
