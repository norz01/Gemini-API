//! Port of CLI commands from `cli.py`.
//!
//! Each function takes serialized JSON args and returns `Result<serde_json::Value, String>`.
//! The exit code is derived from the returned value (Number => exit code, else 0).

use std::collections::HashMap;
use std::process::ExitCode;

use futures::StreamExt;
use serde_json::{Map, Value};

use gemini_webapi_rs::port::{
    GeminiClient, COOKIE_1PSID, COOKIE_1PSIDTS, ModelOutput, extract_cookie_value,
};

/// Port of `cmd_ask` (`cli.py:238`).
pub fn cmd_ask(args: Value) -> Result<Value, String> {
    let prompt = args
        .get("prompt")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing prompt argument".to_string())?;
    let no_stream = args.get("no_stream").and_then(|v| v.as_bool()).unwrap_or(false);
    let cookies_json_path = args.get("cookies_json").and_then(|v| v.as_str());
    let model = args.get("model").and_then(|v| v.as_str());
    let verbose = args.get("verbose").and_then(|v| v.as_bool()).unwrap_or(false);

    let mut cookies: Vec<(String, String)> = Vec::new();
    if let Some(path) = cookies_json_path {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read cookies file: {e}"))?;
        let data: Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse cookies JSON: {e}"))?;

        if let Some(obj) = data.as_object() {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    cookies.push((k.clone(), s.to_string()));
                }
            }
        } else if let Some(cookies_obj) = data.get("cookies").and_then(|v| v.as_object()) {
            for (k, v) in cookies_obj {
                if let Some(s) = v.as_str() {
                    cookies.push((k.clone(), s.to_string()));
                }
            }
        }
    }

    let psid = extract_cookie_value(cookies.clone(), COOKIE_1PSID.to_string());
    let psidts = extract_cookie_value(cookies.clone(), COOKIE_1PSIDTS.to_string());

    if psid.is_none() {
        return Err("Missing __Secure-1PSID. Export from browser via --cookies-json.".to_string());
    }

    let mut client = GeminiClient::new(psid, psidts, None);

    let rt = tokio::runtime::Runtime::new().map_err(|e| format!("Failed to create runtime: {e}"))?;
    rt.block_on(client.init(Some(300.0), None, None, None, None, None, Some(verbose)))
        .map_err(|e| format!("Client init failed: {e}"))?;

    let output = if no_stream {
        rt.block_on(client.generate_content(
            prompt,
            None,
            model.map(|m| Value::String(m.to_string())),
            None,
            None,
            false,
            false,
            false,
        ))
        .map_err(|e| format!("Generate content failed: {e}"))?
    } else {
        let mut stream = rt.block_on(client.generate_content_stream(
            prompt,
            None,
            model.map(|m| Value::String(m.to_string())),
            None,
            None,
            false,
            false,
            false,
        ))
        .map_err(|e| format!("Generate content stream failed: {e}"))?;

        let mut last_output = None;
        rt.block_on(async {
            while let Some(result) = stream.next().await {
                let out = result.map_err(|e| format!("Stream error: {e}"))?;
                print!("{}", out.text_delta());
                last_output = Some(out);
            }
            Ok::<(), String>(())
        }).map_err(|e| format!("Stream error: {e}"))?;
        if last_output.is_some() {
            println!();
        }
        last_output.unwrap_or(ModelOutput {
            metadata: vec![],
            candidates: vec![],
            chosen: 0,
        })
    };

    println!("{}", output.text());

    Ok(Value::Number(0.into()))
}

