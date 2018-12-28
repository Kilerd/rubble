use rocket::http::Status;
use rocket::outcome::Outcome::Failure;
use rocket::outcome::Outcome::Success;
use rocket::Request;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::State;

use crate::models::Token;
use crate::pg_pool::{DbConn, Pool};

#[derive(Debug, FromForm)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, FromForm, Debug)]
pub struct Admin {
    pub id: i32,
    pub username: String,
}

#[derive(Debug)]
pub struct AdminToken {
    pub admin: Admin,
    pub token: String,
}

#[derive(Debug, FromForm)]
pub struct NewPasswordForm {
    pub password: String,
}

#[derive(Debug, FromForm)]
pub struct ArticleEditForm {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub publish_at: String,
    pub url: Option<String>,
}

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        let is_admin = request.cookies().get_private("LOG_ADMIN");
        let user_id = request.cookies().get_private("LOG_ID");
        let username = request.cookies().get_private("LOG_SESSION");

        if let Some(_flag) = is_admin {
            Success(Admin {
                username: username.unwrap().value().to_string(),
                id: user_id.unwrap().value().to_string().parse::<i32>().unwrap(),
            })
        } else {
            Failure((Status::Unauthorized, ()))
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        let pool = request.guard::<State<Pool>>()?;
        let conn: DbConn;
        match pool.get() {
            Ok(pool_conn) => {
                conn = DbConn(pool_conn);
            }
            Err(_) => return Failure((Status::ServiceUnavailable, ())),
        };

        let authorization = request.headers().get_one("Authorization");
        match authorization {
            Some(token) => {
                let tokens: Vec<&str> = token.split(' ').collect();
                if tokens.len() != 2 {
                    return Failure((Status::Unauthorized, ()));
                }
                match Token::validate(tokens[1].to_string(), &conn) {
                    Some(user) => Success(AdminToken {
                            admin: Admin {
                                id: user.id,
                                username: user.username,
                            },
                            token: tokens[1].to_string(),
                        }),
                    None => Failure((Status::Unauthorized, ())),
                }
            }
            None => Failure((Status::Unauthorized, ())),
        }
    }
}
