use rocket::response::content;
use crate::pg_pool::DbConn;
use rocket::State;
use crate::graphql::Schema;
use juniper_rocket::{GraphQLRequest, GraphQLResponse};
use rocket_contrib::json::Json;
use rocket::request::Form;
use crate::request::LoginForm;
use rocket::response::Failure;
use rocket::http::Status;
use crate::models::{User, Token};

#[get("/graphiql")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[post("/graphql/authorization", data = "<user>")]
pub fn graphql_authorization(user: Form<LoginForm>, conn: DbConn) -> Result<Json<Token>, Failure> {
    let user_form = user.get();
    let fetched_user = User::find_by_username(&user_form.username, &conn);

    if let None = fetched_user {
        return Err(Failure(Status::Unauthorized));
    }
    let user: User = fetched_user.unwrap();

    if !user.authenticated(user_form.password.as_str()) {
        return Err(Failure(Status::Unauthorized));
    }
    Ok(Json(Token::create(user.id, &conn)))
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