use serde::Serialize;

use self::error::Error;

mod error;

#[derive(Debug, Serialize)]
pub struct SearchResult {
    value: String
}

pub fn search_query() -> Result<Vec<SearchResult>, Error> {
    Ok(vec![
        SearchResult { value: "Result 1".into() },
        SearchResult { value: "Result 2".into() },
        SearchResult { value: "Result 3".into() },
        SearchResult { value: "Result 4".into() },
    ])
}
