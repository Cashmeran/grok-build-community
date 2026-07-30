#!/usr/bin/env python3
"""Apply clean telemetry entry-point cuts to current working tree."""
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

def apply():
    # 1. client.rs
    client = ROOT / "crates/codegen/xai-grok-telemetry/src/client.rs"
    c = client.read_text(encoding='utf-8')
    
    # is_enabled -> false
    c = re.sub(
        r'(pub fn is_enabled\(\) -> bool\s*\{).*?(\n\})',
        r'\1\n    false // Community Edition: telemetry disabled\n\2',
        c, flags=re.DOTALL
    )
    # is_session_metrics_enabled -> false
    c = re.sub(
        r'(pub fn is_session_metrics_enabled\(\) -> bool\s*\{).*?(\n\})',
        r'\1\n    false // Community Edition: telemetry disabled\n\2',
        c, flags=re.DOTALL
    )
    # track -> return
    c = re.sub(r'(pub async fn track\([^)]+\)\s*\{)', r'\1\n    return; // Community Edition: telemetry disabled', c)
    # sync_profile -> return
    c = re.sub(r'(pub fn sync_profile\(\)\s*\{)', r'\1\n    return; // Community Edition: telemetry disabled', c)
    # init -> return (first match only)
    c = re.sub(r'(pub fn init\([^)]+\)\s*\{)', r'\1\n    return; // Community Edition: telemetry disabled', c, count=1)
    # init_if_needed -> return
    c = re.sub(r'(pub fn init_if_needed\([^)]+\)\s*\{)', r'\1\n    return; // Community Edition: telemetry disabled', c)
    client.write_text(c, encoding='utf-8')
    print("client.rs: 6 entry points blocked")

    # 2. session_ctx.rs
    sctx = ROOT / "crates/codegen/xai-grok-telemetry/src/session_ctx.rs"
    s = sctx.read_text(encoding='utf-8')
    
    s = re.sub(r'(pub fn log_event<T: TelemetryEvent>\(data: T\)\s*\{)', r'\1\n    let _ = data; return; // Community Edition: telemetry disabled', s)
    s = re.sub(r'(pub fn log_event_dual<T: TelemetryEvent>\([^)]+\)\s*\{)', r'\1\n    return; // Community Edition: telemetry disabled', s)
    s = re.sub(r'(pub fn log_session_event<T: TelemetryEvent>\(data: T\)\s*\{)', r'\1\n    let _ = data; return; // Community Edition: telemetry disabled', s)
    s = re.sub(r'(pub fn log_session_event_with_origin<T: TelemetryEvent>\([^)]+\)\s*\{)', r'\1\n    return; // Community Edition: telemetry disabled', s)
    
    # Replace emit_event functions with local logging versions
    s = re.sub(
        r"pub fn emit_event<T: Serialize \+ Send \+ 'static>\(event_suffix: impl Into<String>, data: T\) \{\s*emit_event_with_origin\(EmitterOrigin::Shell, event_suffix, data\);\s*\}",
        'pub fn emit_event<T: Serialize>(event_suffix: impl Into<String>, data: T) {\n    emit_event_with_origin(EmitterOrigin::Shell, event_suffix, data);\n}',
        s
    )
    
    s = re.sub(
        r"pub fn emit_event_with_origin<T: Serialize \+ Send \+ 'static>\(\s*origin: EmitterOrigin,\s*event_suffix: impl Into<String>,\s*data: T,\s*\) \{.*?(?=\n\s*///|\n\s*#\[cfg|\n\s*pub fn|\Z)",
        '''pub fn emit_event_with_origin<T: Serialize>(
    origin: EmitterOrigin,
    event_suffix: impl Into<String>,
    data: T,
) {
    let event_name = format!("{}{}", origin.event_prefix(), event_suffix.into());
    let ctx_snapshot = TELEMETRY_CTX
        .try_with(|c| {
            (
                c.session_id.clone(),
                c.prompt_index.try_lock().map(|g| *g as usize).ok(),
            )
        })
        .ok();
    write_event(&event_name, origin.event_prefix(), &data, ctx_snapshot);
}

fn write_event<T: Serialize>(
    event_name: &str,
    origin: &str,
    data: &T,
    ctx_snapshot: Option<(String, Option<usize>)>,
) {
    use std::io::Write;

    let mut record = match serde_json::to_value(data) {
        Ok(serde_json::Value::Object(map)) => map,
        Ok(other) => {
            let mut m = serde_json::Map::new();
            m.insert("value".into(), other);
            m
        }
        Err(_) => serde_json::Map::new(),
    };

    if let Some((session_id, turn_number)) = ctx_snapshot {
        record.insert("session_id".into(), json!(session_id));
        if let Some(turn) = turn_number {
            record.insert("turn_number".into(), json!(turn));
        }
    }
    record.insert("event".into(), json!(event_name));
    record.insert("origin".into(), json!(origin));
    record.insert("ts".into(), json!(chrono::Utc::now().to_rfc3339()));

    let log_dir = dirs::home_dir().unwrap_or_default().join(".grok").join("logs");
    let _ = std::fs::create_dir_all(&log_dir);
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_dir.join("events.log"))
    {
        let _ = writeln!(f, "{}", serde_json::Value::Object(record));
    }
}''',
        s, flags=re.DOTALL
    )
    
    sctx.write_text(s, encoding='utf-8')
    print("session_ctx.rs: 4 entry points blocked + emit_event local logging")

    # 3. Cargo.toml - add dirs dep
    ct = ROOT / "crates/codegen/xai-grok-telemetry/Cargo.toml"
    c_content = ct.read_text(encoding='utf-8')
    if 'dirs =' not in c_content:
        c_content = c_content.replace(
            'serde_json = { workspace = true }',
            'serde_json = { workspace = true }\ndirs = { workspace = true }'
        )
        ct.write_text(c_content, encoding='utf-8')
        print("Cargo.toml: added dirs dependency")
    
    print("Done.")

if __name__ == "__main__":
    apply()
