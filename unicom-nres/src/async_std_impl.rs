use std::sync::{Arc, Mutex};
use async_std_resolver::{AsyncStdResolver as AStdResolver, resolver as new_resolver, config};
use unicom::{Result, Error};
use crate::{Resolver, Resolving};

#[derive(Clone, Default)]
#[repr(transparent)]
pub struct AsyncStdResolver {
    resolver: Arc<Mutex<Option<Result<AStdResolver>>>>,
}

impl Resolver for AsyncStdResolver {
    fn resolve_str(&self, name: &str) -> Resolving {
        let name = name.to_string();
        let resolver = self.resolver.clone();
        Box::pin(async move {
            if resolver.lock().unwrap().is_none() {
                *resolver.lock().unwrap() = Some(new_resolver(
                    config::ResolverConfig::default(),
                    config::ResolverOpts::default(),
                ).await.map_err(|e| Error::FailedResolve(e.to_string())));
            }

            let resolver = resolver.lock().unwrap().clone().unwrap()?;

            resolver.lookup_ip(name).await
                .map(|addrs| addrs.iter().collect())
                .map_err(|e| Error::FailedResolve(e.to_string()))
        })
    }
}
