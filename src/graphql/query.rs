use crate::graphql::Query;
use crate::pg_pool::DbConn;
use crate::models::Article;
use crate::graphql::model::*;

graphql_object!(Query: DbConn |&self| {
    description: "The root query object of the schema"
    
    field users(&executor) -> Vec<i32> as "AllUsers" {
        vec![1, 3, 4]
    }

    field articles(&executor, article_status: Option<ArticlePublishStatus>) -> Vec<Article> as "all article" {
        let db_conn = executor.context();
        Article::load_all(true, &db_conn)
    }
});