use crate::models::notifications::{NewNotification, Notification};
use crate::models::posts::Post;
use crate::schema::posts;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct NotificationRepository();

impl NotificationRepository {
    pub fn get_all_for_user(conn: &PgConnection, uuid: &str) -> Vec<(Notification, Post)> {
        use crate::schema::notifications::dsl::*;

        let items = notifications
            .inner_join(posts::table)
            .filter(user_uuid.eq(uuid))
            .order(id.desc())
            .limit(20)
            .load(conn)
            .unwrap();

        items
    }

    pub fn get_one_for_user(
        conn: &PgConnection,
        notification_id: i32,
        uuid: &str,
    ) -> Option<(Notification, Post)> {
        use crate::schema::notifications::dsl::*;

        let items = notifications
            .inner_join(posts::table)
            .filter(id.eq(notification_id))
            .filter(user_uuid.eq(uuid))
            .order(id.desc())
            .limit(1)
            .load(conn)
            .unwrap();

        items.into_iter().next()
    }

    pub fn create(conn: &PgConnection, notification: &NewNotification) -> Notification {
        use crate::schema::notifications::dsl::*;

        diesel::insert_into(notifications)
            .values(notification)
            .get_result(conn)
            .unwrap()
    }

    pub fn update_read(
        conn: &PgConnection,
        notification: &Notification,
        notification_read: bool,
    ) -> Notification {
        use crate::schema::notifications::dsl::*;

        diesel::update(notification)
            .set(read.eq(notification_read))
            .get_result(conn)
            .unwrap()
    }
}
