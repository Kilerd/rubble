use diesel::prelude::*;
use models::Post;
use pg_pool::DbConn;
use pulldown_cmark::html;
use pulldown_cmark::Parser;
use response::PostResponse;
use rocket::http::Status;
use rocket::response::Failure;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use schema::{posts, users};
use schema::{posts::dsl::*, users::dsl::*};
use std::path::Path;
use std::path::PathBuf;
use tera::Context;


#[get("/")]
fn index(conn: DbConn) -> Template {
    let mut context = Context::new();

    let result = posts::table.filter(published.eq(true)).order(publish_at.desc()).load::<Post>(&*conn).expect("cannot load posts");

    let post_responses: Vec<PostResponse> = result.iter().map(PostResponse::from).collect();

    context.insert("posts", &post_responses);

    Template::render("index", &context)
}

#[get("/archives/<archives_id>")]
fn single_archives(conn: DbConn, archives_id: i32) -> Result<Template, Failure> {
    let mut context = Context::new();

    let result: Result<_, _> = posts::table.find(archives_id).first::<Post>(&*conn);

    if let Err(_err) = result {
        return Err(Failure(Status::NotFound));
    }

    let post: Post = result.unwrap();

    let post_response = PostResponse::from(&post);

    context.insert("post", &post_response);

    Ok(Template::render("archives", &context))
}

#[get("/statics/<file..>")]
fn static_content(file: PathBuf) -> Result<NamedFile, Failure> {
    let path = Path::new("static/resources/").join(file);
    let result = NamedFile::open(&path);
    if let Ok(file) = result {
        Ok(file)
    } else {
        Err(Failure(Status::NotFound))
    }
}

#[get("/<archive_url>", rank = 5)]
fn get_archive_by_url(conn:DbConn, archive_url: String) -> Result<Template, Failure> {

    let mut context = Context::new();
    let result = posts::table.filter(url.eq(archive_url)).first::<Post>(&*conn);
    if let Err(_err) = result {
        return Err(Failure(Status::NotFound));
    }

    let post = result.unwrap();

    let post_response = PostResponse::from(&post);

    context.insert("post", &post_response);

    Ok(Template::render("archives", &context))
}
