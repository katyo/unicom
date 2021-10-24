/*!

# Raw serial port backend for unicom

This backend can be used for interfacing with devices connected via serial ports.

**IMPORTANT NOTE**: Async runtime feature should be selected explicitly.

## Supported features

* __tokio__ Use [tokio](https://docs.rs/tokio/)
* [TODO] __async-std__ Use [async-std](https://docs.rs/async-std/)

*/

#[cfg(feature = "tokio")]
mod tokio_impl;

#[cfg(feature = "async-std")]
compile_error!("Currently lacks async-std support!");

#[cfg(feature = "tokio")]
pub use tokio_impl::*;
