# Research: loguru replacement

**Context.** `loguru` (~=0.7.3) is used by 2 units (`constants.py` via `warn_deprecated_model`, `cli.py` via `set_log_level` + `logger.debug`). `cli.py` calls `set_log_level("DEBUG")`/`set_log_level("WARNING")` to globally control verbosity, and `logger.debug(...)` for one debug line. loguru's global sink removal/reconfiguration on first `set_log_level` call is noted in README ("Calling set_log_level ... globally remove all existing loguru handlers").

Candidates against primary sources (docs.rs/tracing, README comparisons):

## 1. tracing + tracing-subscriber — RECOMMENDED

- Source: https://docs.rs/tracing/latest/tracing (tracing 0.1.44, MSRV 1.65+, Tokio project).
- `tracing` macros (`trace!`/`debug!`/`info!`/`warn!`/`error!`) mirror loguru's level-based calls; `tracing-subscriber` with `EnvFilter` gives `RUST_LOG`-style filtering equivalent to loguru level control.
- `tracing-subscriber::fmt` writes to stdout/stderr; `tracing-appender` for file rotation (loguru rotation not needed by the app).
- Async-aware (spans across `.await`), important because the app is heavily async (Tokio).
- `tracing-log` compatibility layer lets any `log`-based dependency emit into tracing.

Semantic traps:
- loguru `logger.debug("Persisted cookies ({changed})")` interpolates at call site; tracing uses structured fields (`debug!("Persisted cookies", changed = ?changed)`) — message text changes, which is fine: log lines are not part of the contract (equivalence bar normalizes log lines).
- `set_log_level("DEBUG")` global → `EnvFilter` from env or code; one-time global subscriber set in `main`.

## 2. log + env_logger

- Source: docs.rs/log comparison. Facade only; needs a backend. Simpler but no spans, less async-friendly.
- Appropriate only if the user wants a minimal CLI logger; does not match loguru's structured/global-control feel as closely.

**Recommendation:** `tracing` + `tracing-subscriber` (env-filter + fmt), initialized in the front door's `main`. Replace `set_log_level` with EnvFilter handling and `logger.debug` with `tracing::debug!`.

**Decision needed (D3):** approve tracing, or pick log+env_logger. Record in ADR.
