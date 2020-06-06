#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod routes;

use diesel::pg::PgConnection;
use rocket_contrib::serve::StaticFiles;
use routes::posts;

#[database("pgsql_chat")]
pub struct ChatDbConn(PgConnection);

fn rocket() -> rocket::Rocket {
    let static_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../www");

    rocket::ignite()
        .mount(
            "/api/v1/posts",
            routes![
                posts::create_post,
                posts::create_post_form,
                posts::get_post_list,
                posts::get_post
            ],
        )
        .mount("/", StaticFiles::from(static_dir))
        .attach(ChatDbConn::fairing())
}

fn main() {
    rocket().launch();
}
