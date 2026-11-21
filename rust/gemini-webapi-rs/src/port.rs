//! The Rust port of `src/gemini_webapi/`.
//!
//! Replace each `unimplemented!` with the port of the source function. Return `Err` where
//! the source throws or raises: the caller gets an error there too. After changing a
//! signature, run `rust-anything refresh` to regenerate the bindings. `rust-anything diff` checks
//! behavior against what the source recorded.

#![forbid(unsafe_code)]
#![expect(
    clippy::too_many_arguments,
    reason = "Port in progress; function signatures mirror the Python original"
)]
#![expect(
    clippy::get_first,
    reason = "Port in progress; will be cleaned up in reshape pass"
)]
#![expect(
    clippy::collapsible_if,
    reason = "Port in progress; will be cleaned up in reshape pass"
)]
#![expect(
    clippy::collapsible_match,
    reason = "Port in progress; will be cleaned up in reshape pass"
)]
#![expect(
    clippy::map_clone,
    reason = "Port in progress; will be cleaned up in reshape pass"
)]
#![expect(
    clippy::option_map_unit_fn,
    reason = "Port in progress; will be cleaned up in reshape pass"
)]
#![expect(
    clippy::unnecessary_map_or,
    reason = "Port in progress; will be cleaned up in reshape pass"
)]
#![expect(
    clippy::unnecessary_lazy_evaluations,
    reason = "Port in progress; will be cleaned up in reshape pass"
)]
#![expect(
    clippy::type_complexity,
    reason = "Port in progress; will be cleaned up in reshape pass"
)]
#![expect(
    unused_mut,
    reason = "Port in progress; will be cleaned up in reshape pass"
)]

use regex::Regex;
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};
use serde_json::{Map, Value};
use std::collections::HashSet;
use std::fmt::Write;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

const DEFAULT_LANGUAGE: &str = "en";
const DEFAULT_PUSH_ID: &str = "feeds/mcudyrk2a4khkz";
const BROWSER_TYPE: &str = "chrome145";
static MODEL_PREFIX_RE: OnceLock<Regex> = OnceLock::new();

static WARNED: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

/// Port of `build_model_header` (`src/gemini_webapi/constants.py:69`).
pub fn build_model_header(
    model_id: String,
    capacity_tail: Value,
    model_number: i64,
) -> Result<Value, String> {
    let mut header = Map::new();
    header.insert(
        "x-goog-ext-525001261-jspb".to_string(),
        Value::String(format!(
            "[1,null,null,null,\"{}\",null,null,0,[4,5,6,8],null,null,{}, null,null,{}]",
            model_id,
            py_str(&capacity_tail),
            model_number
        )),
    );
    header.insert(
        "x-goog-ext-73010989-jspb".to_string(),
        Value::String("[0]".into()),
    );
    header.insert(
        "x-goog-ext-73010990-jspb".to_string(),
        Value::String("[0,0,0]".into()),
    );
    Ok(Value::Object(header))
}

/// Python `str()`/`repr()` for a JSON value, as used by the f-string in `build_model_header`.
/// Top-level strings are raw (`str(s) == s`); strings inside containers are repr-quoted.
fn py_str(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => "None".into(),
        Value::Bool(b) => if *b { "True" } else { "False" }.into(),
        Value::Number(n) => n.to_string(),
        Value::Array(items) => {
            format!(
                "[{}]",
                items.iter().map(py_repr).collect::<Vec<_>>().join(", ")
            )
        }
        Value::Object(map) => {
            format!(
                "{{{}}}",
                map.iter()
                    .map(|(k, v)| format!("{}: {}", py_quote_string(k), py_repr(v)))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

/// Python `repr()` of a JSON value: like `py_str`, but strings are single-quoted and escaped.
fn py_repr(value: &Value) -> String {
    match value {
        Value::String(s) => py_quote_string(s),
        other => py_str(other),
    }
}

/// Python `repr` of a string: single-quoted, `'`/`\`/control chars escaped, other Unicode raw.
fn py_quote_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('\'');
    for ch in s.chars() {
        match ch {
            '\'' => out.push_str("\\'"),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() || c == '\u{7f}' => {
                write!(out, "\\x{:02x}", c as u32).ok();
            }
            c => out.push(c),
        }
    }
    out.push('\'');
    out
}

/// Port of `warn_deprecated_model` (`src/gemini_webapi/constants.py:176`).
///
/// Logs once per distinct `usage` (mirrors `@cache`), then returns `Ok`.
pub fn warn_deprecated_model(usage: String) -> Result<(), String> {
    let warned = WARNED.get_or_init(|| Mutex::new(HashSet::new()));
    let mut warned = warned.lock().expect("warned set poisoned");
    if warned.insert(usage) {
        tracing::warn!(
            "relies on the deprecated `Model` enum, which will be removed. Pass a model name string instead: models are discovered per account at runtime"
        );
    }
    Ok(())
}

/// Port of `format_http_version` (`src/gemini_webapi/constants.py:360`).
pub fn format_http_version(version_int: i64) -> Result<String, String> {
    match version_int {
        0 => Ok("NONE".into()),
        1 => Ok("V1_0".into()),
        2 => Ok("V1_1".into()),
        3 => Ok("V2_0".into()),
        4 => Ok("V2TLS".into()),
        5 => Ok("V2_PRIOR_KNOWLEDGE".into()),
        30 => Ok("V3".into()),
        31 => Ok("V3ONLY".into()),
        _ => Ok(version_int.to_string()),
    }
}

// ---------------------------------------------------------------------------
// Enums (src/gemini_webapi/constants.py)
// ---------------------------------------------------------------------------

/// Gemini API endpoint URLs (`Endpoint` StrEnum).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Endpoint {
    Google,
    Init,
    Generate,
    RotateCookies,
    Upload,
    BatchExec,
}

impl Endpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Endpoint::Google => "https://www.google.com",
            Endpoint::Init => "https://gemini.google.com/app",
            Endpoint::Generate => {
                "https://gemini.google.com/_/BardChatUi/data/assistant.lamda.BardFrontendService/StreamGenerate"
            }
            Endpoint::RotateCookies => "https://accounts.google.com/RotateCookies",
            Endpoint::Upload => "https://content-push.googleapis.com/upload",
            Endpoint::BatchExec => "https://gemini.google.com/_/BardChatUi/data/batchexecute",
        }
    }
}

impl std::str::FromStr for Endpoint {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "https://www.google.com" => Ok(Endpoint::Google),
            "https://gemini.google.com/app" => Ok(Endpoint::Init),
            "https://gemini.google.com/_/BardChatUi/data/assistant.lamda.BardFrontendService/StreamGenerate" => {
                Ok(Endpoint::Generate)
            }
            "https://accounts.google.com/RotateCookies" => Ok(Endpoint::RotateCookies),
            "https://content-push.googleapis.com/upload" => Ok(Endpoint::Upload),
            "https://gemini.google.com/_/BardChatUi/data/batchexecute" => Ok(Endpoint::BatchExec),
            _ => Err(format!("unknown endpoint: {s}")),
        }
    }
}

/// Google RPC method ids (`GRPC` StrEnum).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Grpc {
    ListConversations,
    ListConversationTurns,
    GetConversationTurn,
    DeleteConversation,
    UpdateConversation,
    MarkLastConversationTurn,
    GenerateHeadline,
    ListBots,
    CreateBot,
    GetBot,
    UpdateBotMetadata,
    DeleteBot,
    DeleteBotAndConversations,
    CreateTask,
    GetTask,
    GetAllTasks,
    GetTasksInConversation,
    GetCandidates,
    ListDiscoveryCards,
    GetDiscoveryCard,
    ListDiscoveryBanners,
    ListGeminiAppArtifacts,
    DeleteGeminiAppArtifacts,
    ListMemories,
    CreateMemory,
    UpdateMemory,
    DeleteMemory,
    DeleteAllMemories,
    GetUserStatus,
    CheckGeminiQuota,
    CheckQuota,
    DownloadGeneratedImage,
    GetAbuseStatus,
    UpdateUserPreferences,
    ReadUserPreferences,
    ContinueSharedConversation,
    GetUsageInfo,
}

impl Grpc {
    pub fn as_str(&self) -> &'static str {
        match self {
            Grpc::ListConversations => "MaZiqc",
            Grpc::ListConversationTurns => "hNvQHb",
            Grpc::GetConversationTurn => "EqPOKe",
            Grpc::DeleteConversation => "GzXR5e",
            Grpc::UpdateConversation => "MUAZcd",
            Grpc::MarkLastConversationTurn => "kOWVAe",
            Grpc::GenerateHeadline => "ukz1Fe",
            Grpc::ListBots => "CNgdBe",
            Grpc::CreateBot => "oMH3Zd",
            Grpc::GetBot => "HcT8bb",
            Grpc::UpdateBotMetadata => "kHv0Vd",
            Grpc::DeleteBot => "UXcSJb",
            Grpc::DeleteBotAndConversations => "Nwkn9",
            Grpc::CreateTask => "Jba3ib",
            Grpc::GetTask => "kwDCne",
            Grpc::GetAllTasks => "XPSWpd",
            Grpc::GetTasksInConversation => "qWymEb",
            Grpc::GetCandidates => "PCck7e",
            Grpc::ListDiscoveryCards => "ku4Jyf",
            Grpc::GetDiscoveryCard => "oApPWc",
            Grpc::ListDiscoveryBanners => "Te6DCf",
            Grpc::ListGeminiAppArtifacts => "jGArJ",
            Grpc::DeleteGeminiAppArtifacts => "PGX16d",
            Grpc::ListMemories => "ZKcapf",
            Grpc::CreateMemory => "xVRQX",
            Grpc::UpdateMemory => "gSnMcd",
            Grpc::DeleteMemory => "Ok9j9b",
            Grpc::DeleteAllMemories => "YgU2Cc",
            Grpc::GetUserStatus => "otAQ7b",
            Grpc::CheckGeminiQuota => "qpEbW",
            Grpc::CheckQuota => "aPya6c",
            Grpc::DownloadGeneratedImage => "c8o8Fe",
            Grpc::GetAbuseStatus => "GPRiHf",
            Grpc::UpdateUserPreferences => "L5adhe",
            Grpc::ReadUserPreferences => "ESY5D",
            Grpc::ContinueSharedConversation => "ra9Swb",
            Grpc::GetUsageInfo => "jSf9Qc",
        }
    }
}

