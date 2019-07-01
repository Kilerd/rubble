extern crate openssl;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives;
#[macro_use]
extern crate diesel_migrations;

use std::sync::Arc;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    middleware::{Logger, NormalizePath},
    web::{FormConfig, JsonConfig},
    App, HttpServer,
};

use actix_cors::Cors;
use tera::compile_templates;
use time::Duration;

use dotenv::dotenv;
use lazy_static::lazy_static;

use crate::{data::RubbleData, pg_pool::database_pool_establish};

mod data;
mod models;
mod pg_pool;
mod routers;
mod schema;
mod utils;
embed_migrations!();

lazy_static! {
    static ref RANDOM_TOKEN_KEY: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
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
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&RANDOM_TOKEN_KEY)
                    .name("auth-cookie")
                    .secure(false)
                    .max_age_time(Duration::days(3)),
            ))
            .service(routers::routes())
    })
    .bind(("0.0.0.0", 8000))
    .unwrap()
    .system_exit()
    .start();

    sys.run().expect("wrong on actix system run")
}
