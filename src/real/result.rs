use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};
use url::{ParseError as UrlError, Url};

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

    /// Name resolving error
    FailedResolve(String),

    /// Unable to connect
    FailedConnect(String),
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        pub use Error::*;
        match self {
            AlreadyRegistered(name) => {
                "Backend already registered: ".fmt(f)?;
                name.fmt(f)
            }
            InvalidUrl(error) => {
                "Unable to parse URL: ".fmt(f)?;
                error.fmt(f)
            }
            UnsupportedUrl(url) => {
                "Unable to handle specified URL: ".fmt(f)?;
                url.fmt(f)
            }
            FailedResolve(error) => {
                "Unable to resolve name: ".fmt(f)?;
                error.fmt(f)
            }
            FailedConnect(error) => {
                "Unable to connect: ".fmt(f)?;
                error.fmt(f)
            }
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
        Self::FailedConnect(error)
    }
}
