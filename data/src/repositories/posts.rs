use crate::models::posts::{NewPost, Post};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct PostRepository();

impl PostRepository {
    pub fn get_latest(conn: &PgConnection) -> Vec<Post> {
        use crate::schema::posts::dsl::*;

        let mut items = posts
            .order(created_at.desc())
            .limit(100)
            .load(conn)
            .unwrap();

        items.reverse();
        items
    }

    pub fn get_before(conn: &PgConnection, before_id: i32) -> Vec<Post> {
        use crate::schema::posts::dsl::*;

        let mut items = posts
            .filter(id.lt(before_id))
            .order(created_at.desc())
            .limit(100)
            .load(conn)
            .unwrap();

        items.reverse();
        items
    }

    pub fn get_one(conn: &PgConnection, post_id: i32) -> Option<Post> {
        use crate::schema::posts::dsl::*;

        let items: Vec<Post> = posts.filter(id.eq(post_id)).limit(1).load(conn).unwrap();

        items.into_iter().next()
    }

    pub fn create(conn: &PgConnection, post: &NewPost) -> Post {
        use crate::schema::posts::dsl::*;

        diesel::insert_into(posts)
            .values(post)
            .get_result(conn)
            .unwrap()
    }
}
