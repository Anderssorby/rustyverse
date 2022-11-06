#[macro_use]
extern crate rocket;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

pub mod models;
pub mod sessions;
pub mod views;

pub mod config;
pub mod logging;

use config::Config;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response,
};
use views::graphql::{self, Schema};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

/// Build the default server
pub fn default_server(config: &Config) -> anyhow::Result<rocket::Rocket<rocket::Build>> {
    Ok(rocket::build().manage(config.clone()))
        .and_then(views::auth::state)
        .map(views::errors::register_catchers)
        .and_then(|rocket| {
            Ok(rocket
                .attach(CORS)
                .manage(Schema::new(
                    graphql::Query::init(),
                    graphql::Mutation::init(),
                    graphql::Subscription::init(),
                ))
                .mount("/v0/api/", views::auth::routes())
                .mount("/v0/api/", graphql::routes()))
        })
}
