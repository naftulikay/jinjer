use crate::cli::Args;

use log::LevelFilter;

use log4rs;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::Append;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::encode::Encode;

use parking_lot::Once;

use std::env;

use std::str::FromStr;

static APPENDER_NAME: &'static str = "stderr";

static INITIALIZE: Once = Once::new();

static LOG_FORMAT: &'static str = "{d(%Y-%m-%dT%H:%M:%S%.3f%z)} {l:5.5} [{T}] {M}: {m}{n}";

static DEFAULT_ROOT_LEVEL: LevelFilter = LevelFilter::Error;

static ROOT_LEVEL_ENV_VAR: &'static str = "LOG4RS_ROOT_LEVEL";

/// Initialize logging. This function is idempotent.
pub fn init(config: &Args) {
    INITIALIZE.call_once(move || init_once(config));
}

fn init_once(config: &Args) {
    let encoder: Box<Encode> = Box::new(PatternEncoder::new(LOG_FORMAT));
    let appender: Box<Append> = Box::new(
        ConsoleAppender::builder()
            .encoder(encoder)
            .target(Target::Stderr)
            .build(),
    );

    let level = match config.verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    log4rs::init_config(
        Config::builder()
            .appender(Appender::builder().build(APPENDER_NAME, appender))
            .logger(Logger::builder().build("jinjer", level))
            .build(Root::builder().appender(APPENDER_NAME).build(
                match env::var(ROOT_LEVEL_ENV_VAR) {
                    Ok(s) => LevelFilter::from_str(s.as_str()).expect("unknown logging level"),
                    Err(_) => DEFAULT_ROOT_LEVEL,
                }
            ))
            .unwrap(),
    )
    .unwrap();
}
