use crate::models::Article;
use pulldown_cmark::html;
use pulldown_cmark::Parser;

#[derive(Debug, Serialize)]
pub struct ArticleResponse<'a> {
    pub article: &'a Article,
    pub timestamp: i64,
    pub markdown_content: String,
    pub description: String,
}

impl<'a> ArticleResponse<'a> {

    pub fn from(article: &'a Article) -> ArticleResponse {
        let content_split: Vec<_> = article.body.split("<!--more-->").collect();
        let description_parser = Parser::new(&content_split[0]);
        let parser = Parser::new(&article.body);
        let mut description_buf = String::new();
        let mut content_buf = String::new();
        html::push_html(&mut content_buf, parser);
        html::push_html(&mut description_buf, description_parser);
        ArticleResponse {
            article,
            timestamp: article.publish_at.timestamp(),
            markdown_content: content_buf,
            description: description_buf,
        }
    }
}