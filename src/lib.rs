//! [rsdns] is a DNS client library.
//!
//! *rsdns* provides an API to directly communicate with DNS servers using its own implementation
//! of the DNS protocol. It strives to be minimal and fast.
//!
//! # Notable Features
//!
//! * Minimal API
//! * Three independent asynchronous clients for different async runtimes:
//!   [`tokio`], [`async-std`] and [`smol`]
//! * An independent blocking client implemented with [`std::net`]
//! * Zero memory allocations when parsing records with no variable size fields
//!   (e.g. [`A`], [`AAAA`])
//! * Sockets can be bound to network interfaces by name (requires `SO_BINDTODEVICE` support
//!   from the underlying OS)
//! * Minimal set of dependencies
//!
//! [rsdns]: crate
//! [`tokio`]: https://docs.rs/tokio
//! [`async-std`]: https://docs.rs/async-std
//! [`smol`]: https://docs.rs/smol
//! [`std::net`]: https://doc.rust-lang.org/std/net
//! [`A`]: crate::records::data::A
//! [`AAAA`]: crate::records::data::Aaaa

//! # Library Structure
//!
//! *rsdns* is built from two major parts: *message parsing* and *clients*.
//!
//! The *message parsing* part is the core of *rsdns*. It is generic and is suitable for any type
//! of client that you may choose. It can be used even without an *rsdns* client at all,
//! if you have DNS messages obtained by other means. This part is always present and cannot
//! be disabled. See [`MessageIterator`] for more details.
//!
//! The *clients* part is comprised of four independent implementations
//! of the client API. Usually an application will use only one of those.
//! See the [`clients`] module for more information.
//!
//! [`MessageIterator`]: message::reader::MessageIterator

//! ## Cargo Features
//!
//! *rsdns* has several cargo features which conditionally enable different functionalities:
//!
//! 1. `net-tokio` - enables the [`clients::tokio`] module
//! 2. `net-async-std` - enables the [`clients::async_std`] module
//! 3. `net-smol` - enables the [`clients::smol`] module
//! 4. `net-std` - enables the [`clients::std`] module
//! 5. `socket2` - together with `net-tokio` enables `bind-to-device` support
//!    (currently on Linux only)
//!
//! Note that none of the features is enabled by default. The [`clients`] module exists only
//! if one of the `net-*` features is enabled.

//! # Examples
//!
//! The following function retrieves [`A`] records using *rsdns's* asynchronous [`tokio::Client`].
//! To retrieve a different type of record, or use a different asynchronous client, use the
//! relevant types from [`records::data`] and [`clients`] modules respectively.
//!
//! This is a very simplified example of the available API. In a real application initialization
//! of the Client object would be done once for many DNS queries.
//! Also, please note that a full application requires `tokio` runtime initialization,
//! which is out of *rsdns* scope. See the [`tokio`] documentation for details.
//!
//! [`A`]: crate::records::data::A
//! [`tokio::Client`]: crate::clients::tokio::Client
//! [`records::data`]: crate::records::data
//! [`clients`]: crate::clients
//!
//! ```rust
//! use rsdns::{constants::Class, records::data::A};
//! # #[cfg(feature = "net-tokio")]
//! use rsdns::clients::{tokio::Client, ClientConfig};
//! # use std::{error::Error, net::{Ipv4Addr, SocketAddr}, str::FromStr};
//!
//! # #[cfg(feature = "net-tokio")]
//! async fn get_a_records(qname: &str) -> Result<Vec<A>, Box<dyn Error>> {
//!     // use Google's Public DNS recursor as nameserver
//!     let nameserver = SocketAddr::from_str("8.8.8.8:53")?;
//!
//!     // default client configuration; specify nameserver address only
//!     let config = ClientConfig::with_nameserver(nameserver);
//!
//!     // create tokio Client
//!     let mut client = Client::new(config).await?;
//!
//!     // issue an A query
//!     let rrset = client.query_rrset::<A>(qname, Class::In).await?;
//!
//!     Ok(rrset.rdata)
//! }
//! ```
//!
//! The same function using *rsdns's* synchronous [`std::Client`].
//!
//! [`std::Client`]: crate::clients::std::Client
//!
//! ```rust
//! use rsdns::{constants::Class, records::data::A};
//! # #[cfg(feature = "net-std")]
//! use rsdns::clients::{std::Client, ClientConfig};
//! # use std::{error::Error, net::{Ipv4Addr, SocketAddr}, str::FromStr};
//!
//! # #[cfg(feature = "net-std")]
//! fn get_a_records(qname: &str) -> Result<Vec<A>, Box<dyn Error>> {
//!     let nameserver = SocketAddr::from_str("8.8.8.8:53")?;
//!     let mut client = Client::new(ClientConfig::with_nameserver(nameserver))?;
//!     let rrset = client.query_rrset::<A>(qname, Class::In)?;
//!     Ok(rrset.rdata)
//! }
//! ```

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
pub(crate) mod macros;
pub(crate) mod bytes;
pub mod constants;
mod errors;
pub mod message;
pub mod names;
pub mod records;

cfg_any_client! {
    pub mod clients;
}

#[doc(inline)]
pub use errors::{Error, Result};
