// pub type Result<T> = std::result::Result<T, Error>;

pub use anyhow::Result;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown error: {0}")]
    Unknown(String),
}
