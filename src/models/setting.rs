use crate::models::CRUD;
use crate::schema::setting;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::Serialize;
#[derive(Queryable, Debug, Serialize, Insertable, AsChangeset)]
#[table_name = "setting"]
pub struct Setting {
    pub name: String,
    pub value: Option<String>,
}

impl CRUD<(), Setting, String> for Setting {
    fn create(conn: &PgConnection, from: &()) -> Result<Self, Error> {
        unimplemented!()
    }

    fn read(conn: &PgConnection) -> Vec<Self> {
        unimplemented!()
    }

    fn update(conn: &PgConnection, pk: String, value: &Setting) -> Result<Self, Error> {
        diesel::update(setting::table.find(&pk))
            .set(value)
            .get_result(conn)
    }

    fn delete(conn: &PgConnection, pk: String) -> Result<usize, Error> {
        unimplemented!()
    }

    fn get_by_pk(conn: &PgConnection, pk: String) -> Result<Self, Error> {
        unimplemented!()
    }
}
