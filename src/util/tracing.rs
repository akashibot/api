use crates_io_env_vars::var;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{prelude::*, EnvFilter, Layer};

/// Initializes the `tracing` logging framework.
///
/// Regular CLI output is influenced by the optional
/// [`RUST_LOG`](EnvFilter) environment variable
/// and is showing all `INFO` level events by default.
///
/// This function also sets up the Sentry error reporting integration for the
/// `tracing` framework, which is hardcoded to include all `INFO` level events.
pub fn init() {
    init_with_default_level(LevelFilter::INFO)
}

fn init_with_default_level(level: LevelFilter) {
    let env_filter = EnvFilter::builder()
        .with_default_directive(level.into())
        .from_env_lossy();

    let log_format = var("RUST_LOG_FORMAT")
        .inspect_err(|error| {
            warn!("Failed to read RUST_LOG_FORMAT, falling back to default: {error}")
        })
        .unwrap_or_default();

    let log_layer = match log_format.as_deref() {
        Some("json") => json_subscriber::fmt::layer()
            .flatten_event(true)
            .with_flat_span_list(true)
            .with_filter(env_filter)
            .boxed(),
        _ => tracing_subscriber::fmt::layer()
            .compact()
            .without_time()
            .with_filter(env_filter)
            .boxed(),
    };

    tracing_subscriber::registry().with(log_layer).init();
}

/// Initializes the `tracing` logging framework for usage in tests.
pub fn init_for_test() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy()
        .add_directive("tokio_postgres=info".parse().unwrap());

    let _ = tracing_subscriber::fmt()
        .compact()
        .with_env_filter(env_filter)
        .with_test_writer()
        .try_init();
}