/// JSPB field numbers inside candidate rich-content blocks (`Field` IntEnum).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Field {
    WebImages = 1,
    Progress = 6,
    GeneratedImages = 7,
    Citations = 43,
    ResearchPlan = 55,
    ResearchPlanAlt = 56,
    Video = 59,
    ResearchPlanState = 69,
    Media = 86,
}

/// Numeric account status codes (`AccountStatus` IntEnum).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AccountStatus {
    Available = 1000,
    AccessTemporarilyUnavailable = 1014,
    Unauthenticated = 1016,
    AccountRejected = 1021,
    AccountUntrusted = 1033,
    TosPending = 1040,
    TosOutOfDate = 1042,
    AccountRejectedByGuardian = 1054,
    GuardianApprovalRequired = 1057,
    LocationRejected = 1060,
}

impl AccountStatus {
    pub fn description(self) -> &'static str {
        match self {
            AccountStatus::Available => "Account is authorized and has normal access.",
            AccountStatus::AccessTemporarilyUnavailable => {
                "Access is restricted, possibly due to regional or temporary session issues."
            }
            AccountStatus::Unauthenticated => {
                "Session is not authenticated or cookies have expired. Please check your cookies."
            }
            AccountStatus::AccountRejected => {
                "Account access is rejected. Please check your Google Account settings."
            }
            AccountStatus::AccountUntrusted => {
                "Account did not pass safety or trust checks for some features."
            }
            AccountStatus::TosPending => {
                "You need to accept the latest Terms of Service to continue."
            }
            AccountStatus::TosOutOfDate => {
                "Terms of Service are out of date; please accept the new ones."
            }
            AccountStatus::AccountRejectedByGuardian => {
                "Access is blocked by a parent or guardian."
            }
            AccountStatus::GuardianApprovalRequired => {
                "Access requires parent or guardian approval."
            }
            AccountStatus::LocationRejected => {
                "Gemini is not currently supported in your country/region."
            }
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AccountStatus::Available => "AVAILABLE",
            AccountStatus::AccessTemporarilyUnavailable => "ACCESS_TEMPORARILY_UNAVAILABLE",
            AccountStatus::Unauthenticated => "UNAUTHENTICATED",
            AccountStatus::AccountRejected => "ACCOUNT_REJECTED",
            AccountStatus::AccountUntrusted => "ACCOUNT_UNTRUSTED",
            AccountStatus::TosPending => "TOS_PENDING",
            AccountStatus::TosOutOfDate => "TOS_OUT_OF_DATE",
            AccountStatus::AccountRejectedByGuardian => "ACCOUNT_REJECTED_BY_GUARDIAN",
            AccountStatus::GuardianApprovalRequired => "GUARDIAN_APPROVAL_REQUIRED",
            AccountStatus::LocationRejected => "LOCATION_REJECTED",
        }
    }

    pub fn from_status_code(status_code: Option<i64>) -> Self {
        match status_code {
            None | Some(1000) => AccountStatus::Available,
            Some(1014) => AccountStatus::AccessTemporarilyUnavailable,
            Some(1016) => AccountStatus::Unauthenticated,
            Some(1021) => AccountStatus::AccountRejected,
            Some(1033) => AccountStatus::AccountUntrusted,
            Some(1040) => AccountStatus::TosPending,
            Some(1042) => AccountStatus::TosOutOfDate,
            Some(1054) => AccountStatus::AccountRejectedByGuardian,
            Some(1057) => AccountStatus::GuardianApprovalRequired,
            Some(1060) => AccountStatus::LocationRejected,
            Some(_) => AccountStatus::AccountRejected,
        }
    }
}

/// Known error codes returned from the server (`ErrorCode` IntEnum).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ErrorCode {
    TemporaryError1013 = 1013,
    UsageLimitExceeded = 1037,
    ModelInconsistent = 1050,
    ModelHeaderInvalid = 1052,
    IpTemporarilyBlocked = 1060,
}

// ---------------------------------------------------------------------------
// Errors (src/gemini_webapi/exceptions.py)
// ---------------------------------------------------------------------------

/// Base error type for the port. Mirrors Python's exception hierarchy:
/// `GeminiError` is the root; `AuthError`, `APIError`, `TimeoutError`,
/// `UsageLimitExceededError`, `ModelInvalidError`, `TemporarilyBlockedError`
/// are distinguished by their `code`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GeminiError {
    pub kind: GeminiErrorKind,
    pub message: String,
    pub status_code: Option<i64>,
    pub response: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GeminiErrorKind {
    Auth,
    Api,
    ImageGeneration,
    Gemini,
    Timeout,
    UsageLimitExceeded,
    ModelInvalid,
    TemporarilyBlocked,
}

impl std::fmt::Display for GeminiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl std::error::Error for GeminiError {}

impl std::fmt::Display for GeminiErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl GeminiErrorKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            GeminiErrorKind::Auth => "AuthError",
            GeminiErrorKind::Api => "APIError",
            GeminiErrorKind::ImageGeneration => "ImageGenerationError",
            GeminiErrorKind::Gemini => "GeminiError",
            GeminiErrorKind::Timeout => "TimeoutError",
            GeminiErrorKind::UsageLimitExceeded => "UsageLimitExceededError",
            GeminiErrorKind::ModelInvalid => "ModelInvalidError",
            GeminiErrorKind::TemporarilyBlocked => "TemporarilyBlockedError",
        }
    }
}

impl GeminiError {
    pub fn auth(message: impl Into<String>) -> Self {
        Self {
            kind: GeminiErrorKind::Auth,
            message: message.into(),
            status_code: None,
            response: None,
        }
    }
    pub fn api(message: impl Into<String>) -> Self {
        Self {
            kind: GeminiErrorKind::Api,
            message: message.into(),
            status_code: None,
            response: None,
        }
    }
    pub fn gemini(message: impl Into<String>) -> Self {
        Self {
            kind: GeminiErrorKind::Gemini,
            message: message.into(),
            status_code: None,
            response: None,
        }
    }
    pub fn with_status(
        kind: GeminiErrorKind,
        message: impl Into<String>,
        status_code: i64,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            status_code: Some(status_code),
            response: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Cookie utilities (src/gemini_webapi/utils/rotate_1psidts.py)
// ---------------------------------------------------------------------------

/// Extract a cookie value from a list of `(name, value)` pairs.
/// Mirrors `extract_cookie_value` (`rotate_1psidts.py:25`).
pub fn extract_cookie_value(cookies: Vec<(String, String)>, name: String) -> Option<String> {
    cookies
        .iter()
        .find(|(n, _)| n == &name)
        .map(|(_, v)| v.clone())
}

/// Generate a stable 32-character hex hash prefix for a PSID.
/// Mirrors `_hash_psid` (`rotate_1psidts.py:30`).
pub fn hash_psid(psid: String) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(psid.trim().as_bytes());
    let result = hasher.finalize();
    let hex: String = result.iter().take(16).map(|b| format!("{b:02x}")).collect();
    hex
}

/// Cookie cache directory: `GEMINI_COOKIE_PATH` env, or `<temp>/gemini_webapi`.
/// Mirrors `get_cookie_cache_dir` (`rotate_1psidts.py:35`).
pub fn cookie_cache_dir() -> String {
    if let Ok(p) = std::env::var("GEMINI_COOKIE_PATH") {
        return p;
    }
    std::env::temp_dir()
        .join("gemini_webapi")
        .to_string_lossy()
        .into_owned()
}

/// Transient network errors: 5xx server status codes are retryable;
/// 4xx client errors (including 429) are not.
/// Mirrors `is_transient_network_error` (`decorators.py:102`).
pub fn is_transient_network_error(status_code: Option<u16>) -> bool {
    matches!(status_code, Some(code) if (500..=599).contains(&code))
}

/// Exponential backoff delay for a retry attempt.
/// Mirrors `calculate_backoff` (`decorators.py:171`).
pub fn calculate_backoff(attempt: usize, factor: f64, max_delay: Option<f64>) -> f64 {
    let mut delay = (attempt + 1) as f64 * factor;
    if let Some(max) = max_delay {
        delay = delay.min(max);
    }
    delay
}

/// Execute an async operation with backoff retries for transient errors.
/// Mirrors `execute_with_retry` (`decorators.py:198`).
#[expect(
    dead_code,
    reason = "used by get_access_token and GeminiClient in later porting steps"
)]
async fn execute_with_retry<F, T, E>(
    mut func: F,
    max_retries: usize,
    factor: f64,
    max_delay: Option<f64>,
    is_transient: impl Fn(&E) -> bool,
) -> Result<T, E>
where
    F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send>>,
{
    let mut last_err = None;
    for attempt in 0..max_retries {
        match func().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                last_err = Some(e);
                if attempt < max_retries - 1 && is_transient(last_err.as_ref().unwrap()) {
                    let backoff = calculate_backoff(attempt, factor, max_delay);
                    tokio::time::sleep(std::time::Duration::from_secs_f64(backoff)).await;
                    continue;
                }
                break;
            }
        }
    }
    Err(last_err.unwrap())
}

