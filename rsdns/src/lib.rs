#![warn(missing_docs)]

//! DNS client library in Rust.

mod error;
pub mod protocol;

pub use error::Result;
pub use error::RsDnsError;
