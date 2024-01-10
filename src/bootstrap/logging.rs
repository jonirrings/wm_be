//! Setup for the application logging.
//!
//! - `Off`
//! - `Error`
//! - `Warn`
//! - `Info`
//! - `Debug`
//! - `Trace`
use std::str::FromStr;
use std::sync::Once;

use log::{info, LevelFilter};

static INIT: Once = Once::new();

pub fn setup(log_level: &Option<String>) {
    let level = config_level_or_default(log_level);

    if level == log::LevelFilter::Off {
        return;
    }

    INIT.call_once(|| {
        stdout_config(level);
    });
}

fn config_level_or_default(log_level: &Option<String>) -> LevelFilter {
    match log_level {
        None => log::LevelFilter::Info,
        Some(level) => LevelFilter::from_str(level).unwrap(),
    }
}

fn stdout_config(level: LevelFilter) {
    if let Err(_err) = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}][{}] {}",
                chrono::Local::now().format("%+"),
                record.target(),
                record.level(),
                message
            ));
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()
    {
        panic!("Failed to initialize logging.")
    }

    info!("logging initialized.");
}
