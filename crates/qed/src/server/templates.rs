use axum::response::{Html, IntoResponse};
use tera::Tera;

pub use tera::Context;

lazy_static::lazy_static! {
    static ref TERA: Tera = Tera::new("templates/**/*").expect("Failed to compile templates");
}

pub struct Template {
    path: String,
    context: Context,
}

impl Template {
    pub fn render(path: &str, context: Context) -> Self {
        Self {
            path: path.to_owned(),
            context,
        }
    }
}

impl IntoResponse for Template {
    fn into_response(self) -> axum::response::Response {
        let rendered = TERA.render(&self.path, &self.context);

        match rendered {
            Ok(r) => Html(r).into_response(),
            Err(r) => r.to_string().into_response(),
        }
    }
}
