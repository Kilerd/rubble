use rocket_contrib::Template;
use tera::Context;
use rocket::response::Redirect;
use rocket::response::Failure;
use rocket::request::Form;

#[derive_FromForm]
#[derive(Debug)]
struct LoginForm {
    username: String,
    password: String,
}

#[get("/login")]
fn admin_login() -> Template {
    let context = Context::new();
    Template::render("admin/login", &context)
}


#[post("/login", data = "<user>")]
fn admin_authentication(user: Form<LoginForm>) -> Result<Redirect, Failure> {
    println!("{:?}", user);
    Ok(Redirect::to("/admin/login"))
}