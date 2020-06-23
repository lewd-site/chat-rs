#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

mod requests;
mod routes;
mod ws;

use diesel::pg::PgConnection;
use rocket_contrib::serve::StaticFiles;
use routes::{notifications, posts, thumbnails};
use ws::Ws;

#[database("pgsql_chat")]
pub struct ChatDbConn(PgConnection);

fn rocket() -> rocket::Rocket {
    let static_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../www");

    rocket::ignite()
        .mount(
            "/api/v1/posts",
            routes![
                posts::create_post_json,
                posts::create_post_form,
                posts::create_post_multipart,
                posts::get_post_list,
                posts::get_post,
            ],
        )
        .mount(
            "/api/v1/notifications",
            routes![
                notifications::get_notifications,
                notifications::read_notification
            ],
        )
        .mount("/thumb", routes![thumbnails::get_thumbnail])
        .mount("/", StaticFiles::from(static_dir))
        .attach(ChatDbConn::fairing())
}

fn main() {
    let rocket = rocket();

    let ws_address = rocket.config().get_str("ws_address").unwrap_or("127.0.0.1");
    let ws_port = rocket.config().get_int("ws_port").unwrap_or(8001);
    let ws_address = format!("{}:{}", ws_address, ws_port);
    let ws = Ws::new(&ws_address);
    let rocket = rocket.manage(ws);

    rocket.launch();
}
