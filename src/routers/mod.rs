use actix_web::{HttpRequest, HttpResponse, Responder};

pub mod admin;
pub mod article;
use actix_web::error::Error;
use futures::future::{err, ok, FutureResult};

pub mod rss;

pub enum RubbleResponder {
    Html(String),
    Redirect(String),
    NotFound,
    RedirectPermanently(String),
}

impl Responder for RubbleResponder {
    type Error = Error;
    type Future = FutureResult<HttpResponse, Error>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        match self {
            RubbleResponder::Html(content) => ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(content)),
            RubbleResponder::Redirect(to) => ok(HttpResponse::Found()
                .header(http::header::LOCATION, to)
                .finish()),
            RubbleResponder::NotFound => ok(HttpResponse::NotFound().finish()),
            RubbleResponder::RedirectPermanently(to) => ok(HttpResponse::MovedPermanently()
                .header(http::header::LOCATION, to)
                .finish()),
        }
    }
}
