#![allow(dead_code)]
//! External OTEL — Community Edition: all stubs, no network.
//! `schema.rs` is kept for type definitions (needed by events.rs).

pub mod schema;
/// Stub — truncation module was removed.
pub(crate) mod truncate {
    pub const MAX_FILE_EXTENSION_LEN: usize = 0;
    pub fn truncate_string(_s: &str, _max_bytes: usize) -> &str { "" }
    pub fn truncate_json_value(_v: &serde_json::Value, _max_bytes: usize) -> serde_json::Value { serde_json::Value::Null }
    pub fn reduce_tool_input(_v: &serde_json::Value) -> crate::external::schema::AttrValue { crate::external::schema::AttrValue::Bool(false) }
}

/// Stub — config module was removed.
pub mod config {
    #[derive(Default)]
    pub struct ExternalClientInfo {
        pub client_type: Option<String>,
        pub client_version: String,
        pub service_version: String,
        pub app_entrypoint: String,
    }
    pub const ENV_MASTER_SWITCH: &str = "GROK_EXTERNAL_OTEL";
}

pub struct IdentityAttrs {
    pub deployment_id: Option<String>,
    pub api_key_id: Option<String>,
    pub user_id: Option<String>,
    pub organization_id: Option<String>,
}

impl Default for IdentityAttrs {
    fn default() -> Self { Self::empty() }
}
impl IdentityAttrs {
    pub fn empty() -> Self {
        Self { deployment_id: None, api_key_id: None, user_id: None, organization_id: None }
    }
    pub fn from_snapshot(_snapshot: &xai_grok_auth::CredentialSnapshot) -> Self {
        Self::empty()
    }
}

pub struct ExternalOtelRemotePolicy {
    pub disabled: Option<bool>,
    pub force_disable: bool,
    pub lock_content_gates: bool,
}
pub struct ExternalOtelConfig {
    pub client: config::ExternalClientInfo,
    pub transport: OtelProtocol,
    pub logs_endpoint: Option<String>,
    pub gates: ExternalOtelGates,
    pub internal_pipeline_consumed_otel_vars: bool,
}
pub enum OtelProtocol {
    HttpProtobuf,
    Grpc,
    None,
}
impl OtelProtocol {
    pub fn as_protocol_str(&self) -> &str {
        match self { OtelProtocol::HttpProtobuf => "http/protobuf", OtelProtocol::Grpc => "grpc", OtelProtocol::None => "none" }
    }
}
pub struct ExternalOtelGates {
    pub log_user_prompts: bool,
    pub log_tool_details: bool,
}
impl ExternalOtelConfig {
    pub fn resolve_with(
        _getenv: impl Fn(&str) -> Option<String>,
        _file: Option<&ExternalOtelFileConfig>,
    ) -> Option<Self> {
        Some(Self {
            client: config::ExternalClientInfo::default(),
            transport: OtelProtocol::None,
            logs_endpoint: None,
            gates: ExternalOtelGates { log_user_prompts: false, log_tool_details: false },
            internal_pipeline_consumed_otel_vars: false,
        })
    }
}

#[derive(Default)]
pub struct ExternalOtelFileConfig {
    pub enabled: Option<bool>,
    pub metrics_exporter: Option<String>,
    pub logs_exporter: Option<String>,
    pub endpoint: Option<String>,
    pub protocol: Option<String>,
    pub log_user_prompts: Option<bool>,
    pub log_tool_details: Option<bool>,
}

pub fn is_active() -> bool { false }
pub fn init(_cfg: Option<ExternalOtelConfig>) {}
pub fn emit<T: crate::events::TelemetryEvent>(_data: &T) {}
pub fn set_identity(_attrs: IdentityAttrs) {}
pub fn shutdown() {}
pub fn flush() {}
pub fn apply_remote_policy(_policy: ExternalOtelRemotePolicy) {}
