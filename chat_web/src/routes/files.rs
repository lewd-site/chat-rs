use crate::routes::types::Authenticated;
use crate::ChatDbConn;
use chat::models::files::{File, UserFavoriteFile};
use chat::repositories::files::FileRepository;
use rocket::response::status::{Created, NotFound};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateFavoriteRequest {
    md5: String,
}

#[derive(Serialize)]
pub struct ErrorJson {
    message: String,
}

#[derive(Serialize)]
pub struct FileResponse {
    item: File,
}

#[derive(Serialize)]
pub struct FileListResponse {
    items: Vec<File>,
}

#[derive(Responder)]
pub enum UpdateFileResponse {
    Created(Created<Json<FileResponse>>),
    Deleted(Json<FileResponse>),
    NotFound(NotFound<Json<ErrorJson>>),
}

impl UpdateFileResponse {
    fn created(file: File) -> UpdateFileResponse {
        let json = Json(FileResponse { item: file.clone() });
        let location = format!("/api/v1/files/{}", file.id);
        UpdateFileResponse::Created(Created(location, Some(json)))
    }

    fn deleted(file: File) -> UpdateFileResponse {
        let json = Json(FileResponse { item: file });
        UpdateFileResponse::Deleted(json)
    }

    fn not_found(error: &str) -> UpdateFileResponse {
        let message = String::from(error);
        let json = Json(ErrorJson { message });
        UpdateFileResponse::NotFound(NotFound(json))
    }
}

#[get("/", format = "json")]
pub fn get_file_list(auth: Authenticated, conn: ChatDbConn) -> Json<FileListResponse> {
    let files = FileRepository::get_latest_for_user(&*conn, &auth.get_uuid());

    Json(FileListResponse { items: files })
}

#[get("/", format = "json")]
pub fn get_favorite_file_list(auth: Authenticated, conn: ChatDbConn) -> Json<FileListResponse> {
    let files = FileRepository::get_favorites_for_user(&*conn, &auth.get_uuid());

    Json(FileListResponse { items: files })
}

#[post("/", format = "json", data = "<data>")]
pub fn create_favorite_file(
    auth: Authenticated,
    conn: ChatDbConn,
    data: Json<CreateFavoriteRequest>,
) -> UpdateFileResponse {
    let uuid = auth.get_uuid();
    match FileRepository::get_one_by_md5(&conn, &data.md5) {
        Some(file) => {
            let favorite_file = UserFavoriteFile::new(file.id, &uuid);
            FileRepository::create_favorite(&conn, &favorite_file);
            UpdateFileResponse::created(file)
        }
        None => UpdateFileResponse::not_found("Not found"),
    }
}

#[delete("/<hash>")]
pub fn delete_favorite_file(
    auth: Authenticated,
    conn: ChatDbConn,
    hash: String,
) -> UpdateFileResponse {
    let uuid = auth.get_uuid();
    let file = FileRepository::get_one_by_md5(&conn, &hash);
    let favorite_file = FileRepository::get_favorite_by_md5(&conn, &hash, &uuid);
    match (file, favorite_file) {
        (Some(file), Some(favorite_file)) => {
            FileRepository::delete_favorite(&conn, &favorite_file);
            UpdateFileResponse::deleted(file)
        }
        _ => UpdateFileResponse::not_found("Not found"),
    }
}
