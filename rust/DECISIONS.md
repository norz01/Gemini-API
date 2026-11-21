# ADRs — Gemini-API whole-app port to Rust

Source of truth for the port. ADRs are numbered in order and never renumbered; a changed decision gets a new ADR that supersedes the old one. Each ADR records Context / Decision / Consequences.

## ADR-001: Shape — standalone Rust CLI with clap front door
Status: accepted (2026-11-22)
Context: `analyze --whole-app` reports shape `cli`, single entry point `cli.py:704` (`if __name__ == "__main__"`). The app is a command-line tool for Gemini.
Decision: `rust/app` is a binary crate whose `main()` is a clap parser exposing the same commands and flags as `cli.py` (ask, reply, research send/check/get, list, read, models, download, inspect). Everything not yet ported falls back to the original `python3 cli.py` with the same argv/stdin/env, passing output and exit code through.
Consequences: the front door owns the entry point from the first build; original kept as reference/fallback until the lift; CLI flags, env vars, cookies format, exit codes stay identical (ADR-009).

## ADR-002: Scope — port every feature
Status: accepted (2026-11-22)
Context: The CLI exposes ask, reply, research (send/check/get), list, read, models, download, inspect; the library also covers gems, Model shim, uploads.
Decision: Port every feature; drop none. Deprecated `Model` enum shim kept as a compatibility shim (maps to account-discovered model, logs a warning) because `cli.py`/`client.py` still exercise it.
Consequences: the port is larger, but no behavior is lost to "deprecated" hand-waving; no separate maintenance of dropped features.

## ADR-003: Tangled core — one piece in wave 0
Status: accepted (2026-11-22)
Context: `src/gemini_webapi/`, `components/`, `types/`, `utils/` import each other in cycles (6621 lines, `analyze` marks them tangled: one piece).
Decision: Port the four units as one piece in wave 0, with no glue between its parts. Internal Rust module boundaries follow the original package layout (`client`, `components/*`, `types/*`, `utils/*`) inside a single crate.
Consequences: simpler boundaries during the port; the crate is large (~6621 lines) but compile errors surface together; unit claims are granular inside the crate (modules), but the crate gates as one.

## ADR-004: Validation — serde + garde
Status: accepted (2026-11-22)
Context: `pydantic` (partial fit) used by 2 units; validation errors will differ from pydantic's.
Decision: Use `serde` for (de)serialization and `garde` for validation; record cases for behavior pydantic/garde don't cover.
Consequences: validation error text differs from pydantic; not part of the CLI contract (equivalence bar normalizes output), so acceptable.

## ADR-005: HTTP client — reqwest (rquest unavailable in build env)
Status: accepted (2026-11-22) (supersedes earlier rquest choice)
Context: `curl-cffi` (~=0.16.3, fit unknown) is the only HTTP client; Gemini requires browser TLS/HTTP2 impersonation (`chrome145` default, `constants.py:42`). Research identified `rquest` as the best fit, but `rquest` pulls `boring-sys` which needs `bindgen`/`libclang`, which is not installed and cannot be installed without root. `reqwest-impersonate` is yanked; `impit` needs a patched toolchain; `curl-impersonate-cli` is one subprocess per request (no connection pooling — poor fit for the many requests per CLI invocation).
Decision: Use `reqwest` with `rustls-tls`, Tokio runtime. Browser impersonation is not available in this build; this is a known gap. If Gemini rejects the fingerprint, revisit with `curl-impersonate-cli` per-request or obtain a build environment with `libclang`.
Consequences: build uses only crates.io crates, no system C libraries beyond `cc`; fingerprint is plain rustls (not browser-like) — must be re-verified against live Gemini once credentials are available.

## ADR-006: Logging — tracing + tracing-subscriber
Status: accepted (2026-11-22)
Context: `loguru` (~=0.7.3) used by 2 units: `set_log_level("DEBUG"|"WARNING")` global control, one `logger.debug` line.
Decision: `tracing` + `tracing-subscriber` (EnvFilter + fmt), initialized in `rust/app` `main`. `set_log_level` becomes EnvFilter handling; `logger.debug` becomes `tracing::debug!`.
Consequences: log line text/format changes (not part of the contract); log lines are normalized away in cases where they appear.

