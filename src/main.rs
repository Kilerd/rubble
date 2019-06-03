extern crate openssl;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use actix_web::{
    middleware::{
        cors::Cors,
        identity::{CookieIdentityPolicy, Identity, IdentityService},
        Logger, NormalizePath,
    },
    web, App, HttpServer,
};

use dotenv::dotenv;

use crate::data::RubbleData;
use crate::pg_pool::database_pool_establish;
use actix_web::web::{FormConfig, JsonConfig};
use lazy_static::lazy_static;
use std::sync::Arc;
use tera::compile_templates;
use time::Duration;

mod data;
mod models;
mod pg_pool;
mod routers;
mod schema;
mod utils;
mod view;

embed_migrations!();

lazy_static! {
    static ref RANDOM_TOKEN_KEY: Vec<u8> = (0..32).map(|_| 0).collect();
}

fn main() {
    dotenv().ok();
    let sys = actix::System::new("rubble");
    pretty_env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("database_url must be set");

    let data = RubbleData {
        pool: database_pool_establish(&database_url),
        tera: Arc::new(compile_templates!("templates/**/*.html")),
    };

    embedded_migrations::run(&data.pool.get().expect("cannot get connection"))
        .expect("panic on embedded database migration");

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .data(JsonConfig::default().limit(256_000))
            .data(FormConfig::default().limit(256_000))
            .wrap(Logger::default())
            .wrap(Cors::default())
            .wrap(NormalizePath)
            .service(routers::routes())
    })
    .bind(("0.0.0.0", 8000))
    .unwrap()
    .system_exit()
    .start();

    sys.run();
}
