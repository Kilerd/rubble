use crate::{
    data::RubbleData,
    models::{
        article::{view::ArticleView, Article},
        setting::Setting,
        CRUD,
    },
    routers::RubbleResponder,
};
use actix_web::{get, web, Responder};

use tera::Context;

#[get("/")]
pub async fn homepage(data: web::Data<RubbleData>) -> impl Responder {
    let vec: Vec<Article> = Article::read(&data.postgres());
    let article_view: Vec<_> = vec
        .iter()
        .filter(|article| article.published == true)
        .map(ArticleView::from)
        .collect();

    let settings = Setting::load(&data.postgres());

    let mut context = Context::new();
    context.insert("setting", &settings);
    context.insert("articles", &article_view);

    RubbleResponder::html(data.render("homepage.html", &context))
}

#[get("archives/{archives_id}")]
pub async fn single_article(
    archives_id: web::Path<i32>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    let article = Article::get_by_pk(&data.postgres(), archives_id.into_inner());

    if let Err(_e) = article {
        return RubbleResponder::not_found();
    }
    let article1 = article.unwrap();

    if let Some(ref to) = article1.url {
        if to.len() != 0 {
            return RubbleResponder::redirect(format!("/{}", to));
        }
    }

    article1.increase_view(&data.postgres());
    let view = ArticleView::from(&article1);

    let settings = Setting::load(&data.postgres());

    let mut context = Context::new();
    context.insert("setting", &settings);
    context.insert("article", &view);

    RubbleResponder::html(data.render("archives.html", &context))
}

#[get("{url}")]
pub async fn get_article_by_url(
    url: web::Path<String>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    let article = Article::find_by_url(&data.postgres(), &url.into_inner());

    if let Err(_e) = article {
        return RubbleResponder::not_found();
    }
    let article1 = article.unwrap();
    article1.increase_view(&data.postgres());

    let view = ArticleView::from(&article1);

    let settings = Setting::load(&data.postgres());

    let mut context = Context::new();
    context.insert("setting", &settings);
    context.insert("article", &view);

    RubbleResponder::html(data.render("archives.html", &context))
}
