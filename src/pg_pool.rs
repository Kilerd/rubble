use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use r2d2;

pub type ManagedPgConn = ConnectionManager<PgConnection>;

pub type Pool = r2d2::Pool<ManagedPgConn>;

pub fn database_pool_establish(database_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("Failed to create pool.")
}
