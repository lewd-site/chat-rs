use crate::routes::types::Authenticated;
use crate::ChatDbConn;
use data::models::files::File;
use data::repositories::files::FileRepository;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct FileListResponse {
    items: Vec<File>,
}

#[get("/", format = "json")]
pub fn get_file_list(auth: Authenticated, conn: ChatDbConn) -> Json<FileListResponse> {
    let files = FileRepository::get_latest_for_user(&*conn, &auth.get_uuid());

    Json(FileListResponse { items: files })
}
