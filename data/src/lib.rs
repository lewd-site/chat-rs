#[macro_use]
extern crate diesel;

pub mod models;
pub mod repositories;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
