use std::process::ExitCode;

pub fn report_fallbacks() {
    // No fallbacks remain; nothing to log.
}

pub fn run(_args: &[std::ffi::OsString]) -> ExitCode {
    ExitCode::from(2)
}

pub fn run_with_stdin(_args: &[std::ffi::OsString], _input: &[u8]) -> ExitCode {
    ExitCode::from(2)
}
