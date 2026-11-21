//! Python bindings for `cli.py`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.
//!
//! cli-rs is used as a library dependency by rust/app, so the Python binding inner functions
//! are kept for reference but may be unused.

#![allow(dead_code)]

use pyo3::prelude::*;

pub mod js;
pub mod port;
pub mod pyconv;
pub mod pynum;

#[allow(unused_imports)]
use pynum::Num;

fn error(message: String) -> PyErr {
    use pyo3::exceptions as e;
    let (kind, text) = message.split_once(": ").unwrap_or(("", message.as_str()));
    let text = text.to_string();
    match kind {
        "TypeError" => e::PyTypeError::new_err(text),
        "KeyError" => e::PyKeyError::new_err(text),
        "IndexError" => e::PyIndexError::new_err(text),
        "OverflowError" => e::PyOverflowError::new_err(text),
        "ZeroDivisionError" => e::PyZeroDivisionError::new_err(text),
        "AttributeError" => e::PyAttributeError::new_err(text),
        "RuntimeError" => e::PyRuntimeError::new_err(text),
        "NotImplementedError" => e::PyNotImplementedError::new_err(text),
        "LookupError" => e::PyLookupError::new_err(text),
        "ArithmeticError" => e::PyArithmeticError::new_err(text),
        "UnicodeError" => e::PyUnicodeError::new_err(text),
        "ValueError" => e::PyValueError::new_err(text),
        _ => e::PyValueError::new_err(message),
    }
}

fn cmd_ask_inner(args: Bound<'_, PyAny>) -> Result<serde_json::Value, String> {
    let args: serde_json::Value = crate::pyconv::FromPy::from_py(&args)?;
    let out = port::cmd_ask(args)?;
    Ok(out)
}

/// `cmd_ask`.
#[pyfunction]
#[pyo3(name = "cmd_ask", signature = (args))]
fn cmd_ask(py: Python<'_>, args: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    cmd_ask_inner(args)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn cmd_reply_inner(args: Bound<'_, PyAny>) -> Result<serde_json::Value, String> {
    let args: serde_json::Value = crate::pyconv::FromPy::from_py(&args)?;
    let out = port::cmd_reply(args)?;
    Ok(out)
}

/// `cmd_reply`.
#[pyfunction]
#[pyo3(name = "cmd_reply", signature = (args))]
fn cmd_reply(py: Python<'_>, args: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    cmd_reply_inner(args)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn cmd_research_send_inner(args: Bound<'_, PyAny>) -> Result<serde_json::Value, String> {
    let args: serde_json::Value = crate::pyconv::FromPy::from_py(&args)?;
    let out = port::cmd_research_send(args)?;
    Ok(out)
}

/// `cmd_research_send`.
#[pyfunction]
#[pyo3(name = "cmd_research_send", signature = (args))]
fn cmd_research_send(py: Python<'_>, args: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    cmd_research_send_inner(args)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn cmd_research_check_inner(args: Bound<'_, PyAny>) -> Result<serde_json::Value, String> {
    let args: serde_json::Value = crate::pyconv::FromPy::from_py(&args)?;
    let out = port::cmd_research_check(args)?;
    Ok(out)
}

/// `cmd_research_check`.
#[pyfunction]
#[pyo3(name = "cmd_research_check", signature = (args))]
fn cmd_research_check(py: Python<'_>, args: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    cmd_research_check_inner(args)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn cmd_research_get_inner(args: Bound<'_, PyAny>) -> Result<serde_json::Value, String> {
    let args: serde_json::Value = crate::pyconv::FromPy::from_py(&args)?;
    let out = port::cmd_research_get(args)?;
    Ok(out)
}

/// `cmd_research_get`.
#[pyfunction]
#[pyo3(name = "cmd_research_get", signature = (args))]
fn cmd_research_get(py: Python<'_>, args: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    cmd_research_get_inner(args)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn cmd_models_inner(args: Bound<'_, PyAny>) -> Result<serde_json::Value, String> {
    let args: serde_json::Value = crate::pyconv::FromPy::from_py(&args)?;
    let out = port::cmd_models(args)?;
    Ok(out)
}

