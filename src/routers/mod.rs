use actix_web::{HttpRequest, HttpResponse, Responder};

use actix_web::{error::Error, web, Scope};
use futures::future::{err, ok, FutureResult};
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

    pub fn unauthorized(reason: impl Serialize) -> HttpResponse {
        HttpResponse::Unauthorized().json(&ErrorResponse { message: reason })
    }

    pub fn bad_gateway(reason: impl Serialize) -> HttpResponse {
        HttpResponse::BadGateway().json(&ErrorResponse { message: reason })
    }

    pub fn bad_request(reason: impl Serialize) -> HttpResponse {
        HttpResponse::BadRequest().json(&ErrorResponse { message: reason })
    }
}

pub fn routes() -> Scope {
    web::scope("/")
        .service(
            web::scope("/api")
                .service(web::scope("/users").service(api::user::admin_authentication))
                .service(
                    web::scope("/articles")
                        .service(api::article::get_all_article)
                        .service(api::article::get_all_article_by_id)
                        .service(api::article::crate_article)
                        .service(api::article::update_article_by_id)
                        .service(api::article::delete_article_by_id),
                )
                .service(
                    web::scope("/settings")
                        .service(api::setting::get_settings)
                        .service(api::setting::update_setting_by_key),
                ),
        )
        .service(
            web::scope("/admin")
                .service(admin::redirect_to_admin_panel)
                .service(admin::admin_login)
                .service(admin::admin_authentication)
                .service(admin::admin_panel)
                .service(admin::article_creation)
                .service(admin::article_deletion)
                .service(admin::article_edit)
                .service(admin::article_save)
                .service(admin::article_update)
                .service(admin::change_password)
                .service(admin::change_setting)
                .service(admin::admin_show_page),
        )
        .service(article::homepage)
        .service(article::single_article)
        .service(actix_files::Files::new(
            "/statics",
            "./templates/resources/",
        ))
        .service(rss::rss_)
        .service(article::get_article_by_url)
}
