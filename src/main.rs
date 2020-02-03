#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives;
#[macro_use]
extern crate diesel_migrations;
extern crate openssl;

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    middleware::{Logger, NormalizePath},
    web::{FormConfig, JsonConfig},
    App, HttpServer,
};
use once_cell::sync::Lazy;
use tera::Tera;

use dotenv::dotenv;

use crate::{data::RubbleData, pg_pool::database_pool_establish};

mod data;
mod error;
mod models;
mod pg_pool;
mod routers;
mod schema;
mod utils;

embed_migrations!();

const TOKEN_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
    std::env::var("TOKEN_KEY")
        .map(|token| Vec::from(token.as_bytes()))
        .unwrap_or_else(|_| (0..32).into_iter().map(|_| rand::random::<u8>()).collect())
});

#[actix_rt::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("database_url must be set");

    let data = RubbleData {
        pool: database_pool_establish(&database_url),
        tera: Arc::new(Tera::new("templates/**/*.html").unwrap()),
    };

    embedded_migrations::run(&data.pool.get().expect("cannot get connection"))
        .expect("panic on embedded database migration");

    println!("rubble is listening on 127.0.0.1:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .data(data.clone())
            .data(JsonConfig::default().limit(256_000))
            .data(FormConfig::default().limit(256_000))
            .wrap(Logger::default())
            .wrap(Cors::default())
            .wrap(NormalizePath)
            .configure(routers::routes)
    })
    .bind(("0.0.0.0", 8000))
    .unwrap()
    .run()
    .await
    .unwrap()
}
