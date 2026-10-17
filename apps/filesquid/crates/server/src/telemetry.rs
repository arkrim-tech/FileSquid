//! Stdout JSON tracing plus the activity-journal layer. MUST NOT write log files.

use filesquid_api::trace_layer::JournalLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

/// Install the subscriber: JSON to stdout (filtered by `RUST_LOG`, default `info`) and the
/// journal layer (unfiltered, so warnings and errors are always journaled once attached).
pub fn init(journal: JournalLayer) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let stdout = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(std::io::stdout)
        .with_filter(filter);
    tracing_subscriber::registry()
        .with(stdout)
        .with(journal)
        .init();
}
