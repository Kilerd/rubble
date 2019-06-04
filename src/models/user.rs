use crate::models::CRUD;
use chrono::NaiveDateTime;
use diesel::{pg::PgConnection, result::Error};

use crate::schema::users;
use crypto::{digest::Digest, sha3::Sha3};
use diesel::prelude::*;

use diesel::{AsChangeset, Insertable, Queryable};
use serde::Serialize;

use crate::{data::RubbleData, utils::jwt::JWTClaims};
use actix_web::{dev::Payload, error, FromRequest, HttpRequest};
use futures::IntoFuture;

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
    type Config = ();
    type Error = actix_web::error::Error;
    type Future = Result<Self, Self::Error>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let tokens: Vec<&str> = req
            .headers()
            .get("Authorization")
            .ok_or(error::ErrorUnauthorized("cannot find authorization header"))?
            .to_str()
            .map_err(|_| error::ErrorBadRequest("error on deserialize token"))?
            .splitn(2, ' ')
            .collect();

        let user_id = JWTClaims::decode(tokens[1].into())
            .map_err(|_| error::ErrorUnauthorized("invalid jwt token"))?;
        let data = req
            .app_data::<RubbleData>()
            .ok_or(error::ErrorBadGateway("error on get rubble data"))?;

        let result = User::find_by_username(&data.postgres(), &user_id)
            .map_err(|_| error::ErrorUnauthorized("error on get user"))?;

        Ok(result)
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
