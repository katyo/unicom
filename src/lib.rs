#![doc = include_str!("../README.md")]

#[cfg(not(any(feature = "futures", feature = "tokio", feature = "async-std")))]
compile_error!("Either of features should be selected: futures, tokio, async-std");

#[cfg(any(feature = "futures", feature = "tokio", feature = "async-std"))]
mod real;

#[cfg(any(feature = "futures", feature = "tokio", feature = "async-std"))]
pub use real::*;
