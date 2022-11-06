use crate::{
    sessions::{Claims, SessionManager},
    views::errors::ServerError,
};
use anyhow::Result;
use rocket::{
    http::{Cookie, CookieJar},
    serde::json::Json,
    Build, Rocket, Route, State,
};
use serde::{Deserialize, Serialize};

/// Data to log in
#[derive(Debug, Clone, Deserialize, FromForm, GraphQLInputObject)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    /// Signed Json web token
    jwt: String,
    /// Time in milliseconds encoded in base64 when the token expires
    expires: u64,
}

impl LoginResponse {
    pub fn json_from_jwt_claims((jwt, claims): (String, Claims)) -> Json<Self> {
        Json(LoginResponse {
            jwt,
            expires: claims.exp,
        })
    }
}

#[options("/login")]
pub async fn login_options() {}

#[post("/login", format = "json", data = "<credentials>")]
pub async fn login<'r>(
    credentials: Json<Credentials>,
    session_manager: &State<SessionManager>,
    cookies: &CookieJar<'_>,
) -> Result<Json<LoginResponse>, Json<ServerError>> {
    let handle_error = |e| {
        error!("login: {}", e);
        Json(ServerError::new("login failed"))
    };
    let res = session_manager
        .login_user(
            &credentials.email,
            crate::sessions::verify_user(&credentials.password),
        )
        .await
        .map(LoginResponse::json_from_jwt_claims)
        .map_err(handle_error)?;
    let cookie = Cookie::build("jwt", res.jwt.clone())
        .same_site(rocket::http::SameSite::None)
        .finish();
    cookies.add(cookie);
    Ok(res)
}

pub fn routes() -> Vec<Route> {
    routes![login, login_options]
}

pub fn state(rocket: Rocket<Build>) -> anyhow::Result<Rocket<Build>> {
    Ok(rocket.manage(SessionManager::init()))
}
