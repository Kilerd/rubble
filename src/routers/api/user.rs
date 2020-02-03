//! /authentications routes

use actix_web::{post, put, Responder, web};
use actix_web::web::{Data, Json};
use crate::{
    data::RubbleData,
    error::RubbleError,
    models::{
        token::Token,
        user::{input::LoginForm, User},
    },
    routers::RubbleResponder,
    utils::jwt::JWTClaims,
};
use crate::models::CRUD;
use serde::{Deserialize, Serialize};

#[post("/user/token")]
pub async fn admin_authentication(
    user: web::Json<LoginForm>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    let fetched_user = User::find_by_username(&data.postgres(), &user.username);

    match fetched_user {
        Ok(login_user) => {
            if login_user.authenticated(&user.password) {
                let string = JWTClaims::encode(&login_user);

                Ok(RubbleResponder::json(Token { token: string }))
            } else {
                Err(RubbleError::Unauthorized("invalid password"))
            }
        }
        Err(_) => Err(RubbleError::Unauthorized("invalid username")),
    }
}

//
//#[get("")]
// pub fn get_all_users() -> impl Responder {
//    unreachable!()
//}
//
//#[post("")]
// pub fn crate_user() -> impl Responder {
//    unreachable!()
//}
//
//#[put("/{id}")]
// pub fn update_user_by_id() -> impl Responder {
//    unreachable!()
//}
//
//#[delete("/{id}")]
// pub fn delete_user_by_id() -> impl Responder {
//    unreachable!()
//}
//
#[derive(Serialize, Deserialize)]
pub struct UpdatedUserPassword {
    pub password: String
}

#[put("/users/{id}/password")]
pub async fn update_user_password(user: User, id: web::Path<String>, json: Json<UpdatedUserPassword>, data: Data<RubbleData>) -> impl Responder {
    if json.password.eq("") {
        return Err(RubbleError::BadRequest("password can not be empty"));
    }
    let mut admin = User::find_by_username(&data.postgres(), &id);
    admin
        .map(|mut user| {
        user.password = User::password_generate(&json.password).to_string();
        User::update(&data.postgres(), user.id, &user);
        RubbleResponder::json("OK")
    })
        .map_err(|e| RubbleError::BadRequest("cannot get admin"))

}
