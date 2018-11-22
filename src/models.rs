use chrono::NaiveDateTime;
use chrono::prelude::*;
use crate::pg_pool::DbConn;
use crate::request::ArticleEditForm;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::request::FlashMessage;
use crate::schema::{articles, users, tokens, setting};
use rand;

#[derive(Queryable, Debug, Serialize, Insertable, AsChangeset, GraphQLObject)]
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
        use crate::schema::articles::dsl::*;
        use crate::schema::articles;
        if include_unpublished {
            articles::table.order(publish_at.desc()).load::<Article>(&**conn).expect("something wrong")
        } else {
            articles::table.order(publish_at.desc()).filter(published.eq(true)).load::<Article>(&**conn).expect("something wrong")
        }
    }
    pub fn find(fetched_id: i32, conn: &DbConn) -> Result<Article, Error> {
        articles::table.find(fetched_id).first::<Article>(&**conn)
    }

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

    pub fn find_by_id(id:i32, conn: &DbConn) -> Option<User> {
        use crate::schema::users;
        let fetched_user = users::table.filter(users::id.eq(id)).first::<User>(&**conn);
        match fetched_user {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }
    pub fn find_by_username(username:&str, conn: &DbConn) -> Option<User> {
        use crate::schema::users;
        let fetched_user = users::table.filter(users::username.eq(username.to_string())).first::<User>(&**conn);
        match fetched_user {
            Ok(user) => Some(user),
            Err(_) => None,
        }
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

#[derive(Queryable, Debug, Serialize, Insertable, AsChangeset, GraphQLObject)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub value: String,
    pub expire_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[table_name="tokens"]
pub struct NewToken{
    pub user_id: i32,
    pub value: String,
}

const TOKEN_SYMBOLS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";

impl Token {

    pub fn create(user_id: i32, conn: &DbConn) -> Token {
        let token = NewToken{
            user_id: user_id,
            value: Token::rand(64),
        };
        diesel::insert_into(tokens::table).values(&token).get_result(&**conn).expect("can not create token")
    }

    pub fn validate(token:String, conn: &DbConn) -> Option<User> {
        use crate::schema::{tokens, tokens::dsl::*};
        let now = Utc::now().naive_utc();
        let fetched_token = tokens::table.filter(value.eq(token)).filter(expire_at.gt(now)).first::<Token>(&**conn);
        match fetched_token {
            Ok(token) => {
                User::find_by_id(token.user_id, conn)
            },
            Err(_) => None
        }

    }

    pub fn rand(lenth: i32) -> String {
        use rand::Rng;
        let token_string = TOKEN_SYMBOLS.to_string();
        let len = TOKEN_SYMBOLS.len();
        let mut ret = String::new();
        let mut rng = rand::thread_rng();
        for _ in 0..lenth {
            let index: usize = rng.gen_range(0, len);
            ret.push(token_string.chars().nth(index).unwrap());
        }
        ret
    }
}