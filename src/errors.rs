use thiserror::Error;

pub type Result<T> = std::result::Result<T, TasksError>;

#[derive(Error, Debug)]
pub enum TasksError {
    #[error("http error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("http middleware error: {0}")]
    MiddlewareError(#[from] reqwest_middleware::Error),

    #[error("JSON error: {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("invalid response: {0}")]
    ResponseError(String),
}
