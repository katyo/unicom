#![doc = include_str!("../README.md")]

#[cfg(not(any(feature = "async", feature = "tokio")))]
compile_error!("Either feature should be selected: async, tokio");

#[cfg(all(feature = "async", feature = "tokio"))]
compile_error!("Either feature should be selected: async, tokio. Not both together.");

#[cfg(any(feature = "async", feature = "tokio"))]
mod real;

#[cfg(any(feature = "async", feature = "tokio"))]
pub use real::*;
