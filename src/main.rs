mod api;
mod cli;
mod config;
use crate::config::get_config;
mod game;
mod util;

use colored::*;
use inquire::Text;
use log::{error, info};
use std::io::Write;

#[tokio::main]
async fn main() {
    let config = get_config();

    let args = cli::parse_args();

    let log_level = match args.verbose {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .format_level(true)
        .format_module_path(false)
        .format_target(false)
        .format(|buf, record| {
            let level = record.level();
            let message = format!("{}", record.args());

            let level_colored = match level {
                log::Level::Error => level.to_string().red(),
                log::Level::Warn => level.to_string().yellow(),
                log::Level::Info => level.to_string().green(),
                log::Level::Debug => level.to_string().cyan(),
                log::Level::Trace => level.to_string().purple(),
            };

            writeln!(
                buf,
                "[{} {}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                level_colored,
                message
            )
        })
        .filter_level(log_level)
        .init();

    let start = args.start.unwrap_or_else(|| {
        Text::new("Введите начальную статью Википедии: ")
            .prompt()
            .expect("Не удалось прочитать ввод")
    });
    let end = args.end.unwrap_or_else(|| {
        Text::new("Введите конечную статью Википедии: ")
            .prompt()
            .expect("Не удалось прочитать ввод")
    });

    info!("Полученный конфиг: {:#?}", config);
    info!("Начинаем с: {}", start);
    info!("Пытаемся достичь: {}", end);

    if let Err(e) = game::run(&start, &end, &config).await {
        error!("Ошибка в игре: {}", e);
    }
}
