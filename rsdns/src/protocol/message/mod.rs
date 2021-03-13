//! DNS message utilities.

mod bytes;
mod cursor;
pub mod reader;
mod wcursor;

pub(crate) use bytes::*;
pub(crate) use cursor::*;
#[allow(unused_imports)]
pub(crate) use wcursor::*;

pub use reader::MessageReader;
