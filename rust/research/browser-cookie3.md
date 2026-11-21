# Research: browser-cookie3 replacement

**Context.** `browser-cookie3` is an **optional** dependency (the `browser` extra), used by 1 unit (`client.py` docstring + `GeminiClient` fallback path): when no `__Secure-1PSID` is given, the client can import cookies from the local browser (Firefox supported). The CLI itself does not use browser-cookie3; it requires `--cookies-json` (or env `GEMINI_SECURE_1PSID`). pyproject marks it optional, so it is not part of the core dependency set.

Research finding: there is no mature, maintained Rust crate that reads Firefox/Chromium browser cookie stores (SQLite + keyring/OS-encrypted cookies) cross-platform. Primary source check: no equivalent of browser-cookie3 in the crates.io ecosystem surfaced; crates that exist (e.g. cookie-store parsers) do not decrypt browser-bound cookies.

**Recommendation:** treat browser-cookie3 as out of scope for the Rust port — `keep` the unit's behavior outside Rust OR drop the feature, since the CLI's supported auth path is `--cookies-json` / `GEMINI_SECURE_1PSID`, both trivially portable. Document in DECISIONS.md as ADR; if the user needs browser import, the original stays as reference for that feature.

**Decision needed (D4):** confirm browser-cookie3 is out of scope (CLI auth via cookies JSON + env only).
