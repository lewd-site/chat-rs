use crate::requests::CreatePostMultipart;
use crate::requests::UploadedFile;
use crate::ws::Ws;
use crate::ChatDbConn;
use chrono::prelude::*;
use data::models::files::File;
use data::models::message_parser::{MessageParser, Segment};
use data::models::posts::Post;
use data::repositories::files::FileRepository;
use data::repositories::posts::PostRepository;
use rocket::request::Form;
use rocket::response::status::Created;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreatePostJson {
    name: String,
    message: String,
}

#[derive(FromForm)]
pub struct CreatePostForm {
    name: String,
    message: String,
}

#[derive(Serialize)]
struct PostWithFiles {
    id: i32,
    name: String,
    tripcode: String,
    message_raw: String,
    message: Vec<Segment>,
    created_at: NaiveDateTime,
    files: Vec<File>,
}

impl PostWithFiles {
    fn new(post: Post, files: Vec<File>) -> PostWithFiles {
        PostWithFiles {
            id: post.id,
            name: post.name,
            tripcode: post.tripcode,
            message_raw: post.message.clone(),
            message: MessageParser::str_to_segments(&post.message),
            created_at: post.created_at,
            files,
        }
    }
}

#[derive(Serialize)]
pub struct PostListResponse {
    items: Vec<PostWithFiles>,
}

#[derive(Serialize)]
pub struct PostResponse {
    item: PostWithFiles,
}

fn create_post(
    conn: ChatDbConn,
    name: &str,
    message: &str,
    files: Vec<UploadedFile>,
) -> PostWithFiles {
    let new_post = Post::new(name, message);
    let post = PostRepository::create(&*conn, &new_post);
    let files = files
        .into_iter()
        .map(|file| {
            let new_file =
                File::new(file.content_type, file.file_name, file.path, post.id).unwrap();
            FileRepository::create(&*conn, &new_file)
        })
        .collect();

    PostWithFiles::new(post, files)
}

fn send_post_created_event(ws: &Ws, data: &PostWithFiles) {
    let json = json!({
        "event": "post_created",
        "data": { "item": data },
    })
    .to_string();
    ws.send_to_all(&json);
}

#[post("/", format = "json", data = "<data>")]
pub fn create_post_json(
    data: Json<CreatePostJson>,
    conn: ChatDbConn,
    ws: State<Ws>,
) -> Created<Json<PostResponse>> {
    let data = create_post(conn, &data.name, &data.message, Vec::new());
    send_post_created_event(&ws, &data);

    let location = format!("/api/v1/posts/{}", data.id);
    Created(location, Some(Json(PostResponse { item: data })))
}

#[post("/", data = "<data>", rank = 1)]
pub fn create_post_form(data: Form<CreatePostForm>, conn: ChatDbConn, ws: State<Ws>) -> Redirect {
    let data = create_post(conn, &data.name, &data.message, Vec::new());
    send_post_created_event(&ws, &data);

    Redirect::found("/")
}

#[post("/", data = "<data>", rank = 2)]
pub fn create_post_multipart(
    data: CreatePostMultipart,
    conn: ChatDbConn,
    ws: State<Ws>,
) -> Redirect {
    let data = create_post(conn, &data.name, &data.message, data.files);
    send_post_created_event(&ws, &data);

    Redirect::found("/")
}

#[get("/?<before_id>", format = "json")]
pub fn get_post_list(conn: ChatDbConn, before_id: Option<i32>) -> Json<PostListResponse> {
    let posts = match before_id {
        Some(before_id) => PostRepository::get_before(&*conn, before_id),
        None => PostRepository::get_latest(&*conn),
    };

    let files = FileRepository::get_belonging_to_posts(&*conn, &posts);
    let data = posts
        .into_iter()
        .zip(files)
        .map(|(post, files)| PostWithFiles::new(post, files))
        .collect();

    Json(PostListResponse { items: data })
}

#[get("/<post_id>", format = "json")]
pub fn get_post(conn: ChatDbConn, post_id: i32) -> Option<Json<PostResponse>> {
    let post = PostRepository::get_one(&*conn, post_id);
    post.map(|post| {
        let files = Vec::new();
        let data = PostWithFiles::new(post, files);

        Json(PostResponse { item: data })
    })
}
