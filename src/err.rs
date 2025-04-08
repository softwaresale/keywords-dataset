use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error")]
    IO(#[from] std::io::Error),
    #[error("JSON (de)serialization error")]
    JsonDeserialization(#[from] serde_json::Error),
    #[error("PDF Error: {0}")]
    PdfError(#[from] lopdf::Error),
    #[error("Other error: {0}")]
    Other(String),
}

pub type AppResult<T> = Result<T, AppError>;
