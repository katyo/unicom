/*!
# Unified communication for IoT clients

The backends interface was introduced to support different connection methods.

The backend should provide the way to establish bidirectional connection for exchange binary data with device.
 */

mod types;
mod manager;
mod result;
mod to_url;

pub(crate) use log;

pub use types::*;
pub use manager::*;
pub use result::*;
pub use to_url::*;
