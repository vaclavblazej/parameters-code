//! Sets up global logging with: error! warn! info! debug! trace!

use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};

use crate::general::progress::with_progress_suspended;

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init(level_filter: LevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(level_filter))
}

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        with_progress_suspended(|| {
            println!("{} - {}", record.level(), record.args());
        });
    }

    fn flush(&self) {}
}
