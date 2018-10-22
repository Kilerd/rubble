use chrono::NaiveDateTime;
use models::Post;
use pulldown_cmark::Parser;
use pulldown_cmark::html;

#[derive(Debug, Serialize)]
pub struct PostResponse<'a> {
    pub post: &'a Post,
    pub timestamp: i64,
    pub markdown_content: String,
    pub description: String,
}

impl<'a> PostResponse<'a> {

    pub fn from(post: &'a Post) -> PostResponse {
        let content_split: Vec<_> = post.body.split("<!--more-->").collect();
        let description_parser = Parser::new(&content_split[0]);
        let parser = Parser::new(&post.body);
        let mut description_buf = String::new();
        let mut content_buf = String::new();
        html::push_html(&mut content_buf, parser);
        html::push_html(&mut description_buf, description_parser);
        PostResponse {
            post,
            timestamp: post.publish_at.timestamp(),
            markdown_content: content_buf,
            description: description_buf,
        }
    }
}