/// Skarn debugging utilities.

use std::sync;
use fern;

use error::Error as SkarnError;

fn setup_logger() -> Result<(), SkarnError> {
    let logger_config = fern::LoggerConfig {
        format: box |msg: &str, level: &fern::Level| {
            format!("[{}] {}", level, msg)
        },
        output: vec![fern::OutputConfig::Stderr],
        level: fern::Level::Debug,
    };

    let logger = try!(logger_config.into_logger());

    fern::local::set_thread_logger(sync::Arc::new(logger));
    Ok(())
}
