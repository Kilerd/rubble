#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[get("/")]
fn index() -> String {
    format!("hello world")
}

fn main() {
    rocket::ignite().mount("/",routes![index]).launch();
}
