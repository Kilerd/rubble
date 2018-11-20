use juniper::RootNode;
use juniper::Context;

pub mod model;
pub mod input;
pub mod query;
pub mod mutation;

use crate::pg_pool::DbConn;


pub struct Query;
pub struct Mutation;

impl Context for DbConn {}

pub type Schema = RootNode<'static, Query, Mutation>;