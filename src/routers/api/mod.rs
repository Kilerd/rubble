use actix_web::web;

pub mod article;
pub mod setting;
pub mod user;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // user related
        .service(user::admin_authentication)
        .service(user::update_user_password)
        // article related
        .service(article::get_all_article)
        .service(article::crate_article)
        .service(article::get_article_by_id)
        .service(article::update_article_by_id)
        .service(article::delete_article_by_id)
        // setting related
        .service(setting::get_settings)
        .service(setting::update_setting_by_key);
}
