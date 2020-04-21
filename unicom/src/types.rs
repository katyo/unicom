use std::{
    pin::Pin,
    ops::Deref,
    sync::Arc,
};
use std::future::Future;

#[cfg(feature = "futures")]
use futures_io::{AsyncRead, AsyncWrite};

#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead, AsyncWrite};

#[cfg(feature = "async-std")]
use async_std::io::{Read as AsyncRead, Write as AsyncWrite};

use crate::{Result, Url};

/// Boxed connection
pub type BoxedConnection = Box<dyn Connection>;

/// The establishing connection between backend and device
pub type BoxedConnect = Pin<Box<dyn Future<Output = Result<BoxedConnection>> + Send>>;

/// Boxed backend
pub type BoxedConnector = Arc<dyn Connector>;

/// Boxed backend type
pub type BoxedBackend = Box<dyn Backend>;

/// The established connection between backend and device
pub trait Connection: AsyncRead + AsyncWrite + Send + Sync + Unpin {}
impl<T> Connection for T where T: AsyncRead + AsyncWrite + Send + Sync + Unpin {}

/// The definition of backend instance
///
/// Each backend should implements the `connect` method.
pub trait Connector: Send + Sync {
    /// Establish connection to device
    fn connect(&self) -> BoxedConnect;
}

/// The definition of backend
pub trait Backend {
    /// The name of backend
    fn name(&self) -> &str;

    /// The backend description
    fn description(&self) -> &str;

    /// Create connector
    ///
    /// This method should check URL and get connection options from it.
    ///
    /// Method returns connector instance when URL is compatible with backend.
    fn connector(&self, url: &Url) -> Option<BoxedConnector>;
}

impl<T> Backend for T
where
    T: Deref<Target = dyn Backend>,
{
    fn name(&self) -> &str {
        self.deref().name()
    }
    fn description(&self) -> &str {
        self.deref().description()
    }
    fn connector(&self, url: &Url) -> Option<BoxedConnector> {
        self.deref().connector(url)
    }
}
