use super::{Backend, BoxedBackend, BoxedConnector};
use crate::{
    log::{error, info},
    Error, Result, ToUrl,
};
use std::sync::{Arc, RwLock};

/// The list of backends
type Backends = Arc<RwLock<Vec<BoxedBackend>>>;

/// Backends manager
///
/// Manager holds registered backends and helps create client instances.
#[derive(Clone)]
pub struct Manager {
    backends: Backends,
}

impl Default for Manager {
    /// Create new backends manager
    fn default() -> Self {
        Self {
            backends: Arc::new(RwLock::new(Vec::default())),
        }
    }
}

impl Manager {
    /// Register backend
    pub fn register(&self, backend: impl Backend + 'static) -> Result<()> {
        self.with_mut(|backends| {
            let name = backend.name();
            if backends.iter().any(|backend| backend.name() == name) {
                error!("Attempt to register backend \"{}\" twice", name);
                return Err(Error::AlreadyRegistered(name.into()));
            }
            info!("Registering backend \"{}\"", name);
            backends.push(Box::new(backend));
            Ok(())
        })
    }

    /// Create the backend using URL by probing registered backend types
    pub fn create(&self, url: impl ToUrl) -> Result<BoxedConnector> {
        let url = url.to_url()?;
        self.with(|backends| {
            backends
                .iter()
                .filter_map(|backend| backend.connector(&url))
                .next()
                .ok_or_else(|| {
                    error!("No backends which can handle URL {}", url);
                    url.into()
                })
        })
    }

    fn with_raw<R>(&self, func: impl FnOnce(Backends) -> R) -> R {
        func(self.backends.clone())
    }

    fn with<R>(&self, func: impl FnOnce(&[BoxedBackend]) -> R) -> R {
        self.with_raw(|backends: Backends| {
            let backends = backends.read().unwrap();
            func(&backends)
        })
    }

    fn with_mut<R>(&self, func: impl FnOnce(&mut Vec<BoxedBackend>) -> R) -> R {
        self.with_raw(|backends: Backends| {
            let mut backends = backends.write().unwrap();
            func(&mut backends)
        })
    }
}