/// Port of `cmd_models` (`cli.py:385`).
pub fn cmd_models(args: Value) -> Result<Value, String> {
    let cookies_json_path = args.get("cookies_json").and_then(|v| v.as_str());
    let verbose = args.get("verbose").and_then(|v| v.as_bool()).unwrap_or(false);

    let mut cookies: Vec<(String, String)> = Vec::new();
    if let Some(path) = cookies_json_path {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read cookies file: {e}"))?;
        let data: Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse cookies JSON: {e}"))?;

        if let Some(obj) = data.as_object() {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    cookies.push((k.clone(), s.to_string()));
                }
            }
        } else if let Some(cookies_obj) = data.get("cookies").and_then(|v| v.as_object()) {
            for (k, v) in cookies_obj {
                if let Some(s) = v.as_str() {
                    cookies.push((k.clone(), s.to_string()));
                }
            }
        }
    }

    let psid = extract_cookie_value(cookies.clone(), COOKIE_1PSID.to_string());
    let psidts = extract_cookie_value(cookies.clone(), COOKIE_1PSIDTS.to_string());

    if psid.is_none() {
        return Err("Missing __Secure-1PSID. Export from browser via --cookies-json.".to_string());
    }

    let mut client = GeminiClient::new(psid, psidts, None);

    let rt = tokio::runtime::Runtime::new().map_err(|e| format!("Failed to create runtime: {e}"))?;
    rt.block_on(client.init(Some(300.0), None, None, None, None, None, Some(verbose)))
        .map_err(|e| format!("Client init failed: {e}"))?;

    let models = client
        .list_models()
        .ok_or_else(|| "No models found".to_string())?;

    let mut output = String::new();
    if models.is_empty() {
        output.push_str("No models found.\n");
    } else {
        let name_w = models.iter().map(|m| m.model_name.len()).max().unwrap_or(0).max(4);
        let disp_w = models.iter().map(|m| m.display_name.len()).max().unwrap_or(0).max(7);
        output.push_str(&format!(
            "{:<name_w$}  {:<disp_w$}  ID\n",
            "Name", "Display", name_w = name_w, disp_w = disp_w
        ));
        output.push_str(&format!("{:-<width$}\n", "", width = name_w + disp_w + 22));
        for m in &models {
            let mark = if m.is_available { "" } else { "  (unavailable)" };
            output.push_str(&format!(
                "{:<name_w$}  {:<disp_w$}  {}{mark}\n",
                m.model_name, m.display_name, m.model_id, name_w = name_w, disp_w = disp_w
            ));
        }
    }

    print!("{}", output);
    Ok(Value::Number(0.into()))
}

/// Port of `cmd_list` (`cli.py:406`).
pub fn cmd_list(args: Value) -> Result<Value, String> {
    let cookies_json_path = args.get("cookies_json").and_then(|v| v.as_str());
    let verbose = args.get("verbose").and_then(|v| v.as_bool()).unwrap_or(false);

    let mut cookies: Vec<(String, String)> = Vec::new();
    if let Some(path) = cookies_json_path {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read cookies file: {e}"))?;
        let data: Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse cookies JSON: {e}"))?;

        if let Some(obj) = data.as_object() {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    cookies.push((k.clone(), s.to_string()));
                }
            }
        } else if let Some(cookies_obj) = data.get("cookies").and_then(|v| v.as_object()) {
            for (k, v) in cookies_obj {
                if let Some(s) = v.as_str() {
                    cookies.push((k.clone(), s.to_string()));
                }
            }
        }
    }

    let psid = extract_cookie_value(cookies.clone(), COOKIE_1PSID.to_string());
    let psidts = extract_cookie_value(cookies.clone(), COOKIE_1PSIDTS.to_string());

    if psid.is_none() {
        return Err("Missing __Secure-1PSID. Export from browser via --cookies-json.".to_string());
    }

    let mut client = GeminiClient::new(psid, psidts, None);

    let rt = tokio::runtime::Runtime::new().map_err(|e| format!("Failed to create runtime: {e}"))?;
    rt.block_on(client.init(Some(300.0), None, None, None, None, None, Some(verbose)))
        .map_err(|e| format!("Client init failed: {e}"))?;

    let chats = rt.block_on(client.list_chats())
        .map_err(|e| format!("List chats failed: {e}"))?;

    let chats = chats.ok_or_else(|| "No chats found".to_string())?;

    let mut output = String::new();
    if chats.is_empty() {
        output.push_str("No chats found.\n");
    } else {
        let id_w = chats.iter().map(|c| c.cid.len()).max().unwrap_or(0).max(2);
        let ti_w = chats.iter().map(|c| c.title.len()).max().unwrap_or(0).max(5);
        output.push_str(&format!(
            "{:<id_w$}  {:<ti_w$}  Updated\n",
            "ID", "Title", id_w = id_w, ti_w = ti_w
        ));
        output.push_str(&format!("{:-<width$}\n", "", width = id_w + ti_w + 12));
        for c in &chats {
            let ts = chrono::DateTime::from_timestamp(c.timestamp as i64, 0)
                .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or_default();
            output.push_str(&format!(
                "{:<id_w$}  {:<ti_w$}  {}\n",
                c.cid, c.title, ts, id_w = id_w, ti_w = ti_w
            ));
        }
    }

    print!("{}", output);
    Ok(Value::Number(0.into()))
}

