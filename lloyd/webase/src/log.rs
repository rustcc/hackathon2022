use std::path::Path;

use anyhow::Result;
use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use crate::{
    config::{ensure_res_dir, write_content},
    global::RESOURCES_DIR_NAME,
};

pub fn default_config_content() -> String {
    r#"
# refresh_rate: 30 seconds

appenders:
  stdout:
    kind: console
    encoder:
      pattern: "{d(%Y-%m-%d %H:%M:%S)} [{h({l})}] [{f}:{L}] {m}{n}"
  rfile:
    kind: rolling_file
    path: "log/app.log"
    encoder:
      pattern: "{d(%Y-%m-%d %H:%M:%S)} [{h({l})}] [{M}] [{f}:{L}] {m}{n}"
    policy:
      kind: compound
      trigger:
        kind: size
        limit: 50 mb
      roller:
        kind: fixed_window
        pattern: 'log/archive/app.log.{}.gz'
        count: 50

root:
  level: info
  appenders:
    - stdout
    - rfile    

    "#
    .to_string()
}

pub fn init_log() -> Result<()> {
    init_log_with_dir("")
}

pub fn init_log_with_dir(name: &str) -> Result<()> {
    let location = format!("{}{}/log.yaml", RESOURCES_DIR_NAME, name);
    let path = Path::new(&location);
    if !path.exists() {
        ensure_res_dir()?;
        write_content(&default_config_content(), &location)?;
    }
    let result = log4rs::init_file(path, Default::default());
    if let Err(e) = result {
        eprintln!("load log config file failed:{}", e);
        return build_default_log();
    }
    result
}

pub fn build_default_log() -> Result<()> {
    let stdout_pattern = "{d(%Y-%m-%d %H:%M:%S)} [{h({l})}] [{f}:{L}] {m}{n}";
    let stdout_name = "stdout";
    let rfile_name = "rfile";
    let rfile_pattern = "{d(%Y-%m-%d %H:%M:%S)} [{h({l})}] [{M}] [{f}:{L}] {m}{n}";
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(stdout_pattern)))
        .build();

    let trigger = SizeTrigger::new(1024 * 1024 * 50);
    let roller = FixedWindowRoller::builder().build("log/archive/app.log.{}.gz", 50)?;
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
    let rfile = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(rfile_pattern)))
        .build("log/app.log", Box::new(policy))?;

    let config = Config::builder()
        .appender(Appender::builder().build(stdout_name, Box::new(stdout)))
        .appender(Appender::builder().build(rfile_name, Box::new(rfile)))
        .build(
            Root::builder()
                .appender(rfile_name)
                .appender(stdout_name)
                .build(log::LevelFilter::Info),
        )?;

    log4rs::init_config(config)?;
    Ok(())
}
