use chrono::Local;
use rocket::yansi::Paint;
use tracing_log::log::Level;
use tracing_subscriber::{EnvFilter, Layer};
use tracing_subscriber::field::MakeExt;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::LookupSpan;

pub use tracing_fairing::TracingFairing;
pub use tracing_fairing::TracingSpan;

use crate::common::result::Result;

mod request_id;
mod tracing_fairing;

struct LogTimeFormat;

impl FormatTime for LogTimeFormat {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S.%3f"))
    }
}

pub fn setup_logger(log_level: Level) -> Result<()> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(logging_layer())
            .with(filter_layer(log_level))
    )?;
    Ok(())
}

// shamelessly copied most of it from
// https://github.com/somehowchris/rocket-tracing-fairing-example/blob/main/src/main.rs
pub fn logging_layer<S>() -> impl Layer<S> where S: tracing::Subscriber, S: for<'span> LookupSpan<'span> {
    let field_format = tracing_subscriber::fmt::format::debug_fn(|writer, field, value| {
        if field.name() == "request_id" {
            write!(writer, "{:?}", Paint::new(value).dimmed())
        }
        else if field.name() == "message" {
            write!(writer, "{:?}", Paint::new(value).bold())
        } else {
            write!(writer, "{}: {:?}", field, Paint::default(value).bold())
        }
    })
        .delimited(", ")
        .display_messages();

    tracing_subscriber::fmt::layer()
        .fmt_fields(field_format)
        // .event_format(PantsuFormatter)
        .with_timer(LogTimeFormat)
        .with_test_writer()
}

pub fn filter_layer(level: Level) -> EnvFilter {
    let filter_str = match level {
        Level::Error => "warn,hyper=off,rustls=off",
        Level::Warn => "warn,rocket::support=info,hyper=off,rustls=off",
        Level::Info => "info,hyper=off,rustls=off",
        Level::Debug => "debug,hyper=off,rustls=off,tokio_util=off",
        Level::Trace => "trace,hyper=off,rustls=off,tokio_util=off",
    };

    EnvFilter::try_new(filter_str).expect("filter string must parse")
}
