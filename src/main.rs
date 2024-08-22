use axum::{extract::State, routing::get, Router};
use clap::{Arg, Command};
use std::env;
mod calendar;
mod config;

#[tokio::main]
async fn main() {
  // 设置默认日志级别
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info");
  }
  env_logger::init();

  let matches = set_command();
  let config_file_path = get_config_file_path(&matches);
  let config = get_config(config_file_path).unwrap();

  let app = Router::new()
    .route("/", get(create_calendar))
    .with_state(config);
  let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
  log::info!("Listening on: {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
}

async fn create_calendar(State(config): State<config::BirthdayConfig>) -> String {
  // 在这里拿到 main 函数里的 config
  let calendar_info = calendar::create_calendar(config);
  return calendar_info.to_string();
}

fn read_config_file(config_file_path: &str) -> anyhow::Result<String> {
  let config = std::fs::read_to_string(config_file_path).map_err(|err| {
    anyhow::anyhow!(
      "Failed to read config file '{}': {}. You can set config file with -c",
      config_file_path,
      err
    )
  });
  config
}

fn set_command() -> clap::ArgMatches {
  let matches = Command::new("lunar-birthday-calendar")
    .version("1.0")
    .author("alanlang")
    .arg(
      Arg::new("config")
        .short('c')
        .long("config")
        .value_name("FILE")
        .help("Sets a lunar birthday config file")
        .required(false),
    )
    .get_matches();
  matches
}

fn get_config_file_path(matches: &clap::ArgMatches) -> &str {
  match matches.get_one::<String>("config") {
    Some(config_path) => config_path,
    None => "birthdays.json",
  }
}

fn get_config(config_file_path: &str) -> anyhow::Result<config::BirthdayConfig> {
  let config_str = read_config_file(config_file_path)?;
  let config = config::get_config(config_str)?;
  log::info!("Config read {:?} birthday items", config.birthdays.len());
  Ok(config)
}
