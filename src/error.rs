use thiserror::Error;

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    Utf8StrError(#[from] std::str::Utf8Error),
    #[error(transparent)]
    Utf8StringError(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Couldn't find Internet Gateway Device (IGD)!")]
    SearchError(String),
    #[cfg(feature = "reqwest")]
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[cfg(feature = "esp32")]
    #[error(transparent)]
    EspError(#[from] esp_idf_sys::EspError),
    #[cfg(feature = "esp32")]
    #[error(transparent)]
    RequestError(#[from] esp_idf_svc::errors::EspIOError),
    #[error(transparent)]
    DeserializationError(#[from] serde_xml_rs::Error),
    #[error("Unsuccessful status code: {status_code}\n{message}")]
    StatusCodeError { status_code: u16, message: String },
    #[error("Unknown Error")]
    UnknownError,
}
