mod resolver {
    include!(concat!(env!("OUT_DIR"), "/resolver_tokio.rs"));
}

mod resolver_impl {
    include!(concat!(env!("OUT_DIR"), "/async_resolver_impl_tokio.rs"));
}

pub use resolver::*;
use resolver_impl::ResolverImpl;
