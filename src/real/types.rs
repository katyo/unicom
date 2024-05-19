use core::{future::Future, ops::Deref, pin::Pin};
use std::sync::Arc;

#[cfg(feature = "async")]
use futures_io::{AsyncRead, AsyncWrite};

#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead, AsyncWrite};

use crate::{Result, Url};

/// Boxed connection
pub type BoxedConnection = Box<dyn Connection>;

/// The establishing connection between backend and device
pub type BoxedConnect = Pin<Box<dyn Connect>>;

/// Unified connector object
pub type BoxedConnector = Arc<dyn Connector>;

/// Unified backend object
pub type BoxedBackend = Box<dyn Backend>;

/// The establishing connect between backend and device
pub trait Connect: Future<Output = Result<BoxedConnection>> + Send {}
impl<T> Connect for T where T: Future<Output = Result<BoxedConnection>> + Send {}

/// The established connection between backend and device
pub trait Connection: AsyncRead + AsyncWrite + Send + Unpin {}
impl<T> Connection for T where T: AsyncRead + AsyncWrite + Send + Unpin {}

/// Backend connector interface
pub trait Connector: Send + Sync {
    /// Get device URL
    fn url(&self) -> &Url;

    /// Establish connection to device
    fn connect(&self) -> BoxedConnect;
}

/// Backend interface
pub trait Backend {
    /// The name of backend
    ///
    /// Examples: tcp, serial.
    fn name(&self) -> &str;

    /// The backend description
    fn description(&self) -> &str;

    /// Create connector
    ///
    /// This method should check URL and extract connection options from it.
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
