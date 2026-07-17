//! Telemetry engine for Grok Build Community Edition.
//!
//! All network upload paths have been removed. Events and metrics are
//! written to `~/.grok/logs/` locally. No data leaves the machine.

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
    pub use crate::compat::{init, is_enabled, is_session_metrics_enabled};
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
