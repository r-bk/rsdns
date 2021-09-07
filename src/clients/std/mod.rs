mod client {
    include!(concat!(env!("OUT_DIR"), "/client_std.rs"));
}

mod client_impl;
pub use client::*;
use client_impl::ClientImpl;
