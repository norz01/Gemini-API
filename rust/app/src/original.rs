//! Fallback logging stub.
//!
//! The front door no longer falls back to the original, so this is a no-op.
//! Kept for compatibility with `rust-anything app lift`.

use std::process::ExitCode;

pub fn report_fallbacks() {
    // No fallbacks remain; nothing to log.
}

pub fn run(_args: &[std::ffi::OsString]) -> ExitCode {
    // Should never be reached: all commands are handled by Rust.
    ExitCode::from(2)
}

pub fn run_with_stdin(_args: &[std::ffi::OsString], _input: &[u8]) -> ExitCode {
    ExitCode::from(2)
}
