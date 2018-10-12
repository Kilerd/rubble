use chrono::NaiveDateTime;
use models::Post;
use pulldown_cmark::Parser;
use pulldown_cmark::html;

#[derive(Debug, Serialize)]
pub struct PostResponse<'a> {
    pub post: &'a Post,
    pub timestamp: i64,
    pub markdown_content: String,
}

impl<'a> PostResponse<'a> {

    pub fn from(post: &'a Post) -> PostResponse {
        let parser = Parser::new(&post.body);

        let mut content_buf = String::new();
        html::push_html(&mut content_buf, parser);

        PostResponse {
            post,
            timestamp: post.publish_at.timestamp(),
            markdown_content: content_buf
        }
    }
}