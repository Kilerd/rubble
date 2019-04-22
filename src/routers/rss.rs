use crate::data::RubbleData;
use crate::models::{article::Article, setting::Setting, CRUD};
use crate::pg_pool::Pool;
use crate::view::article::ArticleView;
use actix_web::{get, web, HttpResponse, Responder};
use rss::{Channel, ChannelBuilder, Item, ItemBuilder};
use std::collections::HashMap;

#[get("/rss")]
pub fn rss_page(data: web::Data<RubbleData>) -> impl Responder {
    let articles = Article::read(&data.postgres());
    let setting = Setting::load(&data.postgres());

    let items: Vec<Item> = articles
        .iter()
        .map(ArticleView::from)
        .map(|item| {
            ItemBuilder::default()
                .title(item.article.title.clone())
                .link(format!("{}{}", setting.url, item.article.link()))
                .description(item.description)
                .content(item.markdown_content)
                .pub_date(item.article.publish_at.to_string())
                .build()
                .unwrap()
        })
        .collect();

    let mut namespaces: HashMap<String, String> = HashMap::new();
    namespaces.insert(
        "dc".to_string(),
        "http://purl.org/dc/elements/1.1/".to_string(),
    );
    namespaces.insert(
        "content".to_string(),
        "http://purl.org/rss/1.0/modules/content/".to_string(),
    );
    namespaces.insert(
        "atom".to_string(),
        "http://www.w3.org/2005/Atom".to_string(),
    );
    namespaces.insert(
        "media".to_string(),
        "http://search.yahoo.com/mrss/".to_string(),
    );

    let channel: Channel = ChannelBuilder::default()
        .title(setting.title)
        .description(setting.description)
        .generator("Rubble".to_string())
        .link(setting.url.clone())
        .items(items)
        .namespaces(namespaces)
        .build()
        .unwrap();
    HttpResponse::Ok()
        .content_type("text/xml; charset=utf-8")
        .body(channel.to_string())
}
