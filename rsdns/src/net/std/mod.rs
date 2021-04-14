mod resolver {
    include!(concat!(env!("OUT_DIR"), "/resolver_std.rs"));
}

mod resolver_impl;
pub use resolver::*;
use resolver_impl::ResolverImpl;