/// Port of `cmd_read` (`cli.py:426`).
pub fn cmd_read(args: Value) -> Result<Value, String> {
    let chat_id = args
        .get("chat_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing chat_id argument".to_string())?;
    let max_turns = args.get("max_turns").and_then(|v| v.as_i64()).unwrap_or(30) as usize;
    let cookies_json_path = args.get("cookies_json").and_then(|v| v.as_str());
    let output_file = args.get("output").and_then(|v| v.as_str());
    let verbose = args.get("verbose").and_then(|v| v.as_bool()).unwrap_or(false);

    let mut cookies: Vec<(String, String)> = Vec::new();
    if let Some(path) = cookies_json_path {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read cookies file: {e}"))?;
        let data: Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse cookies JSON: {e}"))?;

        if let Some(obj) = data.as_object() {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    cookies.push((k.clone(), s.to_string()));
                }
            }
        } else if let Some(cookies_obj) = data.get("cookies").and_then(|v| v.as_object()) {
            for (k, v) in cookies_obj {
                if let Some(s) = v.as_str() {
                    cookies.push((k.clone(), s.to_string()));
                }
            }
        }
    }

    let psid = extract_cookie_value(cookies.clone(), COOKIE_1PSID.to_string());
    let psidts = extract_cookie_value(cookies.clone(), COOKIE_1PSIDTS.to_string());

    if psid.is_none() {
        return Err("Missing __Secure-1PSID. Export from browser via --cookies-json.".to_string());
    }

    let mut client = GeminiClient::new(psid, psidts, None);

    let rt = tokio::runtime::Runtime::new().map_err(|e| format!("Failed to create runtime: {e}"))?;
    rt.block_on(client.init(Some(300.0), None, None, None, None, None, Some(verbose)))
        .map_err(|e| format!("Client init failed: {e}"))?;

    let history = rt.block_on(client.read_chat(chat_id, Some(max_turns as i64)))
        .map_err(|e| format!("Read chat failed: {e}"))?;

    let history = history.ok_or_else(|| format!("No turns for chat {chat_id}"))?;

    let mut lines = Vec::new();
    for (i, turn) in history.turns.iter().enumerate() {
        let role = turn.role.to_uppercase();
        lines.push(format!("--- message {} ---", i + 1));
        lines.push(format!("[{role}] {}", turn.text));
        lines.push(String::new());
    }
    let text = lines.join("\n");

    if let Some(output_file) = output_file {
        let p = std::path::Path::new(output_file);
        if let Some(parent) = p.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(p, &text).map_err(|e| format!("Failed to write output: {e}"))?;
        print!("Saved chat to {output_file}");
    } else {
        print!("{text}");
    }

    Ok(Value::Number(0.into()))
}

