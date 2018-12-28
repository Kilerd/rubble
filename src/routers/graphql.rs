use rocket::response::content;
use crate::pg_pool::DbConn;
use rocket::State;
use crate::graphql::Schema;
use juniper_rocket::{GraphQLRequest, GraphQLResponse};
use rocket_contrib::json::Json;
use crate::request::{LoginForm, AdminToken};
use rocket::http::Status;
use crate::models::{User, Token};
use rocket::request::Form;

#[get("/graphiql")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[post("/graphql/authorization", data = "<user>")]
pub fn graphql_authorization(user: Form<LoginForm>, conn: DbConn) -> Result<Json<Token>, Status> {
    let fetched_user = User::find_by_username(&user.username, &conn);

    if let None = fetched_user {
        return Err(Status::Unauthorized);
    }
    let user: User = fetched_user.unwrap();

    if !user.authenticated(user.password.as_str()) {
        return Err(Status::Unauthorized);
    }
    Ok(Json(Token::create(user.id, &conn)))
}


#[get("/graphql?<request>")]
pub fn get_graphql_handler(token: AdminToken, context: DbConn, request: GraphQLRequest, state: State<Schema>) -> GraphQLResponse {
    let schema = state;
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(token: AdminToken, context: DbConn, request: GraphQLRequest, state: State<Schema>) -> GraphQLResponse {
    let schema = state;
    request.execute(&schema, &context)
}