use tera::Context;

use super::templates::Template;

pub async fn search_handler() -> Template {
    Template::render("search/index.html", Context::new())
}

pub async fn advanced_search_handler() -> Template {
    Template::render("search/advanced.html", Context::new())
}
