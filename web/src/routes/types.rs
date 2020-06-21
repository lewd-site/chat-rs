use chrono::prelude::*;
use data::models::files::File;
use data::models::message_parser::{MessageParser, Segment};
use data::models::notifications::Notification;
use data::models::posts::Post;
use serde::Serialize;

#[derive(Serialize)]
pub struct PostWithFiles {
    pub id: i32,
    pub name: String,
    pub tripcode: String,
    pub message_raw: String,
    pub message: Vec<Segment>,
    pub created_at: NaiveDateTime,
    pub files: Vec<File>,
    pub user_uuid: Option<String>,
}

impl PostWithFiles {
    pub fn new(post: Post, files: Vec<File>) -> PostWithFiles {
        PostWithFiles {
            id: post.id,
            name: post.name,
            tripcode: post.tripcode,
            message_raw: post.message.clone(),
            message: MessageParser::str_to_segments(&post.message),
            created_at: post.created_at,
            files,
            user_uuid: post.user_uuid,
        }
    }
}

#[derive(Serialize)]
pub struct NotificationWithPost {
    pub id: i32,
    pub user_uuid: String,
    pub read: bool,
    pub post: PostWithFiles,
}

impl NotificationWithPost {
    pub fn new(notification: Notification, post: PostWithFiles) -> NotificationWithPost {
        NotificationWithPost {
            id: notification.id,
            user_uuid: notification.user_uuid,
            read: notification.read,
            post,
        }
    }
}
