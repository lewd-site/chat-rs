use crate::routes::types::{Authenticated, NotificationWithPost, PostWithFiles};
use crate::ChatDbConn;
use data::repositories::files::FileRepository;
use data::repositories::notifications::NotificationRepository;
use rocket::response::status::NotFound;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorJson {
    message: String,
}

#[derive(Serialize)]
pub struct NotificationJson {
    item: NotificationWithPost,
}

#[derive(Responder)]
pub enum UpdateNotificationResponse {
    Updated(Json<NotificationJson>),
    NotFound(NotFound<Json<ErrorJson>>),
}

impl UpdateNotificationResponse {
    fn ok(notification: NotificationWithPost) -> UpdateNotificationResponse {
        let json = Json(NotificationJson { item: notification });
        UpdateNotificationResponse::Updated(json)
    }

    fn not_found(error: &str) -> UpdateNotificationResponse {
        let message = String::from(error);
        let json = Json(ErrorJson { message });
        UpdateNotificationResponse::NotFound(NotFound(json))
    }
}

#[derive(Serialize)]
pub struct NotificationListResponse {
    items: Vec<NotificationWithPost>,
}

#[get("/", format = "json")]
pub fn get_notifications(auth: Authenticated, conn: ChatDbConn) -> Json<NotificationListResponse> {
    let uuid = auth.get_uuid();
    let notifications = NotificationRepository::get_all_for_user(&*conn, &uuid);

    let (notifications, posts) = notifications.into_iter().fold(
        (Vec::new(), Vec::new()),
        |(mut notifications, mut posts), (notification, post)| {
            notifications.push(notification);
            posts.push(post);
            (notifications, posts)
        },
    );

    let files = FileRepository::get_belonging_to_posts(&*conn, &posts);

    let data = notifications
        .into_iter()
        .zip(posts)
        .zip(files)
        .map(|((notification, post), files)| {
            NotificationWithPost::new(notification, PostWithFiles::new(post, files))
        })
        .collect();

    Json(NotificationListResponse { items: data })
}

#[post("/<id>/read")]
pub fn read_notification(
    auth: Authenticated,
    conn: ChatDbConn,
    id: i32,
) -> UpdateNotificationResponse {
    let uuid = auth.get_uuid();
    match NotificationRepository::get_one_for_user(&conn, id, &uuid) {
        Some((notification, post)) => {
            let notification = NotificationRepository::update_read(&conn, &notification, true);
            let files = FileRepository::get_belonging_to_post(&conn, &post);
            let post = PostWithFiles::new(post, files);
            let notification = NotificationWithPost::new(notification, post);
            UpdateNotificationResponse::ok(notification)
        }
        None => UpdateNotificationResponse::not_found("Not found"),
    }
}

#[delete("/<id>")]
pub fn delete_notification(
    auth: Authenticated,
    conn: ChatDbConn,
    id: i32,
) -> UpdateNotificationResponse {
    let uuid = auth.get_uuid();
    match NotificationRepository::get_one_for_user(&conn, id, &uuid) {
        Some((notification, post)) => {
            let notification = NotificationRepository::delete(&conn, &notification);
            let files = FileRepository::get_belonging_to_post(&conn, &post);
            let post = PostWithFiles::new(post, files);
            let notification = NotificationWithPost::new(notification, post);
            UpdateNotificationResponse::ok(notification)
        }
        None => UpdateNotificationResponse::not_found("Not found"),
    }
}
