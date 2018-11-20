use rocket::response::content;
use crate::pg_pool::DbConn;
use rocket::State;
use crate::graphql::Schema;
use juniper_rocket::{GraphQLRequest, GraphQLResponse};


#[get("/graphiql")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(context: DbConn, request: GraphQLRequest, state: State<Schema>) -> GraphQLResponse {
    let schema = state;
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(context: DbConn, request: GraphQLRequest, state: State<Schema>) -> GraphQLResponse {
    let schema = state;
    request.execute(&schema, &context)
}