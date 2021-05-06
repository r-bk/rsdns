mod resolver {
    include!(concat!(env!("OUT_DIR"), "/resolver_smol.rs"));
}

mod resolver_impl {
    include!(concat!(env!("OUT_DIR"), "/async_resolver_impl_smol.rs"));
}

pub use resolver::*;
use resolver_impl::ResolverImpl;
