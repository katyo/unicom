/*!

# Unified domain name to IP address resolver

**IMPORTANT NOTE**: Async runtime feature should be selected explicitly.

## Supported features

* __tokio__ Use [tokio](https://docs.rs/tokio/)
* __async-std__ Use [async-std](https://docs.rs/async-std/)
* __c-ares__ Use [c-ares-resolver](https://docs.rs/c-ares-resolver/)

*/

mod types;
mod addrs;

#[cfg(feature = "c-ares")]
mod c_ares_impl;

#[cfg(feature = "async-std")]
mod async_std_impl;

#[cfg(feature = "tokio")]
mod tokio_impl;

pub use types::*;
pub use addrs::*;

#[cfg(feature = "c-ares")]
pub use c_ares_impl::*;

#[cfg(feature = "async-std")]
pub use async_std_impl::*;

#[cfg(feature = "tokio")]
pub use tokio_impl::*;

#[cfg(all(not(feature = "tokio"), not(feature = "async-std"), feature = "c-ares"))]
pub type DefaultResolver = CAresResolver;

#[cfg(all(feature = "tokio", not(feature = "async-std"), not(feature = "c-ares")))]
pub type DefaultResolver = TokioResolver;

#[cfg(all(not(feature = "tokio"), feature = "async-std", not(feature = "c-ares")))]
pub type DefaultResolver = AsyncStdResolver;
