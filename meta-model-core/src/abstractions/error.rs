// Abstraction layer errors

use thiserror::Error;

pub type AbstractionResult<T> = Result<T, AbstractionError>;

#[derive(Error, Debug)]
pub enum AbstractionError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Transform error: {0}")]
    TransformError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch {
        expected: String,
        actual: String,
    },

    #[error("Calendar error: {0}")]
    CalendarError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("XML error: {0}")]
    XmlError(String),
}

impl From<quick_xml::Error> for AbstractionError {
    fn from(err: quick_xml::Error) -> Self {
        AbstractionError::XmlError(err.to_string())
    }
}