use rocket_contrib::Template;
use tera::Context;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::response::Failure;

use form::LoginForm;
use rocket::http::Cookies;
use models::User;
use diesel::prelude::*;
use schema::{users, users::dsl::*};
use rocket::http::Status;
use pg_pool::DbConn;
use rocket::http::Cookie;
use crypto::sha3::Sha3;


#[get("/login")]
fn admin_login() -> Template {
    let context = Context::new();
    Template::render("admin/login", &context)
}


#[post("/login", data = "<user>")]
fn admin_authentication(user: Form<LoginForm>, conn: DbConn, mut cookies: Cookies) -> Result<Redirect, Failure> {
    let user_form = user.get();
    let fetched = users::table.filter(username.eq(&user_form.username)).first::<User>(&*conn);
    if let Err(_) = fetched {
        return Err(Failure(Status::Unauthorized))
    }
    let user: User = fetched.unwrap();

    if !user.authenticated(user_form.password.as_str()) {
        return Err(Failure(Status::Unauthorized))
    }

    cookies.add_private(Cookie::new("LOG_SESSION", user.username));
    cookies.add_private(Cookie::new("LOG_ADMIN", "1"));

    Ok(Redirect::to("/admin/login"))
}