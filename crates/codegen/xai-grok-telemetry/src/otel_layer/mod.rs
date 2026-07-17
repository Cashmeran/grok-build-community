//! OpenTelemetry layer — Community Edition: all stubs, no export.
//! Tracing spans remain local-only via the `tracing` crate.

pub struct OtelLayerConfig {}
pub struct OtelClientInfo {}
pub struct OtelExporterConfig {}

/// Returns a no-op layer — no spans are exported.
pub fn build_otel_layer<S>(
    _config: OtelLayerConfig,
    _exporter: OtelExporterConfig,
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
