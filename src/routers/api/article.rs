use crate::{
    data::RubbleData,
    models::{article::Article, user::User, CRUD},
    routers::RubbleResponder,
};
use actix_web::{delete, get, post, put, web, Responder};

#[get("")]
pub fn get_all_article(user: User, data: web::Data<RubbleData>) -> impl Responder {
    RubbleResponder::json(Article::read(&data.postgres()))
}

#[get("/{id}")]
pub fn get_all_article_by_id(
    user: User,
    id: web::Path<i32>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    Article::get_by_pk(&data.postgres(), *id)
        .map(|data| RubbleResponder::json(data))
        .map_err(|_| RubbleResponder::not_found())
}

#[post("")]
pub fn crate_article(
    user: User,
    article: web::Json<crate::models::article::NewArticle>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    Article::create(&data.postgres(), &article)
        .map(RubbleResponder::json)
        .map_err(|_| RubbleResponder::bad_request("something wrong when creating article"))
}

#[put("/{id}")]
pub fn update_article_by_id(
    user: User,
    id: web::Path<i32>,
    article: web::Json<crate::models::article::NewArticle>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    Article::update(&data.postgres(), *id, &article)
        .map(|data| RubbleResponder::json(data))
        .map_err(|_| RubbleResponder::bad_request("something wrong when updating article"))
}

#[delete("/{id}")]
pub fn delete_article_by_id(
    user: User,
    id: web::Path<i32>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    Article::delete(&data.postgres(), *id)
        .map(|_| RubbleResponder::json("Ok"))
        .map_err(|_| RubbleResponder::bad_request("something wrong when deleting article"))
}
