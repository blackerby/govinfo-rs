use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request error")]
    HttpRequestError(#[from] reqwest::Error),
}
