use std::sync::{Arc, Mutex};
use futures_util::join;
use c_ares_resolver::{FutureResolver};
use unicom::{Result, Error};
use crate::{Resolver, Resolving};

#[derive(Clone, Default)]
#[repr(transparent)]
pub struct CAresResolver {
    resolver: Arc<Mutex<Option<Result<Arc<FutureResolver>>>>>,
}

impl Resolver for CAresResolver {
    fn resolve_str(&self, name: &str) -> Resolving {
        let name = name.to_string();
        let resolver = self.resolver.clone();
        Box::pin(async move {
            if resolver.lock().unwrap().is_none() {
                *resolver.lock().unwrap() = Some(
                    FutureResolver::new()
                        .map(|resolver| Arc::new(resolver))
                        .map_err(|e| Error::FailedResolve(e.to_string()))
                );
            }

            let resolver = resolver.lock().unwrap().clone().unwrap()?;

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

#[cfg(test)]
mod test {
    use async_std_rs as async_std;
    use super::{Resolver, CAresResolver};

    #[async_std::test]
    async fn test_success() {
        let r = CAresResolver::default();
        let r = r.resolve_name("illumium.org").await;

        eprintln!("!!! {:?}", r);
        assert!(r.is_ok());
    }

    #[async_std::test]
    async fn test_failed() {
        let r = CAresResolver::default();
        let r = r.resolve_name("nihil.illumium.org").await;

        assert!(r.is_err());
    }
}
