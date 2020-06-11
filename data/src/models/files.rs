use super::posts::Post;
use crate::schema::files;
use chrono::prelude::*;
use image::io::Reader;
use mime::Mime;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Insertable)]
#[table_name = "files"]
pub struct NewFile {
    pub md5: String,
    pub size: i64,
    pub name: String,
    pub mimetype: String,
    pub extension: String,
    pub created_at: NaiveDateTime,
    pub post_id: i32,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub length: Option<i32>,
}

#[derive(Identifiable, Queryable, Associations, Serialize)]
#[belongs_to(Post)]
pub struct File {
    pub id: i32,
    pub md5: String,
    pub size: i64,
    pub name: String,
    pub mimetype: String,
    pub extension: String,
    pub created_at: NaiveDateTime,
    pub post_id: i32,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub length: Option<i32>,
}

impl File {
    pub fn new(
        content_type: Option<Mime>,
        file_name: Option<String>,
        path: PathBuf,
        post_id: i32,
    ) -> Result<NewFile, String> {
        let name = file_name.unwrap_or(String::from(""));

        let content_type = content_type.unwrap_or(mime::APPLICATION_OCTET_STREAM);
        let mimetype = String::from(content_type.to_string());

        let extension = String::from(match &mimetype[..] {
            "image/jpeg" => "jpg",
            "image/pjpeg" => "jpg",
            "image/png" => "png",
            "image/x-png" => "png",
            "image/gif" => "gif",
            "image/webp" => "webp",
            _ => return Err(format!("Unknown mimetype: {}", mimetype)),
        });

        let created_at = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);

        let reader = match Reader::open(path.clone()) {
            Ok(reader) => reader,
            Err(_) => return Err(format!("Can't read file: {}", path.to_string_lossy())),
        };

        let reader = match reader.with_guessed_format() {
            Ok(reader) => reader,
            Err(_) => return Err(format!("Can't read file: {}", path.to_string_lossy())),
        };

        let (width, height) = match reader.into_dimensions() {
            Ok(dimensions) => dimensions,
            Err(_) => {
                return Err(format!(
                    "Can't determine image dimensions: {}",
                    path.to_string_lossy()
                ))
            }
        };

        let content = match fs::read(path.clone()) {
            Ok(content) => content,
            Err(_) => return Err(format!("Can't read file: {}", path.to_string_lossy())),
        };

        let size = content.len() as i64;
        let hash = format!("{:x}", md5::compute(content));

        let filename = format!("{}.{}", hash, extension);
        let dest_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../www/src");
        let dest_path = Path::new(dest_dir).join(filename);
        if let Err(msg) = fs::copy(path, dest_path) {
            return Err(format!("Can't copy file: {}", msg));
        }

        let result = NewFile {
            size,
            md5: hash,
            name,
            mimetype,
            extension,
            created_at,
            post_id,
            width: Some(width as i32),
            height: Some(height as i32),
            length: None,
        };

        Ok(result)
    }
}
