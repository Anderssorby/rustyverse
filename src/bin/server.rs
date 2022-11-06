use anyhow::anyhow;

use clap::Parser;
use rustyverse::config::Config;
/// Arguments to the server
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(short, long)]
  config_file: Option<String>,
}
#[rocket::main]
async fn main() -> anyhow::Result<()> {
  env_logger::init();
  let args = Args::parse();
  let config = Config::load(args.config_file)?;
  let _res = rustyverse::default_server(&config)
    .map_err(|e| anyhow!("Ignite error: {}", e))?
    .launch()
    .await;
  println!("Rocket deorbits. Goodbye!");
  Ok(())
}
