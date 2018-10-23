use diesel::prelude::*;
use guard::SettingMap;
use models::Article;
use pg_pool::DbConn;
use response::ArticleResponse;
use rocket::http::Status;
use rocket::response::Failure;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use schema::{articles::dsl::*};
use schema::articles;
use std::path::Path;
use std::path::PathBuf;
use tera::Context;


#[get("/")]
fn index(setting: SettingMap, conn: DbConn) -> Template {
    let mut context = Context::new();

    let result = articles::table.filter(published.eq(true)).order(publish_at.desc()).load::<Article>(&*conn).expect("cannot load articles");

    let article_responses: Vec<ArticleResponse> = result.iter().map(ArticleResponse::from).collect();

    println!("{:?}", setting);
    context.insert("setting", &setting);
    context.insert("articles", &article_responses);

    Template::render("index", &context)
}

#[get("/archives/<archives_id>")]
fn single_article(conn: DbConn, archives_id: i32) -> Result<Template, Failure> {
    let mut context = Context::new();

    let result: Result<_, _> = articles::table.find(archives_id).first::<Article>(&*conn);

    if let Err(_err) = result {
        return Err(Failure(Status::NotFound));
    }

    let article: Article = result.unwrap();

    let article_response = ArticleResponse::from(&article);

    context.insert("article", &article_response);

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
fn get_article_by_url(conn: DbConn, archive_url: String) -> Result<Template, Failure> {
    let mut context = Context::new();
    let result = articles::table.filter(url.eq(archive_url)).first::<Article>(&*conn);
    if let Err(_err) = result {
        return Err(Failure(Status::NotFound));
    }

    let article = result.unwrap();

    let article_response = ArticleResponse::from(&article);

    context.insert("article", &article_response);

    Ok(Template::render("archives", &context))
}
