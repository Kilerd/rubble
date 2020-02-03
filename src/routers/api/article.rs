use crate::{
    data::RubbleData,
    error::RubbleError,
    models::{article::Article, user::User, CRUD},
    routers::RubbleResponder,
};
use actix_web::{delete, get, post, put, web, Responder};

#[get("/articles")]
pub async fn get_all_article(_user: User, data: web::Data<RubbleData>) -> impl Responder {
    RubbleResponder::json(Article::read(&data.postgres()))
}

#[get("/articles/{id}")]
pub async fn get_article_by_id(
    _user: User,
    id: web::Path<i32>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    Article::get_by_pk(&data.postgres(), *id)
        .map(RubbleResponder::json)
        .map_err(|_| RubbleResponder::not_found())
}

#[post("/articles")]
pub async fn crate_article(
    _user: User,
    article: web::Json<crate::models::article::NewArticle>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    Article::create(&data.postgres(), &article)
        .map(RubbleResponder::json)
        .map_err(|_| RubbleError::BadRequest("something wrong when creating article"))
}

#[put("/articles/{id}")]
pub async fn update_article_by_id(
    _user: User,
    id: web::Path<i32>,
    article: web::Json<crate::models::article::NewArticle>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    Article::update(&data.postgres(), *id, &article)
        .map(|data| RubbleResponder::json(data))
        .map_err(|_| RubbleError::BadRequest("something wrong when updating article"))
}

#[delete("/articles/{id}")]
pub async fn delete_article_by_id(
    _user: User,
    id: web::Path<i32>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    Article::delete(&data.postgres(), *id)
        .map(|_| RubbleResponder::json("Ok"))
        .map_err(|_| RubbleError::BadRequest("something wrong when deleting article"))
}
