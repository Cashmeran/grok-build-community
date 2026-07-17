//! Stubs for deleted modules — Community Edition.
//! External code that directly called these APIs now gets no-op implementations.

// ── client stubs ──

/// No-op — upload client is deleted. Matches original 9-arg signature.
pub fn init(
    _cfg: crate::config::TelemetryConfig,
    _mode: crate::config::TelemetryMode,
    _user_id: Option<String>,
    _team_id: Option<String>,
    _deployment_key: Option<String>,
    _origin_client: Option<impl std::any::Any + Send>,
    _version: String,
    _subscription_tier: Option<String>,
    _client: reqwest::Client,
) {
}
/// No-op.
pub fn init_if_needed(
    _cfg: crate::config::TelemetryConfig,
    _mode: crate::config::TelemetryMode,
    _user_id: Option<String>,
    _team_id: Option<String>,
    _deployment_key: Option<String>,
    _origin_client: Option<impl std::any::Any + Send>,
    _version: String,
    _subscription_tier: Option<String>,
    _client: reqwest::Client,
) {
}
/// Always false — no upload, so never "enabled".
pub fn is_enabled() -> bool {
    false
}
/// Always false.
pub fn is_session_metrics_enabled() -> bool {
    false
}

// ── sentry stubs ──

pub struct SentryConfig;
/// No-op — sentry crash reporting is deleted.
pub fn sentry_init(_config: SentryConfig) -> SentryGuard {
    SentryGuard
}
pub struct SentryGuard;
impl Drop for SentryGuard {
    fn drop(&mut self) {}
}
/// No-op.
pub fn flush_on_shutdown() {}

// ── hooks_log stubs ──

/// No-op layer — no hook events are logged.
pub fn hooks_log_layer<S>() -> Box<dyn tracing_subscriber::layer::Layer<S> + Send + Sync>
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    Box::new(tracing_subscriber::layer::Identity::new())
}
