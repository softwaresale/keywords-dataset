use std::str::Utf8Error;
use pdf_extract::OutputError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error")]
    IO(#[from] std::io::Error),
    #[error("JSON (de)serialization error")]
    JsonDeserialization(#[from] serde_json::Error),
    #[error("PDF extraction error: {0}")]
    PdfError(#[from] OutputError),
    #[error("No keywords section")]
    NoKeywords,
    #[error("UTF8 Error: {0}")]
    Utf8Error(#[from] Utf8Error),
    #[error("SQL Error: {0}")]
    DbError(#[from] rusqlite::Error),
    #[error("Other error: {0}")]
    Other(String),
}

pub type AppResult<T> = Result<T, AppError>;
