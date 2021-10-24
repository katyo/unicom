#![doc = include_str!("../README.md")]

#[cfg(feature = "tokio")]
mod tokio_impl;

#[cfg(feature = "async-std")]
compile_error!("Currently lacks async-std support!");

#[cfg(feature = "tokio")]
pub use tokio_impl::*;
