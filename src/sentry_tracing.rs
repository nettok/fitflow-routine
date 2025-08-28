use sentry::integrations::tracing::EventFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing_with_sentry() {
    let sentry_layer =
        sentry::integrations::tracing::layer().event_filter(|md| match *md.level() {
            // Capture error level events as Sentry events
            // These are grouped into issues, representing high-severity errors to act upon
            tracing::Level::ERROR => EventFilter::Event,
            // Ignore trace level events, as they're too verbose
            tracing::Level::TRACE => EventFilter::Ignore,
            // Capture everything else as a traditional structured log
            _ => EventFilter::Log,
        });

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(sentry_layer)
        .init();
}
