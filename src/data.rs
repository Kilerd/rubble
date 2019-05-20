use crate::pg_pool::{ManagedPgConn, Pool};
use r2d2::PooledConnection;
use std::sync::Arc;
use tera::{Context, Tera};

#[derive(Clone)]
pub struct RubbleData {
    pub pool: Pool,
    pub tera: Arc<Tera>,
}

impl RubbleData {
    pub fn postgres(&self) -> PooledConnection<ManagedPgConn> {
        let pool = self.pool.clone();
        pool.get().unwrap()
    }
    pub fn render(&self, template_name: &str, data: &Context) -> String {
        self.tera.render(template_name, data).unwrap()
    }
}
