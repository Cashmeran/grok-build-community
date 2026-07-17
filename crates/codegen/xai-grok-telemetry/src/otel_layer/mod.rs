//! OpenTelemetry layer — Community Edition: all stubs, no export.

use std::time::Duration;

pub struct OtelLayerConfig {
    pub credentials: (),
    pub token_header_value: String,
    pub alpha_test_key: Option<String>,
    pub exporter: OtelExporterConfig,
}
#[derive(Default)]
pub struct OtelClientInfo {
    pub client_type: Option<String>,
    pub client_version: String,
    pub client_name: String,
    pub service_version: String,
    pub app_entrypoint: String,
}
pub struct OtelExporterConfig {
    pub traces_url: String,
    pub extra_headers: Vec<(String, String)>,
    pub export_interval: Option<Duration>,
    pub timeout: Option<Duration>,
    pub enabled: bool,
}

/// Returns a no-op layer — no spans are exported.
pub fn build_otel_layer<S>(
    _client_info: OtelClientInfo,
    _config: OtelLayerConfig,
) -> Box<dyn tracing_subscriber::layer::Layer<S> + Send + Sync>
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    Box::new(tracing_subscriber::layer::Identity::new())
}

pub struct OtelGuard;
impl Drop for OtelGuard {
    fn drop(&mut self) {}
}
pub fn otel_guard() -> OtelGuard { OtelGuard }
pub fn shutdown_otel() {}
