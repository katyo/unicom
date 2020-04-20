use std::{
    pin::Pin,
    future::Future,
};
use unicom::{Result};
use crate::{IpAddrs};

/// Domain name resolving operation
pub type Resolving = Pin<Box<dyn Future<Output = Result<IpAddrs>> + Send + 'static>>;

/// Domain name resolver service
pub trait Resolver {
    /// Resolvers should implement this method
    fn resolve_str(&self, name: &str) -> Resolving;

    /// Resolve domain name to IP addresses
    fn resolve_name(&self, name: impl AsRef<str>) -> Resolving {
        self.resolve_str(name.as_ref())
    }
}
