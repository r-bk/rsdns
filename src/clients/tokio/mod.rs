mod client {
    include!(concat!(env!("OUT_DIR"), "/client_tokio.rs"));
}

mod client_impl {
    include!(concat!(env!("OUT_DIR"), "/async_client_impl_tokio.rs"));
}

pub use client::*;
use client_impl::ClientImpl;
