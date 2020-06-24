use crate::requests::{CreatePostMultipart, UploadedFile};
use crate::routes::types::{NotificationWithPost, PostWithFiles};
use crate::ws::Ws;
use crate::ChatDbConn;
use data::models::files::File;
use data::models::notifications::Notification;
use data::models::posts::Post;
use data::repositories::files::FileRepository;
use data::repositories::notifications::NotificationRepository;
use data::repositories::posts::PostRepository;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use rocket::http::Status;
use rocket::request::{Form, FromRequest, Outcome};
use rocket::response::status::Created;
use rocket::response::Redirect;
use rocket::{Request, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::fs;

lazy_static! {
    static ref DECODING_KEY_RAW: Vec<u8> =
        fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../public.pem")).unwrap();
    static ref DECODING_KEY: DecodingKey<'static> =
        DecodingKey::from_rsa_pem(&DECODING_KEY_RAW).unwrap();
}

pub struct BearerToken<'a>(&'a str);

impl<'a, 'r> FromRequest<'a, 'r> for BearerToken<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        let headers = request.headers();
        let header = headers.get("Authorization").last();
        match header {
            Some(header) => {
                let mut header = header.splitn(2, ' ');
                let auth_type = header.next().unwrap();
                if auth_type != "Bearer" {
                    return Outcome::Forward(());
                }

                let token = header.next().unwrap();
                Outcome::Success(BearerToken(token))
            }
            None => Outcome::Forward(()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_uuid: String,
    pub user_name: String,
    pub user_email: String,
    pub iat: u64,
    pub nbf: u64,
    pub exp: u64,
}

pub struct Authenticated(Claims);

impl Authenticated {
    pub fn get_uuid(&self) -> String {
        self.0.user_uuid.clone()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Authenticated {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        let BearerToken(token) = request.guard::<BearerToken>()?;

        let validation = Validation {
            algorithms: vec![Algorithm::RS256],
            validate_nbf: true,
            validate_exp: true,
            leeway: 60,
            aud: None,
            iss: None,
            sub: None,
        };

        match jsonwebtoken::decode::<Claims>(token, &*DECODING_KEY, &validation) {
            Ok(token_data) => Outcome::Success(Authenticated(token_data.claims)),
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

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
    user_uuid: Option<&str>,
) -> (PostWithFiles, Vec<NotificationWithPost>) {
    let new_post = Post::new(name, message, user_uuid).unwrap();
    let post = PostRepository::create(&*conn, &new_post);
    let files = files
        .into_iter()
        .map(|file| {
            let new_file =
                File::new(file.content_type, file.file_name, file.path, post.id).unwrap();
            FileRepository::create(&*conn, &new_file)
        })
        .collect();

    let post = PostWithFiles::new(post, files);

    // Create notifications for referenced posts.
    let mut ref_links: Vec<i32> = post
        .message
        .iter()
        .filter_map(|s| s.get_ref_link())
        .map(|i| i as i32)
        .collect();

    ref_links.sort_unstable();
    ref_links.dedup();

    let mut uuids: Vec<String> = PostRepository::get_many_by_id(&conn, ref_links)
        .into_iter()
        .filter_map(|ref_post| ref_post.user_uuid)
        .collect();

    uuids.sort_unstable();
    uuids.dedup();

    let notifications = uuids
        .into_iter()
        .map(|uuid| {
            let new_notification = Notification::new(post.id, &uuid, false);
            let notification = NotificationRepository::create(&conn, &new_notification);
            NotificationWithPost::new(notification, post.clone())
        })
        .collect();

    (post, notifications)
}

fn send_post_created_event(ws: &Ws, data: &PostWithFiles) {
    let json = json!({
        "event": "post_created",
        "data": { "item": data },
    })
    .to_string();
    ws.send_to_all(&json);
}

fn send_notification_created_event(ws: &Ws, data: &NotificationWithPost) {
    let json = json!({
        "event": "notification_created",
        "data": { "item": data },
    })
    .to_string();
    ws.send_to_all(&json);
}

#[post("/", format = "json", data = "<data>")]
pub fn create_post_json(
    auth: Authenticated,
    data: Json<CreatePostJson>,
    conn: ChatDbConn,
    ws: State<Ws>,
) -> Created<Json<PostResponse>> {
    let user_uuid = auth.get_uuid();
    let (post, notifications) = create_post(
        conn,
        &data.name,
        &data.message,
        Vec::new(),
        Some(&user_uuid),
    );
    send_post_created_event(&ws, &post);
    for notification in notifications {
        send_notification_created_event(&ws, &notification);
    }

    let location = format!("/api/v1/posts/{}", post.id);
    Created(location, Some(Json(PostResponse { item: post })))
}

#[post("/", data = "<data>", rank = 1)]
pub fn create_post_form(
    auth: Authenticated,
    data: Form<CreatePostForm>,
    conn: ChatDbConn,
    ws: State<Ws>,
) -> Redirect {
    let user_uuid = auth.get_uuid();
    let (post, notifications) = create_post(
        conn,
        &data.name,
        &data.message,
        Vec::new(),
        Some(&user_uuid),
    );
    send_post_created_event(&ws, &post);
    for notification in notifications {
        send_notification_created_event(&ws, &notification);
    }

    Redirect::found("/")
}

#[post("/", data = "<data>", rank = 2)]
pub fn create_post_multipart(
    auth: Authenticated,
    data: CreatePostMultipart,
    conn: ChatDbConn,
    ws: State<Ws>,
) -> Redirect {
    let user_uuid = auth.get_uuid();
    let (post, notifications) = create_post(
        conn,
        &data.name,
        &data.message,
        data.files,
        Some(&user_uuid),
    );
    send_post_created_event(&ws, &post);
    for notification in notifications {
        send_notification_created_event(&ws, &notification);
    }

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
