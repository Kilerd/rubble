use actix_web::{dev::Payload, error, FromRequest, HttpRequest};
use chrono::NaiveDateTime;
use crypto::{digest::Digest, sha3::Sha3};
use diesel::{pg::PgConnection, result::Error};
use diesel::{AsChangeset, Insertable, Queryable};
use diesel::prelude::*;
use futures::future::{err, ok, Ready};
use serde::Serialize;

use crate::{data::RubbleData, utils::jwt::JWTClaims};
use crate::models::CRUD;
use crate::schema::users;
use crate::error::RubbleError;

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
        self.password.eq(&result)
    }

    pub fn password_generate(password: &str) -> String {
        let mut hasher = Sha3::sha3_256();
        hasher.input_str(password);
        hasher.result_str()
    }

    pub fn find_by_username(conn: &PgConnection, username: &str) -> Result<Self, Error> {
        users::table
            .filter(users::username.eq(username.to_string()))
            .first::<User>(conn)
    }
}

impl CRUD<(), User, i32> for User {
    fn create(conn: &PgConnection, from: &()) -> Result<Self, Error> {
        unreachable!()
    }

    fn read(conn: &PgConnection) -> Vec<Self> {
        unreachable!()
    }

    fn update(conn: &PgConnection, pk: i32, value: &User) -> Result<Self, Error> {
        diesel::update(users::table.find(pk))
            .set(value)
            .get_result(conn)
    }

    fn delete(conn: &PgConnection, pk: i32) -> Result<usize, Error> {
        unreachable!()
    }

    fn get_by_pk(conn: &PgConnection, pk: i32) -> Result<Self, Error> {
        users::table.filter(users::id.eq(pk)).first::<User>(conn)
    }
}

impl FromRequest for User {
    type Error = RubbleError<&'static str>;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {

        let data = req.app_data::<RubbleData>().expect("cannot get app data");
        let user = req
            .headers()
            .get("Authorization")
            .ok_or(RubbleError::Unauthorized("cannot find authorization header"))
            .and_then(|header| {
                header.to_str().map_err(|_| RubbleError::BadRequest("error on deserialize token"))
            })
            .map(|header| header.splitn(2, ' ').collect::<Vec<&str>>())
            .and_then(|tokens| {
                if tokens.len() == 2 {
                    Ok(tokens[1])
                }else {
                    Err(RubbleError::BadRequest("error on deserialize token"))
                }
            })
            .and_then(|jwt| {
                JWTClaims::decode(jwt.into()).map_err(|_| RubbleError::Unauthorized("invalid jwt token"))
            })
            .and_then(|user_id| {
                User::find_by_username(&data.postgres(), &user_id)
                    .map_err(|_| RubbleError::Unauthorized("error on get user"))
            });


        match user {
            Ok(user) => ok(user),
            Err(e) => err(e)
        }

    }
}

pub mod input {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct LoginForm {
        pub username: String,
        pub password: String,
    }
}
