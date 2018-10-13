use std::time::SystemTime;
use chrono::NaiveDate;
use chrono::NaiveTime;
use serde::Serialize;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug, Serialize)]
#[belongs_to(User)]
#[table_name= "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: i32,
    pub publish_at: NaiveDateTime,
    pub url: Option<String>,
}

#[derive(Queryable, Debug, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub create_at: NaiveDateTime,
    pub last_login_at: NaiveDateTime,
}

impl User {

    pub fn authenticated(&self, password: &str) -> bool {
        true
    }
}
