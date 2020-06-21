use crate::models::notifications::{NewNotification, Notification};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct NotificationRepository();

impl NotificationRepository {
    pub fn get_all_for_user(conn: &PgConnection, uuid: &str) -> Vec<Notification> {
        use crate::schema::notifications::dsl::*;

        let mut items = notifications
            .filter(user_uuid.eq(uuid))
            .order(id.desc())
            .limit(20)
            .load(conn)
            .unwrap();

        items.reverse();
        items
    }

    pub fn create(conn: &PgConnection, notification: &NewNotification) -> Notification {
        use crate::schema::notifications::dsl::*;

        diesel::insert_into(notifications)
            .values(notification)
            .get_result(conn)
            .unwrap()
    }
}
