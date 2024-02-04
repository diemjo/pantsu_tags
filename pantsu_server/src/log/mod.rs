use chrono::Local;
use tracing::{Level};
use tracing_subscriber::field::MakeExt;
use tracing_subscriber::filter::{Targets};
use tracing_subscriber::fmt::format::{Writer};
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::{SubscriberExt};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::util::SubscriberInitExt;
use yansi::Paint;

pub(crate) mod request_id;
// mod tracing_fairing;

struct LogTimeFormat;

impl FormatTime for LogTimeFormat {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S.%3f"))
    }
}

pub fn setup_logger(log_level: Level) {
    tracing_subscriber::registry()
        .with(logging_layer())
        .with(filter_layer(log_level))
        .init();
}

// shamelessly copied most of it from
// https://github.com/somehowchris/rocket-tracing-fairing-example/blob/main/src/main.rs
pub fn logging_layer<S>() -> impl Layer<S> where S: tracing::Subscriber, S: for<'span> LookupSpan<'span> {
    let field_format = tracing_subscriber::fmt::format::debug_fn(|writer, field, value| {
        match field.name() {
            "request_id" => write!(writer, "{:?}", Paint::new(value).dim()),
            "message" => write!(writer, "{:?}", Paint::new(value).bold()),
            // name if name.starts_with("log.") => Ok(()),
            _ => write!(writer, "{}: {:?}", field, Paint::new(value).bold()),
        }
    })
        .delimited(", ")
        .display_messages();

    tracing_subscriber::fmt::layer()
        .fmt_fields(field_format)
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        // .event_format(PantsuFormatter)
        .with_timer(LogTimeFormat)
        .with_test_writer()
}

pub fn filter_layer(level: Level) -> Targets {
    let filter = Targets::new()
        .with_target("tower_http::trace::on_request", Level::DEBUG)
        .with_target("tower_http::trace::on_response", Level::DEBUG)
        .with_default(level);
    return filter;
}
