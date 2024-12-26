pub mod user_post;

pub use user_post::UserPostAPI;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("network error")]
    Network(#[from] reqwest::Error),
    #[error("deserialization error")]
    Deserialization(#[from] serde_json::Error),
    #[error("business error: {retcode} {message}")]
    Business { retcode: i32, message: String },
}

type Result<T> = std::result::Result<T, Error>;
