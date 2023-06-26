#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod models;
mod repositories;
mod schema;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
