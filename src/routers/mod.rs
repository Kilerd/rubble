use actix_web::{error::Error, HttpRequest, HttpResponse, Responder, Scope, web};
use futures::future::{err, ok};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod admin;
pub mod api;
pub mod article;
pub mod rss;

#[derive(Deserialize, Serialize)]
pub struct JsonResponse<T> {
    data: T,
}

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse<T> {
    message: T,
}

pub struct RubbleResponder;

impl RubbleResponder {
    pub fn html(content: impl Into<String>) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(content.into())
    }

    pub fn json(data: impl Serialize) -> HttpResponse {
        HttpResponse::Ok()
            .header(
                http::header::CONTENT_TYPE,
                "application/json; charset=utf-8",
            )
            .json(JsonResponse { data })
    }

    pub fn text(content: impl Into<String>) -> HttpResponse {
        HttpResponse::Ok().body(content.into())
    }

    pub fn redirect(to: impl Into<String>) -> HttpResponse {
        HttpResponse::Found()
            .header(http::header::LOCATION, to.into())
            .finish()
    }

    pub fn redirect_permanently(to: impl Into<String>) -> HttpResponse {
        HttpResponse::MovedPermanently()
            .header(http::header::LOCATION, to.into())
            .finish()
    }

    pub fn not_found() -> HttpResponse {
        HttpResponse::NotFound().finish()
    }

}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/api").configure(api::routes))
        .service(article::homepage)
        .service(article::single_article)
        .service(actix_files::Files::new(
            "/statics",
            "./templates/resources/",
        ))
        .service(rss::rss_)
        .service(article::get_article_by_url);
}
