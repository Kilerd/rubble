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

use crate::models::article::{Article, NewArticle};

use crate::models::setting::Setting;
use crate::models::user::User;
use crate::models::CRUD;
use crate::pg_pool::Pool;
use crate::routers::RubbleResponder;
use actix_web::middleware::identity::Identity;
use actix_web::web::Form;
use actix_web::{delete, get, post, web, Either, HttpResponse, Responder};
use chrono::{NaiveDateTime, Utc};
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

#[get("/article/new")]
pub fn article_creation(
    id: Identity,
    tera: web::Data<Arc<Tera>>,
    conn: web::Data<Pool>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::Redirect("/admin/login".into());
    }
    let connection = conn.get().unwrap();

    let admin = User::find_by_username(&*connection, &id.identity().unwrap())
        .expect("cannot found this user");

    let mut context = Context::new();

    let article = NewArticle {
        id: None,
        title: String::new(),
        body: String::new(),
        published: true,
        user_id: admin.id,
        publish_at: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        url: None,
    };

    context.insert("article", &article);
    RubbleResponder::Html(tera.render("admin/article_add.html", &context).unwrap())
}

#[get("/article/{article_id}")]
pub fn article_edit(
    id: Identity,
    article_id: web::Path<i32>,
    tera: web::Data<Arc<Tera>>,
    conn: web::Data<Pool>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::Redirect("/admin/login".into());
    }
    let connection = conn.get().unwrap();

    let admin = User::find_by_username(&*connection, &id.identity().unwrap())
        .expect("cannot found this user");

    let result = Article::get_by_pk(&connection, article_id.into_inner());

    match result {
        Ok(article) => {
            let mut context = Context::new();
            context.insert("article", &article);
            RubbleResponder::Html(tera.render("admin/article_add.html", &context).unwrap())
        }
        Err(_) => RubbleResponder::Redirect("/admin/panel".into()),
    }
}

#[post("/article")]
pub fn article_save(
    id: Identity,
    tera: web::Data<Arc<Tera>>,
    conn: web::Data<Pool>,
    article: Form<NewArticle>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::Redirect("/admin/login".into());
    }
    let connection = conn.get().unwrap();

    let admin = User::find_by_username(&*connection, &id.identity().unwrap())
        .expect("cannot found this user");

    let res = if let Some(article_id) = article.id {
        Article::update(&connection, article_id, &article)
    } else {
        Article::create(&connection, &article)
    };
    RubbleResponder::Redirect("/admin/panel".into())
}
#[post("/article/delete/{article_id}")]
pub fn article_deletion(
    id: Identity,
    article_id: web::Path<i32>,
    conn: web::Data<Pool>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::Redirect("/admin/login".into());
    }
    let connection = conn.get().unwrap();

    let admin = User::find_by_username(&*connection, &id.identity().unwrap())
        .expect("cannot found this user");

    Article::delete(&connection, article_id.into_inner());
    RubbleResponder::Redirect("/admin/panel".into())
}
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
