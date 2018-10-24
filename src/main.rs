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
use routers::{admin, article, catacher};

mod models;
mod response;
mod pg_pool;
mod schema;
mod routers;
mod request;
mod guard;

fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("database_url must be set");

    rocket::ignite()
        .catch(catchers![catacher::not_found_catcher])
        .manage(pg_pool::init(&database_url))
        .mount("/", routes![article::index, article::single_article, article::get_article_by_url, article::static_content])
        .mount("/admin", routes![
            admin::admin_login,
            admin::admin_authentication,
            admin::admin_index,
            admin::article_edit,
            admin::save_article,
            admin::article_creation,
            admin::change_password,
            admin::change_setting
        ])
        .attach(Template::fairing())
        .launch();
}
