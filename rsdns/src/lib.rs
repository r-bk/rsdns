#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! DNS client library in Rust.

mod error;
pub mod protocol;

pub use error::Result;
pub use error::RsDnsError;
