//use chrono::NaiveDateTime;
//use chrono::Utc;
//use crate::models::Article;
//use crate::models::SerializeFlashMessage;
//use crate::models::Setting;
//use crate::models::User;
//use crate::pg_pool::DbConn;
//use crate::request::Admin;
//use crate::request::ArticleEditForm;
//use crate::request::LoginForm;
//use crate::request::NewPasswordForm;
//use diesel;
//use diesel::prelude::*;
//use rocket::http::Cookie;
//use rocket::http::Cookies;
//use rocket::http::Status;
//use rocket::request::FlashMessage;
//use rocket::request::Form;
//use rocket::response::Flash;
//use rocket::response::Redirect;
//use tera::Context;
//use rocket_contrib::templates::Template;
//

use crate::models::article::Article;

use crate::models::setting::Setting;
use crate::models::user::User;
use crate::models::CRUD;
use crate::pg_pool::Pool;
use crate::routers::RubbleResponder;
use actix_web::middleware::identity::Identity;
use actix_web::web::Form;
use actix_web::{get, post, web, Either, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Arc;
use tera::{Context, Tera};

#[derive(Deserialize)]
struct LoginForm {
    pub username: String,
    pub password: String,
}

#[get("/admin")]
pub fn redirect_to_admin_panel() -> impl Responder {
    RubbleResponder::Redirect("/admin/panel".into())
}

#[get("/panel")]
pub fn admin_panel(
    id: Identity,
    tera: web::Data<Arc<Tera>>,
    conn: web::Data<Pool>,
) -> impl Responder {
    let connection = conn.get().unwrap();

    if id.identity().is_none() {
        return RubbleResponder::Redirect("/admin/login".into());
    }

    let articles = Article::read(&connection);
    let settings = Setting::load(&connection);

    let admin = User::find_by_username(&*connection, &id.identity().unwrap())
        .expect("cannot found this user");

    let mut context = Context::new();
    context.insert("setting", &settings);
    context.insert("articles", &articles);
    context.insert("admin", &admin);

    RubbleResponder::Html(tera.render("admin/panel.html", &context).unwrap())
}

#[get("/login")]
pub fn admin_login(id: Identity, tera: web::Data<Arc<Tera>>) -> impl Responder {
    match id.identity() {
        Some(_) => RubbleResponder::Redirect("/admin/panel".into()),
        None => RubbleResponder::Html(tera.render("admin/login.html", &Context::new()).unwrap()),
    }
}

#[post("/login")]
pub fn admin_authentication(
    id: Identity,
    user: Form<LoginForm>,
    conn: web::Data<Pool>,
) -> impl Responder {
    let connection = conn.get().unwrap();

    let fetched_user = User::find_by_username(&connection, &user.username);

    match fetched_user {
        Ok(login_user) => {
            if login_user.authenticated(&user.password) {
                id.remember(login_user.username);
                RubbleResponder::Redirect("/admin/panel".into())
            } else {
                // TODO flash message or throw unauthorized
                RubbleResponder::Redirect("/admin/login".into())
            }
        }
        Err(_) => RubbleResponder::Redirect("/admin/login".into()),
    }
}
//
//
//#[get("/")]
//pub fn admin_index(admin: Admin, conn: DbConn, flash: Option<FlashMessage>) -> Template {
//    let mut context = Context::new();
//
//    let articles = Article::load_all(true, &conn);
//
//    context.insert("admin", &admin);
//    context.insert("articles", &articles);
//    context.insert("flash", &SerializeFlashMessage::from(&flash));
//    Template::render("admin/index", &context)
//}
//
//
//#[get("/article/new")]
//pub fn article_creation(_admin: Admin) -> Result<Template, Status> {
//    let mut context = Context::new();
//
//    let article = Article {
//        id: -1,
//        title: String::new(),
//        body: String::new(),
//        published: true,
//        user_id: 0,
//        publish_at: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
//        url: None,
//    };
//
//    context.insert("article", &article);
//    Ok(Template::render("admin/edit", context))
//}
//
//
//#[get("/article/<article_id>")]
//pub fn article_edit(_admin: Admin, conn: DbConn, article_id: i32) -> Result<Template, Status> {
//    let mut context = Context::new();
//    let fetched_article = Article::find(article_id, &conn);
//
//    if let Err(_err) = fetched_article {
//        return Err(Status::NotFound);
//    }
//
//    let article: Article = fetched_article.unwrap();
//
//    context.insert("article", &article);
//    Ok(Template::render("admin/edit", context))
//}
//
//#[post("/article", data = "<article>")]
//pub fn save_article(admin: Admin, conn: DbConn, article: Form<ArticleEditForm>) -> Result<Flash<Redirect>, Status> {
//    use crate::schema::{articles};
//
//    let new_article = Article::form_article_edit_form(&article, admin.id);
//    let _fetched_article: QueryResult<Article> = match new_article.id {
//        Some(article_id) => diesel::update(articles::table.find(article_id)).set(&new_article).get_result(&*conn),
//
//        None => diesel::insert_into(articles::table).values(&new_article).get_result(&*conn),
//    };
//
//    Ok(Flash::new(Redirect::to("/admin"), "success", "created"))
//}
//#[delete("/article/<article_id>")]
//pub fn delete_article(admin: Admin, conn:DbConn, article_id: i32) -> Flash<Redirect> {
//    use crate::schema::articles;
//    diesel::delete(articles::table.filter(articles::id.eq(article_id))).execute(&*conn);
//    Flash::new(Redirect::to("/admin"), "success", "deleted")
//}
//
//#[post("/password", data = "<password_form>")]
//pub fn change_password(admin: Admin, conn: DbConn, password_form: Form<NewPasswordForm>) -> Flash<Redirect> {
//    use crate::schema::{users};
//
//    let mut admin_user: User = users::table.find(admin.id).first::<User>(&*conn).unwrap();
//
//    admin_user.password = User::password_generate(&password_form.password).to_string();
//    let _result: QueryResult<User> = diesel::update(users::table.find(admin_user.id)).set(&admin_user).get_result(&*conn);
//    Flash::new(Redirect::moved("/admin"), "success", "password is changed successfully")
//}
//
//#[post("/setting", data = "<setting_form>")]
//pub fn change_setting(_admin: Admin, conn: DbConn, setting_form: Form<Setting>) -> Flash<Redirect> {
//    use crate::schema::{setting};
//
//    let new_setting = Setting { name: setting_form.name.clone(), value: setting_form.value.clone() };
//    let _fetched_setting: QueryResult<Setting> = diesel::update(setting::table.find(&setting_form.name)).set(&new_setting).get_result(&*conn);
//    Flash::new(Redirect::to("/admin"), "success", "setting changed")
//}
