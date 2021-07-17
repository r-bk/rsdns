macro_rules! rr_data {
    ($RR:ident) => {
        impl $RR {
            /// The record type as associated constant.
            pub const RTYPE: RType = RType::$RR;

            /// Returns the record type.
            ///
            /// This is a convenience method to obtain the instance record type.
            #[inline]
            pub const fn rtype(&self) -> RType {
                Self::RTYPE
            }
        }
    };
}

macro_rules! rr_dn_data {
    ($(#[$outer:meta])* $RR:ident, $(#[$dn_outer:meta])* $DN:ident) => {
        $(#[$outer])*
        #[derive(Clone, Eq, PartialEq, Hash, Default, Debug, Ord, PartialOrd)]
        pub struct $RR {
            $(#[$dn_outer])*
            pub $DN: crate::Name,
        }

        rr_data!($RR);

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
