use axum::extract::Query;
use serde::Deserialize;
use tera::Context;

use crate::search::search_query;

use super::templates::Template;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    query: String
}

pub async fn search_handler(Query(query): Query<SearchQuery>) -> Template {
    let results = search_query().unwrap();
    
    let mut context = Context::new();
    context.insert("query", &query.query);
    context.insert("results", &results);

    Template::render("search/index.html", context)
}

pub async fn advanced_search_handler() -> Template {
    Template::render("search/advanced.html", Context::new())
}
