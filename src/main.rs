#![feature(proc_macro_hygiene, decl_macro, custom_attribute, plugin)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{
    middleware::{cors::Cors, Logger},
    web, App, HttpServer,
};

use dotenv::dotenv;

use crate::pg_pool::database_pool_establish;
use std::rc::Rc;
use std::sync::Arc;
use tera::compile_templates;

mod guard;
mod models;
mod pg_pool;
mod request;
mod response;
mod routers;
mod schema;
mod template;
mod view;

embed_migrations!();

fn main() -> std::io::Result<()> {
    dotenv().ok();
    //    let sys = actix::System::new("lemmy");

    let database_url = std::env::var("DATABASE_URL").expect("database_url must be set");
    let pool = database_pool_establish(&database_url);

    let tera = Arc::new(compile_templates!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/templates/**/*.html"
    )));

    embedded_migrations::run(&pool.get().expect("cannot get connection"));

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(tera.clone())
            .wrap(Logger::default())
            .wrap(Cors::default())
            .service(routers::article::homepage)
            .service(routers::article::single_article)
            .service(actix_files::Files::new(
                "/statics",
                "./templates/resources/",
            ))
            .service(routers::article::get_article_by_url)
    })
    .bind(("127.0.0.1", 8000))?
    .system_exit()
    .run()
}
