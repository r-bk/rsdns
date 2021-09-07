mod client {
    include!(concat!(env!("OUT_DIR"), "/client_smol.rs"));
}

mod client_impl {
    include!(concat!(env!("OUT_DIR"), "/async_client_impl_smol.rs"));
}

pub use client::*;
use client_impl::ClientImpl;
