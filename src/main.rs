#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use rocket_dyn_templates::Template;

pub mod db;
pub mod models;
pub mod schema;
pub mod services;
pub mod verify;

use self::services::{create_post::*, delete_post::*, files::*, index::*, post::*};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, post, create_post, delete_post])
        .attach(Template::fairing())
        .mount("/static", routes![files])
}
