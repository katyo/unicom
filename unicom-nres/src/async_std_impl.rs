use async_std_resolver::{AsyncStdResolver as AStdResolver, resolver, config};
use unicom::{Result, Error};
use crate::{Resolver, Resolving};

#[derive(Clone)]
pub struct AsyncStdResolver {
    resolver: AStdResolver,
}

impl AsyncStdResolver {
    pub async fn new() -> Result<Self> {
        let resolver = resolver(
            config::ResolverConfig::default(),
            config::ResolverOpts::default(),
        ).await.map_err(|e| Error::FailedResolve(e.to_string()))?;
        Ok(Self { resolver })
    }
}

impl Resolver for AsyncStdResolver {
    fn resolve_str(&self, name: &str) -> Resolving {
        let name = name.to_string();
        let resolver = self.resolver.clone();
        Box::pin(async move {
            resolver.lookup_ip(name).await
                .map(|addrs| addrs.iter().collect())
                .map_err(|e| Error::FailedResolve(e.to_string()))
        })
    }
}
