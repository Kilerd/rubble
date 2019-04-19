use crate::models::CRUD;
use crate::schema::articles;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use diesel::query_builder::AsChangeset;
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
#[table_name = "articles"]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: i32,
    pub publish_at: NaiveDateTime,
    pub url: Option<String>,
}

#[derive(Debug, Insertable, AsChangeset, Serialize)]
#[table_name = "articles"]
pub struct NewArticle {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: i32,
    pub publish_at: NaiveDateTime,
    pub url: Option<String>,
}

impl Article {
    pub fn find_by_url(conn: &PgConnection, url: &str) -> Result<Self, Error> {
        articles::table
            .filter(articles::url.eq(url))
            .filter(articles::published.eq(true))
            .first::<Article>(conn)
    }
}

impl CRUD<NewArticle, NewArticle, i32> for Article {
    fn create(conn: &PgConnection, from: &NewArticle) -> Result<Self, Error> {
        diesel::insert_into(articles::table)
            .values(from)
            .get_result(conn)
    }

    fn read(conn: &PgConnection) -> Vec<Self> {
        articles::table
            .order(articles::publish_at.desc())
            .load::<Self>(conn)
            .expect("something wrong")
    }

    fn update(conn: &PgConnection, pk: i32, value: &NewArticle) -> Result<Self, Error> {
        diesel::update(articles::table.find(pk))
            .set(value)
            .get_result(conn)
    }

    fn delete(conn: &PgConnection, pk: i32) -> Result<usize, Error> {
        diesel::delete(articles::table.filter(articles::id.eq(pk))).execute(conn)
    }

    fn get_by_pk(conn: &PgConnection, pk: i32) -> Result<Self, Error> {
        articles::table.find(pk).first::<Article>(conn)
    }
}
