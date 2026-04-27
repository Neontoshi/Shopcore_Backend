use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use tracing_appender::non_blocking::WorkerGuard;

pub fn init_logger() -> WorkerGuard {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
    
    tracing_subscriber::registry()
        .with(fmt::Layer::new().with_writer(non_blocking))
        .with(env_filter)
        .init();
    
    guard
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        tracing::error!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        tracing::warn!($($arg)*)
    };
}