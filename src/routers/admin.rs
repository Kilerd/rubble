use actix_web::middleware::identity::Identity;
use actix_web::web::Form;
use actix_web::{get, post, web, Either, HttpResponse, Responder};
use chrono::{NaiveDateTime, Utc};
use serde::Deserialize;
use tera::{Context, Tera};

use crate::data::RubbleData;
use crate::models::article::{Article, NewArticle};
use crate::models::setting::Setting;
use crate::models::user::User;
use crate::models::CRUD;
use crate::routers::RubbleResponder;

#[derive(Deserialize)]
pub struct LoginForm {
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
pub fn admin_panel(id: Identity, data: web::Data<RubbleData>) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::Redirect("/admin/login".into());
    }

    let articles = Article::read(&data.postgres());
    let settings = Setting::load(&data.postgres());

    let admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");

    let mut context = Context::new();
    context.insert("setting", &settings);
    context.insert("articles", &articles);
    context.insert("admin", &admin);

    RubbleResponder::Html(data.render("admin/panel.html", &context))
}

#[get("/login")]
pub fn admin_login(id: Identity, data: web::Data<RubbleData>) -> impl Responder {
    match id.identity() {
        Some(_) => RubbleResponder::Redirect("/admin/panel".into()),
        None => RubbleResponder::Html(data.render("admin/login.html", &Context::new())),
    }
}

#[post("/login")]
pub fn admin_authentication(
    id: Identity,
    user: Form<LoginForm>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    let fetched_user = User::find_by_username(&data.postgres(), &user.username);

    match fetched_user {
        Ok(login_user) => {
            if login_user.authenticated(&user.password) {
                id.remember(login_user.username);
                info!("admin login");
                RubbleResponder::Redirect("/admin/panel".into())
            } else {
                // TODO flash message or throw unauthorized
                warn!("try logining admin with wrong password '{}'", user.password);
                RubbleResponder::Redirect("/admin/login".into())
            }
        }
        Err(_) => RubbleResponder::Redirect("/admin/login".into()),
    }
}

#[get("/article/new")]
pub fn article_creation(id: Identity, data: web::Data<RubbleData>) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::Redirect("/admin/login".into());
    }

    let admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
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
        keywords: vec![],
    };

    context.insert("article", &article);
    RubbleResponder::Html(data.render("admin/article_add.html", &context))
}

#[get("/article/{article_id}")]
pub fn article_edit(
    id: Identity,
    article_id: web::Path<i32>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::Redirect("/admin/login".into());
    }

    let admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");

    let result = Article::get_by_pk(&data.postgres(), article_id.into_inner());

    match result {
        Ok(article) => {
            let mut context = Context::new();
            context.insert("article", &article);
            RubbleResponder::Html(data.render("admin/article_add.html", &context))
        }
        Err(_) => RubbleResponder::Redirect("/admin/panel".into()),
    }
}

pub fn article_save(
    id: Identity,
    article: Form<crate::models::article::form::NewArticleForm>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::Redirect("/admin/login".into());
    }

    let article_title = article.title.clone();

    let admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");
    let res = if let Some(article_id) = article.id {
        info!("updating article #{} {}", article_id, article_title);
        Article::update(&data.postgres(), article_id, &article.into_inner().into())
    } else {
        info!("creating new article {}", article_title);
        Article::create(&data.postgres(), &article.into_inner().into())
    };

    if res.is_err() {
        error!("error on updating/creating article {}", article_title);
    }
    RubbleResponder::Redirect("/admin/panel".into())
}

#[post("/article/delete/{article_id}")]
pub fn article_deletion(
    id: Identity,
    article_id: web::Path<i32>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::Redirect("/admin/login".into());
    }

    let admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");

    let i = article_id.into_inner();
    Article::delete(&data.postgres(), i);
    info!("deleting article {}", i);
    RubbleResponder::Redirect("/admin/panel".into())
}

#[post("/password")]
pub fn change_password(
    id: Identity,
    password: web::Form<NewPassword>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::Redirect("/admin/login".into());
    }

    let mut admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");
    admin.password = User::password_generate(&password.password).to_string();
    User::update(&data.postgres(), admin.id, &admin);
    id.forget();
    info!("updating password");
    RubbleResponder::Redirect("/admin/panel".into())
}

#[post("/setting")]
pub fn change_setting(
    id: Identity,
    setting: web::Form<Setting>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::Redirect("/admin/login".into());
    }

    let mut admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");

    Setting::update(&data.postgres(), setting.name.clone(), &setting);
    info!("updating setting {:?} to {:?}", setting.name, setting.value);
    RubbleResponder::Redirect("/admin/panel".into())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_normal() {
        assert_eq!(1, 1);
    }
}
