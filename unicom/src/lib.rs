/*!

# Unified communication for IoT clients

The backends interface was introduced to support different connection methods.

The backend should provide the way to establish bidirectional connection for exchange binary data with device.

**IMPORTANT NOTE**: Async crate feature should be selected, because no default is set.

## Supported features

* __futures__ Use [futures](https://docs.rs/futures/) only
* __tokio__ Use [tokio](https://docs.rs/tokio/)
* __async-std__ Use [async-std](https://docs.rs/async-std/)

 */

mod types;
mod manager;
mod result;
mod to_url;

pub(crate) use log;

pub use types::*;
pub use manager::*;
pub use result::*;
pub use to_url::*;
