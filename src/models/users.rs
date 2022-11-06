use crate::{logging::opaque_field_error, sessions::Claims, views::graphql::Context};
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use juniper::{FieldError, FieldResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A User struct which can be displayed through the API
#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description = "A generic user")]
pub struct User {
    pub _id: String,
    pub _key: String,
    pub _rev: String,
    pub email: String,
    pub name: String,
    pub uuid: Uuid,
}
impl User {}

#[derive(Debug, Clone)]
pub struct DbUser {
    pub _id: String,
    pub _key: String,
    pub _rev: String,
    pub email: String,
    pub name: String,
    pub uuid: Uuid,
    pub password_hash: String,
}
impl DbUser {}
/// User token
#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct UserToken {
    pub _key: String,
    pub claims: serde_json::Value,
    pub expires: NaiveDateTime,
    pub created: NaiveDateTime,
    pub user_uuid: Uuid,
    pub uuid: Uuid,
}

/// To register a new external API token
#[derive(Serialize, Clone, PartialEq, Debug)]
pub struct NewUserToken {
    pub claims: serde_json::Value,
    pub expires: NaiveDateTime,
    pub created: NaiveDateTime,
    pub user_uuid: Uuid,
    pub uuid: Uuid,
}

impl NewUserToken {
    pub fn from(claims: Claims) -> Self {
        NewUserToken {
            uuid: claims.uuid,
            claims: serde_json::json!(claims),
            expires: NaiveDateTime::from_timestamp(claims.exp as i64, 0),
            created: chrono::Utc::now().naive_utc(),
            user_uuid: claims.user_uuid,
        }
    }
}

pub struct UserQuery;

#[graphql_object(context = Context)]
impl UserQuery {
    /// List users
    async fn list(context: &Context, limit: Option<i32>) -> FieldResult<Vec<User>> {
        todo!()
    }
}

/// Shared user mutations
pub struct UserMutation;

#[graphql_object(context = Context)]
impl UserMutation {
    /// Create a new  user
    async fn create(&self, context: &Context) -> FieldResult<User> {
        todo!()
    }
}
