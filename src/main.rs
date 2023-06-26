#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket_sync_db_pools::database;

mod models;
mod repositories;
mod routes;
mod schema;

#[database("postgres")]
pub struct DbConn(rocket_sync_db_pools::diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                routes::rustaceans::get_rustaceans,
                routes::rustaceans::view_rustacean,
                routes::rustaceans::create_rustacean,
                routes::rustaceans::update_rustacean,
                routes::rustaceans::delete_rustacean,
            ],
        )
        .attach(DbConn::fairing())
}
