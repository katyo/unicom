use std::{
    error::Error as StdError,
    result::Result as StdResult,
    fmt::{Display, Formatter, Result as FmtResult},
};
use url::{Url, ParseError as UrlError};

/// Result type
pub type Result<T> = StdResult<T, Error>;

/// Error type
#[derive(Debug, Clone)]
pub enum Error {
    /// The backend with same name already registered
    AlreadyRegistered(String),

    /// Invalid URL
    InvalidUrl(UrlError),

    /// The URL cannot be handled by registered backends
    UnsupportedUrl(Url),

    /// Connection error occurred
    ConnectionError(String),
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        pub use Error::*;
        match self {
            AlreadyRegistered(name) => { "Backend already registered: ".fmt(f)?; name.fmt(f) },
            InvalidUrl(error) => { "Unable to parse URL: ".fmt(f)?; error.fmt(f) },
            UnsupportedUrl(url) => { "Unable to handle specified URL: ".fmt(f)?; url.fmt(f) },
            ConnectionError(error) => { "Connection error: ".fmt(f)?; error.fmt(f) },
        }
    }
}

impl From<Url> for Error {
    fn from(url: Url) -> Self {
        Self::UnsupportedUrl(url)
    }
}

impl From<UrlError> for Error {
    fn from(error: UrlError) -> Self {
        Self::InvalidUrl(error)
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Self::ConnectionError(error)
    }
}
