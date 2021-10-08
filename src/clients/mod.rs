//! Various client implementations.
//!
//! *rsdns* is first and foremost an asynchronous DNS client. It supports three different
//! async runtimes: `tokio`, `async-std` and `smol`. Additionally, *rsdns* has a synchronous
//! client implemented using the Rust standard library's `std::net` primitives.
//! Each runtime has its own submodule named after the runtime: [`tokio`], [`async_std`] and
//! [`smol`]. The synchronous client is implemented in the [`std`] submodule.
//!
//! All clients have exactly the same API. The set of functions is the same, while in
//! asynchronous clients the functions are `async`.
//!
//! Usually an application will use only one of the clients. Hence, none of them is enabled by
//! default. The crate features `net-tokio`, `net-async-std` and `net-smol` enable the async
//! clients for the corresponding runtime. `net-std` enables the synchronous client.
//! The `clients` module is enabled only if one of the client implementations is enabled.
//!
//! [`tokio`]: crate::clients::tokio
//! [`async_std`]: crate::clients::async_std
//! [`smol`]: crate::clients::smol
//! [`std`]: crate::clients::std
//! [`std::net`]: https://doc.rust-lang.org/std/net
//! [`clients`]: crate::clients

#[cfg(feature = "net-tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "net-tokio")))]
/// Client implementation with [`tokio`](https://docs.rs/tokio).
pub mod tokio;

#[cfg(feature = "net-async-std")]
#[cfg_attr(docsrs, doc(cfg(feature = "net-async-std")))]
/// Client implementation with [`async-std`](https://docs.rs/async-std).
pub mod async_std;

#[cfg(feature = "net-smol")]
#[cfg_attr(docsrs, doc(cfg(feature = "net-smol")))]
/// Client implementation with [`smol`](https://docs.rs/smol).
pub mod smol;

#[cfg(feature = "net-std")]
#[cfg_attr(docsrs, doc(cfg(feature = "net-std")))]
/// Client implementation with [`std::net`](https://doc.rust-lang.org/std/net).
pub mod std;

mod config;
pub use config::*;
