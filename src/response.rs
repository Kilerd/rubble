use chrono::NaiveDateTime;
use models::Post;

#[derive(Debug, Serialize)]
pub struct PostResponse<'a> {
    pub post: &'a Post,
    pub timestamp: i64
}

impl<'a> PostResponse<'a> {

    pub fn from(post: &'a Post) -> PostResponse {
        PostResponse {
            post,
            timestamp: post.publish_at.timestamp(),
        }
    }
}