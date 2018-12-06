#![feature(custom_attribute, plugin)]
#![plugin(rocket_codegen)]
extern crate chrono;
extern crate crypto;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate juniper_codegen;
extern crate juniper_rocket;
extern crate pulldown_cmark;
extern crate r2d2;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tera;

extern crate rand;

use dotenv::dotenv;
use rocket_contrib::Template;

mod guard;
mod models;
mod pg_pool;
mod request;
mod response;
mod routers;
mod schema;
mod graphql;

use std::collections::HashMap;

use crate::graphql::{Schema, Query, Mutation};

fn main() {
    use crate::routers::{admin, article, catacher, graphql, rss};
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("database_url must be set");

    rocket::ignite()
        .catch(catchers![
            catacher::not_found_catcher,
            catacher::unauthorized,
            ])
        .manage(pg_pool::init(&database_url))
        .manage(Schema::new(Query{}, Mutation{}))
        .mount("/", routes![
            article::index,
            article::single_article,
            article::get_article_by_url,
            article::static_content,

            rss::rss,

            graphql::graphql_authorization,
            graphql::graphiql,
            graphql::get_graphql_handler,
            graphql::post_graphql_handler
            ])
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