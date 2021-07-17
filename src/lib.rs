//! [rsdns] is a library implementing a [DNS Stub Resolver][^rfc1034].
//!
//! DNS is a large, distributed and hierarchical system comprised of many types of servers.
//! The data is held in *Authoritative Servers*, which are responsible for specific domains only.
//! The client DNS servers are called *resolvers*. Resolving a DNS query may require a *resolver*
//! to communicate with several *authoritative servers*.
//! This process is called *recursion* and resolvers implementing it are usually called *recursors*.
//!
//! *rsdns* implements a *Stub Resolver*, which is the simplest resolver in DNS.
//! It delegates queries to another DNS server, usually (but not necessarily) a *recursor*.
//!
//! *rsnds* provides an API to directly communicate with DNS servers using its own implementation
//! of the DNS protocol. It strives to be minimal and fast.
//!
//! # Notable Features
//!
//! * Minimal API
//! * Three independent asynchronous resolvers for different async runtimes:
//!   [`tokio`], [`async-std`] and [`smol`]
//! * An independent blocking resolver implemented on top of [`std::net`]
//! * Zero memory allocations when parsing records with no variable size fields
//!   (e.g. [`A`], [`AAAA`])
//! * Sockets can be bound to network interfaces by name (requires `SO_BINDTODEVICE` support
//!   from the underlying OS)
//! * Minimal set of dependencies
//!
//! [^rfc1034]: Initial definition of stub resolvers is in [RFC 1034].
//!
//! [rsdns]: crate
//! [RFC 1034]: https://www.rfc-editor.org/rfc/rfc1034.html#section-5.3.1
//! [DNS Stub Resolver]: https://en.wikipedia.org/wiki/Domain_Name_System#DNS_resolvers
//! [`tokio`]: https://docs.rs/tokio
//! [`async-std`]: https://docs.rs/async-std
//! [`smol`]: https://docs.rs/smol
//! [`std::net`]: https://doc.rust-lang.org/std/net/index.html
//! [`A`]: crate::records::data::A
//! [`AAAA`]: crate::records::data::Aaaa

//! # Library Structure
//!
//! *rsdns* is built from two major parts: *DNS message parsing utilities* and *resolvers*.
//!
//! The *message parsing* part is the core of *rsdns*. It is generic and is suitable for any type
//! of resolver that you may choose. It may be used even without an *rsdns* resolver at all,
//! if you have DNS messages obtained by other means. This part is always present and cannot
//! be disabled.
//!
//! The *resolvers* part is comprised of four independent implementations
//! of the resolver API. Usually an application will use only one of those. None of the resolvers
//! is enabled by default. You need to enable a resolver via one of the `net-*` crate features.
//! See the [`resolvers`] module for more information.

//! # `std::net::ToSocketAddrs`
//!
//! [`ToSocketAddrs`] is the Rust standard library interface for programmatically
//! obtaining addresses of hostnames. This interface hides the low-level details of how addresses
//! are obtained (usually it uses pre-configured facilities provided by the underlying operating
//! system), and returns an iterator over the resulting set of addresses.
//!
//! The following is a list of possible limitations that you may experience with this interface:
//!
//! * it returns IP addresses only (`A` and `AAAA` records in DNS lingo), and doesn't allow
//!   retrieval of other types of data stored in DNS
//! * it doesn't allow you to choose which DNS server to consult
//! * it doesn't allow you to control how the communication is performed (network protocol,
//!   network interface, timeout etc.)
//! * it is blocking - harder to use in an asynchronous context
//!
//! If any of the above is an issue in your application, you may find *rsdns* useful.
//! Otherwise, if all you need is a host address, and a blocking API call is not an issue,
//! consider using [`ToSocketAddrs`] instead. It comes built-in with the Rust standard library,
//! and is very simple to use.
//!
//! [`ToSocketAddrs`]: https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
pub(crate) mod macros;
pub(crate) mod bytes;
pub mod constants;
mod domain_name;
pub use domain_name::*;
pub mod errors;
pub mod message;
pub mod records;

cfg_any_resolver! {
    pub mod resolvers;
}

pub(crate) use errors::ProtocolResult;

#[doc(inline)]
pub use errors::{Error, Result};