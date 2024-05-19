#![doc = include_str!("../README.md")]

#[cfg(feature = "tokio")]
mod tokio_impl;

#[cfg(feature = "async")]
compile_error!("Currently lacks non-tokio runtimes support!");

#[cfg(feature = "tokio")]
pub use tokio_impl::*;
