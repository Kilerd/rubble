use diesel::pg::{Pg, PgConnection};
use diesel::result::Error;

pub trait CRUD<CreatedModel, UpdateModel, PK> {
    fn create(conn: &PgConnection, from: &CreatedModel) -> Result<Self, Error>;
    fn read(conn: &PgConnection) -> Vec<Self>;
    fn update(conn: &PgConnection, pk: PK, value: &UpdateModel) -> Result<Self, Error>;
    fn delete(conn: &PgConnection, pk: PK) -> Result<usize, Error>;
    fn get_by_pk(conn: &PgConnection, pk: PK) -> Result<Self, Error>;
}
