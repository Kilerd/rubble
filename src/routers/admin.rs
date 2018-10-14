use crypto::sha3::Sha3;
use diesel::prelude::*;
use models::User;
use pg_pool::DbConn;
use request::Admin;
use request::LoginForm;
use rocket::http::Cookie;
use rocket::http::Cookies;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Failure;
use rocket::response::Flash;
use rocket::response::Redirect;
use rocket_contrib::Template;
use schema::{users, users::dsl::*};
use tera::Context;
use models::Post;


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
        return Err(Failure(Status::Unauthorized));
    }
    let user: User = fetched.unwrap();

    if !user.authenticated(user_form.password.as_str()) {
        return Err(Failure(Status::Unauthorized));
    }

    cookies.add_private(Cookie::new("LOG_SESSION", user.username));
    cookies.add_private(Cookie::new("LOG_ADMIN", "1"));

    Ok(Redirect::to("/admin"))
}


#[get("/")]
fn admin_index(admin: Admin, conn: DbConn) -> Template {
    let mut context = Context::new();

    let posts = Post::load_all(true, &conn);

    context.insert("admin", &admin);
    context.insert("posts", &posts);
    Template::render("admin/index", &context)
}