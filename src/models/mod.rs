use diesel::{pg::PgConnection, result::Error};

pub mod article;
pub mod setting;
pub mod token;
pub mod user;
pub trait CRUD<CreatedModel, UpdateModel, PK> {
    fn create(conn: &PgConnection, from: &CreatedModel) -> Result<Self, Error>
    where
        Self: Sized;

    fn read(conn: &PgConnection) -> Vec<Self>
    where
        Self: Sized;

    fn update(conn: &PgConnection, pk: PK, value: &UpdateModel) -> Result<Self, Error>
    where
        Self: Sized;

    fn delete(conn: &PgConnection, pk: PK) -> Result<usize, Error>
    where
        Self: Sized;

    fn get_by_pk(conn: &PgConnection, pk: PK) -> Result<Self, Error>
    where
        Self: Sized;
}
