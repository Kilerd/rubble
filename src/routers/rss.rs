//use rss::ChannelBuilder;
//use rss::Channel;
//use rss::{Item, ItemBuilder};
//use rocket::response::content;
//use crate::guard::SettingMap;
//use crate::pg_pool::DbConn;
//use crate::models::Article;
//use crate::response::ArticleResponse;
//use std::collections::HashMap;
//
//#[get("/rss")]
//pub fn rss(setting: SettingMap, conn: DbConn) -> content::Xml<String> {
//    let result = Article::load_all(false, &conn);
//    let article_responses: Vec<ArticleResponse> = result.iter().map(ArticleResponse::from).collect();
//
//    let items: Vec<Item> = article_responses.into_iter().map(|item| {
//        let url = match item.article.url.clone() {
//            Some(content) => if !content.eq("") {
//                format!("{}/{}", setting.url, content)
//            } else {
//                format!("{}/archives/{}", setting.url, item.article.id)
//            },
//            None => format!("{}/archives/{}", setting.url, item.article.id)
//        };
//        ItemBuilder::default()
//            .title(item.article.title.clone())
//            .link(url)
//            .description(item.description)
//            .content(item.markdown_content)
//            .pub_date(item.article.publish_at.to_string())
//            .build()
//            .unwrap()
//    }).collect();
//
//    let mut namespaces: HashMap<String, String> = HashMap::new();
//    namespaces.insert("dc".to_string(), "http://purl.org/dc/elements/1.1/".to_string());
//    namespaces.insert("content".to_string(), "http://purl.org/rss/1.0/modules/content/".to_string());
//    namespaces.insert("atom".to_string(), "http://www.w3.org/2005/Atom".to_string());
//    namespaces.insert("media".to_string(), "http://search.yahoo.com/mrss/".to_string());
//
//    let channel: Channel = ChannelBuilder::default()
//        .title(setting.title)
//        .description(setting.description)
//        .generator("Rubble".to_string())
//        .link(setting.url.clone())
//        .items(items)
//        .namespaces(namespaces)
//        .build()
//        .unwrap();
//    content::Xml(channel.to_string())
//}
