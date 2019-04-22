use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2;
use std::ops::Deref;

pub type ManagedPgConn = ConnectionManager<PgConnection>;

pub type Pool = r2d2::Pool<ManagedPgConn>;

pub struct DbConn(pub r2d2::PooledConnection<ManagedPgConn>);

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn database_pool_establish(database_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("Failed to create pool.")
}