## ADR-007: browser-cookie3 — out of scope
Status: accepted (2026-11-22)
Context: `browser-cookie3` is an optional extra, referenced only in `client.py` docs/fallback for importing cookies from the local browser. No mature Rust crate reads encrypted Firefox/Chromium cookie stores (research: rust/research/browser-cookie3.md).
Decision: Out of scope. CLI auth is `--cookies-json` / `GEMINI_SECURE_1PSID` (+ `GEMINI_SECURE_1PSIDTS`) only. The original remains reference for browser import.
Consequences: users who relied on browser import keep using the original; no cookie-store decryption code in Rust.

## ADR-008: Equivalence bar — normalized output
Status: accepted (2026-11-22)
Context: Output contains live-server dynamic values (chat IDs, timestamps, usage numbers, model names); the original's own runs disagree on these (noise).
Decision: Normalize dynamic values to wildcards (`[..]` for changed parts of a line, `...` for line stretches) in the recorded cases; require byte-for-byte match otherwise, including exact exit codes. Log lines count as noise where they vary.
Consequences: replay can never prove byte-exact server output; the contract is "same structure, same static text, dynamic parts wildcarded". A mismatch outside a wildcard is a port bug.

## ADR-009: Compatibility — identical integration surface
Status: accepted (2026-11-22)
Context: Users depend on CLI flags, env vars (`GEMINI_SECURE_1PSID`, `GEMINI_SECURE_1PSIDTS`, `HTTPS_PROXY`/`http_proxy`), the `cookies.json` file format (flat map / `{"cookies": {...}}` / list of `{name,value}`), and exit codes.
Decision: Keep them identical. The clap parser mirrors `build_parser()`; env fallbacks and cookie loading mirror `_build_client`/`_load_cookies_with_meta`; exit codes mirror `main()`/`run()`.
Consequences: no config migration for users; cookie persistence (`_persist_cookies`, sorted map, `updated_at`, indent 2) must be byte-compatible where exercised.

## ADR-010: Runtime & targets — Tokio, Linux x86_64, MSRV 1.85
Status: accepted (2026-11-22)
Decision: Tokio async runtime (`full` features), `rquest` HTTP, `tracing` logging; target `x86_64-unknown-linux-gnu` only; MSRV 1.85.
Consequences: no Windows/macOS targets for now; `rquest`'s BoringSSL build needs a C toolchain (present).

## ADR-011: Checkpoints — per wave
Status: accepted (2026-11-22)
Decision: Stop for user go-ahead after each wave (wave 0 = gemini_webapi package; wave 1 = cli), reporting units done, gate/replay results, fallback rate; update `MAP.md` and write `rust/HANDOFF.md` at each checkpoint.
Consequences: user gates each wave; no silent forward progress past a checkpoint.

## ADR-012: Cutover — side-by-side, then user decides
Status: accepted (2026-11-22)
Decision: Lift only when fallback rate is zero and every unit passes `gate`. Run Rust beside the original with kill switches (`RUST_ANYTHING_UNIT_<NAME>=original`); user decides the switch. `rust/cutover.sh` walks the user through each step with a way back (original kept beside).
Consequences: original stays in the repo as reference until the user retires it; no production traffic switched by the agent.

## ADR-013: Recording scope — no live credentials available
Status: accepted (2026-11-22)
Context: curl-cffi/loguru not installed here; `tests/test_cli.py` skips unless `SECURE_1PSID` is set; no Gemini credentials available to the agent.
Decision: `app record` covers CLI parsing, help, exit codes, and cookie-JSON error paths; live Gemini interactions (ask, models, list, research, download, inspect) are ported by faithful translation plus unit-level cases, replayed over what is recordable.
Consequences: fallback rate and replay coverage measured over recordable cases only; network-dependent units carry residual risk until recorded with real cookies.
