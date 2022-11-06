use std::{fs, path::PathBuf};

use libipld::{
    cbor::DagCborCodec,
    multihash::{Code, MultihashDigest},
    prelude::Codec,
    Cid, Ipld,
};
use rocket::{
    form::Form,
    fs::{FileName, TempFile},
    http::Status,
    serde::json::Json,
    Route, State,
};
use serde::Serialize;

use crate::{
    models::{files::NewFile, users::User},
    views::errors::ServerError,
};

#[derive(FromForm)]
struct Upload<'r> {
    file: TempFile<'r>,
    description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UploadResponse {
    hash: String,
}

/// Returns the corresponding dag-cbor v1 Cid
/// to the passed IPLD
/// # Panics
/// Panics if x could not be encoded into a dag-cbor bytecursor
pub fn cid(x: &Ipld) -> Cid {
    Cid::new_v1(
        0x71,
        Code::Blake2b256.digest(DagCborCodec.encode(x).unwrap().as_ref()),
    )
}

#[options("/upload")]
async fn upload_options() {}

#[post("/upload", data = "<upload>")]
async fn upload(
    mut upload: Form<Upload<'_>>,
    user: &User,
) -> Result<Json<UploadResponse>, Json<ServerError>> {
    let handle_err = |e: std::io::Error| {
        error!("upload: {:}", e);
        Json(ServerError::new("upload failed"))
    };

    // let file = match &upload.file {
    //     TempFile::File {
    //         file_name,
    //         content_type,
    //         path,
    //         ..
    //     } => {
    //         let bytes = fs::read(&path).map_err(|e| {
    //             error!("upload: {:?}", e);
    //             Json(ServerError::new("upload failed"))
    //         })?;
    //         let ipld = Ipld::Bytes(bytes);
    //         let cid = cid(&ipld);
    //         NewFile {
    //             cid: cid.to_string(),
    //             name: file_name
    //                 .map(FileName::as_str)
    //                 .unwrap_or_default()
    //                 .unwrap_or_default()
    //                 .to_string(),
    //             description: upload.description.clone(),
    //             content_type: content_type.clone().map(|ct| ct.to_string()),
    //         }
    //     }
    //     TempFile::Buffered { content } => {
    //         let ipld = Ipld::Bytes(content.as_bytes().to_vec());
    //         let cid = cid(&ipld);
    //         NewFile {
    //             cid: cid.to_string(),
    //             description: upload.description.clone(),
    //             name: Default::default(),
    //             content_type: None,
    //         }
    //     }
    // }
    // .insert(&db)
    // .await
    // .map_err(|e| {
    //     error!("upload: {:}", e);
    //     Json(ServerError::new("upload failed"))
    // })?;
    // let mut file_root =
    //     PathBuf::try_from(std::env::var("FILE_STORAGE_ROOT").unwrap_or("/tmp".to_string()))
    //         .map_err(|e| {
    //             error!("upload: {:}", e);
    //             Json(ServerError::new("upload failed"))
    //         })?;
    // let dir = file_root.as_path();
    // fs::create_dir_all(&dir).map_err(handle_err)?;
    // file_root.push(file.cid.to_string());
    // let path = file_root.as_path();
    // upload.file.persist_to(&path).await.map_err(handle_err)?;
    // Ok(Json(UploadResponse {
    //     hash: file.cid.to_string(),
    // }))
    todo!()
}

#[options("/<_cid>")]
async fn get_file_options(_cid: &str) {}

#[get("/<cid>")]
async fn get_file(cid: &str, user: &User) -> Result<Vec<u8>, Status> {
    let cid_v = Cid::try_from(cid).map_err(|_| Status::NotFound)?;
    let mut file_root =
        PathBuf::try_from(std::env::var("FILE_STORAGE_ROOT").unwrap_or("/tmp".to_string()))
            .map_err(|e| {
                error!("upload: {:}", e);
                Status::InternalServerError
            })?;
    file_root.push(cid_v.to_string());
    let path = file_root.as_path();
    let bytes = fs::read(&path).map_err(|_| Status::NotFound)?;
    Ok(bytes)
}

pub fn routes() -> Vec<Route> {
    routes![upload, upload_options, get_file, get_file_options]
}
