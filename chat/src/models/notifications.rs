use super::posts::Post;
use crate::schema::notifications;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "notifications"]
pub struct NewNotification {
    pub post_id: i32,
    pub user_uuid: String,
    pub read: bool,
}

#[derive(Identifiable, Queryable, Associations, Serialize)]
#[belongs_to(Post)]
pub struct Notification {
    pub id: i32,
    pub post_id: i32,
    pub user_uuid: String,
    pub read: bool,
}

impl Notification {
    pub fn new(post_id: i32, user_uuid: &str, read: bool) -> NewNotification {
        NewNotification {
            post_id,
            user_uuid: String::from(user_uuid),
            read,
        }
    }
}
