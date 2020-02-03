use actix_web::error::ResponseError;
use derive_more::Display;

use actix_web::HttpResponse;
use http::StatusCode;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Display)]
pub enum RubbleError<T> {
    Unauthorized(T),
    BadRequest(T),
}

#[derive(Serialize)]
struct ErrorMsg<T> {
    message: T,
}

impl<T> ResponseError for RubbleError<T>
where
    T: Debug + std::fmt::Display + Serialize,
{
    fn status_code(&self) -> StatusCode {
        match self {
            RubbleError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            RubbleError::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            RubbleError::Unauthorized(message) => {
                HttpResponse::Unauthorized().json(&ErrorMsg { message })
            }
            RubbleError::BadRequest(message) => {
                HttpResponse::BadRequest().json(&ErrorMsg { message })
            }
        }
    }
}
