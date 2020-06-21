use crate::routes::posts::Authenticated;
use crate::routes::types::{NotificationWithPost, PostWithFiles};
use crate::ChatDbConn;
use data::repositories::files::FileRepository;
use data::repositories::notifications::NotificationRepository;
use data::repositories::posts::PostRepository;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct NotificationListResponse {
    items: Vec<NotificationWithPost>,
}

#[get("/", format = "json")]
pub fn get_notifications(auth: Authenticated, conn: ChatDbConn) -> Json<NotificationListResponse> {
    let uuid = auth.get_uuid();
    let notifications = NotificationRepository::get_all_for_user(&*conn, &uuid);

    let post_ids: Vec<i32> = notifications.iter().map(|n| n.post_id).collect();
    let posts = PostRepository::get_many_by_id(&*conn, post_ids);
    let files = FileRepository::get_belonging_to_posts(&*conn, &posts);
    let posts: Vec<PostWithFiles> = posts
        .into_iter()
        .zip(files)
        .map(|(post, files)| PostWithFiles::new(post, files))
        .collect();

    let data = notifications
        .into_iter()
        .zip(posts)
        .map(|(notification, post)| NotificationWithPost::new(notification, post))
        .collect();

    Json(NotificationListResponse { items: data })
}
