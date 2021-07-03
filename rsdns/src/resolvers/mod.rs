//! Resolvers and networking.

#[cfg(feature = "net-tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "net-tokio")))]
/// Resolver implementation with [`tokio`](https://crates.io/crates/tokio).
pub mod tokio;

#[cfg(feature = "net-async-std")]
#[cfg_attr(docsrs, doc(cfg(feature = "net-async-std")))]
/// Resolver implementation with [`async-std`](https://crates.io/crates/async-std).
pub mod async_std;

#[cfg(feature = "net-smol")]
#[cfg_attr(docsrs, doc(cfg(feature = "net-smol")))]
/// Resolver implementation with [`smol`](https://crates.io/crates/smol).
pub mod smol;

#[cfg(feature = "net-std")]
#[cfg_attr(docsrs, doc(cfg(feature = "net-std")))]
/// Resolver implementation with [`std`](https://doc.rust-lang.org/std).
pub mod std;

mod answer;
pub use answer::*;

pub mod config;
