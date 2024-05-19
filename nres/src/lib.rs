#![doc = include_str!("../README.md")]

mod addrs;
mod types;

#[cfg(feature = "async")]
mod async_net_impl;

#[cfg(feature = "tokio")]
mod tokio_impl;

pub use addrs::*;
pub use types::*;

#[cfg(feature = "async")]
pub use async_net_impl::*;

#[cfg(feature = "tokio")]
pub use tokio_impl::*;

#[cfg(feature = "async")]
pub type DefaultResolver = AsyncResolver;

#[cfg(feature = "tokio")]
pub type DefaultResolver = TokioResolver;
