use crate::{Resolver, Resolving};
use tokio_rs::net::lookup_host;
use unicom::Error;

#[derive(Clone, Default)]
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
    use tokio_rs as tokio;

    #[tokio::test]
    async fn test_success() {
        let r = TokioResolver::default();
        let r = r.resolve_name("illumium.org").await;

        eprintln!("!!! {:?}", r);
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_failed() {
        let r = TokioResolver::default();
        let r = r.resolve_name("nihil.illumium.org").await;

        assert!(r.is_err());
    }
}
