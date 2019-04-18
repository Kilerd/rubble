use crate::models::CRUD;
use crate::schema::articles;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

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

#[derive(Debug, Insertable, AsChangeset)]
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

impl CRUD<NewArticle, NewArticle, i32> for Article {
    fn create(conn: &PgConnection, from: &NewArticle) -> Result<Self, Error> {
        unimplemented!()
    }

    fn read(conn: &PgConnection) -> Vec<Self> {
        use crate::schema::articles::dsl::*;
        articles::table
            .order(publish_at.desc())
            .load::<Article>(conn)
            .expect("something wrong")
    }

    fn update(conn: &PgConnection, pk: i32, value: &NewArticle) -> Result<Self, Error> {
        unimplemented!()
    }

    fn delete(conn: &PgConnection, pk: i32) -> Result<usize, Error> {
        unimplemented!()
    }

    fn get_by_pk(conn: &PgConnection, pk: i32) -> Result<Self, Error> {
        unimplemented!()
    }
}
