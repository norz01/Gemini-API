# Glossary — Gemini-API domain terms

Each term: meaning in the original, and its name in the Rust port (type / module / function).

| Term | Meaning (original) | Name in Rust |
|---|---|---|
| `__Secure-1PSID` / psid | Primary Google session cookie value, required for auth | `Secure1psid` (String), config field |
| `__Secure-1PSIDTS` / psidts | Token-binding cookie, optional for some accounts | `Secure1psidts` (Option<String>) |
| `GeminiClient` | Async HTTP client to gemini.google.com, holds cookies + state | `GeminiClient` struct, `client/` unit crate |
| `init()` | First call: discovers models, headers, tier capacity per account | `GeminiClient::init()` |
| `ModelOutput` | Parsed response: text, images, thoughts, candidates, metadata | `ModelOutput` struct, `types/model_output.rs` |
| `ChatSession` / `start_chat` | Multi-turn conversation, history managed automatically | `ChatSession`, `chat/` unit |
| `ChatHistory` / `ChatTurn` / `ChatInfo` | Conversation history (newest→oldest) and chat list entries | `ChatHistory`, `ChatTurn`, `ChatInfo` |
| `AvailableModel` | Model discovered per-account at init (name, display name, id, available) | `AvailableModel`, `types/available_model.rs` |
| `Gem` | Gemini Gems system prompt; predefined or user-created | `Gem`, `types/gem.rs` |
| `DeepResearchPlan` / `DeepResearchDocument` | Deep research: plan (title, ETA, steps) and finished report | `DeepResearchPlan`, `DeepResearchDocument`, `research/` unit |
| JSPB | Google's JSON protobuf wire format returned by endpoints | `jspb/` module, `get_field` by number |
| `Endpoint` | Target URLs: StreamGenerate, RotateCookies, upload, batchexecute | `Endpoint` enum, `constants.rs` |
| `GRPC` ids | Google RPC method ids (e.g. `MaZiqc` list conversations) | `GRPC` enum, `constants.rs` |
| `AccountStatus` / `ErrorCode` | Numeric account status / server error codes | `AccountStatus`, `ErrorCode` enums |
| `cookies.json` | CLI cookie file: flat map, `{"cookies": {...}}`, or list of `{name,value}` | `load_cookies()` in `cookie/` unit, same format |
| `GeneratedImage` / `WebImage` | Web-fetched vs AI-generated images in output | `GeneratedImage`, `WebImage` |
| `text_delta` | Incremental text chunk from streaming | `text_delta` field on streamed output |
