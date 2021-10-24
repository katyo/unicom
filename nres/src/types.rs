use crate::IpAddrs;
use std::{future::Future, pin::Pin};
use unicom::Result;

/// Domain name resolving operation
pub type Resolving = Pin<Box<dyn Future<Output = Result<IpAddrs>> + Send + 'static>>;

/// Domain name resolver service
pub trait Resolver: Sync + Send + Clone + 'static {
    /// Resolvers should implement this method
    fn resolve_str(&self, name: &str) -> Resolving;

    /// Resolve domain name to IP addresses
    fn resolve_name(&self, name: impl AsRef<str>) -> Resolving {
        self.resolve_str(name.as_ref())
    }
}
