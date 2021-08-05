//! Resolvers and networking.

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
