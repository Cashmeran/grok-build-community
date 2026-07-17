//! External OTEL — Community Edition: all stubs, no network.
//! `schema.rs` is kept for type definitions (needed by events.rs).

pub mod schema;

pub struct IdentityAttrs {
    pub deployment_id: Option<String>,
    pub api_key_id: Option<String>,
    pub user_id: Option<String>,
    pub organization_id: Option<String>,
}

impl IdentityAttrs {
    pub fn empty() -> Self {
        Self { deployment_id: None, api_key_id: None, user_id: None, organization_id: None }
    }
}

pub struct ExternalOtelRemotePolicy { pub disabled: Option<bool> }
pub struct ExternalOtelConfig {}
pub struct ExternalOtelFileConfig {}

pub fn is_active() -> bool { false }
pub fn init(_cfg: Option<ExternalOtelConfig>) {}
pub fn emit<T: crate::events::TelemetryEvent>(_data: &T) {}
pub fn set_identity(_attrs: IdentityAttrs) {}
pub fn shutdown() {}
pub fn flush() {}
pub fn apply_remote_policy(_policy: ExternalOtelRemotePolicy) {}
