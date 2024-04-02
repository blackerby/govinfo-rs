use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request error")]
    HttpRequestError(#[from] reqwest::Error),
    #[error("Unsupported endpoint: {0}")]
    UnsupportedEndpoint(String),
    #[error("Invalid endpoint parameter: {0}")]
    InvalidPathParam(String),
    #[error("Invalid Congress parameter: {0}")]
    InvalidCongressParam(usize),
    #[error("Invalid GovInfo collection: {0}")]
    InvalidCollectionParam(String),
}
