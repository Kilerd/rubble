//! /authentications routes

use crate::{
    data::RubbleData,
    models::{
        token::Token,
        user::{input::LoginForm, User},
    },
    routers::RubbleResponder,
    utils::jwt::JWTClaims,
};
use actix_web::{delete, get, post, put, web, Responder};

#[post("/token")]
pub fn admin_authentication(
    user: web::Json<LoginForm>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    let fetched_user = User::find_by_username(&data.postgres(), &user.username);

    match fetched_user {
        Ok(login_user) => {
            if login_user.authenticated(&user.password) {
                let string = JWTClaims::encode(&login_user);

                RubbleResponder::json(Token { token: string })
            } else {
                RubbleResponder::unauthorized("invalid password")
            }
        }
        Err(_) => RubbleResponder::unauthorized("invalid username"),
    }
}

#[get("")]
pub fn get_all_users() -> impl Responder {
    unreachable!()
}

#[post("")]
pub fn crate_user() -> impl Responder {
    unreachable!()
}

#[put("/{id}")]
pub fn update_user_by_id() -> impl Responder {
    unreachable!()
}

#[delete("/{id}")]
pub fn delete_user_by_id() -> impl Responder {
    unreachable!()
}

#[put("/{id}/password")]
pub fn update_user_password() -> impl Responder {
    unreachable!()
}
