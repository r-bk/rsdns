#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! `rsdns` is a DNS Client library providing functionality of a Stub Resolver defined in
//! [RFC 1034](https://tools.ietf.org/html/rfc1034#section-5.3.1).

mod error;
#[cfg(any(
    feature = "net-async-std",
    feature = "net-smol",
    feature = "net-std",
    feature = "net-tokio"
))]
pub mod net;
pub mod protocol;

pub use error::Error;
pub use error::Result;
