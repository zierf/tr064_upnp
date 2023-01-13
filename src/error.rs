use thiserror::Error;

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    DeserializationError(#[from] serde_xml_rs::Error),
    #[error("Unsuccessful status code: {status_code}\n{message}")]
    StatusCodeError { status_code: u16, message: String },
    #[error("Unknown Error")]
    UnknownError,
}
