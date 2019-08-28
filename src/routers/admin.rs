use actix_identity::Identity;
use actix_web::{
    get, post,
    web::{self, Form},
    Either, HttpResponse, Responder,
};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use crate::{
    data::RubbleData,
    models::{
        article::{Article, NewArticle},
        setting::{Setting, UpdateSetting},
        token::Token,
        user::User,
        CRUD,
    },
    routers::RubbleResponder,
};

use crate::utils::jwt::JWTClaims;

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
pub fn redirect_to_admin_panel() -> impl Responder {
    RubbleResponder::redirect("/admin/panel")
}

#[get("/login")]
pub fn admin_login(id: Identity, data: web::Data<RubbleData>) -> impl Responder {
    if id.identity().is_some() {
        RubbleResponder::redirect("/admin/panel")
    } else {
        RubbleResponder::html(data.render("admin/login.html", &Context::new()))
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

                RubbleResponder::redirect("/admin/panel")
            } else {
                RubbleResponder::redirect("/admin/login")
            }
        }
        Err(_) => RubbleResponder::redirect("/admin/login"),
    }
}

#[get("/panel")]
pub fn admin_panel(id: Identity, data: web::Data<RubbleData>) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::redirect("/admin/login");
    }

    let articles = Article::read(&data.postgres());
    let settings = Setting::load(&data.postgres());

    let admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");

    let mut context = Context::new();
    context.insert("setting", &settings);
    context.insert("articles", &articles);
    context.insert("admin", &admin);

    RubbleResponder::html(data.render("admin/panel.html", &context))
}

#[get("/{path}")]
pub fn admin_show_page(
    id: Identity,
    path: web::Path<String>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::redirect("/admin/login");
    }

    let settings = Setting::load(&data.postgres());

    let admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");

    let mut context = Context::new();
    context.insert("setting", &settings);
    context.insert("admin", &admin);
    RubbleResponder::html(data.render(&format!("admin/{}.html", path), &context))
}

#[get("/article/new")]
pub fn article_creation(id: Identity, data: web::Data<RubbleData>) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::redirect("/admin/login");
    }
    let settings = Setting::load(&data.postgres());
    let admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");

    let mut context = Context::new();

    let article = NewArticle {
        title: String::new(),
        body: String::new(),
        published: true,
        user_id: admin.id,
        publish_at: Some(NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)),
        url: None,
        keywords: vec![],
    };

    context.insert("article", &article);
    context.insert("setting", &settings);
    context.insert("admin", &admin);

    RubbleResponder::html(data.render("admin/article_add.html", &context))
}

#[get("/article/{article_id}")]
pub fn article_edit(
    id: Identity,
    article_id: web::Path<i32>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::redirect("/admin/login");
    }
    let settings = Setting::load(&data.postgres());
    let admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");

    let result = Article::get_by_pk(&data.postgres(), article_id.into_inner());

    match result {
        Ok(article) => {
            let mut context = Context::new();
            context.insert("article", &article);
            context.insert("setting", &settings);
            context.insert("admin", &admin);
            RubbleResponder::html(data.render("admin/article_add.html", &context))
        }
        Err(_) => RubbleResponder::redirect("/admin/panel"),
    }
}

#[post("/article")]
pub fn article_save(
    id: Identity,
    article: Form<crate::models::article::form::NewArticleFrom>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::redirect("/admin/login");
    }

    let article_title = article.title.clone();

    let admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");
    Article::create(&data.postgres(), &article.into_inner().into());

    RubbleResponder::redirect("/admin/panel")
}

#[post("/article/{aid}")]
pub fn article_update(
    id: Identity,
    aid: web::Path<i32>,
    article: Form<crate::models::article::form::NewArticleFrom>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::redirect("/admin/login");
    }

    let admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");
    Article::update(&data.postgres(), *aid, &article.into_inner().into());

    RubbleResponder::redirect("/admin/panel")
}

#[post("/article/delete/{article_id}")]
pub fn article_deletion(
    id: Identity,
    article_id: web::Path<i32>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::redirect("/admin/login");
    }

    let admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");

    let i = article_id.into_inner();
    Article::delete(&data.postgres(), i);

    RubbleResponder::redirect("/admin/panel")
}

#[post("/password")]
pub fn change_password(
    id: Identity,
    password: web::Form<NewPassword>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::redirect("/admin/login");
    }

    let mut admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");
    admin.password = User::password_generate(&password.password).to_string();
    User::update(&data.postgres(), admin.id, &admin);
    id.forget();
    RubbleResponder::redirect("/admin/panel")
}

#[post("/setting")]
pub fn change_setting(
    id: Identity,
    setting: web::Form<Setting>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    if id.identity().is_none() {
        return RubbleResponder::redirect("/admin/login");
    }

    let mut admin = User::find_by_username(&data.postgres(), &id.identity().unwrap())
        .expect("cannot found this user");

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
