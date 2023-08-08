mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{create_user, get_user, get_users};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/api/v1", routes![create_user])
        .mount("/api/v1", routes![get_user])
        .mount("/api/v1", routes![get_users])
}