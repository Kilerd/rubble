use crate::graphql::Mutation;
use crate::pg_pool::DbConn;

use crate::graphql::input::*;

graphql_object!(Mutation: DbConn |&self| {
    field create(&executor, new: NewHuman) -> i32 {
        2
    }
});