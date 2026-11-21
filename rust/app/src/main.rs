mod cli;
mod original;
mod port;
mod units;

use std::ffi::OsString;
use std::process::ExitCode;

fn main() -> ExitCode {
    let argv: Vec<OsString> = std::env::args_os().collect();
    if let Ok(matches) = cli::command().try_get_matches_from(&argv) {
        let (command, sub) = matches.subcommand().unwrap_or((cli::ROOT, &matches));
        if let Some(code) = units::dispatch(command, sub) {
            return code;
        }
    }
    // No fallback: all commands are handled by Rust. If we reach here, something is unhandled.
    eprintln!("error: unhandled command");
    ExitCode::from(2)
}
