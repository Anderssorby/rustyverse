use anyhow::Result;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use chrono::{NaiveDateTime, Utc};
use rand::rngs::OsRng;

pub mod files;
pub mod users;
use serde::{Deserialize, Serialize, Serializer};

pub fn hash_option_password<S>(so: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(s) = so {
        hash_password(&s, serializer)
    } else {
        serializer.serialize_none()
    }
}
pub fn hash_password<S>(s: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hash = hash_password_str(&s).map_err(|e| serde::ser::Error::custom(e))?;
    serializer.serialize_str(&hash)
}
/// A hashing password serializer
pub fn hash_password_str(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    argon2
        .hash_password(password.as_ref(), &salt)
        .map_err(|e| anyhow!("{}", e))
        .map(|h| h.to_string())
}

/// Extract the _key value from a JSON Value as a string from a vec with at
/// least one element.
///
/// # Errors
///
/// This function will return an error if any of the assumtions are not met.
pub fn extract_key(mut v: Vec<serde_json::Value>) -> Result<String> {
    Ok(v.pop()
        .ok_or(anyhow!("Empty result"))?
        .get("_key")
        .ok_or(anyhow!("No _key"))?
        .as_str()
        .ok_or(anyhow!("_key not string"))?
        .to_string())
}
/// Extract the _id value from a JSON Value as a string from a vec with at
/// least one element.
///
/// # Errors
///
/// This function will return an error if any of the assumtions are not met.
pub fn extract_id(mut v: Vec<serde_json::Value>) -> Result<String> {
    Ok(v.pop()
        .ok_or(anyhow!("Empty result"))?
        .get("_id")
        .ok_or(anyhow!("No _id"))?
        .as_str()
        .ok_or(anyhow!("_id not string"))?
        .to_string())
}
