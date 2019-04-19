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
#[derive(Deserialize)]
struct NewPassword {
    password: String,
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

#[post("/password")]
pub fn change_password(
    id: Identity,
    password: web::Form<NewPassword>,
    conn: web::Data<Pool>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::Redirect("/admin/login".into());
    }
    let connection = conn.get().unwrap();

    let mut admin = User::find_by_username(&*connection, &id.identity().unwrap())
        .expect("cannot found this user");
    admin.password = User::password_generate(&password.password).to_string();
    User::update(&connection, admin.id, &admin);
    id.forget();
    RubbleResponder::Redirect("/admin/panel".into())
}

#[post("/setting")]
pub fn change_setting(
    id: Identity,
    setting: web::Form<Setting>,
    conn: web::Data<Pool>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::Redirect("/admin/login".into());
    }
    let connection = conn.get().unwrap();

    let mut admin = User::find_by_username(&*connection, &id.identity().unwrap())
        .expect("cannot found this user");

    Setting::update(&connection, setting.name.clone(), &setting);

    RubbleResponder::Redirect("/admin/panel".into())
}
