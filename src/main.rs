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
        Logger,
    },
    web, App, HttpServer,
};

use dotenv::dotenv;

use crate::data::RubbleData;
use crate::pg_pool::database_pool_establish;
use actix_web::web::JsonConfig;
use std::sync::Arc;
use tera::compile_templates;
use time::Duration;

mod data;
mod models;
mod pg_pool;
mod routers;
mod schema;
mod view;

embed_migrations!();

fn main() {
    dotenv().ok();
    let sys = actix::System::new("rubble");
    pretty_env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("database_url must be set");
    let random_cookie_key: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();

    let data = RubbleData {
        pool: database_pool_establish(&database_url),
        tera: Arc::new(compile_templates!("templates/**/*.html")),
    };

    embedded_migrations::run(&data.pool.get().expect("cannot get connection"));

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .data(JsonConfig::default().limit(266000))
            .wrap(Logger::default())
            .wrap(Cors::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&random_cookie_key)
                    .name("auth-cookie")
                    .secure(false)
                    .max_age_time(Duration::days(3)),
            ))
            .service(routers::article::homepage)
            .service(routers::article::single_article)
            .service(actix_files::Files::new(
                "/statics",
                "./templates/resources/",
            ))
            .service(routers::admin::redirect_to_admin_panel)
            .service(
                web::scope("/admin/")
                    .service(routers::admin::admin_panel)
                    .service(routers::admin::admin_login)
                    .service(routers::admin::admin_authentication)
                    .service(routers::admin::article_creation)
                    .service(
                        web::resource("/article").route(
                            web::post()
                                .data(web::FormConfig::default().limit(256_000))
                                .to(routers::admin::article_save),
                        ),
                    )
                    .service(routers::admin::article_edit)
                    .service(routers::admin::article_deletion)
                    .service(routers::admin::change_password)
                    .service(routers::admin::change_setting),
            )
            .service(routers::rss::rss_page)
            .service(routers::article::get_article_by_url)
    })
    .bind(("0.0.0.0", 8000))
    .unwrap()
    .system_exit()
    .start();

    sys.run();
}
