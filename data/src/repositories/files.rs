use crate::models::files::{File, NewFile};
use crate::models::posts::Post;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct FileRepository();

impl FileRepository {
    pub fn get_belonging_to_post(conn: &PgConnection, post: &Post) -> Vec<File> {
        File::belonging_to(post).load(conn).unwrap()
    }

    pub fn get_belonging_to_posts(conn: &PgConnection, posts: &Vec<Post>) -> Vec<Vec<File>> {
        File::belonging_to(posts)
            .load(conn)
            .unwrap()
            .grouped_by(&posts)
    }

    pub fn get_one_by_md5(conn: &PgConnection, hash: &str) -> Option<File> {
        use crate::schema::files::dsl::*;

        let items: Vec<File> = files.filter(md5.eq(hash)).limit(1).load(conn).unwrap();

        items.into_iter().next()
    }

    pub fn get_latest_for_user(conn: &PgConnection, uuid: &str) -> Vec<File> {
        use crate::schema::files::dsl::*;
        use crate::schema::posts;

        files
            .inner_join(posts::table)
            .select(files::all_columns())
            .filter(posts::user_uuid.eq(uuid))
            .order(posts::created_at.desc())
            .limit(100)
            .load(conn)
            .unwrap()
    }

    pub fn create(conn: &PgConnection, file: &NewFile) -> File {
        use crate::schema::files::dsl::*;

        diesel::insert_into(files)
            .values(file)
            .get_result(conn)
            .unwrap()
    }
}
