#![doc = include_str!("../README.md")]

use std::{net::SocketAddr, sync::Arc};

#[cfg(feature = "tokio")]
use tokio::net::TcpStream;

#[cfg(feature = "async-std")]
use async_std::net::TcpStream;

use unicom::{Backend, BoxedConnect, BoxedConnection, BoxedConnector, Connector, Error, Host, Url};

use unicom_nres::{IpAddr, Resolver};

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
                Some(Host::Domain(name)) => {
                    if let Ok(host) = Host::parse(&name) {
                        host
                    } else {
                        return None;
                    }
                }
                Some(host) => host.to_owned(),
                _ => return None,
            };
            let port = url.port().unwrap_or(23);
            let url = url.clone();
            let resolver = self.resolver.clone();
            Some(Arc::new(TcpConnector {
                url,
                host,
                port,
                resolver,
            }))
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct TcpConnector<R> {
    resolver: R,
    url: Url,
    host: Host,
    port: u16,
}

impl<R> Connector for TcpConnector<R>
where
    R: Resolver,
{
    fn url(&self) -> &Url {
        &self.url
    }

    fn connect(&self) -> BoxedConnect {
        let this = self.clone();
        Box::pin(async move {
            let port = this.port;
            let addr: IpAddr = match this.host {
                Host::Domain(name) => *this
                    .resolver
                    .resolve_name(name.clone())
                    .await?
                    .iter(Default::default())
                    .next()
                    .ok_or("No IP address found")
                    .map_err(|e| Error::FailedResolve(e.into()))?,
                Host::Ipv6(addr) => addr.into(),
                Host::Ipv4(addr) => addr.into(),
            };
            let addr = SocketAddr::new(addr, port);
            let stm = TcpStream::connect(addr)
                .await
                .map_err(|e| Error::FailedConnect(e.to_string()))?;
            Ok(Box::new(stm) as BoxedConnection)
        })
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "tokio")]
    use tokio::{io::copy, net::TcpListener, prelude::*, task::spawn};

    #[cfg(feature = "async-std")]
    use async_std::{io::copy, net::TcpListener, prelude::*, task::spawn};

    use super::TcpSocket;
    use unicom::Manager;
    use unicom_nres::DefaultResolver;

    #[cfg_attr(feature = "tokio", tokio::test)]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn connect() {
        let manager = Manager::default();
        manager
            .register(TcpSocket::new(DefaultResolver::default()))
            .unwrap();

        echo_server("127.0.0.1:43210").await.unwrap();

        let connector = manager.create("tcp://127.0.0.1:43210/").unwrap();
        let mut connection = connector.connect().await.unwrap();
        let input = b"abc\n\0";
        connection.write_all(input).await.unwrap();
        let mut data = [0; 32];
        let n = connection.read(&mut data).await.unwrap();
        assert_eq!(n, input.len());
        assert_eq!(&data[..n], input);
    }

    async fn echo_server(addr: &str) -> std::io::Result<()> {
        let mut listener = TcpListener::bind(addr).await?;
        //println!("listenning: {:?}", listener);

        spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await?;
                //println!("accepted: {:?}", socket);

                #[cfg(feature = "tokio")]
                let (mut rd, mut wr) = socket.split();

                #[cfg(feature = "async-std")]
                let (mut rd, mut wr) = &mut (&socket, &socket);

                if copy(&mut rd, &mut wr).await? == 0 {
                    break;
                }
            }
            Ok(()) as std::io::Result<_>
        });

        Ok(())
    }
}
