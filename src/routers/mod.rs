use actix_web::{HttpRequest, HttpResponse, Responder};

pub mod article;
use actix_web::error::Error;
use futures::future::{err, ok, FutureResult};
//pub mod admin;
//pub mod rss;
//pub mod catacher;
//pub mod graphql;

pub enum RubbleResponder {
    Html(String),
    Redirect(String),
    NotFound,
}

impl Responder for RubbleResponder {
    type Error = Error;
    type Future = FutureResult<HttpResponse, Error>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        match self {
            RubbleResponder::Html(content) => ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(content)),
            RubbleResponder::Redirect(content) => ok(HttpResponse::Ok()
                .content_type("text/html")
                .body("redirect")),
            RubbleResponder::NotFound => ok(HttpResponse::NotFound().finish()),
        }
    }
}
