use rocket::{
  serde::json::Json,
  Build,
  Rocket,
};
use serde::Serialize;

/// A generic error type
#[derive(Debug, Clone, Serialize)]
pub struct ServerError {
  error: String,
}

impl ServerError {
  pub fn new<E: ToString>(error: E) -> ServerError {
    ServerError { error: error.to_string() }
  }
}

#[catch(500)]
fn internal_error() -> Json<ServerError> {
  Json(ServerError::new("internal server error"))
}

#[catch(404)]
fn not_found() -> Json<ServerError> { Json(ServerError::new("not found")) }

#[catch(403)]
fn unauthorized() -> Json<ServerError> {
  Json(ServerError::new("unauthorized"))
}

#[catch(default)]
fn default(req: &rocket::Request) -> Json<ServerError> {
  Json(ServerError::new(format!("error in {}", req.uri())))
}

/// Register error catchers
pub fn register_catchers(rocket: Rocket<Build>) -> Rocket<Build> {
  rocket.register("/", catchers![
    internal_error,
    not_found,
    unauthorized,
    default
  ])
}
