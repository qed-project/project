use tera::Context;

use super::templates::Template;

pub async fn legal_docs_page() -> Template {
    Template::render("legal/index.html", Context::new())
}

pub async fn cookie_policy_page() -> Template {
    Template::render("legal/cookies.html", Context::new())
}

pub async fn terms_of_service_page() -> Template {
    Template::render("legal/terms.html", Context::new())
}

pub async fn privacy_policy_page() -> Template {
    Template::render("legal/privacy.html", Context::new())
}

pub async fn security_policy_page() -> Template {
    Template::render("legal/security.html", Context::new())
}
