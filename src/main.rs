use axum::{routing::get, Router};
use clap::{Arg, Command};
use std::env;

#[tokio::main]
async fn main() {
  // 设置默认日志级别
  if env::var("RUST_LOG").is_err() {
    env::set_var("RUST_LOG", "info");
  }
  env_logger::init();

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
  let config_file_path = match matches.get_one::<String>("config") {
    Some(config_path) => &config_path,
    None => "birthdays.json",
  };
  let config_file = read_config_file(config_file_path);
  if let Err(err) = config_file {
    eprintln!("{}", err);
    std::process::exit(1);
  }

  // build our application with a single route
  let app = Router::new().route("/", get(|| async { "Hello, World!" }));

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
  log::info!("Listening on: {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
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
