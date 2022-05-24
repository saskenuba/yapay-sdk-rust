use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SDKError {
    #[error("{0}")]
    CredentialsError(String),

    #[error(transparent)]
    NetworkError(#[from] reqwest::Error),

    #[error(transparent)]
    InvalidError(#[from] InvalidError),

    #[error("{0:?}")]
    PayloadError(ApiError),

    #[error("Something wrong happened.")]
    GenericError,
}

#[derive(Error, Debug)]
pub enum CreditCardError {
    #[error("Card has expired.")]
    Expired,
}

#[derive(Error, Debug)]
pub enum InvalidError {
    #[error("Card has expired.")]
    CreditCardExpired,

    #[error("Item validation error: {0}")]
    ItemError(String),

    #[error(transparent)]
    URLError(#[from] reqwest::Error),

    #[error(transparent)]
    ValidatorLibError(#[from] validator::ValidationErrors),
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub status: i32,
    pub error: String,
    pub cause: Option<Vec<ErrorCause>>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorCause {
    pub description: String,
    pub code: String,
}
