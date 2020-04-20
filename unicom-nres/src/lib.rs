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