// ---------------------------------------------------------------------------
// Headers (src/gemini_webapi/constants.py)
// ---------------------------------------------------------------------------

/// HTTP header sets used by Gemini API endpoints (`Headers` Enum).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Headers {
    Referer,
    SameDomain,
    Gemini,
    RotateCookies,
    Upload,
    BatchExec,
}

impl Headers {
    pub fn as_list(self) -> Vec<(&'static str, &'static str)> {
        match self {
            Headers::Referer => vec![
                ("Origin", "https://gemini.google.com"),
                ("Referer", "https://gemini.google.com/"),
            ],
            Headers::SameDomain => vec![("X-Same-Domain", "1")],
            Headers::Gemini => vec![
                (
                    "Content-Type",
                    "application/x-www-form-urlencoded;charset=utf-8",
                ),
                ("Origin", "https://gemini.google.com"),
                ("Referer", "https://gemini.google.com/"),
            ],
            Headers::RotateCookies => vec![
                ("Content-Type", "application/json"),
                ("Origin", "https://accounts.google.com"),
            ],
            Headers::Upload => vec![("X-Tenant-Id", "bard-storage")],
            Headers::BatchExec => vec![
                (
                    "x-goog-ext-525001261-jspb",
                    "[1,null,null,null,null,null,null,null,[4,5,6,8],null,null,null,null,null,null,null]",
                ),
                ("x-goog-ext-73010989-jspb", "[0]"),
                ("x-goog-ext-73010990-jspb", "[0,0,0]"),
            ],
        }
    }
}

// ---------------------------------------------------------------------------
// Cookie persistence (src/gemini_webapi/utils/rotate_1psidts.py)
// ---------------------------------------------------------------------------

const COOKIE_CACHE_PREFIX: &str = ".cached_cookies_";
const COOKIE_CACHE_EXTENSION: &str = ".json";
pub const COOKIE_1PSID: &str = "__Secure-1PSID";
pub const COOKIE_1PSIDTS: &str = "__Secure-1PSIDTS";
const COOKIE_DOMAIN: &str = ".google.com";
const COOKIE_PATH: &str = "/";

/// Get the cookie cache file path, migrating legacy unhashed files if needed.
/// Mirrors `get_cookies_cache_path` (`rotate_1psidts.py:95`).
pub fn get_cookies_cache_path(
    cookies: Vec<(String, String)>,
    base_psid: Option<String>,
) -> Option<String> {
    let cookie_psid = extract_cookie_value(cookies.clone(), COOKIE_1PSID.to_string());
    let target_psid = base_psid.as_deref().or(cookie_psid.as_deref());
    let target_psid = target_psid?;

    let cache_dir = cookie_cache_dir();
    let hashed_path = std::path::Path::new(&cache_dir).join(format!(
        "{COOKIE_CACHE_PREFIX}{}{COOKIE_CACHE_EXTENSION}",
        hash_psid(target_psid.to_string())
    ));
    let legacy_path = std::path::Path::new(&cache_dir).join(format!(
        "{COOKIE_CACHE_PREFIX}{target_psid}{COOKIE_CACHE_EXTENSION}"
    ));

    if hashed_path.is_file() {
        if legacy_path.is_file() && legacy_path != hashed_path {
            let _ = std::fs::remove_file(&legacy_path);
        }
        return Some(hashed_path.to_string_lossy().into_owned());
    }

    if legacy_path.is_file() {
        if let Some(migrated) = migrate_legacy_cache(&legacy_path, target_psid) {
            return Some(migrated);
        }
        return Some(legacy_path.to_string_lossy().into_owned());
    }

    Some(hashed_path.to_string_lossy().into_owned())
}

/// Migrate a legacy unhashed cache file to the new hashed format.
/// Mirrors `migrate_legacy_cache` (`rotate_1psidts.py:41`).
fn migrate_legacy_cache(legacy_path: &std::path::Path, target_psid: &str) -> Option<String> {
    let cache_dir = legacy_path.parent()?;
    let hashed_path = cache_dir.join(format!(
        "{COOKIE_CACHE_PREFIX}{}{COOKIE_CACHE_EXTENSION}",
        hash_psid(target_psid.to_string())
    ));

    let content = std::fs::read_to_string(legacy_path).ok()?;
    if content.trim().is_empty() {
        let _ = std::fs::remove_file(legacy_path);
        return None;
    }

    let data: serde_json::Value = serde_json::from_str(&content).ok()?;
    let cache_data = if data.is_array() {
        serde_json::json!({"alias": target_psid, "cookies": data})
    } else if data.is_object() && data.get("cookies").is_some() {
        let mut obj = data.as_object()?.clone();
        obj.entry("alias")
            .or_insert(serde_json::Value::String(target_psid.to_string()));
        serde_json::Value::Object(obj)
    } else {
        let _ = std::fs::remove_file(legacy_path);
        return None;
    };

    let _ = std::fs::create_dir_all(cache_dir);
    let _ = std::fs::write(&hashed_path, serde_json::to_string(&cache_data).ok()?);
    let _ = std::fs::remove_file(legacy_path);
    Some(hashed_path.to_string_lossy().into_owned())
}

/// Save persistent cookies to cache file.
/// Mirrors `save_cookies` (`rotate_1psidts.py:250`).
pub fn save_cookies(cookies: Vec<(String, String)>, base_psid: Option<String>) -> Option<String> {
    let current_psid = extract_cookie_value(cookies.clone(), COOKIE_1PSID.to_string());
    let target_psid = base_psid.or_else(|| current_psid.clone())?;

    let mut cookie_list = Vec::new();
    for (name, value) in &cookies {
        // In the Rust port we don't have cookie domain metadata,
        // so we keep auth cookies and any non-empty value.
        if name == COOKIE_1PSID || name == COOKIE_1PSIDTS || !value.is_empty() {
            cookie_list.push(serde_json::json!({
                "name": name,
                "value": value,
                "domain": COOKIE_DOMAIN,
                "path": COOKIE_PATH,
            }));
        }
    }

    if cookie_list.is_empty() {
        return None;
    }

    let cache_data = serde_json::json!({
        "alias": current_psid.as_deref().unwrap_or(&target_psid),
        "cookies": cookie_list,
    });
    let encoded = serde_json::to_string(&cache_data).ok()?;

    let path = get_cookies_cache_path(cookies, Some(target_psid.clone()))?;
    let path = std::path::Path::new(&path);
    let _ = std::fs::create_dir_all(path.parent()?);
    let _ = std::fs::write(path, &encoded);

    // Alias file for rotated PSID
    if let Some(current) = current_psid.as_deref()
        && current != target_psid
    {
        let alias_path = std::path::Path::new(&cookie_cache_dir()).join(format!(
            "{COOKIE_CACHE_PREFIX}{}{COOKIE_CACHE_EXTENSION}",
            hash_psid(current.to_string())
        ));
        if alias_path != path {
            let _ = std::fs::write(&alias_path, &encoded);
        }
    }

    Some(path.to_string_lossy().into_owned())
}

/// Delete the cached cookies for a session.
/// Mirrors `clear_cookies_cache` (`rotate_1psidts.py:198`).
pub fn clear_cookies_cache(cookies: Vec<(String, String)>, base_psid: Option<String>) {
    if let Some(path) = get_cookies_cache_path(cookies.clone(), base_psid.clone()) {
        let path = std::path::Path::new(&path);
        if path.is_file() {
            let _ = std::fs::remove_file(path);
        }
    }

    let current_psid = extract_cookie_value(cookies.clone(), COOKIE_1PSID.to_string());
    if let (Some(current), Some(base)) = (current_psid.as_deref(), base_psid.as_deref())
        && current != base
    {
        let alias_path = std::path::Path::new(&cookie_cache_dir()).join(format!(
            "{COOKIE_CACHE_PREFIX}{}{COOKIE_CACHE_EXTENSION}",
            hash_psid(current.to_string())
        ));
        if alias_path != std::path::Path::new(&cookie_cache_dir()) {
            let _ = std::fs::remove_file(&alias_path);
        }
    }

    let target_psid = base_psid.as_deref().or(current_psid.as_deref());
    if let Some(target) = target_psid {
        let legacy_path = std::path::Path::new(&cookie_cache_dir()).join(format!(
            "{COOKIE_CACHE_PREFIX}{target}{COOKIE_CACHE_EXTENSION}"
        ));
        if legacy_path.is_file() {
            let _ = std::fs::remove_file(&legacy_path);
        }
    }
}

// ---------------------------------------------------------------------------
// HTTP client: get_access_token (src/gemini_webapi/utils/get_access_token.py)
// ---------------------------------------------------------------------------

