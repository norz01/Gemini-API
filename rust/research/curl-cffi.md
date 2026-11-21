# Research: curl-cffi replacement

**Context.** `curl-cffi` (~=0.16.3) is used by 5 units (`client.py`, `components/`, `utils/load_browser_cookies.py`, `utils/upload_file.py`, `cli.py` download). It is the app's only HTTP client, and its non-negotiable feature is **browser TLS/HTTP/2 impersonation**: `GeminiClient` defaults to `impersonate="chrome145"` (`constants.py:42`) because Device Bound Session Credentials block non-browser fingerprints against gemini.google.com / accounts.google.com. The Rust replacement must impersonate a browser (TLS ClientHello + HTTP/2 SETTINGS/header order), support async requests, cookies, proxies, multipart file uploads, and response body/headers/status access. It does NOT need to be byte-exact curl; it must be good enough that Gemini accepts the session.

Candidates evaluated against primary sources (crate READMEs, docs.rs, GitHub repos):

## 1. rquest (StarSparse/rquest) — RECOMMENDED

- Source: https://github.com/StarSparse/rquest (README), usage `rquest = "5"`, `rquest-util = "2"`.
- A fork of reqwest with TLS/JA3/JA4/HTTP2 fingerprint impersonation; HTTPS via BoringSSL; impersonates Chrome/Edge/Safari/Firefox/OkHttp presets (e.g. `Emulation::Firefox136`).
- Feature set matches curl-cffi usage: plain/JSON/urlencoded/multipart bodies, cookie store, header order, redirect policy, rotating proxies, WebSocket upgrade.
- Async, Tokio-based, reqwest-shaped API (`Client::builder().emulation(...).build()`, `client.get(url).send().await`).
- crates.io-cited in search results as "rquest-util 2"; version pinning to verify at `cargo add` time.

Semantic traps:
- BoringSSL compile pulls in C build (`cc`), needs a C compiler and CMake toolchain present (this env has `cc` 16.2.1, cmake availability unverified).
- Not byte-exact curl-impersonate; "close — BoringSSL, browser-like presets" per krowdev comparison. Gemini acceptance must be re-verified with real cookies after the port.
- Two forks exist in search results (StarSparse/rquest v5, Ulyssedev/rquest v3); pin exactly one crate+version from crates.io before writing code.

## 2. impit (apify/impit)

- Source: https://github.com/apify/impit. Rust + reqwest + rustls + tokio; impersonates Firefox/Chrome presets.
- Trap: requires git dependencies with **patched** rustls/h2 (`[patch.crates-io]`), so the toolchain is non-standard; not recommended for a standalone app.

## 3. curl-impersonate-cli

- Source: https://crates.io/crates/curl-impersonate-cli. Async subprocess wrapper around `curl_chromeNNN` binaries; **byte-exact** fingerprint.
- Trap: one subprocess per request — no connection pooling; the app makes many requests per CLI invocation (init + generate + cookie rotation). Poor fit; keep as fallback only if nothing else works.

## 4. reqwest-impersonate (0x676e67/reqwest-impersonate)

- crates.io v0.11.91 is **yanked**. Rejected.

## 5. wreq / specter

- BoringSSL-based pure-Rust clients with browser-like presets; viable but less proven against Gemini than rquest.

**Recommendation:** `rquest` (StarSparse fork), pinned to a crates.io version, Tokio runtime. If Gemini rejects the fingerprint in testing, fall back to `curl-impersonate-cli` per request or revisit.

**Decision needed (D2):** approve rquest, or pick another. Record in ADR.
