//! A logger implementation to use RTT with the Rust `log` crate.
//!
//! ```
//! use log::{info, LevelFilter};
//! use rtt_logger::RTTLogger;
//! use rtt_target::rtt_init_print;
//!
//! // logger configuration
//! const LOG_LEVEL: LevelFilter = LevelFilter::Info;
//! static LOGGER: RTTLogger = RTTLogger::new(LOG_LEVEL);
//!
//! fn main() {
//!     // logger setup
//!     rtt_init_print!();
//!     log::set_logger(&LOGGER)
//!         .map(|()| log::set_max_level(LOG_LEVEL))
//!         .unwrap();
//!
//!     // logger usage in main binary or in any library
//!     info!("Hello World!");
//! }
//! ```

#![no_std]

use log::{LevelFilter, Metadata, Record};

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
        RTTLogger { level_filter }
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

    fn flush(&self) {}
}
