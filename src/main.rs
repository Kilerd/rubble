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

mod pg_pool;
mod schema;
mod models;
use dotenv::dotenv;

use pg_pool::DbConn;

use diesel::prelude::*;
use schema::posts::dsl::*;
use schema::posts;
use models::Post;
use tera::{Context};
use rocket_contrib::Template;


#[get("/")]
fn index(conn: DbConn) -> Template {
    let mut context = Context::new();

    let result = posts.load::<Post>(&*conn).expect("cannot load posts");
    context.add("posts", &result);

    println!("{:?}", result);

    Template::render("index", &context)
}


fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("database_url must be set");

    rocket::ignite()
        .manage(pg_pool::init(&database_url))
        .mount("/",routes![index])
        .attach(Template::fairing())
        .launch();
}
