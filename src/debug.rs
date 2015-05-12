/// Skarn debugging utilities.

use log::LogLevelFilter::Trace;
use fern::{DispatchConfig, OutputConfig};

use error::Error as SkarnError;

fn setup_logger() -> Result<(), SkarnError> {
    let logger_config = DispatchConfig {
        format: box |msg, level, _location| {
            format!("[{}] {}", level, msg)
        },
        output: vec![OutputConfig::stderr()],
        level: fern::Level::Debug,
    };

    let logger = try!(logger_config.into_logger());

    fern::local::set_thread_logger(sync::Arc::new(logger));
    Ok(())
}
