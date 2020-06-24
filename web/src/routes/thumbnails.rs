use crate::ChatDbConn;
use data::repositories::files::FileRepository;
use image::imageops::FilterType;
use image::io::Reader;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rocket::response::{self, NamedFile, Responder};
use rocket::{Request, Response};
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub struct CachedFile {
    file: NamedFile,
    max_age: u32,
}

impl CachedFile {
    pub fn new(file: NamedFile, max_age: u32) -> CachedFile {
        CachedFile { file, max_age }
    }
}

impl<'r> Responder<'r> for CachedFile {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build()
            .merge(self.file.respond_to(req)?)
            .raw_header(
                "Cache-Control",
                format!("public, immutable, max-age={}", self.max_age),
            )
            .ok()
    }
}

fn get_thumb_extension(file_extension: &str) -> &str {
    match file_extension {
        "gif" => "png",
        "png" => "png",
        "webp" => "png",
        _ => "jpg",
    }
}

fn create_temp_path() -> PathBuf {
    let temp_name: String = thread_rng().sample_iter(&Alphanumeric).take(32).collect();
    let mut temp_path = env::temp_dir();
    temp_path.push(temp_name);
    temp_path
}

fn create_image_thumbnail(src: &Path, dst: &Path, max_size: u32) -> Result<(), Box<dyn Error>> {
    let temp_path = create_temp_path().with_extension(dst.extension().unwrap());

    let image = Reader::open(src)?.with_guessed_format()?.decode()?;
    image
        .resize(max_size, max_size, FilterType::Lanczos3)
        .save(&temp_path)?;

    fs::copy(&temp_path, dst)?;
    fs::remove_file(temp_path)?;

    Ok(())
}

fn create_video_thumbnail(src: &Path, dst: &Path, max_size: u32) -> Result<(), Box<dyn Error>> {
    let temp_path = create_temp_path().with_extension(dst.extension().unwrap());

    Command::new("ffmpeg")
        .arg("-ss")
        .arg("1")
        .arg("-i")
        .arg(src)
        .arg("-vframes")
        .arg("1")
        .arg("-vf")
        .arg(format!(
            "scale={}:{}:force_original_aspect_ratio=decrease",
            max_size, max_size
        ))
        .arg(&temp_path)
        .arg("-v")
        .arg("quiet")
        .arg("-y")
        .output()?;

    fs::copy(&temp_path, dst)?;
    fs::remove_file(temp_path)?;

    Ok(())
}

#[get("/<hash>?<max_width>")]
pub fn get_thumbnail(conn: ChatDbConn, hash: String, max_width: Option<i32>) -> Option<CachedFile> {
    let max_width = max_width.unwrap_or(360);
    // TODO: Check list of allowed thumbnail sizes.
    if max_width != 360 {
        return None;
    }

    let file = FileRepository::get_one_by_md5(&*conn, &hash);
    file.map(|file| {
        let thumb_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../www/thumb");
        let thumb_extension = get_thumb_extension(&file.extension);
        let thumb_filename = format!("{}_{}.{}", hash, max_width, thumb_extension);
        let thumb_path = Path::new(thumb_dir).join(thumb_filename);
        if !thumb_path.exists() {
            let src_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../www/src");
            let src_filename = format!("{}.{}", hash, file.extension);
            let src_path = Path::new(src_dir).join(src_filename);

            match file.mimetype.clone() {
                mimetype if mimetype.starts_with("image/") => {
                    create_image_thumbnail(&src_path, &thumb_path, max_width as u32).unwrap();
                }
                mimetype if mimetype.starts_with("video/") => {
                    create_video_thumbnail(&src_path, &thumb_path, max_width as u32).unwrap();
                }
                _ => {}
            };
        }

        NamedFile::open(thumb_path)
            .map(|file| CachedFile::new(file, 31536000))
            .unwrap()
    })
}
