/*!

# Raw TCP socket backend for unicom

This backend can be used to connect to devices which mapped to TCP ports, such as RS232-TO-TCP connectors.

**IMPORTANT NOTE**: Async crate feature should be selected, because no default is set.

## Supported features

* __tokio__ Use [tokio](https://docs.rs/tokio/)
* __async-std__ Use [async-std](https://docs.rs/async-std/)

 */

use std::{
    net::SocketAddr,
    sync::Arc,
};

#[cfg(feature = "tokio")]
use tokio_rs::net::TcpStream;

#[cfg(feature = "async-std")]
use async_std_rs::net::TcpStream;

use unicom::{
    Host, Url, Error,
    Backend, Connector, BoxedConnector, BoxedConnect, BoxedConnection,
};

use unicom_nres::{Resolver, IpAddr};

/// TCP socket backend
///
/// Support connecting to devices using network TCP sockets
#[derive(Clone)]
pub struct TcpSocket<R> {
    name: String,
    description: String,
    resolver: R,
}

impl<R> TcpSocket<R> {
    pub fn new(resolver: R) -> Self {
        Self {
            name: "tcp-socket".into(),
            description: "Support for tcp/ip network socket connections.".into(),
            resolver,
        }
    }
}

impl<R> Backend for TcpSocket<R>
where
    R: Resolver,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn connector(&self, url: &Url) -> Option<BoxedConnector> {
        if (url.scheme() == "socket" || url.scheme() == "tcp")
            && url.has_host()
            && url.path() == "/"
        {
            let host = match url.host() {
                // parse domain name as host because Url doesn't parse it for speculative protocols
                Some(Host::Domain(name)) => if let Ok(host) = Host::parse(&name) {
                    host
                } else {
                    return None;
                },
                Some(host) => host.to_owned(),
                _ => return None,
            };
            let port = url.port().unwrap_or(23);
            Some(Arc::new(TcpConnector { host, port, resolver: self.resolver.clone() }))
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct TcpConnector<R> {
    resolver: R,
    host: Host,
    port: u16,
}

impl<R> Connector for TcpConnector<R>
where
    R: Resolver,
{
    fn connect(&self) -> BoxedConnect {
        let this = self.clone();
        Box::pin(async move {
            let port = this.port;
            let addr: IpAddr = match this.host {
                Host::Domain(name) => *this.resolver.resolve_name(name.clone()).await?
                    .iter(Default::default()).next().ok_or("No IP address found")
                    .map_err(|e| Error::FailedResolve(e.into()))?,
                Host::Ipv6(addr) => addr.into(),
                Host::Ipv4(addr) => addr.into(),
            };
            let addr = SocketAddr::new(addr, port);
            let stm = TcpStream::connect(addr).await
                .map_err(|e| Error::FailedConnect(e.to_string()))?;
            Ok(Box::new(stm) as BoxedConnection)
        })
    }
}
