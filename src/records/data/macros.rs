macro_rules! rr_data {
    ($RR:ident) => {
        impl $RR {
            /// The record type as associated constant.
            pub const RTYPE: Type = Type::$RR;

            /// Returns the record type.
            ///
            /// This is a convenience method to obtain the instance record type.
            #[inline]
            pub const fn rtype(&self) -> Type {
                Self::RTYPE
            }
        }

        impl crate::records::data::private::RDataBase for $RR {
            #[inline]
            fn from(rd: crate::records::data::RecordData) -> crate::Result<Self> {
                match rd {
                    crate::records::data::RecordData::$RR(d) => Ok(d),
                    _ => Err(crate::Error::InternalError("record data conversion failed")),
                }
            }

            #[inline]
            fn from_cursor(c: &mut Cursor<'_>, rdlen: usize) -> crate::Result<Self> {
                c.read_rr_data(rdlen)
            }
        }

        impl crate::records::data::RData for $RR {
            const RTYPE: Type = Self::RTYPE;
        }
    };
}

macro_rules! rr_dn_data {
    ($(#[$outer:meta])* $RR:ident, $(#[$dn_outer:meta])* $DN:ident) => {
        $(#[$outer])*
        #[derive(Clone, Eq, PartialEq, Hash, Default, Debug, Ord, PartialOrd)]
        pub struct $RR {
            $(#[$dn_outer])*
            pub $DN: crate::names::Name,
        }

        rr_data!($RR);

        impl crate::bytes::RrDataReader<$RR> for crate::bytes::Cursor<'_> {
            fn read_rr_data(&mut self, rd_len: usize) -> crate::Result<$RR> {
                use crate::bytes::Reader;
                self.window(rd_len)?;
                let rr = Ok($RR{$DN: self.read()?});
                self.close_window()?;
                rr
            }
        }
    };
}
