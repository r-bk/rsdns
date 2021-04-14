mod resolver {
    include!(concat!(env!("OUT_DIR"), "/resolver_async_std.rs"));
}

mod resolver_impl {
    include!(concat!(
        env!("OUT_DIR"),
        "/async_resolver_impl_async_std.rs"
    ));
}

pub use resolver::*;
use resolver_impl::ResolverImpl;
