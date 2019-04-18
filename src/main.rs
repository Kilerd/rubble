#![feature(proc_macro_hygiene, decl_macro, custom_attribute, plugin)]

#[macro_use]
extern crate diesel;

use actix_web::{
    middleware::{cors::Cors, Logger},
    App, HttpServer,
};
use diesel_migrations::embed_migrations;
use dotenv::dotenv;

use crate::pg_pool::database_pool_establish;

mod guard;
mod models;
mod modelss;
mod pg_pool;
mod request;
mod response;
mod routers;
mod schema;

embed_migrations!();

fn main() -> std::io::Result<()> {
    let sys = actix::System::new("lemmy");
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("database_url must be set");
    let pool = database_pool_establish(&database_url);

    embed_migrations::run(&poll.get());

    HttpServer::new(move || {
        App::new()
            .data(pool)
            .wrap(Logger::default())
            .wrap(Cors::default())
    })
    .bind(("127.0.0.1", 8000))?
    .system_exit()
    .start();

    sys.run();
    Ok(())
}
