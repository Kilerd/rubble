#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate pulldown_cmark;
extern crate r2d2;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate tera;

use diesel::prelude::*;
use dotenv::dotenv;
use models::Post;
use pg_pool::DbConn;
use pulldown_cmark::{html, Parser};
use rocket::http::Status;
use rocket::response::Failure;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use schema::posts;
use schema::posts::dsl::*;
use std::path::Path;
use std::path::PathBuf;
use tera::Context;

mod pg_pool;
mod schema;
mod models;
mod admin;


#[get("/")]
fn index(conn: DbConn) -> Template {
    let mut context = Context::new();

    let result = posts.filter(published.eq(true)).load::<Post>(&*conn).expect("cannot load posts");
    context.insert("posts", &result);

    Template::render("index", &context)
}

#[get("/archives/<archives_id>")]
fn single_archives(conn: DbConn, archives_id: i32) -> Result<Template, Failure> {
    let mut context = Context::new();

    let result: Result<_, _> = posts.find(archives_id).first::<Post>(&*conn);

    if let Err(_err) = result {
        return Err(Failure(Status::NotFound));
    }

    let post = result.unwrap();

    let parser = Parser::new(&post.body);

    let mut content_buf = String::new();
    html::push_html(&mut content_buf, parser);

    context.insert("post", &post);
    context.insert("content", &content_buf);

    Ok(Template::render("archives", &context))
}

#[get("/statics/<file..>")]
fn static_content(file: PathBuf) -> Result<NamedFile, Failure> {
    let path = Path::new("static/resources/").join(file);
    let result = NamedFile::open(&path);
    if let Ok(file) = result {
        Ok(file)
    } else {
        Err(Failure(Status::NotFound))
    }
}

#[catch(404)]
fn not_found_catcher() -> String {
    "not found".to_string()
}


fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("database_url must be set");

    rocket::ignite()
        .catch(catchers![not_found_catcher])
        .manage(pg_pool::init(&database_url))
        .mount("/", routes![index, single_archives, static_content])
        .mount("/admin", routes![admin::admin_login, admin::admin_authentication])
        .attach(Template::fairing())
        .launch();
}
