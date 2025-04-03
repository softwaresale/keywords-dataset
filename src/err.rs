use std::str::Utf8Error;
use pdf_extract::OutputError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
    #[error("JSON (de)serialization error: {0}")]
    JsonDeserialization(#[from] serde_json::Error),
    #[error("PDF extraction error: {0}")]
    PdfError(#[from] OutputError),
    #[error("No keywords section")]
    NoKeywords,
    #[error("Desired section '{0}' is missing from paper")]
    MissingSection(String),
    #[error("UTF8 Error: {0}")]
    Utf8Error(#[from] Utf8Error),
    #[error("SQL Error: {0}")]
    DbError(#[from] rusqlite::Error),
    #[error("Networking error: {0}")]
    NetworkError(#[from] ureq::Error),
    #[error("Invalid HTTP response status code: {0}")]
    HttpStatusError(ureq::http::status::StatusCode),
    #[error("No GCS bucket object for arxiv id {0}")]
    NoBucketObject(String),
    #[error("Other error: {0}")]
    Other(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    pub fn extraction_status_code(&self) -> &'static str {
        match self {
            AppError::IO(_) => "IO",
            AppError::JsonDeserialization(_) => "JSON_DESER",
            AppError::PdfError(_) => "PDF",
            AppError::NoKeywords => "NO_KEYWORDS",
            AppError::MissingSection(_) => "MISSING_SECTION",
            AppError::Utf8Error(_) => "UTF8",
            AppError::DbError(_) => "DB",
            AppError::NetworkError(_) => "NETWORK",
            AppError::HttpStatusError(_) => "HTTP_STAT",
            AppError::NoBucketObject(_) => "NO_GCS_OBJ",
            AppError::Other(_) => "OTHER"
        }
    }
}
