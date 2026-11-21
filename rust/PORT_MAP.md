# Port map: Gemini-API

2 of 5 units have a Rust port; no original changed since it was recorded.

## Ports

| Unit | Crate | In Rust | Kept in the original | Cases | Last diff | Original |
| --- | --- | --- | --- | ---: | ---: | --- |
| `cli.py` | `rust/cli-rs` | `cmd_ask`, `cmd_reply`, `cmd_research_send`, `cmd_research_check`, `cmd_research_get`, `cmd_models`, `cmd_list`, `cmd_read`, `cmd_download`, `cmd_inspect`, `build_parser`, `run` | `main` | 0 | not run | unchanged |
| `src/gemini_webapi/` | `rust/gemini-webapi-rs` | `build_model_header`, `warn_deprecated_model`, `format_http_version` | `GeminiClient`, `ChatSession`, `Field`, `Endpoint`, `GRPC`, `Headers`, `Model`, `AccountStatus`, `ErrorCode`, `AuthError`, `APIError`, `ImageGenerationError`, `GeminiError`, `TimeoutError`, `UsageLimitExceededError`, `ModelInvalidError`, `TemporarilyBlockedError` | 477 | 477/477 | unchanged |

## Whole app

| Glue | Runs |
| --- | --- |
| `rust/app/src/units/ask.rs` | Rust, handing some cases to the original |
| `rust/app/src/units/gemini_webapi.rs` | Rust, handing some cases to the original |

## Not ported yet

In the order `analyze` suggests.

| Unit | Lines | Exports | Readiness |
| --- | ---: | ---: | ---: |
| `src/gemini_webapi/utils/` | 1828 | 29 | 0 |
| `src/gemini_webapi/components/` | 845 | 3 | 52 |
| `src/gemini_webapi/types/` | 1274 | 19 | 19 |

Regenerate with `rust-anything map --write`. Commit this file: when the original changes upstream, the next map marks every port whose recording it outdated.
