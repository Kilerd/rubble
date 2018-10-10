#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate tera;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

extern crate pulldown_cmark;

mod pg_pool;
mod schema;
mod models;

use dotenv::dotenv;

use pg_pool::DbConn;

use diesel::prelude::*;
use schema::posts::dsl::*;
use schema::posts;
use models::Post;
use tera::Context;
use rocket_contrib::Template;
use rocket::response::Redirect;

use pulldown_cmark::{html, Parser};
use rocket::response::Failure;
use rocket::http::Status;


#[get("/")]
fn index(conn: DbConn) -> Template {
    let mut context = Context::new();

    let result = posts.filter(published.eq(true)).load::<Post>(&*conn).expect("cannot load posts");
    context.insert("posts", &result);

    println!("{:?}", result);

    Template::render("index", &context)
}

#[get("/archives/<archives_id>")]
fn single_archives(conn: DbConn, archives_id: i32) -> Result<Template, Failure> {
    let mut context = Context::new();

    let result: Result<_,_> = posts.find(archives_id).first::<Post>(&*conn);

    if let Err(err) = result {
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

#[catch(404)]
fn not_found_catcher() -> String{
    "not found".to_string()
}


fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("database_url must be set");

    rocket::ignite()
        .catch(catchers![not_found_catcher])
        .manage(pg_pool::init(&database_url))
        .mount("/", routes![index, single_archives])
        .attach(Template::fairing())
        .launch();
}
