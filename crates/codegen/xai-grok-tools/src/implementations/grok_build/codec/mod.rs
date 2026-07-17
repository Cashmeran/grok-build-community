//! `codec` — deterministic encode/decode and hash operations.
//! Hash functions are for fingerprint/checksum use, NOT cryptographic security.
//!
//! Adapted from Caelum's codec tool.

use crate::types::tool::{ToolKind, ToolNamespace};

const MAX_INPUT_LEN: usize = 1_000_000;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct CodecInput {
    #[schemars(description = "Operation: base64_encode/decode, base64url_encode/decode, hex_encode/decode, url_encode/decode, sha256, sha512, md5, crc32, blake3")]
    pub operation: String,

    #[schemars(description = "Input string")]
    pub input: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CodecOutput {
    pub result: String,
}
impl xai_tool_runtime::ToolOutput for CodecOutput {}

#[derive(Debug, Default)]
pub struct CodecTool;

impl crate::types::tool_metadata::ToolMetadata for CodecTool {
    fn kind(&self) -> ToolKind { ToolKind::Other }
    fn tool_namespace(&self) -> ToolNamespace { ToolNamespace::GrokBuild }
    fn description_template(&self) -> &str {
        "Encode, decode, or hash a string. Operations: base64_encode/decode, \
         base64url_encode/decode, hex_encode/decode, url_encode/decode, \
         sha256, sha512, md5 (checksum only), crc32, blake3."
    }
}

impl xai_tool_runtime::Tool for CodecTool {
    type Args = CodecInput;
    type Output = CodecOutput;

    fn id(&self) -> xai_tool_protocol::ToolId {
        xai_tool_protocol::ToolId::new("codec").expect("valid")
    }
    fn description(&self, _: &xai_tool_runtime::ListToolsContext) -> xai_tool_types::ToolDescription {
        xai_tool_types::ToolDescription::new("codec", crate::types::tool_metadata::ToolMetadata::description_template(self))
    }
    fn capabilities(&self) -> xai_tool_protocol::ToolCapabilities {
        xai_tool_protocol::ToolCapabilities { is_read_only: true, ..Default::default() }
    }

    async fn run(&self, _: xai_tool_runtime::ToolCallContext, input: CodecInput) -> Result<CodecOutput, xai_tool_runtime::ToolError> {
        let err = |s| xai_tool_runtime::ToolError::execution(xai_tool_protocol::ToolId::new("codec").expect("valid"), s);
        if input.input.len() > MAX_INPUT_LEN {
            return Err(err(format!("input too long (max {} bytes)", MAX_INPUT_LEN)));
        }

        let result = match input.operation.as_str() {
            "base64_encode" => ok("base64_encode", base64::Engine::encode(&base64::engine::general_purpose::STANDARD, input.input.as_bytes())),
            "base64_decode" => decode_base64(&input.input, false)?,
            "base64url_encode" => ok("base64url_encode", base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE, input.input.as_bytes())),
            "base64url_decode" => decode_base64(&input.input, true)?,
            "hex_encode" => ok("hex_encode", hex::encode(input.input.as_bytes())),
            "hex_decode" => decode_hex(&input.input)?,
            "url_encode" => ok("url_encode", url_encode(&input.input)),
            "url_decode" => ok("url_decode", url_decode(&input.input)),
            "sha256" => { { use sha2::Digest; ok("sha256", format!("{:x}", sha2::Sha256::digest(input.input.as_bytes()))) } }
            "sha512" => { { use sha2::Digest; ok("sha512", format!("{:x}", sha2::Sha512::digest(input.input.as_bytes()))) } }
            "md5" => { ok("md5", format!("{:x}", md5::compute(input.input.as_bytes()))) }
            "crc32" => ok("crc32", format!("{:08x}", crc32fast::hash(input.input.as_bytes()))),
            "blake3" => ok("blake3", blake3::hash(input.input.as_bytes()).to_hex().to_string()),
            _ => return Err(err(format!("unknown operation '{}'", input.operation))),
        };

        Ok(CodecOutput { result })
    }
}

fn ok(op: &str, result: String) -> String {
    serde_json::json!({"operation":op,"result":result}).to_string()
}

fn decode_base64(input: &str, url_safe: bool) -> Result<String, xai_tool_runtime::ToolError> {
    let err = |s| xai_tool_runtime::ToolError::execution(xai_tool_protocol::ToolId::new("codec").expect("valid"), s);
    let engine = if url_safe { &base64::engine::general_purpose::URL_SAFE } else { &base64::engine::general_purpose::STANDARD };
    let bytes = base64::Engine::decode(engine, input).map_err(|e| err(format!("Invalid base64: {}", e)))?;
    String::from_utf8(bytes).map_err(|_| err("decoded bytes are not valid UTF-8".to_string()))
}

fn decode_hex(input: &str) -> Result<String, xai_tool_runtime::ToolError> {
    let err = |s| xai_tool_runtime::ToolError::execution(xai_tool_protocol::ToolId::new("codec").expect("valid"), s);
    let bytes = hex::decode(input).map_err(|e| err(format!("Invalid hex: {}", e)))?;
    String::from_utf8(bytes).map_err(|_| err("decoded bytes are not valid UTF-8".to_string()))
}

fn url_encode(input: &str) -> String {
    input.bytes().map(|b| match b {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => format!("{}", b as char),
        _ => format!("%{:02X}", b),
    }).collect()
}

fn url_decode(input: &str) -> String {
    let mut result = String::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(hex) = u8::from_str_radix(&String::from_utf8_lossy(&bytes[i + 1..i + 3]), 16) {
                result.push(hex as char); i += 3; continue;
            }
        } else if bytes[i] == b'+' { result.push(' '); i += 1; continue; }
        result.push(bytes[i] as char); i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::resources::Resources;
    use crate::types::tool_metadata::test_ctx;

    async fn run(input: CodecInput) -> Result<CodecOutput, xai_tool_runtime::ToolError> {
        let r = Resources::new();
        xai_tool_runtime::Tool::run(&CodecTool, test_ctx(r.into_shared()), input).await
    }

    #[tokio::test]
    async fn base64_roundtrip() {
        let enc = run(CodecInput { operation: "base64_encode".into(), input: "hello".into() }).await.unwrap();
        let v: serde_json::Value = serde_json::from_str(&enc.result).unwrap();
        let encoded = v["result"].as_str().unwrap();
        let dec = run(CodecInput { operation: "base64_decode".into(), input: encoded.into() }).await.unwrap();
        let w: serde_json::Value = serde_json::from_str(&dec.result).unwrap();
        assert_eq!(w["result"].as_str().unwrap(), "hello");
    }

    #[tokio::test]
    async fn sha256_hashes() {
        let r = run(CodecInput { operation: "sha256".into(), input: "hello".into() }).await.unwrap();
        let v: serde_json::Value = serde_json::from_str(&r.result).unwrap();
        assert_eq!(v["result"].as_str().unwrap().len(), 64);
    }

    #[tokio::test]
    async fn invalid_op_errors() {
        assert!(run(CodecInput { operation: "nonexistent".into(), input: "x".into() }).await.is_err());
    }

    #[test]
    fn tool_name() { assert_eq!(xai_tool_runtime::Tool::id(&CodecTool).as_str(), "codec"); }
}
