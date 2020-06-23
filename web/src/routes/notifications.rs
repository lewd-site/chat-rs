use crate::routes::posts::Authenticated;
use crate::routes::types::{NotificationWithPost, PostWithFiles};
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
pub enum ReadNotificationResponse {
    Updated(Json<NotificationJson>),
    NotFound(NotFound<Json<ErrorJson>>),
}

impl ReadNotificationResponse {
    fn updated(notification: NotificationWithPost) -> ReadNotificationResponse {
        let json = Json(NotificationJson { item: notification });
        ReadNotificationResponse::Updated(json)
    }

    fn not_found(error: &str) -> ReadNotificationResponse {
        let message = String::from(error);
        let json = Json(ErrorJson { message });
        ReadNotificationResponse::NotFound(NotFound(json))
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
) -> ReadNotificationResponse {
    let uuid = auth.get_uuid();
    match NotificationRepository::get_one_for_user(&conn, id, &uuid) {
        Some((notification, post)) => {
            let notification = NotificationRepository::update_read(&conn, &notification, true);
            let files = FileRepository::get_belonging_to_post(&conn, &post);
            let post = PostWithFiles::new(post, files);
            let notification = NotificationWithPost::new(notification, post);
            ReadNotificationResponse::updated(notification)
        }
        None => ReadNotificationResponse::not_found("Not found"),
    }
}
