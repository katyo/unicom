#![doc = include_str!("../README.md")]

mod addrs;
mod types;

#[cfg(feature = "c-ares")]
mod c_ares_impl;

#[cfg(feature = "async-std")]
mod async_std_impl;

#[cfg(feature = "tokio")]
mod tokio_impl;

pub use addrs::*;
pub use types::*;

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
