//! Glue from the front door to `gemini-webapi-rs`.
//!
//! `gemini-webapi-rs` is the ported Gemini client core and a library dependency of the front
//! door. It does not itself handle a CLI command, so the `cli` route (no subcommand, `--help`,
//! unknown commands) is handled here directly instead of falling back to the original.
//! Command-specific glue lives in the per-command unit files, which call `gemini_webapi_rs::port`
//! once the client core is ported.

use std::ffi::OsString;
use std::process::ExitCode;

use clap::ArgMatches;

const HELP_TEXT: &str = "usage: cli.py [-h] [--cookies-json COOKIES_JSON] [--proxy PROXY]\n              [--account-index ACCOUNT_INDEX] [--model MODEL] [--verbose]\n              [--no-persist] [--request-timeout REQUEST_TIMEOUT]\n              [--skip-verify]\n              {ask,reply,research,list,read,models,download,inspect} ...\n\nCLI for gemini-webapi\n\npositional arguments:\n  {ask,reply,research,list,read,models,download,inspect}\n    ask                 Single-turn question\n    reply               Continue chat\n    research            Deep research\n    list                List chats\n    read                Read chat\n    models              List the models available to this account\n    download            Download image\n    inspect             Account probe\n\noptions:\n  -h, --help            show this help message and exit\n  --cookies-json COOKIES_JSON\n                        Path to JSON cookie file\n  --proxy PROXY\n  --account-index ACCOUNT_INDEX\n                        Google account index\n  --model MODEL         Model name, alias or id (see the 'models' command).\n                        Defaults to the account's default\n  --verbose             Enable debug logging\n  --no-persist          Do not write updated cookies back\n  --request-timeout REQUEST_TIMEOUT\n                        Per-request HTTP timeout in seconds\n  --skip-verify         Skip SSL certificate verification\n";

const USAGE_TEXT: &str = "usage: cli.py [-h] [--cookies-json COOKIES_JSON] [--proxy PROXY]\n              [--account-index ACCOUNT_INDEX] [--model MODEL] [--verbose]\n              [--no-persist] [--request-timeout REQUEST_TIMEOUT]\n              [--skip-verify]\n              {ask,reply,research,list,read,models,download,inspect} ...\n";

pub fn run(_command: &str, matches: &ArgMatches) -> ExitCode {
    let args: Vec<OsString> = matches
        .get_many::<OsString>("args")
        .into_iter()
        .flatten()
        .cloned()
        .collect();

    // Handle --help
    if args.iter().any(|a| a.to_string_lossy() == "--help") {
        print!("{HELP_TEXT}");
        return ExitCode::from(0);
    }

    // Handle no args: original calls parser.print_help() then returns 1
    if args.is_empty() {
        print!("{HELP_TEXT}");
        return ExitCode::from(1);
    }

    // Handle unknown command - first non-flag argument
    let unknown = args
        .iter()
        .find(|a| !a.to_string_lossy().starts_with('-'))
        .map(|a| a.to_string_lossy().into_owned())
        .unwrap_or_default();

    eprint!("{USAGE_TEXT}");
    eprintln!(
        "cli.py: error: argument command: invalid choice: '{unknown}' (choose from 'ask', 'reply', 'research', 'list', 'read', 'models', 'download', 'inspect')"
    );
    ExitCode::from(2)
}
