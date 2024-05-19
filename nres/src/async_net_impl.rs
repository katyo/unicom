use crate::{Resolver, Resolving};
use async_net::resolve;
use unicom::Error;

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct AsyncResolver;

impl Resolver for AsyncResolver {
    fn resolve_str(&self, name: &str) -> Resolving {
        let name = name.to_string() + ":0";
        Box::pin(async move {
            Ok(resolve(&name)
                .await
                .map_err(|e| Error::FailedResolve(e.to_string()))?
                .into_iter()
                .map(|addr| addr.ip())
                .collect())
        })
    }
}

#[cfg(test)]
mod test {
    use super::{AsyncResolver, Resolver};
    use macro_rules_attribute::apply;
    use smol_macros::test;

    #[apply(test!)]
    async fn test_success() {
        let r = AsyncResolver::default();
        let r = r.resolve_name("github.com").await;

        assert!(r.is_ok());
    }

    #[apply(test!)]
    async fn test_failed() {
        let r = AsyncResolver::default();
        let r = r.resolve_name("nothing.nowhere").await;

        assert!(r.is_err());
    }
}
