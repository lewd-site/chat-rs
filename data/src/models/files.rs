use super::posts::Post;
use crate::schema::files;
use chrono::prelude::*;
use image::io::Reader;
use mime::Mime;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const MAX_WIDTH: i32 = 8000;
const MAX_HEIGHT: i32 = 8000;

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

#[derive(Identifiable, Queryable, Associations, Serialize, Clone)]
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
    fn get_extension_by_mime_type(mimetype: &str) -> Result<&str, String> {
        match mimetype {
            "image/jpeg" => Ok("jpg"),
            "image/pjpeg" => Ok("jpg"),
            "image/png" => Ok("png"),
            "image/x-png" => Ok("png"),
            "image/gif" => Ok("gif"),
            "image/webp" => Ok("webp"),

            "audio/mp3" => Ok("mp3"),
            "audio/mpeg" => Ok("mp3"),
            "audio/mpeg3" => Ok("mp3"),
            "audio/mpeg-3" => Ok("mp3"),
            "audio/mpg" => Ok("mp3"),
            "audio/x-mp3" => Ok("mp3"),
            "audio/x-mpeg" => Ok("mp3"),
            "audio/x-mpeg3" => Ok("mp3"),
            "audio/x-mpeg-3" => Ok("mp3"),
            "audio/x-mpg" => Ok("mp3"),
            "audio/mp4" => Ok("mp4"),
            "audio/m4a" => Ok("mp4"),
            "audio/x-m4a" => Ok("mp4"),
            "audio/webm" => Ok("webm"),

            "video/mp4" => Ok("mp4"),
            "video/mpeg4" => Ok("mp4"),
            "video/x-m4v" => Ok("mp4"),
            "video/webm" => Ok("webm"),
            _ => Err(format!("Unknown mimetype: {}", mimetype)),
        }
    }

    fn get_image_dimensions(
        path: &PathBuf,
    ) -> Result<(Option<i32>, Option<i32>, Option<i32>), String> {
        let reader = match Reader::open(path.clone()) {
            Ok(reader) => reader,
            Err(_) => return Err(format!("Can't read file: {}", path.to_string_lossy())),
        };

        let reader = match reader.with_guessed_format() {
            Ok(reader) => reader,
            Err(_) => {
                return Err(format!(
                    "Can't guess file format: {}",
                    path.to_string_lossy()
                ))
            }
        };

        match reader.into_dimensions() {
            Ok((width, height)) => Ok((Some(width as i32), Some(height as i32), None)),
            Err(_) => Err(format!(
                "Can't determine file dimensions: {}",
                path.to_string_lossy()
            )),
        }
    }

    fn get_audio_dimensions(
        path: &PathBuf,
    ) -> Result<(Option<i32>, Option<i32>, Option<i32>), String> {
        let output = Command::new("ffprobe")
            .arg("-i")
            .arg(path)
            .arg("-select_streams")
            .arg("a:0")
            .arg("-show_entries")
            .arg("format=duration")
            .arg("-v")
            .arg("quiet")
            .arg("-of")
            .arg("csv=p=0:nk=1")
            .output();

        match output {
            Ok(output) => {
                let output = String::from_utf8(output.stdout).unwrap();
                let lines: Vec<&str> = output.split(|c| c == ',' || c == '\n').collect();
                let mut lines = lines.into_iter();
                let duration: f32 = lines.next().unwrap().parse().unwrap();
                let duration = duration.ceil() as i32;

                Ok((None, None, Some(duration)))
            }
            Err(_) => Err(format!("Can't read file: {}", path.to_string_lossy())),
        }
    }

    fn get_video_dimensions(
        path: &PathBuf,
    ) -> Result<(Option<i32>, Option<i32>, Option<i32>), String> {
        let output = Command::new("ffprobe")
            .arg("-i")
            .arg(path)
            .arg("-select_streams")
            .arg("v:0")
            .arg("-show_entries")
            .arg("stream=width,height")
            .arg("-show_entries")
            .arg("format=duration")
            .arg("-v")
            .arg("quiet")
            .arg("-of")
            .arg("csv=p=0:nk=1")
            .output();

        match output {
            Ok(output) => {
                let output = String::from_utf8(output.stdout).unwrap();
                let lines: Vec<&str> = output.split(|c| c == ',' || c == '\n').collect();
                let mut lines = lines.into_iter();
                let width: i32 = lines.next().unwrap().parse().unwrap();
                let height: i32 = lines.next().unwrap().parse().unwrap();
                let duration: f32 = lines.next().unwrap().parse().unwrap();
                let duration = duration.ceil() as i32;

                Ok((Some(width), Some(height), Some(duration)))
            }
            Err(_) => Err(format!("Can't read file: {}", path.to_string_lossy())),
        }
    }

    pub fn new(
        content_type: Option<Mime>,
        file_name: Option<String>,
        path: PathBuf,
        post_id: i32,
    ) -> Result<NewFile, String> {
        let name = file_name.unwrap_or(String::from(""));

        let content_type = content_type.unwrap_or(mime::APPLICATION_OCTET_STREAM);
        let mimetype = String::from(content_type.to_string());
        let extension = String::from(File::get_extension_by_mime_type(&mimetype)?);

        let created_at = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);

        let (width, height, length) = match mimetype.clone() {
            mimetype if mimetype.starts_with("image/") => File::get_image_dimensions(&path)?,
            mimetype if mimetype.starts_with("audio/") => File::get_audio_dimensions(&path)?,
            mimetype if mimetype.starts_with("video/") => File::get_video_dimensions(&path)?,
            _ => (None, None, None),
        };

        if width.unwrap_or(0) > MAX_WIDTH || height.unwrap_or(0) > MAX_HEIGHT {
            return Err(format!("File is too large: {}", path.to_string_lossy()));
        }

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
            width,
            height,
            length,
        };

        Ok(result)
    }
}
