#![no_std]

use log::{Metadata, Record, LevelFilter};

use rtt_target::*;

/// An RTT-based logger implementation.
pub struct RTTLogger {
    level_filter: LevelFilter,
}

impl RTTLogger {
    /// Static-friendly const initializer.
    ///
    /// * `level_filter`: The default level to enable.
    pub const fn new(level_filter: LevelFilter) -> RTTLogger {
        RTTLogger {
            level_filter
        }
    }
}

impl log::Log for RTTLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.level_filter.ge(&metadata.level())
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            rprintln!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
    }
}

