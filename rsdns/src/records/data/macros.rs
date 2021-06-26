macro_rules! rr_data {
    ($RR:ty, $RRT:expr) => {
        impl $RR {
            /// The RR type.
            pub const RTYPE: RType = $RRT;
        }
    };
}

macro_rules! rr_dn_data {
    ($(#[$outer:meta])* $RR:ident, $RRT:expr, $(#[$dn_outer:meta])* $DN:ident) => {
        $(#[$outer])*
        #[derive(Clone, Eq, PartialEq, Hash, Default, Debug, Ord, PartialOrd)]
        pub struct $RR {
            $(#[$dn_outer])*
            pub $DN: crate::Name,
        }

        rr_data!($RR, $RRT);

        impl crate::bytes::RrDataReader<$RR> for crate::bytes::Cursor<'_> {
            fn read_rr_data(&mut self, rd_len: usize) -> crate::ProtocolResult<$RR> {
                use crate::bytes::Reader;
                self.window(rd_len)?;
                let rr = Ok($RR{$DN: self.read()?});
                self.close_window()?;
                rr
            }
        }
    };
}
