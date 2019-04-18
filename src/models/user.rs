use crate::models::CRUD;
use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::result::Error;

use crate::schema::users;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use diesel::prelude::*;

use diesel::{AsChangeset, Insertable, Queryable};
use serde::Serialize;

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

    pub fn find_by_username(conn: &PgConnection, username: &str) -> Result<Self, Error> {
        users::table
            .filter(users::username.eq(username.to_string()))
            .first::<User>(conn)
    }
}

impl CRUD<(), (), i32> for User {
    fn create(conn: &PgConnection, from: &()) -> Result<Self, Error> {
        unreachable!()
    }

    fn read(conn: &PgConnection) -> Vec<Self> {
        unreachable!()
    }

    fn update(conn: &PgConnection, pk: i32, value: &()) -> Result<Self, Error> {
        unreachable!()
    }

    fn delete(conn: &PgConnection, pk: i32) -> Result<usize, Error> {
        unreachable!()
    }

    fn get_by_pk(conn: &PgConnection, pk: i32) -> Result<Self, Error> {
        users::table.filter(users::id.eq(pk)).first::<User>(conn)
    }
}
