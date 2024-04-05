use tera::Context;

use super::templates::Template;

pub async fn about_page() -> Template {
    Template::render("about/index.html", Context::new())
}

pub async fn manifesto_page() -> Template {
    Template::render("about/manifesto.html", Context::new())
}

pub async fn foundation_page() -> Template {
    Template::render("about/foundation.html", Context::new())
}

pub async fn contributing_page() -> Template {
    Template::render("about/contributing.html", Context::new())
}

pub async fn people_page() -> Template {
    Template::render("about/people.html", Context::new())
}
