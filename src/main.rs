#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate diesel;
extern crate dotenv;
extern crate r2d2;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

mod pg_pool;
mod schema;
use dotenv::dotenv;


#[get("/")]
fn index() -> String {
    format!("hello world")
}

fn main() {
    dotenv().ok();
    let database_url = std::env::var("database_url").expect("database_url must be set");

    rocket::ignite()
        .manage(pg_pool::init(&database_url))
        .mount("/",routes![index])
        .launch();
}
