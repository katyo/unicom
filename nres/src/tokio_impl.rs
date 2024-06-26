use crate::{Resolver, Resolving};
use tokio::net::lookup_host;
use unicom::Error;

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct TokioResolver;

impl Resolver for TokioResolver {
    fn resolve_str(&self, name: &str) -> Resolving {
        let name = name.to_string() + ":0"; // add port number because tokio resolves socket addr only
        Box::pin(async move {
            Ok(lookup_host(name)
                .await
                .map_err(|e| Error::FailedResolve(e.to_string()))?
                .map(|addr| addr.ip())
                .collect())
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Resolver, TokioResolver};

    #[tokio::test]
    async fn test_success() {
        let r = TokioResolver::default();
        let r = r.resolve_name("github.com").await;

        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_failed() {
        let r = TokioResolver::default();
        let r = r.resolve_name("nothing.nowhere").await;

        assert!(r.is_err());
    }
}
