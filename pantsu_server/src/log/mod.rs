use chrono::{Local};
use tracing::Level;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::FmtSubscriber;

use crate::common::result::Result;

struct LogTimeFormat;

impl FormatTime for LogTimeFormat {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S.%3f"))
    }
}

pub fn setup_logger(log_level: Level) -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_timer(LogTimeFormat)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
