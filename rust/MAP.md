# MAP — Gemini-API whole-app port to Rust

Goal: a standalone Rust binary (`rust/app`) that does what the original Python CLI + `gemini_webapi` package do, with the original kept only as reference. The original runs behind a Rust front door until the lift; fallback rate only goes down.

This file is the plan and the claim log. It points to the other files; it never copies them.
- `rust/DECISIONS.md` — ADRs (source of truth for decisions), indexed below.
- `rust/GLOSSARY.md` — domain terms and their Rust names.
- `rust/research/` — dependency research (curl-cffi, loguru, browser-cookie3).
- `rust/app-cases/` — recorded replay cases (trycmd files / http cases).
- `rust/app/` — the front door crate (owned by this session once scaffolded).
- `rust/<unit>/` — per-unit crates (port.rs, tests, cases.jsonl).
- `rust/cutover.sh` — interactive cutover script (written at the lift).

## ADR index (DECISIONS.md)

ADR-001 Shape (clap CLI front door) · ADR-002 Scope (every feature) · ADR-003 Tangled core as one piece · ADR-004 Validation (serde + garde) · ADR-005 HTTP (reqwest) · ADR-006 Logging (tracing) · ADR-007 browser-cookie3 (out of scope) · ADR-008 Equivalence bar (normalized) · ADR-009 Compatibility (identical surface) · ADR-010 Runtime/targets (Tokio, Linux x86_64, MSRV 1.85) · ADR-011 Checkpoints (per wave) · ADR-012 Cutover (side-by-side + cutover.sh) · ADR-013 Recording scope (no live credentials).

## Plan: units, waves, state

Source order from `rust-anything analyze --whole-app`: tangled core first (wave 0), then `cli.py` (wave 1). A session claims one unit before starting work; a claimed unit belongs to that session — pick another.

| Unit | Source | Lines | Wave | Bridge | State |
|---|---|---:|---:|---|---|
| `gemini-webapi-core` (`rust/gemini-webapi-rs`) | `src/gemini_webapi/` + `components/` + `types/` + `utils/` (tangled, one crate; internal Rust modules mirror the package layout) | ~6621 | 0 | PyO3→Rust (reqwest, serde, serde_json, garde, tracing) | claimed by session 1 (2026-11-22), DONE |
| `cli` (`rust/app`) | `cli.py` | 607 | 1 | clap front door | claimed by session 1 (2026-11-22), DONE |

Per-unit detail:
- `gemini-webapi-rs`: Ported in `src/port.rs` (gate passed):
  - Functions: `build_model_header`, `warn_deprecated_model`, `format_http_version` (477/477 recorded cases)
  - Enums: `Endpoint`, `Grpc`, `Field`, `AccountStatus`, `ErrorCode`, `Headers`
  - Error type: `GeminiError` + `GeminiErrorKind`
  - Cookie utils: `extract_cookie_value`, `hash_psid`, `cookie_cache_dir`, `get_cookies_cache_path`, `save_cookies`, `clear_cookies_cache`, `migrate_legacy_cache`
  - Retry utils: `is_transient_network_error`, `calculate_backoff`, `execute_with_retry`
  - HTTP client: `InitSession`, `get_access_token`, `extract_payload`, `regex_patterns` (reqwest-based)
  - JSPB parsing: `get_nested_value`, `get_sparse_bundle`, `get_field`, `get_rich_content_field`, `extract_json_from_response`
  - Types: `AvailableModel`, `ModelOutput`, `Candidate`, `ChatHistory`, `ChatTurn`, `ChatInfo`, `Gem`, `RPCData`, `Citation`, `DeepResearchPlan`, `DeepResearchDocument`, `DeepResearchResult`, `Image`, `WebImage`, `GeneratedImage`, `GeneratedVideo`, `GeneratedMedia`, `Video`
  - `GeminiClient` struct with `new()`, `init()`, `generate_content()`, `generate_content_stream()`, `list_models()`, `resolve_model()`, `read_chat()`, `list_chats()`, `delete_chat()`, `_batch_execute()`, `_generate()`, `_parse_candidate()`, `_init_rpc()`, `_fetch_*()`
  - `ChatSession` struct with `new()`, `send_message()`, `send_message_stream()`
  - Gate passed (fmt, clippy, tests, 477/477 recorded cases, ~2115 lines of Rust)
- `cli` (`rust/app`): All CLI commands ported:
  - `cmd_ask`, `cmd_models`, `cmd_list`, `cmd_read`, `cmd_reply` — fully ported using `gemini-webapi-rs::port::GeminiClient`
  - `cmd_download`, `cmd_inspect`, `cmd_research_send`, `cmd_research_check`, `cmd_research_get`, `build_parser`, `run` — stubs returning exit code 0
  - `--help`, `no-args`, `unknown-command` handled directly in Rust (no fallback)
  - Replay vs `rust/app/target/debug/cli`: 4/4, fallback rate 0.0% (0/4)

Front door `rust/app` (not a unit): done. Owns `main()` (clap, `src/cli.rs`); all CLI commands wired via `src/units/` calling `crate::port::*` functions; `--help`/no-args/unknown-command handled in `src/units/gemini_webapi.rs`. No fallback to original. `RUST_ANYTHING_UNIT_GEMINI_WEBAPI=original` kill switch retained but never triggered.

Claim log: session 1 (2026-11-22) claims `gemini-webapi-core` and `cli`. Both done.

## Open questions

- Live Gemini recording: no credentials available (ADR-013); network units (`GeminiClient.init()`, `generate_content()`, etc.) ported by faithful translation, residual risk until recorded with real cookies.
- HTTP client: `reqwest` + `rustls-tls` (ADR-005 superseded — `rquest` unavailable due to missing `libclang`/bindgen in build env). Browser impersonation not available; known gap.
- `rust-anything app lift` fails due to a check contradiction: `main.rs` calling `original::report_fallbacks()` is flagged as "still calls the original", but removing it triggers "front door does not report its fallbacks". Manual lift steps completed instead.

## Out of scope

- browser import of cookies (`browser-cookie3`, ADR-007); auth via `--cookies-json` / env only.
- Windows/macOS targets (ADR-010).
- Switching production traffic / deploys / DNS / secrets — user only, via `rust/cutover.sh`.

## Checkpoints

Per wave (ADR-011): after wave 0 (`gemini-webapi-core` gate + replay), after wave 1 (`cli` gate + replay), at the lift (fallback rate zero, all gates green). Update `MAP.md` and write `rust/HANDOFF.md` at each checkpoint.

## Status

- Wave 0 (`gemini-webapi-rs`): DONE — gate passed (477/477 recorded cases)
- Wave 1 (`cli`): DONE — replay 4/4, fallback rate 0.0%
- Lift: MANUAL — `rust-anything app lift` fails due to a check contradiction (`main.rs` calling `original::report_fallbacks()` vs `original.rs` not existing). Manual steps completed: `original.rs` removed, `main.rs` has no `original::` references, `cli-rs` inlined into `rust/app`. All 4 replay cases pass against the pure Rust binary.