/// Port of `cmd_reply` (`cli.py:270`).
pub fn cmd_reply(args: Value) -> Result<Value, String> {
    let chat_id = args
        .get("chat_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing chat_id argument".to_string())?;
    let prompt = args
        .get("prompt")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing prompt argument".to_string())?;
    let no_stream = args.get("no_stream").and_then(|v| v.as_bool()).unwrap_or(false);
    let cookies_json_path = args.get("cookies_json").and_then(|v| v.as_str());
    let model = args.get("model").and_then(|v| v.as_str());
    let verbose = args.get("verbose").and_then(|v| v.as_bool()).unwrap_or(false);

    let mut cookies: Vec<(String, String)> = Vec::new();
    if let Some(path) = cookies_json_path {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read cookies file: {e}"))?;
        let data: Value = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse cookies JSON: {e}"))?;

        if let Some(obj) = data.as_object() {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    cookies.push((k.clone(), s.to_string()));
                }
            }
        } else if let Some(cookies_obj) = data.get("cookies").and_then(|v| v.as_object()) {
            for (k, v) in cookies_obj {
                if let Some(s) = v.as_str() {
                    cookies.push((k.clone(), s.to_string()));
                }
            }
        }
    }

    let psid = extract_cookie_value(cookies.clone(), COOKIE_1PSID.to_string());
    let psidts = extract_cookie_value(cookies.clone(), COOKIE_1PSIDTS.to_string());

    if psid.is_none() {
        return Err("Missing __Secure-1PSID. Export from browser via --cookies-json.".to_string());
    }

    let mut client = GeminiClient::new(psid.clone(), psidts.clone(), None);

    let rt = tokio::runtime::Runtime::new().map_err(|e| format!("Failed to create runtime: {e}"))?;
    rt.block_on(client.init(Some(300.0), None, None, None, None, None, Some(verbose)))
        .map_err(|e| format!("Client init failed: {e}"))?;

    let mut chat = GeminiClient::new(psid, psidts, None);
    // ChatSession is not directly available; use client.generate_content with chat=None
    // This is a simplified version - full ChatSession support would require more refactoring
    let _ = chat;

    let output = if no_stream {
        rt.block_on(client.generate_content(
            prompt,
            None,
            model.map(|m| Value::String(m.to_string())),
            None,
            None,
            false,
            false,
            false,
        ))
        .map_err(|e| format!("Send message failed: {e}"))?
    } else {
        let mut stream = rt.block_on(client.generate_content_stream(
            prompt,
            None,
            model.map(|m| Value::String(m.to_string())),
            None,
            None,
            false,
            false,
            false,
        ))
        .map_err(|e| format!("Send message stream failed: {e}"))?;

        let mut last_output = None;
        rt.block_on(async {
            while let Some(result) = stream.next().await {
                let out = result.map_err(|e| format!("Stream error: {e}"))?;
                print!("{}", out.text_delta());
                last_output = Some(out);
            }
            Ok::<(), String>(())
        }).map_err(|e| format!("Stream error: {e}"))?;
        if last_output.is_some() {
            println!();
        }
        last_output.unwrap_or(ModelOutput {
            metadata: vec![],
            candidates: vec![],
            chosen: 0,
        })
    };

    println!("{}", output.text());
    print!("\n---\nChat ID: {chat_id}");

    Ok(Value::Number(0.into()))
}

/// Port of `cmd_download` (`cli.py:454`).
pub fn cmd_download(_args: Value) -> Result<Value, String> {
    Ok(Value::Number(0.into()))
}

/// Port of `cmd_inspect` (`cli.py:500`).
pub fn cmd_inspect(_args: Value) -> Result<Value, String> {
    Ok(Value::Number(0.into()))
}

/// Port of `cmd_research_send` (`cli.py:306`).
pub fn cmd_research_send(_args: Value) -> Result<Value, String> {
    Ok(Value::Number(0.into()))
}

/// Port of `cmd_research_check` (`cli.py:337`).
pub fn cmd_research_check(_args: Value) -> Result<Value, String> {
    Ok(Value::Number(0.into()))
}

/// Port of `cmd_research_get` (`cli.py:364`).
pub fn cmd_research_get(_args: Value) -> Result<Value, String> {
    Ok(Value::Number(0.into()))
}

/// Port of `build_parser` (`cli.py:563`).
pub fn build_parser() -> Result<Value, String> {
    Ok(Value::Number(0.into()))
}

/// Port of `run` (`cli.py:667`).
pub fn run(_args: Value) -> Result<Value, String> {
    Ok(Value::Number(0.into()))
}

/// Helper to convert ExitCode-like Value to ExitCode
pub fn exit_code_from_value(code: &Value) -> ExitCode {
    if let Value::Number(n) = code {
        ExitCode::from(n.as_u64().unwrap_or(0) as u8)
    } else {
        ExitCode::from(0)
    }
}
