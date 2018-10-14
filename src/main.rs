#![feature(custom_attribute, plugin)]
#![plugin(rocket_codegen)]
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate pulldown_cmark;
extern crate r2d2;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tera;

extern crate crypto;


use dotenv::dotenv;
use rocket_contrib::Template;
use routers::{admin, archive, catacher, rss};

mod models;
mod response;
mod pg_pool;
mod schema;
mod routers;
mod request;

fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("database_url must be set");

    rocket::ignite()
        .catch(catchers![catacher::not_found_catcher])
        .manage(pg_pool::init(&database_url))
        .mount("/", routes![archive::index, archive::single_archives, archive::get_archive_by_url, archive::static_content])
        .mount("/admin", routes![admin::admin_login, admin::admin_authentication, admin::admin_index])
        .attach(Template::fairing())
        .launch();
}
