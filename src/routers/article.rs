//use crate::guard::SettingMap;
//use crate::models::Article;
//use crate::pg_pool::DbConn;
//use crate::response::ArticleResponse;
//use crate::schema::{articles::dsl::*};
//use crate::schema::articles;
//use diesel::prelude::*;
//use rocket::http::Status;
//use rocket::response::NamedFile;
//use rocket_contrib::templates::Template;
//use std::path::Path;
//use std::path::PathBuf;
//use tera::Context;
//
//

use crate::models::article::Article;
use crate::models::setting::Setting;
use crate::models::CRUD;
use crate::pg_pool::Pool;
use crate::routers::RubbleResponder;
use crate::view::article::ArticleView;
use actix_web::{get, web, Either, HttpResponse, Responder};
use std::sync::Arc;
use tera::{Context, Tera};

#[get("/")]
pub fn homepage(tera: web::Data<Arc<Tera>>, conn: web::Data<Pool>) -> impl Responder {
    let connection = conn.get().unwrap();
    let vec: Vec<Article> = Article::read(&connection);

    let article_view: Vec<_> = vec
        .iter()
        .filter(|article| article.published == true)
        .map(ArticleView::from)
        .collect();

    //    let articles: Vec<ArticleView> = vec.iter().map(ArticleView::from).collect();
    let settings = Setting::load(&connection);

    let mut context = Context::new();
    context.insert("setting", &settings);
    context.insert("articles", &article_view);

    RubbleResponder::Html(tera.render("homepage.html", &context).unwrap())
}

#[get("/archives/{archives_id}")]
pub fn single_article(
    archives_id: web::Path<i32>,
    tera: web::Data<Arc<Tera>>,
    conn: web::Data<Pool>,
) -> impl Responder {
    let connection = conn.get().unwrap();
    let article = Article::get_by_pk(&connection, archives_id.into_inner());

    if let Err(e) = article {
        return RubbleResponder::NotFound;
    }
    let article1 = article.unwrap();

    let view = ArticleView::from(&article1);

    let settings = Setting::load(&connection);

    let mut context = Context::new();
    context.insert("setting", &settings);
    context.insert("article", &view);

    RubbleResponder::Html(tera.render("archives.html", &context).unwrap())
}

#[get("/{url}")]
pub fn get_article_by_url(
    url: web::Path<String>,
    tera: web::Data<Arc<Tera>>,
    conn: web::Data<Pool>,
) -> impl Responder {
    let connection = conn.get().unwrap();
    let article = Article::find_by_url(&connection, &url.into_inner());

    if let Err(e) = article {
        return RubbleResponder::NotFound;
    }
    let article1 = article.unwrap();

    let view = ArticleView::from(&article1);

    let settings = Setting::load(&connection);

    let mut context = Context::new();
    context.insert("setting", &settings);
    context.insert("article", &view);

    RubbleResponder::Html(tera.render("archives.html", &context).unwrap())
}
