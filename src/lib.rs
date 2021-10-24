#![doc = include_str!("../README.md")]

#[cfg(not(any(feature = "futures", feature = "tokio", feature = "async-std")))]
compile_error!("Either of features should be selected: futures, tokio, async-std");

#[cfg(any(feature = "futures", feature = "tokio", feature = "async-std"))]
mod real {
    mod manager;
    mod result;
    mod to_url;
    mod types;

    pub(crate) use log;

    pub use manager::*;
    pub use result::*;
    pub use to_url::*;
    pub use types::*;
}

#[cfg(any(feature = "futures", feature = "tokio", feature = "async-std"))]
pub use real::*;
