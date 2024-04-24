#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown error: `{0}`")]
    Unknown(String),
}
