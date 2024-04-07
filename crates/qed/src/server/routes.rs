use axum::{routing::get, Router};
use hyper::StatusCode;
use tera::Context;
use tower_http::services::ServeDir;

use super::templates::Template;

pub fn build_routes() -> Router {
    Router::new()
        .route("/", get(homepage_handler))
        // about
        .route("/about", get(super::about::about_page))
        .route("/about/manifesto", get(super::about::manifesto_page))
        .route("/about/foundation", get(super::about::foundation_page))
        .route("/about/contributing", get(super::about::contributing_page))
        .route("/about/people", get(super::about::people_page))
        // search
        .route("/search", get(super::search::search_handler))
        .route("/search/advanced", get(super::search::advanced_search_handler))
        // legal
        .route("/legal", get(super::legal::legal_docs_page))
        .route("/legal/cookies", get(super::legal::cookie_policy_page))
        .route("/legal/terms", get(super::legal::terms_of_service_page))
        .route("/legal/privacy", get(super::legal::privacy_policy_page))
        .route("/legal/security", get(super::legal::security_policy_page))
        // static files
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(fallback_handler)
}

async fn homepage_handler() -> Template {
    Template::render("index.html", Context::new())
}

async fn fallback_handler() -> (StatusCode, Template) {
    (StatusCode::NOT_FOUND, Template::render("error/404.html", Context::new()))
}
