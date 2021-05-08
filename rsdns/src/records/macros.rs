macro_rules! rr {
    ($(#[$outer:meta])* $RR:ident, $RRT:expr) => {
        $(#[$outer])*
        pub struct $RR {
            /// A domain name to which this resource record pertains.
            pub name: crate::DomainName,
            /// Resource record class.
            pub rclass: crate::constants::RClass,
            /// The time (in seconds) that the resource record may be cached before it should
            /// be discarded. Zero value means the RR should not be cached.
            pub ttl: u32,
            /// The RR data.
            pub data: crate::records::data::$RR,
        }

        impl $RR {
            /// The RR type.
            pub const RTYPE: crate::constants::RType = $RRT;
        }
    };
}