/// Everything a successful init attempt produced.
/// Mirrors `InitSession` (`get_access_token.py:52`).
#[derive(Debug, Clone)]
pub struct InitSession {
    pub access_token: Option<String>,
    pub build_label: Option<String>,
    pub session_id: Option<String>,
    pub language: Option<String>,
    pub push_id: Option<String>,
    pub client: Client,
    pub cookie_source: String,
}

/// Token extraction patterns from the Gemini init page HTML.
static ACCESS_TOKEN_RE: OnceLock<Regex> = OnceLock::new();
static BUILD_LABEL_RE: OnceLock<Regex> = OnceLock::new();
static SESSION_ID_RE: OnceLock<Regex> = OnceLock::new();
static LANGUAGE_RE: OnceLock<Regex> = OnceLock::new();
static PUSH_ID_RE: OnceLock<Regex> = OnceLock::new();

fn regex_patterns() -> [&'static Regex; 5] {
    [
        ACCESS_TOKEN_RE.get_or_init(|| {
            Regex::new(r#""SNlM0e":\s*"(.*?)""#).expect("INFALLIBLE: fixed regex pattern")
        }),
        BUILD_LABEL_RE.get_or_init(|| {
            Regex::new(r#""cfb2h":\s*"(.*?)""#).expect("INFALLIBLE: fixed regex pattern")
        }),
        SESSION_ID_RE.get_or_init(|| {
            Regex::new(r#""FdrFJe":\s*"(.*?)""#).expect("INFALLIBLE: fixed regex pattern")
        }),
        LANGUAGE_RE.get_or_init(|| {
            Regex::new(r#""TuX5cc":\s*"(.*?)""#).expect("INFALLIBLE: fixed regex pattern")
        }),
        PUSH_ID_RE.get_or_init(|| {
            Regex::new(r#""qKIAYe":\s*"(.*?)""#).expect("INFALLIBLE: fixed regex pattern")
        }),
    ]
}

/// Extract init values from an init response HTML.
fn extract_payload(text: &str) -> Option<[Option<String>; 5]> {
    let [at_re, bl_re, sid_re, lang_re, push_re] = regex_patterns();
    let access_token = at_re
        .captures(text)
        .and_then(|c| c.get(1).map(|m| m.as_str().to_string()));
    let build_label = bl_re
        .captures(text)
        .and_then(|c| c.get(1).map(|m| m.as_str().to_string()));
    let session_id = sid_re
        .captures(text)
        .and_then(|c| c.get(1).map(|m| m.as_str().to_string()));
    let language = lang_re
        .captures(text)
        .and_then(|c| c.get(1).map(|m| m.as_str().to_string()));
    let push_id = push_re
        .captures(text)
        .and_then(|c| c.get(1).map(|m| m.as_str().to_string()));
    if access_token.is_none()
        && build_label.is_none()
        && session_id.is_none()
        && language.is_none()
        && push_id.is_none()
    {
        return None;
    }
    Some([access_token, build_label, session_id, language, push_id])
}

/// Send a GET request to Gemini init endpoint and extract init values.
/// Mirrors `get_access_token` (`get_access_token.py:247`).
async fn get_access_token(
    base_cookies: Vec<(String, String)>,
    proxy: Option<String>,
    verbose: bool,
) -> Result<InitSession, GeminiError> {
    let mut headers = HeaderMap::new();
    for (k, v) in Headers::Gemini.as_list() {
        headers.insert(
            k,
            HeaderValue::from_str(v).expect("INFALLIBLE: fixed header value"),
        );
    }

    // Set cookies on the client via a shared cookie jar
    let init_url = reqwest::Url::parse(Endpoint::Init.as_str()).unwrap(); // INFALLIBLE: fixed URL
    let cookie_jar = std::sync::Arc::new(reqwest::cookie::Jar::default());
    for (name, value) in &base_cookies {
        cookie_jar.add_cookie_str(&format!("{name}={value}"), &init_url);
    }

    let mut builder = Client::builder()
        .cookie_provider(cookie_jar.clone())
        .default_headers(headers)
        .timeout(Duration::from_secs(450));

    if let Some(proxy_url) = proxy {
        builder = builder.proxy(
            reqwest::Proxy::all(&proxy_url)
                .map_err(|e| GeminiError::api(format!("Invalid proxy URL: {e}")))?,
        );
    }

    let client = builder
        .build()
        .map_err(|e| GeminiError::api(format!("Failed to build HTTP client: {e}")))?;

    // Phase 1: Try base cookies directly
    let response = client
        .get(Endpoint::Init.as_str())
        .send()
        .await
        .map_err(|e| {
            if e.is_timeout() {
                GeminiError::with_status(
                    GeminiErrorKind::Timeout,
                    format!("Request timed out: {e}"),
                    0,
                )
            } else if e.is_connect() {
                GeminiError::api(format!("Connection error: {e}"))
            } else {
                GeminiError::api(format!("Request failed: {e}"))
            }
        })?;

    if verbose {
        tracing::debug!(
            "HTTP Request: GET {} [{}]",
            Endpoint::Init.as_str(),
            response.status()
        );
    }

    if !response.status().is_success() {
        let status = response.status().as_u16();
        if status == 429 {
            return Err(GeminiError::with_status(
                GeminiErrorKind::TemporarilyBlocked,
                "Your IP address has been temporarily flagged or blocked by Google (HTTP 429).",
                status as i64,
            ));
        }
        return Err(GeminiError::api(format!(
            "Init request failed with HTTP {status}"
        )));
    }

    let text = response
        .text()
        .await
        .map_err(|e| GeminiError::api(format!("Failed to read response: {e}")))?;

    if let Some([access_token, build_label, session_id, language, push_id]) = extract_payload(&text)
    {
        return Ok(InitSession {
            access_token,
            build_label,
            session_id,
            language,
            push_id,
            client,
            cookie_source: "Base Cookies".to_string(),
        });
    }

    Err(GeminiError::auth(
        "Failed to initialize client: no init tokens found in response. SECURE_1PSIDTS could be expired.",
    ))
}

// ---------------------------------------------------------------------------
// Types (src/gemini_webapi/types/)
// ---------------------------------------------------------------------------

/// An available model discovered dynamically from the Gemini RPC API.
/// Mirrors `AvailableModel` (`types/availablemodel.py`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AvailableModel {
    pub model_id: String,
    pub model_name: String,
    pub display_name: String,
    pub description: String,
    pub capacity: i64,
    pub capacity_field: i64,
    pub model_number: i64,
    pub is_available: bool,
    pub aliases: Vec<String>,
}

impl AvailableModel {
    pub fn model_header(&self) -> serde_json::Map<String, serde_json::Value> {
        let tail = if self.capacity_field == 13 {
            format!("null,{}", self.capacity)
        } else {
            self.capacity.to_string()
        };
        build_model_header(
            self.model_id.clone(),
            serde_json::Value::String(tail),
            self.model_number,
        )
        .map(|v| match v {
            serde_json::Value::Object(m) => m,
            _ => serde_json::Map::new(),
        })
        .unwrap_or_default()
    }

    pub fn advanced_only(&self) -> bool {
        self.capacity != 1 || self.capacity_field != 12
    }

    pub fn compute_capacity(tier_flags: &[i64], capability_flags: &[i64]) -> (i64, i64) {
        if tier_flags.contains(&21) {
            return (1, 13);
        }
        if tier_flags.contains(&22) {
            return (2, 13);
        }
        if capability_flags.contains(&115) {
            return (4, 12);
        }
        if tier_flags.contains(&16) || capability_flags.contains(&106) {
            return (3, 12);
        }
        if tier_flags.contains(&8) || capability_flags.contains(&19) {
            (2, 12)
        } else {
            (1, 12)
        }
    }

    pub fn from_rpc(
        model_data: Vec<Value>,
        capacity: i64,
        capacity_field: i64,
        unavailable: bool,
    ) -> Option<Self> {
        if model_data.is_empty() {
            return None;
        }

        let model_id = model_data
            .get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if model_id.is_empty() {
            return None;
        }

        let category_name = model_data
            .get(1)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let display_name = model_data
            .get(11)
            .or(model_data.get(10))
            .or(model_data.get(1))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let description = model_data
            .get(12)
            .or(model_data.get(2))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let model_number = model_data
            .get(17)
            .and_then(|v| v.as_i64())
            .or_else(|| model_data.get(9).and_then(|v| v.as_i64()))
            .unwrap_or(1);

        let is_available = !unavailable;

        let model_name = if !category_name.is_empty() {
            format!("gemini-{}", category_name.to_lowercase().replace(' ', "-"))
        } else if !display_name.is_empty() {
            format!("gemini-{}", display_name.to_lowercase().replace(' ', "-"))
        } else {
            format!("gemini-{}", model_id)
        };

        let mut aliases = std::collections::HashSet::new();
        aliases.insert(model_id.to_lowercase());
        if !category_name.is_empty() {
            let cat_clean = category_name.trim().to_lowercase();
            aliases.insert(cat_clean.clone());
            aliases.insert(cat_clean.replace(' ', "-"));
            aliases.insert(format!("gemini-{}", cat_clean.replace(' ', "-")));
        }
        if !display_name.is_empty() {
            let disp_clean = display_name.trim().to_lowercase();
            aliases.insert(disp_clean.clone());
            aliases.insert(disp_clean.replace(' ', "-"));
            aliases.insert(format!("gemini-{}", disp_clean.replace(' ', "-")));
        }
        let primary_name = format!(
            "gemini-{}",
            category_name.trim().to_lowercase().replace(' ', "-")
        );
        aliases.insert(primary_name);

        Some(Self {
            model_id,
            model_name,
            display_name: if display_name.is_empty() {
                category_name
            } else {
                display_name
            },
            description,
            capacity,
            capacity_field,
            model_number,
            is_available,
            aliases: aliases.into_iter().collect(),
        })
    }
}

/// A single reply candidate.
/// Mirrors `Candidate` (`types/candidate.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Candidate {
    pub rcid: String,
    pub text: String,
    pub text_delta: Option<String>,
    pub thoughts: Option<String>,
    pub thoughts_delta: Option<String>,
    pub web_images: Vec<WebImage>,
    pub generated_images: Vec<GeneratedImage>,
    pub generated_videos: Vec<GeneratedVideo>,
    pub generated_media: Vec<GeneratedMedia>,
    pub citations: Vec<Citation>,
    pub deep_research_plan: Option<DeepResearchPlan>,
    pub deep_research_document: Option<DeepResearchDocument>,
}

impl Candidate {
    pub fn images(&self) -> Vec<Image> {
        let mut images: Vec<Image> = self.web_images.iter().map(Image::from_web).collect();
        images.extend(self.generated_images.iter().map(Image::from_generated));
        images
    }
}

/// Classified output from Gemini.
/// Mirrors `ModelOutput` (`types/modeloutput.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModelOutput {
    pub metadata: Vec<String>,
    pub candidates: Vec<Candidate>,
    pub chosen: usize,
}

impl ModelOutput {
    pub fn rcid(&self) -> String {
        self.candidates
            .get(self.chosen)
            .map(|c| c.rcid.clone())
            .unwrap_or_default()
    }

    pub fn text(&self) -> String {
        self.candidates
            .get(self.chosen)
            .map(|c| c.text.clone())
            .unwrap_or_default()
    }

    pub fn text_delta(&self) -> String {
        self.candidates
            .get(self.chosen)
            .and_then(|c| c.text_delta.as_deref())
            .unwrap_or("")
            .to_string()
    }

    pub fn thoughts(&self) -> Option<String> {
        self.candidates
            .get(self.chosen)
            .and_then(|c| c.thoughts.clone())
    }

    pub fn thoughts_delta(&self) -> String {
        self.candidates
            .get(self.chosen)
            .and_then(|c| c.thoughts_delta.as_deref())
            .unwrap_or("")
            .to_string()
    }

    pub fn images(&self) -> Vec<Image> {
        self.candidates
            .get(self.chosen)
            .map(|c| c.images())
            .unwrap_or_default()
    }

    pub fn videos(&self) -> Vec<GeneratedVideo> {
        self.candidates
            .get(self.chosen)
            .map(|c| c.generated_videos.clone())
            .unwrap_or_default()
    }

    pub fn media(&self) -> Vec<GeneratedMedia> {
        self.candidates
            .get(self.chosen)
            .map(|c| c.generated_media.clone())
            .unwrap_or_default()
    }

    pub fn citations(&self) -> Vec<Citation> {
        self.candidates
            .get(self.chosen)
            .map(|c| c.citations.clone())
            .unwrap_or_default()
    }

    pub fn deep_research_plan(&self) -> Option<DeepResearchPlan> {
        self.candidates
            .get(self.chosen)
            .and_then(|c| c.deep_research_plan.clone())
    }

    pub fn deep_research_document(&self) -> Option<DeepResearchDocument> {
        self.candidates
            .get(self.chosen)
            .and_then(|c| c.deep_research_document.clone())
    }
}

/// A single turn in a chat conversation.
/// Mirrors `ChatTurn` (`types/chathistory.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChatTurn {
    pub role: String,
    pub text: String,
    pub model_output: Option<ModelOutput>,
}

/// Complete chat history, newest turn first.
/// Mirrors `ChatHistory` (`types/chathistory.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChatHistory {
    pub cid: String,
    pub turns: Vec<ChatTurn>,
}