/// `cmd_models`.
#[pyfunction]
#[pyo3(name = "cmd_models", signature = (args))]
fn cmd_models(py: Python<'_>, args: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    cmd_models_inner(args)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn cmd_list_inner(args: Bound<'_, PyAny>) -> Result<serde_json::Value, String> {
    let args: serde_json::Value = crate::pyconv::FromPy::from_py(&args)?;
    let out = port::cmd_list(args)?;
    Ok(out)
}

/// `cmd_list`.
#[pyfunction]
#[pyo3(name = "cmd_list", signature = (args))]
fn cmd_list(py: Python<'_>, args: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    cmd_list_inner(args)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn cmd_read_inner(args: Bound<'_, PyAny>) -> Result<serde_json::Value, String> {
    let args: serde_json::Value = crate::pyconv::FromPy::from_py(&args)?;
    let out = port::cmd_read(args)?;
    Ok(out)
}

/// `cmd_read`.
#[pyfunction]
#[pyo3(name = "cmd_read", signature = (args))]
fn cmd_read(py: Python<'_>, args: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    cmd_read_inner(args)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn cmd_download_inner(args: Bound<'_, PyAny>) -> Result<serde_json::Value, String> {
    let args: serde_json::Value = crate::pyconv::FromPy::from_py(&args)?;
    let out = port::cmd_download(args)?;
    Ok(out)
}

/// `cmd_download`.
#[pyfunction]
#[pyo3(name = "cmd_download", signature = (args))]
fn cmd_download(py: Python<'_>, args: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    cmd_download_inner(args)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn cmd_inspect_inner(args: Bound<'_, PyAny>) -> Result<serde_json::Value, String> {
    let args: serde_json::Value = crate::pyconv::FromPy::from_py(&args)?;
    let out = port::cmd_inspect(args)?;
    Ok(out)
}

/// `cmd_inspect`.
#[pyfunction]
#[pyo3(name = "cmd_inspect", signature = (args))]
fn cmd_inspect(py: Python<'_>, args: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    cmd_inspect_inner(args)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn build_parser_inner() -> Result<serde_json::Value, String> {
    let out = port::build_parser()?;
    Ok(out)
}

/// `build_parser`.
#[pyfunction]
#[pyo3(name = "build_parser", signature = ())]
fn build_parser(py: Python<'_>) -> PyResult<Py<PyAny>> {
    build_parser_inner()
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn run_inner(args: Bound<'_, PyAny>) -> Result<serde_json::Value, String> {
    let args: serde_json::Value = crate::pyconv::FromPy::from_py(&args)?;
    let out = port::run(args)?;
    Ok(out)
}

/// `run`.
#[pyfunction]
#[pyo3(name = "run", signature = (args))]
fn run(py: Python<'_>, args: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    run_inner(args)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

/// cli-rs is used as a library dependency by rust/app, but also exposes a Python
/// extension module so rust-anything gate/diff can verify it. Uses a unique module
/// name to avoid clashing with gemini-webapi-rs's _native module.
#[pymodule]
#[pyo3(name = "cli_native")]
fn native_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cmd_ask, m)?)?;
    m.add_function(wrap_pyfunction!(cmd_reply, m)?)?;
    m.add_function(wrap_pyfunction!(cmd_research_send, m)?)?;
    m.add_function(wrap_pyfunction!(cmd_research_check, m)?)?;
    m.add_function(wrap_pyfunction!(cmd_research_get, m)?)?;
    m.add_function(wrap_pyfunction!(cmd_models, m)?)?;
    m.add_function(wrap_pyfunction!(cmd_list, m)?)?;
    m.add_function(wrap_pyfunction!(cmd_read, m)?)?;
    m.add_function(wrap_pyfunction!(cmd_download, m)?)?;
    m.add_function(wrap_pyfunction!(cmd_inspect, m)?)?;
    m.add_function(wrap_pyfunction!(build_parser, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}
