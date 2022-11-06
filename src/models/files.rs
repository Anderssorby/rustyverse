use std::{fs, path::PathBuf};

use anyhow::Result;
use juniper::FieldResult;
use serde::{Deserialize, Serialize};

use crate::{logging::opaque_field_error, views::graphql::Context};

use super::users::User;

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description = "")]
pub struct File {
    pub _id: String,
    pub _key: String,
    pub _rev: String,
    pub name: String,
    pub cid: String,
    pub content_type: Option<String>,
}

impl File {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewFile {
    pub name: String,
    pub description: Option<String>,
    pub cid: String,
    pub content_type: Option<String>,
}

impl NewFile {}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description = "Input to change the values of a File")]
pub struct UpdateFile {
    pub _key: String,
    pub name: String,
    pub description: Option<String>,
}

impl UpdateFile {}

pub struct FileQuery;

#[graphql_object(context = Context)]
impl FileQuery {
    async fn list(context: &Context, limit: Option<i32>) -> FieldResult<Vec<File>> {
        todo!()
    }
}
///  file mutations
pub struct FileMutation;

#[graphql_object(context = Context)]
impl FileMutation {
    async fn update_file(context: &Context, file: UpdateFile) -> FieldResult<&str> {
        todo!()
    }

    async fn remove_file(context: &Context, cid: String) -> FieldResult<&str> {
        todo!()
    }
}
