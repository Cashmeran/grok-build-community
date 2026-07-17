#![allow(dead_code)]
//! Telemetry engine for Grok Build Community Edition.
//!
//! All network upload paths have been removed. Events and metrics are
//! written to `~/.grok/logs/` locally. No data leaves the machine.

/// Stub — appender module was removed. Writes debug logs to local files.
pub(crate) mod appender {
    use tracing_appender::non_blocking::NonBlocking;

    pub fn non_blocking_file_writer(
        dir: &std::path::Path,
        name: &str,
    ) -> std::io::Result<NonBlocking> {
        let _ = std::fs::create_dir_all(dir);
        let file = std::fs::File::create(dir.join(name))?;
        let (non_blocking, _guard) = tracing_appender::non_blocking(file);
        Ok(non_blocking)
    }
    /// Overload: takes a single path to a log file.
    pub fn non_blocking_file_writer_from_path(
        path: &std::path::Path,
    ) -> std::io::Result<NonBlocking> {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let file = std::fs::File::create(path)?;
        let (non_blocking, _guard) = tracing_appender::non_blocking(file);
        Ok(non_blocking)
    }
    pub fn flush_file_log_guards() {}
}

pub mod config;
pub mod context;
pub mod debug_log;
pub mod enums;
pub mod events;
pub mod external;
pub mod id;
pub mod instrumentation;
pub mod memory_log;
pub mod memory_telemetry;
pub mod otel_layer;
pub mod prompt_timing;
pub mod sampling_log;
pub mod session_ctx;
pub mod session_metrics;
pub mod unified_log;

mod compat;

// Re-export stubs under the original module paths for external compatibility.
pub mod client {
    pub use crate::compat::{init, init_if_needed, is_enabled, is_session_metrics_enabled};
}
pub mod sentry {
    pub use crate::compat::{
        SentryConfig as Config, SentryGuard, flush_on_shutdown, sentry_init as init,
    };
}
pub mod hooks_log {
    pub use crate::compat::hooks_log_layer as layer;
}

pub use events::TelemetryEvent;
pub use session_ctx::{
    EmitterOrigin, TelemetryCtx, emit_event, emit_event_with_origin, log_event, log_session_event,
    log_session_event_with_origin, with_session_ctx,
};
