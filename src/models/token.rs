use diesel::{AsChangeset, Insertable, Queryable};
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub value: String,
    pub expire_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "tokens"]
pub struct NewToken {
    pub user_id: i32,
    pub value: String,
}

const TOKEN_SYMBOLS: &'static str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";

impl CRUD<(), NewToken, i32> for Token {}

impl Token {
    //    pub fn create(user_id: i32, conn: &DbConn) -> Token {
    //        let token = NewToken {
    //            user_id,
    //            value: Token::rand(64),
    //        };
    //        diesel::insert_into(tokens::table).values(&token).get_result(&**conn).expect("can not create token")
    //    }

    pub fn validate(token: String, conn: &DbConn) -> Option<User> {
        use crate::schema::{tokens, tokens::dsl::*};
        let now = Utc::now().naive_utc();
        let fetched_token = tokens::table
            .filter(value.eq(token))
            .filter(expire_at.gt(now))
            .first::<Token>(&**conn);
        match fetched_token {
            Ok(token) => User::find_by_id(token.user_id, conn),
            Err(_) => None,
        }
    }

    pub fn rand(length: i32) -> String {
        use rand::prelude::SliceRandom;
        let mut rng = rand::thread_rng();
        let v: Vec<u8> = TOKEN_SYMBOLS
            .as_bytes()
            .choose_multiple(&mut rng, length as usize)
            .cloned()
            .collect();
        String::from_utf8(v).expect("error on generating token")
    }
}
