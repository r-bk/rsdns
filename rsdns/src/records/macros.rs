macro_rules! rr {
    ($(#[$outer:meta])* $RR:ident, $RRT:expr) => {
        $(#[$outer])*
        pub struct $RR {
            /// A domain name to which this resource record pertains.
            pub name: crate::InlineName,
            /// Resource record class.
            pub rclass: crate::constants::RClass,
            /// The time (in seconds) that the resource record may be cached before it should
            /// be discarded. Zero value means the record should not be cached.
            pub ttl: u32,
            /// The record data.
            pub rdata: crate::records::data::$RR,
        }

        impl $RR {
            /// The record type.
            pub const RTYPE: crate::constants::RType = $RRT;
        }
    };
}
