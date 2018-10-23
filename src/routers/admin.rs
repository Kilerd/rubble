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
use schema::{users, users::dsl::*, articles};
use tera::Context;
use models::Article;
use response::ArticleResponse;
use request::ArticleEditForm;
use diesel;


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
    cookies.add_private(Cookie::new("LOG_ID", user.id.to_string()));
    cookies.add_private(Cookie::new("LOG_ADMIN", "1"));

    Ok(Redirect::to("/admin"))
}


#[get("/")]
fn admin_index(admin: Admin, conn: DbConn) -> Template {
    let mut context = Context::new();

    let articles = Article::load_all(true, &conn);

    context.insert("admin", &admin);
    context.insert("articles", &articles);
    Template::render("admin/index", &context)
}


#[get("/article/new")]
fn article_creation(admin: Admin) -> Result<Template, Failure> {
    let mut context = Context::new();
    Ok(Template::render("admin/edit", context))
}


#[get("/article/<article_id>")]
fn article_edit(admin: Admin, conn: DbConn, article_id: i32) -> Result<Template, Failure>{
    let mut context = Context::new();
    let fetched_article = Article::find(article_id, &conn);

    if let Err(_err) = fetched_article {
        return Err(Failure(Status::NotFound));
    }

    let article: Article = fetched_article.unwrap();

    context.insert("article", &article);
    Ok(Template::render("admin/edit", context))
}

#[post("/article", data="<article>")]
fn save_article(admin:Admin, conn: DbConn, article: Form<ArticleEditForm>) -> Result<Flash<Redirect>, Failure> {
    let article = Article::form_article_edit_form(article.get(), admin.id);
    let fetched_article: QueryResult<Article> = if article.id == -1 {
        // insert a new record
        diesel::insert_into(articles::table).values(&article).get_result(&*conn)

    } else {
        // update exist one
        // TODO only update record with id
        diesel::update(articles::table.find(article.id)).set(&article).get_result(&*conn)
    };

    Ok(Flash::new(Redirect::to("/admin"), "success", "created"))
}