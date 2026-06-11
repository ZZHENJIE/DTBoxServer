use tracing_appender::non_blocking;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub fn setup(config: &crate::config::LoggingConfig) -> tracing_appender::non_blocking::WorkerGuard {
    let log_filter = EnvFilter::new(&config.level);

    let console_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(log_filter.clone());

    let file_appender = tracing_appender::rolling::daily(&config.log_dir, "dtbox.log");
    let (file_writer, _guard) = non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false)
        .with_filter(log_filter);

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();

    _guard
}
