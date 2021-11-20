cfg_any_client! {
    use crate::{constants::Type, Result};
}

/// OPT pseudo-record.
///
/// - [RFC 2671](https://www.rfc-editor.org/rfc/rfc2671.html)
/// - [RFC 6891](https://www.rfc-editor.org/rfc/rfc6891.html)
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Opt {
    udp_payload_size: u16,
    rcode_extension: u8,
    version: u8,
    flags: u16,
}

impl Opt {
    cfg_any_client! {
        #[allow(dead_code)]
        #[inline]
        pub(crate) fn new(version: u8, udp_payload_size: u16) -> Opt {
            Opt {
                udp_payload_size,
                version,
                ..Default::default()
            }
        }

        fn ttl(&self) -> u32 {
            (self.rcode_extension as u32) << 24 | (self.version as u32) << 16 | self.flags as u32
        }
    }

    #[inline]
    pub(crate) fn from_msg(rclass: u16, ttl: u32) -> Opt {
        Opt {
            udp_payload_size: rclass,
            rcode_extension: ((ttl & 0xFF000000u32) >> 24) as u8,
            version: ((ttl & 0x00FF0000u32) >> 16) as u8,
            flags: (ttl & 0x0000FFFF) as u16,
        }
    }

    /// Returns the UDP payload size
    #[inline]
    pub fn udp_payload_size(&self) -> u16 {
        self.udp_payload_size
    }

    /// Returns the EDNS `RCODE` extension value (upper 8 bits).
    ///
    /// See [`RCodeValue::extended`] for a way to combine a base `RCODE` value
    /// from the message header and this extension to a final extended `RCODE` value.
    ///
    /// [RFC 6891 section 6.1.3](https://www.rfc-editor.org/rfc/rfc6891.html#section-6.1.3)
    ///
    /// [`RCodeValue::extended`]: crate::message::RCodeValue::extended
    #[inline]
    pub fn rcode_extension(&self) -> u8 {
        self.rcode_extension
    }

    /// Returns the `OPT` version.
    #[inline]
    pub fn version(&self) -> u8 {
        self.version
    }

    /// Returns the `DNSSEC OK` bit.
    ///
    /// [RFC3225](https://www.rfc-editor.org/rfc/rfc3225.html)
    #[inline]
    pub fn dnssec_ok(&self) -> bool {
        (self.flags & 0b1000_0000_0000_0000) != 0
    }
}

cfg_any_client! {
    impl crate::bytes::WCursor<'_> {
        pub(crate) fn write_opt(&mut self, opt: &Opt) -> Result<()> {
            self.u8(0)?; // DNAME
            self.u16_be(Type::Opt as u16)?; // TYPE
            self.u16_be(opt.udp_payload_size)?; // CLASS
            self.u32_be(opt.ttl())?; // TTL
            self.u16_be(0)?; // RDLEN
            Ok(())
        }
    }
}