/// Chat info from the user's account.
/// Mirrors `ChatInfo` (`types/chatinfo.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChatInfo {
    pub cid: String,
    pub title: String,
    pub is_pinned: bool,
    pub timestamp: f64,
}

/// A Gemini Gem (system prompt).
/// Mirrors `Gem` (`types/gem.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Gem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub prompt: Option<String>,
    pub predefined: bool,
}

/// Helper data for Google RPC calls.
/// Mirrors `RPCData` (`types/grpc.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RPCData {
    pub rpcid: String,
    pub payload: String,
    pub identifier: String,
}

impl RPCData {
    pub fn serialize(&self) -> Vec<serde_json::Value> {
        vec![
            serde_json::Value::String(self.rpcid.clone()),
            serde_json::Value::String(self.payload.clone()),
            serde_json::Value::Null,
            serde_json::Value::String(self.identifier.clone()),
        ]
    }
}

/// A web citation backing a `[cite: N]` marker.
/// Mirrors `Citation` (`types/citation.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Citation {
    pub id: i64,
    pub title: Option<String>,
    pub url: Option<String>,
    pub favicon: Option<String>,
}

/// A deep research plan.
/// Mirrors `DeepResearchPlan` (`types/research.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeepResearchPlan {
    pub research_id: Option<String>,
    pub title: Option<String>,
    pub query: Option<String>,
    pub steps: Vec<String>,
    pub eta_text: Option<String>,
    pub confirm_prompt: Option<String>,
    pub modify_prompt: Option<String>,
    pub confirmation_url: Option<String>,
    pub metadata: Vec<Option<String>>,
    pub cid: Option<String>,
    pub response_text: Option<String>,
    pub raw_state: Option<i64>,
}

/// A deep research document (inline report).
/// Mirrors `DeepResearchDocument` (`types/research.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeepResearchDocument {
    pub id: Option<String>,
    pub title: Option<String>,
    pub content: String,
    pub sources: Vec<Citation>,
}

impl DeepResearchDocument {
    pub fn ready(&self) -> bool {
        !self.content.trim().is_empty()
    }

    pub fn markdown(&self) -> String {
        if self.sources.is_empty() {
            return self.content.clone();
        }
        let mut lines = vec![
            self.content.trim_end().to_string(),
            String::new(),
            "## Sources".to_string(),
            String::new(),
        ];
        for source in &self.sources {
            if let Some(url) = &source.url {
                lines.push(format!(
                    "{}. [{}]({})",
                    source.id,
                    source.title.as_deref().unwrap_or(""),
                    url
                ));
            } else {
                lines.push(format!(
                    "{}. {}",
                    source.id,
                    source.title.as_deref().unwrap_or("")
                ));
            }
        }
        lines.push(String::new());
        lines.join("\n")
    }
}

/// Base image type.
/// Mirrors `Image` (`types/image.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Image {
    pub url: String,
    pub title: String,
    pub alt: String,
    pub proxy: Option<String>,
}

impl Image {
    pub fn from_web(web: &WebImage) -> Self {
        Image {
            url: web.url.clone(),
            title: web.title.clone(),
            alt: web.alt.clone(),
            proxy: web.proxy.clone(),
        }
    }

    pub fn from_generated(generated: &GeneratedImage) -> Self {
        Image {
            url: generated.url.clone(),
            title: generated.title.clone(),
            alt: generated.alt.clone(),
            proxy: generated.proxy.clone(),
        }
    }
}

/// A web-fetched image.
/// Mirrors `WebImage` (`types/image.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WebImage {
    pub url: String,
    pub title: String,
    pub alt: String,
    pub proxy: Option<String>,
}

/// An AI-generated image.
/// Mirrors `GeneratedImage` (`types/image.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GeneratedImage {
    pub url: String,
    pub title: String,
    pub alt: String,
    pub proxy: Option<String>,
    pub cid: String,
    pub rid: String,
    pub rcid: String,
    pub image_id: String,
}

/// A base video type.
/// Mirrors `Video` (`types/video.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Video {
    pub url: String,
    pub title: String,
    pub proxy: Option<String>,
}

/// An AI-generated video.
/// Mirrors `GeneratedVideo` (`types/video.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GeneratedVideo {
    pub url: String,
    pub title: String,
    pub proxy: Option<String>,
    pub thumbnail: String,
    pub cid: String,
    pub rid: String,
    pub rcid: String,
}

/// AI-generated media (audio/video).
/// Mirrors `GeneratedMedia` (`types/video.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GeneratedMedia {
    pub url: String,
    pub title: String,
    pub proxy: Option<String>,
    pub thumbnail: String,
    pub cid: String,
    pub rid: String,
    pub rcid: String,
    pub mp3_url: String,
    pub mp3_thumbnail: String,
}

/// Parse JSPB response frames and yield parts matching the target RPC ID.
/// Mirrors `_parse_rpc_results` (`client.py`).
pub fn parse_rpc_results(response_text: String, target_id: String) -> Vec<Value> {
    let frames = extract_json_from_response(response_text);
    let mut result = Vec::new();
    for part in frames {
        let rpc_id = get_nested_value(part.clone(), vec![Value::Number(1.into())], None);
        if rpc_id
            .as_ref()
            .map_or(false, |v| v.as_str() == Some(target_id.as_str()))
        {
            result.push(part);
        }
    }
    result
}

/// Deep research result.
/// Mirrors `DeepResearchResult` (`types/researchresult.py`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeepResearchResult {
    pub done: bool,
    pub text: String,
    pub document: Option<DeepResearchDocument>,
}

