use crate::config::LogConf;
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::fs::{File, create_dir_all};
use std::path::Path;
use chrono::Local;

pub fn setup(conf: &LogConf) {
    // Get the current date and time
    let now = Local::now();
    let formatted_date = now.format("%Y-%m-%d_%H-%M-%S").to_string();

    // Combine the directory path with the formatted date to create the log file name
    let log_file_name = format!("log_{}.txt", formatted_date);
    let log_file_path = Path::new(&conf.dir).join(log_file_name);

    // Ensure the directory exists
    if let Some(parent) = log_file_path.parent() {
        create_dir_all(parent).unwrap();
    }

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create(&log_file_path).unwrap(),
        ),
    ])
        .unwrap();
}
