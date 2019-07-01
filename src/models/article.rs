use crate::{models::CRUD, schema::articles};
use chrono::NaiveDateTime;
use diesel::{pg::PgConnection, prelude::*, result::Error};

use diesel::{query_builder::AsChangeset, sql_types::Integer, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: i32,
    pub publish_at: NaiveDateTime,
    pub url: Option<String>,
    pub keywords: Vec<String>,
    pub view: i32,
}
//
#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "articles"]
pub struct NewArticle {
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: i32,
    pub publish_at: Option<NaiveDateTime>,
    pub url: Option<String>,
    pub keywords: Vec<String>,
}

impl Article {
    pub fn link(&self) -> String {
        match self.url {
            Some(ref to) if to.len() != 0 => format!("/{}", to),
            _ => format!("/archives/{}", self.id),
        }
    }

    pub fn find_by_url(conn: &PgConnection, url: &str) -> Result<Self, Error> {
        articles::table
            .filter(articles::url.eq(url))
            .filter(articles::published.eq(true))
            .first::<Article>(conn)
    }

    pub fn increase_view(&self, conn: &PgConnection) {
        diesel::sql_query(r#"UPDATE articles SET "view" = "view" + 1 where articles.id = $1"#)
            .bind::<Integer, _>(self.id)
            .execute(conn)
            .expect("error on incr view");
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

pub mod form {
    use crate::models::article::NewArticle;
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct NewArticleFrom {
        pub title: String,
        pub body: String,
        pub published: bool,
        pub user_id: i32,
        pub publish_at: Option<NaiveDateTime>,
        pub url: Option<String>,
        pub keywords: String,
    }

    impl From<NewArticleFrom> for NewArticle {
        fn from(form: NewArticleFrom) -> Self {
            Self {
                title: form.title,
                body: form.body,
                published: form.published,
                user_id: form.user_id,
                publish_at: form.publish_at,
                url: form.url,
                keywords: if form.keywords.is_empty() {
                    vec![]
                } else {
                    form.keywords.split(",").map(String::from).collect()
                },
            }
        }
    }
}

pub mod view {
    use crate::models::article::Article;
    use pulldown_cmark::{html, Parser};
    use serde::Serialize;
    use std::ops::Deref;

    #[derive(Debug, Serialize)]
    pub struct ArticleView<'a> {
        article: &'a Article,
        pub timestamp: i64,
        pub markdown_content: String,
        pub description: String,
    }

    impl<'a> Deref for ArticleView<'a> {
        type Target = Article;

        fn deref(&self) -> &Self::Target {
            self.article
        }
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
}
