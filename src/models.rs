use chrono::NaiveDateTime;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use diesel::prelude::*;
use schema::articles::dsl::*;
use schema::articles;
use pg_pool::DbConn;
use diesel::result::Error;
use request::ArticleEditForm;
use chrono::prelude::*;
use schema::users;
use rocket::request::FlashMessage;
use schema::setting;

#[derive(Queryable, Debug, Serialize, Insertable, AsChangeset)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub user_id: i32,
    pub publish_at: NaiveDateTime,
    pub url: Option<String>,
}

impl Article {
    pub fn load_all(include_unpublished: bool, conn: &DbConn) -> Vec<Article> {
        if include_unpublished {
            articles::table.order(publish_at.desc()).load::<Article>(&**conn).expect("something wrong")
        } else {
            articles::table.order(publish_at.desc()).filter(published.eq(true)).load::<Article>(&**conn).expect("something wrong")
        }
    }
    pub fn find(fetched_id: i32, conn: &DbConn) -> Result<Article, Error> {
        articles::table.find(fetched_id).first::<Article>(&**conn)
    }

//    pub fn new(article: ArticleEditForm) -> Post {
//    }

    pub fn form_article_edit_form(article: &ArticleEditForm, current_user_id: i32) -> NewArticle {
        let timestamp = if article.publish_at.eq("") {
            Utc::now().timestamp()
        } else {
            NaiveDateTime::parse_from_str(&article.publish_at, "%Y-%m-%dT%H:%M").unwrap().timestamp()
        };

        let article_id = match article.id {
            Some(-1) => None,
            Some(i) => Some(i),
            _ => None,
        };
        NewArticle {
            id: article_id,
            title: article.title.clone(),
            body: article.body.clone(),
            published: article.published,
            user_id: current_user_id,
            publish_at: NaiveDateTime::from_timestamp(timestamp, 0),
            url: article.url.clone(),
        }
    }
}

#[derive(Insertable, AsChangeset)]
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

#[derive(Queryable, Debug, Serialize, Insertable, AsChangeset)]
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
        let mut hasher = Sha3::sha3_256();
        hasher.input_str(password);
        let result = hasher.result_str();

        if self.password.eq(&result) {
            true
        } else {
            false
        }
    }

    pub fn password_generate(password: &str) -> String {
        let mut hasher = Sha3::sha3_256();
        hasher.input_str(password);
        hasher.result_str()
    }
}

#[derive_FromForm]
#[derive(Queryable, Debug, Serialize, Insertable, AsChangeset)]
#[table_name = "setting"]
pub struct Setting {
    pub name: String,
    pub value: Option<String>,
}


#[derive(Debug, Serialize)]
pub struct SerializeFlashMessage <'a> {
    pub name: &'a str,
    pub message: &'a str,
}

impl <'a> SerializeFlashMessage<'a> {

    pub fn from(flash: &'a Option<FlashMessage>) -> Option<Self> {
        match flash {
            None => None,
            Some(f) => Some(SerializeFlashMessage{ name: &f.name().clone(), message: &f.msg().clone() })
        }
    }
}