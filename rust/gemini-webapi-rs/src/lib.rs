//! Python bindings for `src/gemini_webapi/`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use pyo3::prelude::*;
use serde_json::Value;

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

fn build_model_header_inner(
    model_id: String,
    capacity_tail: Bound<'_, PyAny>,
    model_number: i64,
) -> Result<Value, String> {
    let capacity_tail: Value = crate::pyconv::FromPy::from_py(&capacity_tail)?;
    let out = port::build_model_header(model_id, capacity_tail, model_number)?;
    Ok(out)
}

/// `build_model_header`.
#[pyfunction]
#[pyo3(name = "build_model_header", signature = (model_id, capacity_tail, model_number))]
fn build_model_header(
    py: Python<'_>,
    model_id: String,
    capacity_tail: Bound<'_, PyAny>,
    model_number: i64,
) -> PyResult<Py<PyAny>> {
    build_model_header_inner(model_id, capacity_tail, model_number)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn warn_deprecated_model_inner(usage: String) -> Result<(), String> {
    port::warn_deprecated_model(usage)?;
    Ok(())
}

/// `warn_deprecated_model`.
#[pyfunction]
#[pyo3(name = "warn_deprecated_model", signature = (usage))]
fn warn_deprecated_model(usage: String) -> PyResult<()> {
    warn_deprecated_model_inner(usage).map_err(error)
}

fn format_http_version_inner(version_int: i64) -> Result<String, String> {
    let out = port::format_http_version(version_int)?;
    Ok(out)
}

/// `format_http_version`.
#[pyfunction]
#[pyo3(name = "format_http_version", signature = (version_int))]
fn format_http_version(version_int: i64) -> PyResult<String> {
    format_http_version_inner(version_int).map_err(error)
}

fn extract_cookie_value_inner(
    cookies: Bound<'_, PyAny>,
    name: String,
) -> Result<Option<String>, String> {
    let cookies: Vec<(String, String)> = crate::pyconv::FromPy::from_py(&cookies)?;
    let out = port::extract_cookie_value(cookies, name);
    Ok(out)
}

/// `extractCookieValue`.
#[pyfunction]
#[pyo3(name = "extractCookieValue", signature = (cookies, name))]
fn extract_cookie_value(cookies: Bound<'_, PyAny>, name: String) -> PyResult<Option<String>> {
    extract_cookie_value_inner(cookies, name).map_err(error)
}

fn hash_psid_inner(psid: String) -> Result<String, String> {
    let out = port::hash_psid(psid);
    Ok(out)
}

/// `hashPsid`.
#[pyfunction]
#[pyo3(name = "hashPsid", signature = (psid))]
fn hash_psid(psid: String) -> PyResult<String> {
    hash_psid_inner(psid).map_err(error)
}

fn cookie_cache_dir_inner() -> Result<String, String> {
    let out = port::cookie_cache_dir();
    Ok(out)
}

/// `cookieCacheDir`.
#[pyfunction]
#[pyo3(name = "cookieCacheDir", signature = ())]
fn cookie_cache_dir() -> PyResult<String> {
    cookie_cache_dir_inner().map_err(error)
}

fn is_transient_network_error_inner(status_code: Option<Bound<'_, PyAny>>) -> Result<bool, String> {
    let status_code = match &status_code {
        None => None,
        Some(v) => Some(crate::pyconv::arg(v)?),
    };
    let out = port::is_transient_network_error(status_code);
    Ok(out)
}

/// `isTransientNetworkError`.
#[pyfunction]
#[pyo3(name = "isTransientNetworkError", signature = (status_code))]
fn is_transient_network_error(status_code: Option<Bound<'_, PyAny>>) -> PyResult<bool> {
    is_transient_network_error_inner(status_code).map_err(error)
}

fn calculate_backoff_inner(
    attempt: Bound<'_, PyAny>,
    factor: f64,
    max_delay: Option<f64>,
) -> Result<f64, String> {
    let attempt = crate::pyconv::arg(&attempt)?;
    let out = port::calculate_backoff(attempt, factor, max_delay);
    Ok(out)
}

/// `calculateBackoff`.
#[pyfunction]
#[pyo3(name = "calculateBackoff", signature = (attempt, factor, max_delay))]
fn calculate_backoff(
    attempt: Bound<'_, PyAny>,
    factor: f64,
    max_delay: Option<f64>,
) -> PyResult<f64> {
    calculate_backoff_inner(attempt, factor, max_delay).map_err(error)
}

fn get_cookies_cache_path_inner(
    cookies: Bound<'_, PyAny>,
    base_psid: Option<String>,
) -> Result<Option<String>, String> {
    let cookies: Vec<(String, String)> = crate::pyconv::FromPy::from_py(&cookies)?;
    let out = port::get_cookies_cache_path(cookies, base_psid);
    Ok(out)
}

/// `getCookiesCachePath`.
#[pyfunction]
#[pyo3(name = "getCookiesCachePath", signature = (cookies, base_psid))]
fn get_cookies_cache_path(
    cookies: Bound<'_, PyAny>,
    base_psid: Option<String>,
) -> PyResult<Option<String>> {
    get_cookies_cache_path_inner(cookies, base_psid).map_err(error)
}

fn save_cookies_inner(
    cookies: Bound<'_, PyAny>,
    base_psid: Option<String>,
) -> Result<Option<String>, String> {
    let cookies: Vec<(String, String)> = crate::pyconv::FromPy::from_py(&cookies)?;
    let out = port::save_cookies(cookies, base_psid);
    Ok(out)
}

/// `saveCookies`.
#[pyfunction]
#[pyo3(name = "saveCookies", signature = (cookies, base_psid))]
fn save_cookies(cookies: Bound<'_, PyAny>, base_psid: Option<String>) -> PyResult<Option<String>> {
    save_cookies_inner(cookies, base_psid).map_err(error)
}

fn clear_cookies_cache_inner(
    cookies: Bound<'_, PyAny>,
    base_psid: Option<String>,
) -> Result<(), String> {
    let cookies: Vec<(String, String)> = crate::pyconv::FromPy::from_py(&cookies)?;
    port::clear_cookies_cache(cookies, base_psid);
    Ok(())
}

/// `clearCookiesCache`.
#[pyfunction]
#[pyo3(name = "clearCookiesCache", signature = (cookies, base_psid))]
fn clear_cookies_cache(cookies: Bound<'_, PyAny>, base_psid: Option<String>) -> PyResult<()> {
    clear_cookies_cache_inner(cookies, base_psid).map_err(error)
}

fn parse_rpc_results_inner(response_text: String, target_id: String) -> Result<Vec<Value>, String> {
    let out = port::parse_rpc_results(response_text, target_id);
    Ok(out)
}

/// `parseRpcResults`.
#[pyfunction]
#[pyo3(name = "parseRpcResults", signature = (response_text, target_id))]
fn parse_rpc_results(
    py: Python<'_>,
    response_text: String,
    target_id: String,
) -> PyResult<Py<PyAny>> {
    parse_rpc_results_inner(response_text, target_id)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn get_nested_value_inner(
    data: Bound<'_, PyAny>,
    path: Bound<'_, PyAny>,
    default: Option<Bound<'_, PyAny>>,
) -> Result<Option<Value>, String> {
    let data: Value = crate::pyconv::FromPy::from_py(&data)?;
    let path: Vec<Value> = crate::pyconv::FromPy::from_py(&path)?;
    let default: Option<Value> = match &default {
        None => None,
        Some(v) => Some(crate::pyconv::FromPy::from_py(v)?),
    };
    let out = port::get_nested_value(data, path, default);
    Ok(out)
}

/// `getNestedValue`.
#[pyfunction]
#[pyo3(name = "getNestedValue", signature = (data, path, default))]
fn get_nested_value(
    py: Python<'_>,
    data: Bound<'_, PyAny>,
    path: Bound<'_, PyAny>,
    default: Option<Bound<'_, PyAny>>,
) -> PyResult<Py<PyAny>> {
    get_nested_value_inner(data, path, default)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn get_sparse_bundle_inner(container: Bound<'_, PyAny>) -> Result<Option<Value>, String> {
    let container: Value = crate::pyconv::FromPy::from_py(&container)?;
    let out = port::get_sparse_bundle(container);
    Ok(out)
}

/// `getSparseBundle`.
#[pyfunction]
#[pyo3(name = "getSparseBundle", signature = (container))]
fn get_sparse_bundle(py: Python<'_>, container: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    get_sparse_bundle_inner(container)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn get_field_inner(
    container: Bound<'_, PyAny>,
    index: Bound<'_, PyAny>,
    default: Option<Bound<'_, PyAny>>,
) -> Result<Option<Value>, String> {
    let container: Value = crate::pyconv::FromPy::from_py(&container)?;
    let index = crate::pyconv::arg(&index)?;
    let default: Option<Value> = match &default {
        None => None,
        Some(v) => Some(crate::pyconv::FromPy::from_py(v)?),
    };
    let out = port::get_field(container, index, default);
    Ok(out)
}

/// `getField`.
#[pyfunction]
#[pyo3(name = "getField", signature = (container, index, default))]
fn get_field(
    py: Python<'_>,
    container: Bound<'_, PyAny>,
    index: Bound<'_, PyAny>,
    default: Option<Bound<'_, PyAny>>,
) -> PyResult<Py<PyAny>> {
    get_field_inner(container, index, default)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn get_rich_content_field_inner(
    candidate_data: Bound<'_, PyAny>,
    index: Bound<'_, PyAny>,
    default: Option<Bound<'_, PyAny>>,
) -> Result<Option<Value>, String> {
    let candidate_data: Value = crate::pyconv::FromPy::from_py(&candidate_data)?;
    let index = crate::pyconv::arg(&index)?;
    let default: Option<Value> = match &default {
        None => None,
        Some(v) => Some(crate::pyconv::FromPy::from_py(v)?),
    };
    let out = port::get_rich_content_field(candidate_data, index, default);
    Ok(out)
}

/// `getRichContentField`.
#[pyfunction]
#[pyo3(name = "getRichContentField", signature = (candidate_data, index, default))]
fn get_rich_content_field(
    py: Python<'_>,
    candidate_data: Bound<'_, PyAny>,
    index: Bound<'_, PyAny>,
    default: Option<Bound<'_, PyAny>>,
) -> PyResult<Py<PyAny>> {
    get_rich_content_field_inner(candidate_data, index, default)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

fn extract_json_from_response_inner(text: String) -> Result<Vec<Value>, String> {
    let out = port::extract_json_from_response(text);
    Ok(out)
}

/// `extractJsonFromResponse`.
#[pyfunction]
#[pyo3(name = "extractJsonFromResponse", signature = (text))]
fn extract_json_from_response(py: Python<'_>, text: String) -> PyResult<Py<PyAny>> {
    extract_json_from_response_inner(text)
        .map_err(error)
        .and_then(|v| crate::pyconv::ToPy::to_py(&v, py))
}

#[pymodule]
#[pyo3(name = "_native")]
fn native_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(build_model_header, m)?)?;
    m.add_function(wrap_pyfunction!(warn_deprecated_model, m)?)?;
    m.add_function(wrap_pyfunction!(format_http_version, m)?)?;
    m.add_function(wrap_pyfunction!(extract_cookie_value, m)?)?;
    m.add_function(wrap_pyfunction!(hash_psid, m)?)?;
    m.add_function(wrap_pyfunction!(cookie_cache_dir, m)?)?;
    m.add_function(wrap_pyfunction!(is_transient_network_error, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_backoff, m)?)?;
    m.add_function(wrap_pyfunction!(get_cookies_cache_path, m)?)?;
    m.add_function(wrap_pyfunction!(save_cookies, m)?)?;
    m.add_function(wrap_pyfunction!(clear_cookies_cache, m)?)?;
    m.add_function(wrap_pyfunction!(parse_rpc_results, m)?)?;
    m.add_function(wrap_pyfunction!(get_nested_value, m)?)?;
    m.add_function(wrap_pyfunction!(get_sparse_bundle, m)?)?;
    m.add_function(wrap_pyfunction!(get_field, m)?)?;
    m.add_function(wrap_pyfunction!(get_rich_content_field, m)?)?;
    m.add_function(wrap_pyfunction!(extract_json_from_response, m)?)?;
    Ok(())
}
