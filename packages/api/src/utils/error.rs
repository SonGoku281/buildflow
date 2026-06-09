use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(&'static str),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Auth error: {0}")]
    Auth(&'static str),

    #[error("Payment error: {0}")]
    Payment(&'static str),
}
