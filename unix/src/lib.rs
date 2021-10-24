#![doc = include_str!("../README.md")]

use std::{path::PathBuf, sync::Arc};

#[cfg(feature = "tokio")]
use tokio_rs::net::UnixStream;

#[cfg(feature = "async-std")]
use async_std_rs::os::unix::net::UnixStream;

use unicom::{Backend, BoxedConnect, BoxedConnection, BoxedConnector, Connector, Error, Url};

/// Unix socket backend
///
/// Support connecting to devices using unix domain sockets
#[derive(Clone)]
pub struct UnixSocket {
    name: String,
    description: String,
}

impl Default for UnixSocket {
    fn default() -> Self {
        Self {
            name: "unix-socket".into(),
            description: "Support for local unix domain socket connections.".into(),
        }
    }
}

impl Backend for UnixSocket {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn connector(&self, url: &Url) -> Option<BoxedConnector> {
        if (url.scheme() == "socket" || url.scheme() == "unix")
            && !url.has_host()
            && url.path() != "/"
        {
            let path = url.path().into();
            let url = url.clone();
            Some(Arc::new(UnixConnector { url, path }))
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct UnixConnector {
    url: Url,
    path: PathBuf,
}

impl Connector for UnixConnector {
    fn url(&self) -> &Url {
        &self.url
    }

    fn connect(&self) -> BoxedConnect {
        let this = self.clone();
        Box::pin(async move {
            let stm = UnixStream::connect(this.path)
                .await
                .map_err(|e| Error::FailedConnect(e.to_string()))?;
            Ok(Box::new(stm) as BoxedConnection)
        })
    }
}
