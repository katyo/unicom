use std::sync::Arc;
use futures::join;
use c_ares_resolver::{FutureResolver};
use unicom::{Result, Error};
use crate::{Resolver, Resolving};

#[derive(Clone)]
pub struct CAresResolver {
    resolver: Arc<FutureResolver>,
}

impl CAresResolver {
    pub fn new() -> Result<Self> {
        FutureResolver::new()
            .map(|resolver| Self { resolver: Arc::new(resolver) })
            .map_err(|e| Error::FailedResolve(e.to_string()))
    }
}

impl Resolver for CAresResolver {
    fn resolve_str(&self, name: &str) -> Resolving {
        let name = name.to_string();
        let resolver = self.resolver.clone();
        Box::pin(async move {
            match join!(resolver.query_a(&name),
                        resolver.query_aaaa(&name)) {
                (Ok(v4), Ok(v6)) => Ok(v4.into_iter()
                                       .map(|a| a.ipv4().into())
                                       .chain(v6.into_iter()
                                              .map(|a| a.ipv6().into()))
                                       .collect()),
                (Ok(v4), _) => Ok(v4.into_iter().map(|a| a.ipv4().into()).collect()),
                (_, Ok(v6)) => Ok(v6.into_iter().map(|a| a.ipv6().into()).collect()),
                (Err(v4), Err(_v6)) => Err(Error::FailedResolve(v4.to_string())),
            }
        })
    }
}
