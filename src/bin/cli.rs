#[macro_use]
extern crate anyhow;
use argon2::PasswordHash;
// use chrono::{
//   DateTime,
//   Duration,
//   Utc,
// };
// use either::Either;
// use url::Url;

use clap::Parser;
use rocket::{http::Status, routes, serde::json::Json, Rocket};
use rustyverse::{config::Config, models::hash_password_str};
// use serde_json::json;

#[derive(clap::Subcommand, Clone, Debug)]
pub enum Operation {
    /// Hash a string using the Argon2 hasher
    Hash { text: String },
    /// Experimentation server
    Rocket,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    config_file: Option<String>,
    #[clap(subcommand)]
    operation: Operation,
}
#[rocket::get("/")]
async fn hello() -> Result<Json<String>, Status> {
    let config = Config::load(None).unwrap();
    Ok(Json(String::from("Hello from rust")))
}

async fn rocket() -> anyhow::Result<Rocket<rocket::Ignite>> {
    rocket::build()
        .mount("/", routes![hello])
        .launch()
        .await
        .map_err(|e| anyhow!("Ignite error: {}", e))
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut config = Config::load(args.config_file)?;
    match args.operation {
        Operation::Hash { text } => {
            let hash = hash_password_str(&text).unwrap();
            println!("argon2('{}') = '{:?}'", text, hash);

            let s = hash.to_string();
            let parsed_hash = PasswordHash::new(&s).map_err(|e| anyhow!(e))?;
            println!("{}", parsed_hash);
            Ok(())
        }
        Operation::Rocket => {
            let h = tokio::spawn(async {
                let _r = rocket().await;
            });

            h.await?;
            Ok(())
        }
    }
}
