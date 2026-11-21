# HANDOFF — Gemini-API whole-app port (session 1, 2026-11-22)

## Where the port stands

- Original: untouched, kept as reference and as the fallback behind the Rust front door.
- `rust/`: MAP.md, DECISIONS.md (ADR-001…ADR-013), GLOSSARY.md, research/ (curl-cffi, loguru, browser-cookie3), app-cases/ (4 CLI trycmd cases), app/ (front door), gemini-webapi-rs/ (wave-0 crate), cli-rs/ (wave-1 crate).
- Fallback rate: 75.0% (3/4). Replay vs `rust/app/target/debug/cli`: 4/4 passed (report: rust/app-cases/replay-report.md).
- Wave 0 (gemini-webapi-core) essentially done; wave 1 (cli) partially done.

## What is ported (gate passed)

### `rust/gemini-webapi-rs` (~2115 lines of Rust, gate passed)
**Functions** (477/477 recorded cases):
- `build_model_header`, `warn_deprecated_model`, `format_http_version` (from constants.py)

**Enums**: `Endpoint`, `Grpc`, `Field`, `AccountStatus`, `ErrorCode`, `Headers`

**Error type**: `GeminiError` + `GeminiErrorKind` (mirrors Python exception hierarchy)

**Cookie utils**: `extract_cookie_value`, `hash_psid`, `cookie_cache_dir`, `get_cookies_cache_path`, `save_cookies`, `clear_cookies_cache`, `migrate_legacy_cache`

**Retry utils**: `is_transient_network_error`, `calculate_backoff`, `execute_with_retry`

**HTTP client**: `InitSession`, `get_access_token` (reqwest+rustls-based), `extract_payload`, `regex_patterns`

**JSPB parsing**: `get_nested_value`, `get_sparse_bundle`, `get_field`, `get_rich_content_field`, `extract_json_from_response`

**Types**: `AvailableModel`, `ModelOutput`, `Candidate`, `ChatHistory`, `ChatTurn`, `ChatInfo`, `Gem`, `RPCData`, `Citation`, `DeepResearchPlan`, `DeepResearchDocument`, `DeepResearchResult`, `Image`, `WebImage`, `GeneratedImage`, `GeneratedVideo`, `GeneratedMedia`, `Video`

**`GeminiClient`**: `new()`, `init()`, `generate_content()`, `generate_content_stream()`, `list_models()`, `resolve_model()`, `read_chat()`, `list_chats()`, `delete_chat()`, `_batch_execute()`, `_generate()`, `_parse_candidate()`, `_init_rpc()`, `_fetch_user_status()`, `_fetch_quota()`, `_fetch_abuse_status()`, `_fetch_extra_quota()`, `_fetch_usage_info()`, `_fetch_preferences()`, `_sync_activity()`, `_fetch_recent_chats()`

**`ChatSession`**: `new()`, `send_message()`, `send_message_stream()`

### `rust/cli-rs` (library dependency of rust/app, not gated as Python extension)
**Ported**: `cmd_ask`, `cmd_models`, `cmd_list`, `cmd_read`, `cmd_reply`
**Stubs** (return exit code 0): `cmd_download`, `cmd_inspect`, `cmd_research_send`, `cmd_research_check`, `cmd_research_get`, `build_parser`, `run`
All need live Gemini credentials to test (network calls to gemini.google.com).

### Front door `rust/app`
- Owns `main()` (clap, `src/cli.rs`); falls back to `python3 cli.py` via `src/original.rs`
- `ask` command wired via `src/units/ask.rs` → `cli_rs::port::cmd_ask`
- `models`, `list`, `read`, `reply`, `download`, `inspect`, `research_*` wired via their unit modules
- Kill switches: `RUST_ANYTHING_UNIT_ASK=original`, `RUST_ANYTHING_UNIT_MODELS=original`, etc.
- Replay vs `rust/app/target/debug/cli`: 4/4, fallback rate 75.0% (3/4)

## What is NOT yet ported

- Live Gemini testing: no credentials available (ADR-013). `init()`, `generate_content()`, `list_models()`, etc. ported by faithful translation only.
- Browser TLS impersonation unavailable: `reqwest`+rustls, not browser-impersonating (ADR-005). Known gap.
- `cmd_download`, `cmd_inspect`, `cmd_research_*`, `build_parser`, `run` are stubs.
- Lift not yet done — fallback rate must reach 0 first.

## Next actions

1. Provide live Gemini credentials (`SECURE_1PSID` cookie + cookies JSON) so `init()`, `generate_content()`, etc. can be recorded/diffed/gated against real server responses.
2. Implement remaining CLI command stubs (`download`, `inspect`, `research_*`, `build_parser`, `run`) or leave as stubs if out of scope.
3. Once fallback rate reaches 0 and all gates pass, run `rust-anything app lift .` to produce pure Rust binary.
4. Write `rust/cutover.sh` for user-guided cutover.
5. At each wave boundary: update `MAP.md`, rewrite `HANDOFF.md`, wait for user go-ahead.

## Files that matter now

- `rust/MAP.md` — plan + claim log.
- `rust/DECISIONS.md` — ADRs (do not renumber).
- `rust/GLOSSARY.md` — domain terms and Rust names.
- `rust/research/curl-cffi.md` — documents `reqwest` choice (ADR-005 superseded).
- `rust/app-cases/` — recorded cases; `replay-report.md` — last replay report.
- `rust/app/src/units/mod.rs` — command routing.
- `rust/cli-rs/src/port.rs` — CLI command implementations.
- `rust/gemini-webapi-rs/src/port.rs` — core library port.

## Skill to load next

`rust-anything` (this guide): `/home/norz/.config/opencode/skills/rust-anything/SKILL.md`.
