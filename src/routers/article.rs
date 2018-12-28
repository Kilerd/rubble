use crate::guard::SettingMap;
use crate::models::Article;
use crate::pg_pool::DbConn;
use crate::response::ArticleResponse;
use crate::schema::{articles::dsl::*};
use crate::schema::articles;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::NamedFile;
use rocket_contrib::templates::Template;
use std::path::Path;
use std::path::PathBuf;
use tera::Context;


#[get("/")]
pub fn index(setting: SettingMap, conn: DbConn) -> Template {
    let mut context = Context::new();

    let result = articles::table.filter(published.eq(true)).order(publish_at.desc()).load::<Article>(&*conn).expect("cannot load articles");

    let article_responses: Vec<ArticleResponse> = result.iter().map(ArticleResponse::from).collect();

    context.insert("setting", &setting);
    context.insert("articles", &article_responses);

    Template::render("index", &context)
}

#[get("/archives/<archives_id>")]
pub fn single_article(conn: DbConn, archives_id: i32) -> Result<Template, Status> {
    let mut context = Context::new();

    let result: Result<_, _> = articles::table.find(archives_id).first::<Article>(&*conn);

    if let Err(_err) = result {
        return Err(Status::NotFound);
    }

    let article: Article = result.unwrap();

    let article_response = ArticleResponse::from(&article);

    context.insert("article", &article_response);

    Ok(Template::render("archives", &context))
}

#[get("/statics/<file..>")]
pub fn static_content(file: PathBuf) -> Result<NamedFile, Status> {
    let path = Path::new("static/resources/").join(file);
    let result = NamedFile::open(&path);
    if let Ok(file) = result {
        Ok(file)
    } else {
        Err(Status::NotFound)
    }
}

#[get("/<archive_url>", rank = 5)]
pub fn get_article_by_url(conn: DbConn, archive_url: String) -> Result<Template, Status> {
    let mut context = Context::new();
    let result = articles::table.filter(url.eq(archive_url)).first::<Article>(&*conn);
    if let Err(_err) = result {
        return Err(Status::NotFound);
    }

    let article = result.unwrap();

    let article_response = ArticleResponse::from(&article);

    context.insert("article", &article_response);

    Ok(Template::render("archives", &context))
}
