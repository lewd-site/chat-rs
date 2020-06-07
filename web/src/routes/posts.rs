use crate::ws::Ws;
use crate::ChatDbConn;
use data::models::posts::Post;
use data::repositories::posts::PostRepository;
use rocket::request::Form;
use rocket::response::status::Created;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, FromForm)]
pub struct CreatePostRequest {
    name: String,
    message: String,
}

#[derive(Serialize)]
pub struct PostListResponse {
    items: Vec<Post>,
}

#[derive(Serialize)]
pub struct PostResponse {
    item: Post,
}

#[post("/", format = "json", data = "<data>")]
pub fn create_post(
    data: Json<CreatePostRequest>,
    conn: ChatDbConn,
    ws: State<Ws>,
) -> Created<Json<PostResponse>> {
    let new_post = Post::new(&data.name, &data.message);
    let post = PostRepository::create(&*conn, &new_post);

    let json = json!({
        "event": "post_created",
        "data": { "item": post },
    })
    .to_string();
    ws.send_to_all(&json);

    let location = format!("/api/v1/posts/{}", post.id);
    Created(location, Some(Json(PostResponse { item: post })))
}

#[post("/", data = "<data>", rank = 1)]
pub fn create_post_form(
    data: Form<CreatePostRequest>,
    conn: ChatDbConn,
    ws: State<Ws>,
) -> Redirect {
    let new_post = Post::new(&data.name, &data.message);
    let post = PostRepository::create(&*conn, &new_post);

    let json = json!({
        "event": "post_created",
        "data": { "item": post },
    })
    .to_string();
    ws.send_to_all(&json);

    Redirect::found("/")
}

#[get("/", format = "json")]
pub fn get_post_list(conn: ChatDbConn) -> Json<PostListResponse> {
    let posts = PostRepository::get_latest(&*conn);
    Json(PostListResponse { items: posts })
}

#[get("/<post_id>", format = "json")]
pub fn get_post(conn: ChatDbConn, post_id: i32) -> Option<Json<PostResponse>> {
    let post = PostRepository::get_one(&*conn, post_id);
    post.map(|post| Json(PostResponse { item: post }))
}
