mod client {
    include!(concat!(env!("OUT_DIR"), "/client_async_std.rs"));
}

mod client_impl {
    include!(concat!(env!("OUT_DIR"), "/async_client_impl_async_std.rs"));
}

pub use client::*;
use client_impl::ClientImpl;
