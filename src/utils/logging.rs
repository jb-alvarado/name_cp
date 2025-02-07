use std::io::{self, ErrorKind, Write};

use flexi_logger::{DeferredNow, Level, LogSpecification, Logger};

use log::*;
use paris::formatter::colorize_string;

fn console_formatter(w: &mut dyn Write, _now: &mut DeferredNow, record: &Record) -> io::Result<()> {
    let log_line = match record.level() {
        Level::Debug => colorize_string(format!("<bright-blue>[DEBUG]</> {}", record.args())),
        Level::Error => colorize_string(format!("<bright-red>[ERROR]</> {}", record.args())),
        Level::Info => colorize_string(format!("<bright-green>[ INFO]</> {}", record.args())),
        Level::Trace => colorize_string(format!(
            "<bright-yellow>[TRACE]</> {}:{} {}",
            record.file().unwrap_or_default(),
            record.line().unwrap_or_default(),
            record.args()
        )),
        Level::Warn => colorize_string(format!("<yellow>[ WARN]</> {}", record.args())),
    };

    write!(w, "{log_line}")
}

pub fn init_logging() -> io::Result<flexi_logger::LoggerHandle> {
    // Build the initial log specification
    let mut builder = LogSpecification::builder();
    builder
        .default(LevelFilter::Debug)
        .module("flexi_logger", LevelFilter::Error)
        .module("log", LevelFilter::Error);

    let logger = Logger::with(builder.build())
        .format(console_formatter)
        .start()
        .map_err(|e| io::Error::new(ErrorKind::Other, e.to_string()))?;

    Ok(logger)
}
