use crate::graphql::Query;
use crate::pg_pool::DbConn;

graphql_object!(Query: DbConn |&self| {
    description: "The root query object of the schema"
    
    field users(&executor) -> Vec<i32> as "AllUsers" {
        vec![1, 3, 4]
    }
});