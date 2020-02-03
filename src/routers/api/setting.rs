use crate::{
    data::RubbleData,
    error::RubbleError,
    models::{
        setting::{Setting, UpdateSetting},
        user::User,
        CRUD,
    },
    routers::RubbleResponder,
};
use actix_web::{get, put, web, Responder};

#[get("/settings")]
pub async fn get_settings(_user: User, data: web::Data<RubbleData>) -> impl Responder {
    RubbleResponder::json(Setting::load(&data.postgres()))
}

#[put("settings/{key}")]
pub async fn update_setting_by_key(
    _user: User,
    key: web::Path<String>,
    value: web::Json<UpdateSetting>,
    data: web::Data<RubbleData>,
) -> impl Responder {
    let string = (*key).clone();
    Setting::update(&data.postgres(), string, &value)
        .map(RubbleResponder::json)
        .map_err(|_| RubbleError::BadRequest("error on updating setting"))
}
