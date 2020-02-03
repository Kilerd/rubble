use actix_web::{cookie::Cookie, get, post, web, web::Form, HttpResponse, Responder};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::{
    data::RubbleData,
    models::{
        article::{Article, NewArticle},
        setting::{Setting, UpdateSetting},
        user::User,
        CRUD,
    },
    routers::RubbleResponder,
    utils::jwt::JWTClaims,
};

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct NewPassword {
    password: String,
}

#[get("")]
pub async fn redirect_to_admin_panel(user: Option<User>) -> impl Responder {
    if user.is_some() {
        RubbleResponder::redirect("/admin/panel")
    } else {
        RubbleResponder::redirect("/admin/login")
    }
}

#[get("/login")]
pub async fn admin_login(user: Option<User>, data: web::Data<RubbleData>) -> impl Responder {
    if user.is_some() {
        RubbleResponder::redirect("/admin/panel")
    } else {
        RubbleResponder::html(data.render("admin/login.html", &Context::new()))
    }
}

#[post("/login")]
pub async fn admin_authentication(
    user: Form<LoginForm>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    let fetched_user = User::find_by_username(&data.postgres(), &user.username);

    match fetched_user {
        Ok(login_user) => {
            if login_user.authenticated(&user.password) {
                let jwt = JWTClaims::encode(&login_user);

                HttpResponse::Found()
                    .header(http::header::LOCATION, "/admin/panel")
                    .cookie(
                        Cookie::build("Authorization", format!("JWT {}", jwt))
                            .path("/")
                            .max_age(60 * 60 * 24 * 7)
                            .finish(),
                    )
                    .finish()
            } else {
                RubbleResponder::redirect("/admin/login")
            }
        }
        Err(_) => RubbleResponder::redirect("/admin/login"),
    }
}

#[get("/panel")]
pub async fn admin_panel(user: User, data: web::Data<RubbleData>) -> impl Responder {
    let articles = Article::read(&data.postgres());
    let settings = Setting::load(&data.postgres());

    let mut context = Context::new();
    context.insert("setting", &settings);
    context.insert("articles", &articles);
    context.insert("admin", &user);

    RubbleResponder::html(data.render("admin/panel.html", &context))
}

#[get("/{path}")]
pub async fn admin_show_page(
    user: User,
    path: web::Path<String>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    let settings = Setting::load(&data.postgres());

    let mut context = Context::new();
    context.insert("setting", &settings);
    context.insert("admin", &user);
    RubbleResponder::html(data.render(&format!("admin/{}.html", path), &context))
}

#[get("/article/new")]
pub async fn article_creation(user: User, data: web::Data<RubbleData>) -> impl Responder {
    let settings = Setting::load(&data.postgres());

    let mut context = Context::new();

    let article = NewArticle {
        title: String::new(),
        body: String::new(),
        published: true,
        user_id: user.id,
        publish_at: Some(NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)),
        url: None,
        keywords: vec![],
    };

    context.insert("article", &article);
    context.insert("setting", &settings);
    context.insert("admin", &user);

    RubbleResponder::html(data.render("admin/article_add.html", &context))
}

#[get("/article/{article_id}")]
pub async fn article_edit(
    user: User,
    article_id: web::Path<i32>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    let settings = Setting::load(&data.postgres());

    let result = Article::get_by_pk(&data.postgres(), article_id.into_inner());

    match result {
        Ok(article) => {
            let mut context = Context::new();
            context.insert("article", &article);
            context.insert("setting", &settings);
            context.insert("admin", &user);
            RubbleResponder::html(data.render("admin/article_add.html", &context))
        }
        Err(_) => RubbleResponder::redirect("/admin/panel"),
    }
}

#[post("/article")]
pub async fn article_save(
    _user: User,
    article: Form<crate::models::article::form::NewArticleFrom>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    let _article_title = article.title.clone();

    Article::create(&data.postgres(), &article.into_inner().into());

    RubbleResponder::redirect("/admin/panel")
}

#[post("/article/{aid}")]
pub async fn article_update(
    _user: User,
    aid: web::Path<i32>,
    article: Form<crate::models::article::form::NewArticleFrom>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    Article::update(&data.postgres(), *aid, &article.into_inner().into());

    RubbleResponder::redirect("/admin/panel")
}

#[post("/article/delete/{article_id}")]
pub async fn article_deletion(
    _user: User,
    article_id: web::Path<i32>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    let i = article_id.into_inner();
    Article::delete(&data.postgres(), i);

    RubbleResponder::redirect("/admin/panel")
}

#[post("/password")]
pub async fn change_password(
    mut user: User,
    password: web::Form<NewPassword>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    user.password = User::password_generate(&password.password).to_string();
    User::update(&data.postgres(), user.id, &user);
    RubbleResponder::redirect("/admin/panel")
}

#[post("/setting")]
pub async fn change_setting(
    _user: User,
    setting: web::Form<Setting>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    let update_setting = UpdateSetting {
        value: setting.value.clone(),
    };
    Setting::update(&data.postgres(), setting.name.clone(), &update_setting);
    RubbleResponder::redirect("/admin/site-setting")
}

#[cfg(test)]
mod test {
    #[test]
    fn test_normal() {
        assert_eq!(1, 1);
    }
}
