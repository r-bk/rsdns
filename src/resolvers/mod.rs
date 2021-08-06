//! Various resolver implementations.
//!
//! *rsdns* is first and foremost an asynchronous *stub resolver*. It supports three different
//! async runtimes: `tokio`, `async-std` and `smol`. Additionally, *rsdns* has a synchronous
//! resolver implemented using the Rust standard library's `std::net` primitives.
//! Each runtime has its own submodule named after the runtime: [`tokio`], [`async_std`] and
//! [`smol`]. The synchronous resolver is implemented in the [`std`] submodule.
//!
//! All resolvers expose exactly the same API. The set of functions is the same, while in
//! asynchronous resolvers the functions are `async`.
//!
//! Usually an application will use only one of the resolvers. Hence, none of them is enabled by
//! default. The crate features `net-tokio`, `net-async-std` and `net-smol` enable the async
//! resolvers for the corresponding runtime. `net-std` enables the synchronous resolver.
//! The [`resolvers`] module is enabled only if one of the resolver implementations is enabled.
//!
//! [`tokio`]: crate::resolvers::tokio
//! [`async_std`]: crate::resolvers::async_std
//! [`smol`]: crate::resolvers::smol
//! [`std`]: crate::resolvers::std
//! [`std::net`]: https://doc.rust-lang.org/std/net
//! [`resolvers`]: crate::resolvers

#[cfg(feature = "net-tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "net-tokio")))]
/// Resolver implementation with [`tokio`](https://docs.rs/tokio).
pub mod tokio;

#[cfg(feature = "net-async-std")]
#[cfg_attr(docsrs, doc(cfg(feature = "net-async-std")))]
/// Resolver implementation with [`async-std`](https://docs.rs/async-std).
pub mod async_std;

#[cfg(feature = "net-smol")]
#[cfg_attr(docsrs, doc(cfg(feature = "net-smol")))]
/// Resolver implementation with [`smol`](https://docs.rs/smol).
pub mod smol;

#[cfg(feature = "net-std")]
#[cfg_attr(docsrs, doc(cfg(feature = "net-std")))]
/// Resolver implementation with [`std::net`](https://doc.rust-lang.org/std/net).
pub mod std;

mod config;
pub use config::*;
