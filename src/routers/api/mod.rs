use actix_web::web;

pub mod article;
pub mod setting;
pub mod user;


pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(user::admin_authentication)

        .service(article::get_all_article)
        .service(article::crate_article)
        .service(article::get_article_by_id)
        .service(article::update_article_by_id)
        .service(article::delete_article_by_id)

        .service(setting::get_settings)
        .service(setting::update_setting_by_key)
    ;
}