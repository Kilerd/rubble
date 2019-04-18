use crate::models::article::Article;
use pulldown_cmark::html;
use pulldown_cmark::Parser;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ArticleView<'a> {
    pub article: &'a Article,
    pub timestamp: i64,
    pub markdown_content: String,
    pub description: String,
}

impl<'a> ArticleView<'a> {
    pub fn from(article: &'a Article) -> ArticleView {
        let content_split: Vec<_> = article.body.split("<!--more-->").collect();
        let description_parser = Parser::new(&content_split[0]);
        let parser = Parser::new(&article.body);
        let mut description_buf = String::new();
        let mut content_buf = String::new();
        html::push_html(&mut content_buf, parser);
        html::push_html(&mut description_buf, description_parser);
        ArticleView {
            article,
            timestamp: article.publish_at.timestamp(),
            markdown_content: content_buf,
            description: description_buf,
        }
    }
}
