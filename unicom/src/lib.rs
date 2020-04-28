/*!

# Unified communication for IoT clients

The backends interface was introduced to support different connection methods.

The backend should provide the way to establish bidirectional connection for exchange binary data with device.

**IMPORTANT NOTE**: Async crate feature should be selected, because no default is set.

## Supported features

* __futures__ Use [futures](https://docs.rs/futures/) only
* __tokio__ Use [tokio](https://docs.rs/tokio/)
* __async-std__ Use [async-std](https://docs.rs/async-std/)

## Built-in backends

* _unicom-tcp_ TCP socket connection
* _unicom-unix_ Unix-domain socket connection
* _unicom-serial_ Serial port connection

 */

#[cfg(not(any(feature = "futures", feature = "tokio", feature = "async-std")))]
compile_error!("Either of features should be selected: futures, tokio, async-std");

#[cfg(any(feature = "futures", feature = "tokio", feature = "async-std"))]
mod real {
    mod types;
    mod manager;
    mod result;
    mod to_url;

    pub(crate) use log;

    pub use types::*;
    pub use manager::*;
    pub use result::*;
    pub use to_url::*;
}

#[cfg(any(feature = "futures", feature = "tokio", feature = "async-std"))]
pub use real::*;
