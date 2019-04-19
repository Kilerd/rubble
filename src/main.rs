#![feature(proc_macro_hygiene, decl_macro, custom_attribute, plugin)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{
    middleware::{
        cors::Cors,
        identity::{CookieIdentityPolicy, Identity, IdentityService},
        Logger,
    },
    web, App, HttpServer,
};

use dotenv::dotenv;

use crate::pg_pool::database_pool_establish;
use actix_web::web::route;
use rand::prelude::*;
use std::rc::Rc;
use std::sync::Arc;
use tera::compile_templates;
use time::Duration;

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
    pretty_env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("database_url must be set");
    let pool = database_pool_establish(&database_url);

    embedded_migrations::run(&pool.get().expect("cannot get connection"));

    let tera = Arc::new(compile_templates!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/templates/**/*.html"
    )));

    let random_cookie_key: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(tera.clone())
            .wrap(Logger::default())
            .wrap(Cors::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&random_cookie_key)
                    .name("auth-cookie")
                    .secure(false)
                    .max_age(Duration::days(3)),
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
                    .service(routers::admin::article_save)
                    .service(routers::admin::article_edit)
                    .service(routers::admin::article_deletion)
                    .service(routers::admin::change_password)
                    .service(routers::admin::change_setting),
            )
            .service(routers::rss::rss_page)
            .service(routers::article::get_article_by_url)
    })
    .bind(("127.0.0.1", 8000))?
    .system_exit()
    .run()
}
