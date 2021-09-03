//! Resolver configuration.

mod protocol_strategy;
pub use protocol_strategy::*;

mod recursion;
pub use recursion::*;

mod resolver_config;
pub use resolver_config::*;

#[cfg(windows)]
mod win;

#[cfg(unix)]
mod uni;

//
// ------------------------------------------------------------------------------------------------
//

use crate::Result;
use std::net::IpAddr;

/// Returns the list of nameservers configured on the host operating system.
///
/// On Unix-like systems parses the `/etc/resolv.conf` file.
/// On Windows uses the Win32 API function `GetAdaptersAddresses`.
/// On other operating systems returns an empty list.
///
/// Note that this function may block the calling thread for duration that is undesirable in an
/// `async` block or function. Hence, in async applications, it is recommended to call it
/// either in the initialization stage of the application, or from a thread where blocking is
/// acceptable (e.g. `tokio::task::spawn_blocking`).
pub fn os_nameservers() -> Result<Vec<IpAddr>> {
    cfg_if::cfg_if! {
        if #[cfg(windows)] {
            win::get_dns_servers()
        } else if #[cfg(unix)] {
            uni::get_dns_servers()
        } else {
            Ok(Vec::new())
        }
    }
}
