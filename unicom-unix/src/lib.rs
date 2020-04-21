/*!

# Raw Unix-domain socket backend for unicom

This backend can be used to connect to device emulators which mapped to Unix-domain socket.

**IMPORTANT NOTE**: Async crate feature should be selected, because no default is set.

## Supported features

* __tokio__ Use [tokio](https://docs.rs/tokio/)
* __async-std__ Use [async-std](https://docs.rs/async-std/)

 */

use std::{
    path::PathBuf,
    sync::Arc,
};

#[cfg(feature = "tokio")]
use tokio_rs::net::UnixStream;

#[cfg(feature = "async-std")]
use async_std_rs::os::unix::net::UnixStream;

use unicom::{
    Url, Error,
    Backend, Connector, BoxedConnector, BoxedConnect, BoxedConnection,
};

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
            Some(Arc::new(UnixConnector { path }))
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct UnixConnector {
    path: PathBuf,
}

impl Connector for UnixConnector {
    fn connect(&self) -> BoxedConnect {
        let this = self.clone();
        Box::pin(async move {
            let stm = UnixStream::connect(this.path).await
                .map_err(|e| Error::FailedConnect(e.to_string()))?;
            Ok(Box::new(stm) as BoxedConnection)
        })
    }
}
