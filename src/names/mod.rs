//! Domain name types and utilities.

mod inline_name;
pub use inline_name::*;

mod name;
pub use name::*;

mod dname;
pub use dname::*;

mod utils;
pub(crate) use utils::*;

cfg_any_client! {
    mod writer;
}