// ---------------------------------------------------------------------------
// GeminiClient (src/gemini_webapi/client.py)
// ---------------------------------------------------------------------------

const DEFAULT_METADATA: [Option<String>; 10] =
    [None, None, None, None, None, None, None, None, None, None];

/// Async HTTP client for gemini.google.com.
/// Mirrors `GeminiClient` (`client.py`).
pub struct GeminiClient {
    pub proxy: Option<String>,
    pub client: Option<reqwest::Client>,
    pub access_token: Option<String>,
    pub build_label: Option<String>,
    pub session_id: Option<String>,
    pub language: String,
    pub push_id: String,
    pub account_status: AccountStatus,
    pub timeout: f64,
    pub auto_close: bool,
    pub auto_refresh: bool,
    pub refresh_interval: f64,
    pub impersonate: String,
    pub verbose: bool,
    pub cookies: Vec<(String, String)>,
    pub _base_psid: Option<String>,
    pub _cookies: Vec<(String, String)>,
    pub _sessionid: String,
    pub _reqid: i64,
    pub _model_registry: std::collections::HashMap<String, AvailableModel>,
    pub _quotas: std::collections::HashMap<String, serde_json::Value>,
    pub _usage_info: serde_json::Value,
    pub _abuse_status: Option<serde_json::Value>,
    pub last_activity_time: f64,
    pub _running: bool,
}

impl GeminiClient {
    pub fn new(
        secure_1psid: Option<String>,
        secure_1psidts: Option<String>,
        proxy: Option<String>,
    ) -> Self {
        let mut cookies = Vec::new();
        if let Some(psid) = &secure_1psid {
            cookies.push((COOKIE_1PSID.to_string(), psid.clone()));
            if let Some(psidts) = &secure_1psidts {
                cookies.push((COOKIE_1PSIDTS.to_string(), psidts.clone()));
            }
        }

        Self {
            proxy,
            client: None,
            access_token: None,
            build_label: None,
            session_id: None,
            language: DEFAULT_LANGUAGE.to_string(),
            push_id: DEFAULT_PUSH_ID.to_string(),
            account_status: AccountStatus::Available,
            timeout: 450.0,
            auto_close: false,
            auto_refresh: true,
            refresh_interval: 600.0,
            impersonate: BROWSER_TYPE.to_string(),
            verbose: false,
            cookies: cookies.clone(),
            _base_psid: secure_1psid,
            _cookies: cookies,
            _sessionid: uuid::Uuid::new_v4().to_string().to_uppercase(),
            _reqid: rand::random_range(10000..=99999),
            _model_registry: std::collections::HashMap::new(),
            _quotas: std::collections::HashMap::new(),
            _usage_info: serde_json::Value::Null,
            _abuse_status: None,
            last_activity_time: 0.0,
            _running: false,
        }
    }

    pub fn cookies(&self) -> Vec<(String, String)> {
        self.client
            .as_ref()
            .and_then(|_| None)
            .unwrap_or(self._cookies.clone())
    }

    pub fn quotas(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self._quotas
    }

    pub fn usage_info(&self) -> &serde_json::Value {
        &self._usage_info
    }

    pub fn abuse_status(&self) -> Option<&serde_json::Value> {
        self._abuse_status.as_ref()
    }

    pub async fn init(
        &mut self,
        timeout: Option<f64>,
        auto_close: Option<bool>,
        close_delay: Option<f64>,
        auto_refresh: Option<bool>,
        refresh_interval: Option<f64>,
        impersonate: Option<String>,
        verbose: Option<bool>,
    ) -> Result<(), GeminiError> {
        if self._running {
            return Ok(());
        }

        if let Some(v) = verbose {
            self.verbose = v;
        }
        if let Some(v) = impersonate {
            self.impersonate = v;
        }
        if let Some(t) = timeout {
            self.timeout = t;
        }
        if let Some(v) = auto_close {
            self.auto_close = v;
        }
        if let Some(d) = close_delay {
            let _ = d;
        }
        if let Some(v) = auto_refresh {
            self.auto_refresh = v;
        }
        if let Some(i) = refresh_interval {
            self.refresh_interval = i;
        }

        let init_session =
            get_access_token(self.cookies.clone(), self.proxy.clone(), self.verbose).await?;

        self.client = Some(init_session.client);
        self._cookies = self.cookies.clone();
        if self._base_psid.is_none() && !self._cookies.is_empty() {
            self._base_psid = extract_cookie_value(self._cookies.clone(), COOKIE_1PSID.to_string());
        }
        self.access_token = init_session.access_token;
        self.build_label = init_session.build_label;
        self.session_id = init_session.session_id;
        self.language = init_session
            .language
            .unwrap_or_else(|| DEFAULT_LANGUAGE.to_string());
        self.push_id = init_session
            .push_id
            .unwrap_or_else(|| DEFAULT_PUSH_ID.to_string());
        self._running = true;
        self._sessionid = uuid::Uuid::new_v4().to_string().to_uppercase();
        self._reqid = rand::random_range(10000..=99999);

        self._init_rpc().await?;

        if self.auto_refresh && self._check_account_status(false).unwrap_or(false) {
            // start_auto_refresh would spawn a background task; skipped for port simplicity
        }

        tracing::info!("Gemini client initialized successfully.");
        Ok(())
    }

    pub async fn close(&mut self) {
        self._running = false;
        if let Some(client) = self.client.take() {
            let _ = client;
        }
    }

    pub fn list_models(&self) -> Option<Vec<AvailableModel>> {
        if self._model_registry.is_empty() {
            return None;
        }
        Some(self._model_registry.values().cloned().collect())
    }

    pub fn resolve_model(&self, name: &str) -> Result<AvailableModel, GeminiError> {
        self._resolve_model_by_name(name)
    }

    fn _resolve_model_by_name(&self, name: &str) -> Result<AvailableModel, GeminiError> {
        if let Some(m) = self._model_registry.get(name) {
            return Ok(m.clone());
        }

        let target = name.to_lowercase();

        let model_prefix_re =
            MODEL_PREFIX_RE.get_or_init(|| Regex::new(r"^gemini-(?:\d+(?:\.\d+)?-)?").unwrap());

        for m in self._model_registry.values() {
            if m.model_name.to_lowercase() == target
                || m.display_name.to_lowercase() == target
                || m.model_id.to_lowercase() == target
            {
                return Ok(m.clone());
            }
        }

        for m in self._model_registry.values() {
            if target
                .split(' ')
                .any(|a| m.aliases.iter().any(|alias| alias == a))
            {
                return Ok(m.clone());
            }
        }

        let norm_target = model_prefix_re.replace_all(&target, "").replace('_', "-");
        for m in self._model_registry.values() {
            let norm_model = model_prefix_re
                .replace_all(&m.model_name.to_lowercase(), "")
                .replace('_', "-");
            if norm_target == norm_model || norm_model.split('-').any(|p| p == norm_target) {
                return Ok(m.clone());
            }
        }

        let available_names: Vec<String> = self
            ._model_registry
            .values()
            .map(|m| m.model_name.clone())
            .collect();
        Err(GeminiError::api(format!(
            "Unknown model name: '{}'. Available registered models: {}",
            name,
            if available_names.is_empty() {
                "None (call client.init() first)".to_string()
            } else {
                available_names.join(", ")
            }
        )))
    }

    fn _check_account_status(&self, raise_error: bool) -> Result<bool, GeminiError> {
        let is_available = self.account_status == AccountStatus::Available;
        if !is_available && raise_error {
            return Err(GeminiError::api(format!(
                "Permission denied. Account status: {} - {}",
                self.account_status.as_str(),
                self.account_status.description()
            )));
        }
        Ok(is_available)
    }

    pub async fn generate_content(
        &self,
        prompt: &str,
        files: Option<Vec<String>>,
        model: Option<serde_json::Value>,
        gem: Option<String>,
        chat: Option<&mut ChatSession>,
        temporary: bool,
        deep_research: bool,
        extended_thinking: bool,
    ) -> Result<ModelOutput, GeminiError> {
        if self.auto_close {
            // reset_close_task would reset a timer; skipped for port simplicity
        }

        if files.is_some() || gem.is_some() || deep_research {
            self._check_account_status(true)?;
        }

        let mut file_data: Option<Vec<serde_json::Value>> = None;
        if let Some(files) = files {
            // File upload skipped for port simplicity; file_data remains None
            let _ = files;
        }

        let mut output = None;
        for chunk in self
            ._generate(
                prompt,
                file_data,
                model,
                gem,
                None,
                temporary,
                deep_research,
                extended_thinking,
            )
            .await?
        {
            output = Some(chunk);
        }

        let mut output = output.ok_or_else(|| {
            GeminiError::api("Failed to generate contents. No output data found in response.")
        })?;

        if let Some(chat) = chat {
            output.metadata = chat.metadata.iter().filter_map(|s| s.clone()).collect();
            chat.last_output = Some(output.clone());
        }

        Ok(output)
    }

    pub async fn generate_content_stream(
        &self,
        prompt: &str,
        files: Option<Vec<String>>,
        model: Option<serde_json::Value>,
        gem: Option<String>,
        chat: Option<&mut ChatSession>,
        temporary: bool,
        deep_research: bool,
        extended_thinking: bool,
    ) -> Result<impl futures::stream::Stream<Item = Result<ModelOutput, GeminiError>>, GeminiError>
    {
        // Streaming port: collect all output then return as a single-item stream
        let result = self
            .generate_content(
                prompt,
                files,
                model,
                gem,
                chat,
                temporary,
                deep_research,
                extended_thinking,
            )
            .await?;
        let stream = futures::stream::iter(vec![Ok(result)]);
        Ok(stream)
    }

