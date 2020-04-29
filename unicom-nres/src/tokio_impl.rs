use tokio_rs::net::lookup_host;
use unicom::Error;
use crate::{Resolver, Resolving};

#[derive(Clone, Default)]
pub struct TokioResolver;

impl Resolver for TokioResolver {
    fn resolve_str(&self, name: &str) -> Resolving {
        let name = name.to_string();
        Box::pin(async move {
            Ok(lookup_host(name).await
               .map_err(|e| Error::FailedResolve(e.to_string()))?
               .map(|addr| addr.ip())
               .collect())
        })
    }
}
