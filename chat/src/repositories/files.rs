use crate::models::files::{File, NewFile};
use crate::models::files::{NewUserFavoriteFile, UserFavoriteFile};
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
        use diesel::dsl::*;

        let subquery = files
            .inner_join(posts::table)
            .select(max(id))
            .filter(posts::user_uuid.eq(uuid))
            .group_by(md5);

        files
            .filter(id.nullable().eq_any(subquery))
            .order(id.desc())
            .limit(100)
            .load(conn)
            .unwrap()
    }

    pub fn get_favorites_for_user(conn: &PgConnection, uuid: &str) -> Vec<File> {
        use crate::schema::files::dsl::*;
        use crate::schema::user_favorite_files;

        user_favorite_files::table
            .inner_join(files)
            .select(files::all_columns())
            .filter(user_favorite_files::user_uuid.eq(uuid))
            .order(user_favorite_files::id.desc())
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

    pub fn get_favorite_by_md5(
        conn: &PgConnection,
        hash: &str,
        uuid: &str,
    ) -> Option<UserFavoriteFile> {
        use crate::schema::files::dsl::*;
        use crate::schema::user_favorite_files;

        let items: Vec<UserFavoriteFile> = user_favorite_files::table
            .inner_join(files)
            .select(user_favorite_files::table::all_columns())
            .filter(md5.eq(hash))
            .filter(user_favorite_files::user_uuid.eq(uuid))
            .limit(1)
            .load(conn)
            .unwrap();

        items.into_iter().next()
    }

    pub fn create_favorite(conn: &PgConnection, file: &NewUserFavoriteFile) -> UserFavoriteFile {
        use crate::schema::user_favorite_files::dsl::*;

        diesel::insert_into(user_favorite_files)
            .values(file)
            .get_result(conn)
            .unwrap()
    }

    pub fn delete_favorite(conn: &PgConnection, file: &UserFavoriteFile) -> UserFavoriteFile {
        use crate::schema::user_favorite_files::dsl::*;

        let source = user_favorite_files.filter(id.eq(file.id));
        diesel::delete(source).get_result(conn).unwrap()
    }
}