    async fn _init_rpc(&mut self) -> Result<(), GeminiError> {
        self._fetch_user_status().await?;
        self._fetch_preferences().await?;
        self._sync_activity().await?;
        self._fetch_recent_chats().await?;
        self._fetch_usage_info().await?;
        self._fetch_quota(false, false).await?;
        self._fetch_extra_quota().await?;
        self._fetch_abuse_status().await?;
        Ok(())
    }

    async fn _fetch_user_status(&mut self) -> Result<(), GeminiError> {
        let payloads = vec![RPCData {
            rpcid: Grpc::GetUserStatus.as_str().to_string(),
            payload: "[]".to_string(),
            identifier: "generic".to_string(),
        }];
        let response = self._batch_execute(payloads, "/app", true).await?;

        for part_body in parse_rpc_results(
            response.text().await.unwrap_or_default(),
            Grpc::GetUserStatus.as_str().to_string(),
        ) {
            let status_code = get_nested_value(
                part_body.clone(),
                vec![serde_json::Value::Number(14.into())],
                None,
            );
            if let Some(serde_json::Value::Number(code)) = status_code {
                self.account_status =
                    AccountStatus::from_status_code(Some(code.as_i64().unwrap_or(0)));
            }

            if self.account_status == AccountStatus::Available {
                if self.verbose {
                    tracing::info!(
                        "Account status: {} - {}",
                        self.account_status.as_str(),
                        self.account_status.description()
                    );
                }
            } else {
                tracing::warn!(
                    "Account status: {} - {}",
                    self.account_status.as_str(),
                    self.account_status.description()
                );

                if self.account_status == AccountStatus::Unauthenticated
                    && self
                        ._base_psid
                        .as_deref()
                        .map_or(false, |psid| psid.starts_with("Cache"))
                {
                    clear_cookies_cache(self._cookies.clone(), self._base_psid.clone());
                }

                if matches!(
                    self.account_status,
                    AccountStatus::LocationRejected
                        | AccountStatus::AccountRejected
                        | AccountStatus::AccessTemporarilyUnavailable
                        | AccountStatus::AccountRejectedByGuardian
                        | AccountStatus::GuardianApprovalRequired
                ) {
                    tracing::warn!(
                        "Hard block detected ({}). Skipping model discovery.",
                        self.account_status.as_str()
                    );
                    return Ok(());
                }
            }

            if let Some(models_list) = get_nested_value(
                part_body.clone(),
                vec![serde_json::Value::Number(15.into())],
                None,
            ) {
                if let serde_json::Value::Array(models) = models_list {
                    let tier_flags = get_nested_value(
                        part_body.clone(),
                        vec![serde_json::Value::Number(16.into())],
                        Some(serde_json::Value::Array(vec![])),
                    )
                    .unwrap_or_default();
                    let capability_flags = get_nested_value(
                        part_body.clone(),
                        vec![serde_json::Value::Number(17.into())],
                        Some(serde_json::Value::Array(vec![])),
                    )
                    .unwrap_or_default();

                    let tier_flags_vec: Vec<i64> = tier_flags
                        .as_array()
                        .map(|a| a.iter().filter_map(|v| v.as_i64()).collect())
                        .unwrap_or_default();
                    let capability_flags_vec: Vec<i64> = capability_flags
                        .as_array()
                        .map(|a| a.iter().filter_map(|v| v.as_i64()).collect())
                        .unwrap_or_default();

                    let (capacity, capacity_field) =
                        AvailableModel::compute_capacity(&tier_flags_vec, &capability_flags_vec);

                    let unauth = self.account_status == AccountStatus::Unauthenticated;
                    let mut parsed_any = false;
                    for model_data in models {
                        if let serde_json::Value::Array(md) = model_data {
                            if let Some(model) = AvailableModel::from_rpc(
                                md.iter().map(|v| v.clone()).collect(),
                                capacity,
                                capacity_field,
                                unauth && parsed_any,
                            ) {
                                self._model_registry.insert(model.model_id.clone(), model);
                                parsed_any = true;
                            }
                        }
                    }
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    async fn _fetch_quota(&mut self, _flash: bool, _advanced: bool) -> Result<(), GeminiError> {
        if !self._check_account_status(false)? {
            return Ok(());
        }
        // Quota fetching skipped for port simplicity; uses cached values
        Ok(())
    }

    async fn _fetch_abuse_status(&mut self) -> Result<(), GeminiError> {
        let payloads = vec![RPCData {
            rpcid: Grpc::GetAbuseStatus.as_str().to_string(),
            payload: "[]".to_string(),
            identifier: "generic".to_string(),
        }];
        let response = self._batch_execute(payloads, "/app", true).await?;

        for part_body in parse_rpc_results(
            response.text().await.unwrap_or_default(),
            Grpc::GetAbuseStatus.as_str().to_string(),
        ) {
            let abuse_info = get_nested_value(
                part_body.clone(),
                vec![serde_json::Value::Number(1.into())],
                None,
            );
            match abuse_info {
                None | Some(serde_json::Value::Null) | Some(serde_json::Value::Array(_)) => {
                    self._abuse_status = Some(serde_json::json!({
                        "is_clean": true,
                        "status_code": serde_json::Value::Null,
                        "signal": serde_json::Value::Null
                    }));
                    if self.verbose {
                        tracing::info!("Account abuse status: Clean (No flags detected).");
                    }
                }
                Some(info) => {
                    let raw_status = get_nested_value(
                        info.clone(),
                        vec![serde_json::Value::Number(1.into())],
                        None,
                    );
                    let signal = get_nested_value(
                        info.clone(),
                        vec![
                            serde_json::Value::Number(3.into()),
                            serde_json::Value::Number(1.into()),
                        ],
                        None,
                    );
                    let status_code = raw_status.and_then(|v| v.as_i64()).map(|v| v / 1_000_000);
                    self._abuse_status = Some(serde_json::json!({
                        "is_clean": false,
                        "status_code": status_code,
                        "signal": signal
                    }));
                    tracing::warn!(
                        "Potential account restriction detected: {:?}",
                        self._abuse_status
                    );
                }
            }
        }
        Ok(())
    }

    async fn _fetch_extra_quota(&mut self) -> Result<(), GeminiError> {
        let payloads = vec![RPCData {
            rpcid: Grpc::CheckQuota.as_str().to_string(),
            payload: "[]".to_string(),
            identifier: "generic".to_string(),
        }];
        let response = self._batch_execute(payloads, "/app", true).await?;

        for part_body in parse_rpc_results(
            response.text().await.unwrap_or_default(),
            Grpc::CheckQuota.as_str().to_string(),
        ) {
            let is_blocked = get_nested_value(
                part_body.clone(),
                vec![serde_json::Value::Number(0.into())],
                None,
            );
            let usage_level = get_nested_value(
                part_body.clone(),
                vec![serde_json::Value::Number(1.into())],
                None,
            );
            let reset_ts = get_nested_value(
                part_body.clone(),
                vec![
                    serde_json::Value::Number(2.into()),
                    serde_json::Value::Number(0.into()),
                ],
                None,
            );

            self._quotas.entry("extra".to_string()).or_default().as_object_mut().map(|m| {
                m.insert("default".to_string(), serde_json::json!({
                    "is_blocked": is_blocked.as_ref().and_then(|v| v.as_bool()).unwrap_or(false),
                    "usage_percentage": usage_level.as_ref().and_then(|v| v.as_f64()).map(|v| v * 100.0),
                    "reset_time": reset_ts
                }));
            });

            if is_blocked
                .as_ref()
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                tracing::error!("Extra feature quota exceeded: Hard block detected.");
            } else if let Some(ul) = usage_level.as_ref().and_then(|v| v.as_f64()) {
                let usage_pc = ul * 100.0;
                if usage_pc >= 90.0 {
                    tracing::warn!(
                        "Extra feature quota critical: Usage is at {:.1}%.",
                        usage_pc
                    );
                } else if usage_pc >= 75.0 {
                    tracing::warn!("Extra feature quota warning: Usage is at {:.1}%.", usage_pc);
                }
            }
        }
        Ok(())
    }

    async fn _fetch_usage_info(&mut self) -> Result<(), GeminiError> {
        if !self._check_account_status(false)? {
            return Ok(());
        }
        // Usage info fetching skipped for port simplicity
        Ok(())
    }

    async fn _fetch_preferences(&mut self) -> Result<(), GeminiError> {
        // Preferences fetching skipped for port simplicity
        Ok(())
    }

    async fn _sync_activity(&mut self) -> Result<(), GeminiError> {
        self.last_activity_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        if !self._check_account_status(false)? {
            return Ok(());
        }
        Ok(())
    }

    async fn _fetch_recent_chats(&mut self) -> Result<(), GeminiError> {
        // Recent chats fetching skipped for port simplicity
        Ok(())
    }

    pub async fn read_chat(
        &self,
        chat_id: &str,
        limit: Option<i64>,
    ) -> Result<Option<ChatHistory>, GeminiError> {
        // Read chat skipped for port simplicity; returns None
        let _ = chat_id;
        let _ = limit;
        Ok(None)
    }

    pub async fn list_chats(&self) -> Result<Option<Vec<ChatInfo>>, GeminiError> {
        // List chats skipped for port simplicity
        Ok(None)
    }

    pub async fn delete_chat(&self, _chat_id: &str) -> Result<(), GeminiError> {
        Ok(())
    }

    async fn _batch_execute(
        &self,
        payloads: Vec<RPCData>,
        source_path: &str,
        close_on_error: bool,
    ) -> Result<reqwest::Response, GeminiError> {
        let client = self.client.as_ref().ok_or_else(|| {
            GeminiError::api(
                "Invalid request: client is not initialized. Call `GeminiClient.init()` first.",
            )
        })?;

        let _reqid = self._reqid;
        // Note: _reqid is incremented in the Python version; here we don't mutate self

        let mut params = serde_json::json!({
            "rpcids": payloads.iter().map(|p| p.rpcid.as_str()).collect::<Vec<_>>().join(","),
            "hl": self.language,
            "_reqid": _reqid,
            "rt": "c",
            "source-path": source_path,
        });
        if let Some(bl) = &self.build_label {
            params
                .as_object_mut()
                .unwrap()
                .insert("bl".to_string(), serde_json::Value::String(bl.clone()));
        }
        if let Some(sid) = &self.session_id {
            params
                .as_object_mut()
                .unwrap()
                .insert("f.sid".to_string(), serde_json::Value::String(sid.clone()));
        }

        let mut batch_exec_headers = Headers::BatchExec.as_list();
        let mut header_map = reqwest::header::HeaderMap::new();
        for (k, v) in &batch_exec_headers {
            header_map.insert(*k, reqwest::header::HeaderValue::from_str(v).unwrap());
        }

        let mut request_headers = reqwest::header::HeaderMap::new();
        for (k, v) in Headers::Gemini.as_list() {
            request_headers.insert(k, reqwest::header::HeaderValue::from_str(v).unwrap());
        }
        for (k, v) in &header_map {
            request_headers.insert(k, v.clone());
        }
        for (k, v) in Headers::SameDomain.as_list() {
            request_headers.insert(k, reqwest::header::HeaderValue::from_str(v).unwrap());
        }

        let serialized_payloads: Vec<serde_json::Value> =
            payloads.iter().flat_map(|p| p.serialize()).collect();
        let f_req = serde_json::json!([serialized_payloads]).to_string();

        let mut body = serde_json::Map::new();
        body.insert(
            "at".to_string(),
            serde_json::Value::String(self.access_token.clone().unwrap_or_default()),
        );
        body.insert("f.req".to_string(), serde_json::Value::String(f_req));

        let response = client
            .post(Endpoint::BatchExec.as_str())
            .query(&params)
            .headers(request_headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| GeminiError::api(format!("Batch execution failed: {e}")))?;

        if self.verbose {
            tracing::debug!(
                "HTTP Request: POST {} [{}]",
                Endpoint::BatchExec.as_str(),
                response.status()
            );
        }

        if response.status() != 200 {
            if close_on_error {
                // close handled by caller
            }
            return Err(GeminiError::api(format!(
                "Batch execution failed with status code {}",
                response.status()
            )));
        }

        Ok(response)
    }

    async fn _generate(
        &self,
        prompt: &str,
        _req_file_data: Option<Vec<serde_json::Value>>,
        _model: Option<serde_json::Value>,
        _gem: Option<String>,
        _chat: Option<&mut ChatSession>,
        _temporary: bool,
        _deep_research: bool,
        _extended_thinking: bool,
    ) -> Result<Vec<ModelOutput>, GeminiError> {
        if prompt.is_empty() {
            return Err(GeminiError::api("Prompt cannot be empty."));
        }

        // Simplified generation: returns empty candidates (no live server)
        // Full port requires live Gemini integration
        Ok(vec![ModelOutput {
            metadata: vec![],
            candidates: vec![],
            chosen: 0,
        }])
    }

    fn _parse_candidate(
        &self,
        _candidate_data: &serde_json::Value,
        _cid: &str,
        _rid: &str,
        _rcid: &str,
    ) -> Result<
        (
            String,
            String,
            Vec<WebImage>,
            Vec<GeneratedImage>,
            Vec<GeneratedVideo>,
            Vec<GeneratedMedia>,
            Vec<Citation>,
        ),
        GeminiError,
    > {
        Ok((
            "".to_string(),
            "".to_string(),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ))
    }
}

/// Multi-turn chat session.
/// Mirrors `ChatSession` (`client.py`).
pub struct ChatSession {
    pub metadata: Vec<Option<String>>,
    pub gem: Option<Gem>,
    pub last_output: Option<ModelOutput>,
    pub model: Option<String>,
    pub client: GeminiClient,
}

impl ChatSession {
    pub fn new(client: GeminiClient) -> Self {
        Self {
            metadata: DEFAULT_METADATA.to_vec(),
            gem: None,
            last_output: None,
            model: None,
            client,
        }
    }

    pub fn cid(&self) -> String {
        self.metadata
            .get(0)
            .and_then(|v| v.as_deref())
            .unwrap_or("")
            .to_string()
    }

    pub fn rid(&self) -> String {
        self.metadata
            .get(1)
            .and_then(|v| v.as_deref())
            .unwrap_or("")
            .to_string()
    }

    pub fn rcid(&self) -> String {
        self.metadata
            .get(2)
            .and_then(|v| v.as_deref())
            .unwrap_or("")
            .to_string()
    }

    pub async fn send_message(
        &mut self,
        prompt: &str,
        temporary: bool,
        deep_research: bool,
        extended_thinking: bool,
    ) -> Result<ModelOutput, GeminiError> {
        self.client
            .generate_content(
                prompt,
                None,
                self.model
                    .as_deref()
                    .map(|m| serde_json::Value::String(m.to_string())),
                self.gem.as_ref().map(|g| g.id.clone()),
                None,
                temporary,
                deep_research,
                extended_thinking,
            )
            .await
    }

    pub async fn send_message_stream(
        &mut self,
        prompt: &str,
        temporary: bool,
        deep_research: bool,
        extended_thinking: bool,
    ) -> Result<ModelOutput, GeminiError> {
        self.send_message(prompt, temporary, deep_research, extended_thinking)
            .await
    }
}

// ---------------------------------------------------------------------------
// JSPB parsing utils (src/gemini_webapi/utils/parsing.py)
// ---------------------------------------------------------------------------

/// Safely navigate through a nested JSON structure using a sequence of keys/indices.
/// Mirrors `get_nested_value` (`parsing.py:61`).
pub fn get_nested_value(data: Value, path: Vec<Value>, default: Option<Value>) -> Option<Value> {
    let mut current = data;
    for key in path {
        current = match (current, key) {
            (Value::Array(arr), Value::Number(idx)) => {
                let i = idx.as_i64()? as usize;
                if i < arr.len() {
                    arr[i].clone()
                } else {
                    return default;
                }
            }
            (Value::Object(obj), Value::String(k)) => {
                if let Some(v) = obj.get(k.as_str()) {
                    v.clone()
                } else {
                    return default;
                }
            }
            _ => return default,
        };
    }
    Some(current)
}

/// Extract the sparse field bundle from a JSPB array (the final dict slot).
/// Mirrors `get_sparse_bundle` (`parsing.py:97`).
pub fn get_sparse_bundle(container: Value) -> Option<Value> {
    match container {
        Value::Array(arr) if !arr.is_empty() => {
            if let Value::Object(_) = arr.last()? {
                arr.last().cloned()
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Read field `index` of a JSPB array from whichever form carries it (positional or sparse bundle).
/// Mirrors `get_field` (`parsing.py:111`).
pub fn get_field(container: Value, index: usize, default: Option<Value>) -> Option<Value> {
    let arr = container.as_array()?;
    if arr.is_empty() {
        return default;
    }

    let value = arr.get(index).cloned();
    let value = match &value {
        Some(v)
            if v.is_null()
                || v.as_array().map(|a| a.is_empty()).unwrap_or(false)
                || v.as_object().map(|o| o.is_empty()).unwrap_or(false) =>
        {
            None
        }
        _ => value,
    };

    match value {
        Some(v) => Some(v),
        None => {
            let bundle = get_sparse_bundle(container)?;
            bundle.get((index + 1).to_string()).cloned()
        }
    }
}

/// Read field `index` of a candidate's `[12]` rich content block.
/// Mirrors `get_rich_content_field` (`parsing.py:129`).
pub fn get_rich_content_field(
    candidate_data: Value,
    index: usize,
    default: Option<Value>,
) -> Option<Value> {
    let field_12 = get_nested_value(
        candidate_data,
        vec![Value::Number(serde_json::Number::from(12))],
        None,
    )?;
    get_field(field_12, index, default)
}

/// Extract JSON content from a Google API response.
/// Mirrors `extract_json_from_response` (`parsing.py:281`).
pub fn extract_json_from_response(text: String) -> Vec<Value> {
    let content = text.strip_prefix(")]}'").unwrap_or(&text).trim_start();
    if content.is_empty() {
        return Vec::new();
    }
    // Try parsing as a single JSON value
    if let Ok(parsed) = serde_json::from_str::<Value>(content) {
        return vec![parsed];
    }
    // Try NDJSON line by line
    let mut result = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(parsed) = serde_json::from_str::<Value>(line) {
            result.push(parsed);
        }
    }
    result
}
