use crate::models::users::{DbUser, NewUserToken, User, UserToken};
use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use rand::{distributions::Alphanumeric, Rng};
use rocket::{
    http::{CookieJar, HeaderMap, Status},
    outcome::{IntoOutcome, Outcome::*},
    request::{FromRequest, Outcome},
    Request, State,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
// use serde_json::from_value;

pub struct JwtKeys {
    decoding_key: DecodingKey,
    encoding_key: EncodingKey,
}
fn get_jwt_secret() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect::<String>()
}

impl JwtKeys {
    pub fn init() -> Self {
        let secret: String = get_jwt_secret();

        let decoding_key: DecodingKey = DecodingKey::from_secret(secret.as_ref());
        let encoding_key: EncodingKey = EncodingKey::from_secret(secret.as_ref());
        JwtKeys {
            decoding_key,
            encoding_key,
        }
    }
}

//TODO
pub type DB = ();

/// JWT claims struct
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Unique
    pub uuid: Uuid,
    /// User id
    pub user_uuid: Uuid,
    /// Subject (whom token refers to) email/username.
    pub sub: String,
    /// Required (validate_exp defaults to true in validation). Expiration time
    /// (as UTC timestamp)
    pub exp: u64,
}

impl Claims {
    pub fn new_claims_for_user(user: &User) -> Self {
        let exp: u64 = (Utc::now() + chrono::Duration::days(1))
            .timestamp()
            .abs()
            .try_into()
            .unwrap();
        let uuid = Uuid::new_v4();
        let claims = Claims {
            uuid,
            user_uuid: user.uuid,
            sub: user.email.clone(),
            exp,
        };
        claims
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for &'r Claims {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let claims_result = request
            .local_cache_async(async {
                let session_manager = request
                    .rocket()
                    .state::<SessionManager>()
                    .expect("SessionManager as managed state");
                let headers = request.headers();
                let cookies = request.cookies();
                debug!("Reading Claims");
                let claims = session_manager
                    .extract_jwt(headers, cookies)
                    .map_err(|e| error!("decode jwt: {}", e))
                    .ok()?;
                let db = ();
                if session_manager
                    .verify_claims(&db, claims.clone())
                    .await
                    .map_err(|e| error!("verify claims: {}", e))
                    .unwrap_or(false)
                {
                    Some(claims)
                } else {
                    None
                }
            })
            .await;
        claims_result
            .as_ref()
            .into_outcome((Status::Unauthorized, ()))
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for &'r User {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // let user_result = request
        //     .local_cache_async(async {
        //         let claims = request.guard::<&Claims>().await.succeeded()?;
        //         debug!("Loading User");
        //         // User::get_by_email(&claims.sub, &db)
        //         //   .await
        //         //   .map_err(|e| error!("{}", e))
        //         //   .ok()
        //         todo!()
        //     })
        //     .await;
        // user_result
        //     .as_ref()
        //     .into_outcome((Status::Forbidden, "unauthorized".to_string()))
        todo!()
    }
}

pub struct SessionManager {
    jwt_keys: JwtKeys,
}

impl SessionManager {
    pub fn init() -> Self {
        Self {
            jwt_keys: JwtKeys::init(),
        }
    }

    pub async fn verify_claims(&self, db: &DB, claims: Claims) -> Result<bool> {
        debug!("fetching UserToken");
        // let ut = UserToken::get(claims.uuid, &db).await?;
        // let c = from_value::<Claims>(ut.claims)
        //     .map_err(|e| anyhow!("Failed to deserialize Claims {}", e))?;
        // Ok(c == claims)
        todo!()
    }

    fn extract_jwt(&self, headers: &HeaderMap<'_>, cookies: &CookieJar<'_>) -> Result<Claims> {
        let decoding_key = &self.jwt_keys.decoding_key;
        let jwt = headers
            .get_one("Authorization")
            .map(|bearer| bearer.split_once("Bearer "))
            .flatten()
            .map(|(_, jwt)| jwt)
            .or_else(|| {
                let c = cookies.get("jwt")?;
                Some(c.value())
            })
            .ok_or_else(|| anyhow!("No jwt found in request"))?;
        let token_data =
            jsonwebtoken::decode::<Claims>(&jwt, &decoding_key, &Validation::new(Algorithm::HS256))
                .map_err(|e| anyhow!("decode jwt: {}", e))?;
        Ok(token_data.claims)
    }

    pub async fn add_claims(&self, db: &DB, claims: &Claims) -> Result<UserToken> {
        // let user_token = NewUserToken::from(claims.clone()).insert(db).await?;
        // Ok(user_token)
        todo!()
    }

    pub async fn login_user<F>(&self, email: &str, verify_user: F) -> Result<(String, Claims)>
    where
        F: Fn(&DbUser) -> Result<bool>,
    {
        let db = ();
        // match DbUser::get(email, &db).await {
        //     Ok(user) => {
        //         let verified = verify_user(&user).unwrap_or_else(|e| {
        //             error!("verify_user: {}", e);
        //             false
        //         });
        //         let claims = Claims::new_claims_for_user(&user);
        //         if verified {
        //             let jwt = jsonwebtoken::encode(
        //                 &Header::default(),
        //                 &claims,
        //                 &self.jwt_keys.encoding_key,
        //             )
        //             .unwrap();
        //             self.add_claims(&db, &claims).await?;
        //             Ok((jwt, claims))
        //         } else {
        //             Err(anyhow!("incorrect credentials"))
        //         }
        //     }
        //     Err(e) => Err(anyhow!("{}", e)),
        // }
        todo!()
    }
}

pub fn verify_user<'r>(password: &'r str) -> impl Fn(&DbUser) -> Result<bool> + 'r {
    move |user: &DbUser| {
        // let parsed_hash = PasswordHash::new(user.password.as_str()).map_err(|e| anyhow!(e))?;
        // Ok(Argon2::default()
        //     .verify_password(password.as_bytes(), &parsed_hash)
        //     .is_ok())
        todo!()
    }
}
