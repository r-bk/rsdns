use crate::{
    protocol::{OpCode, RCode},
    Result,
};
use std::convert::TryFrom;

/// DNS message header flags.
///
/// [RFC1035 ~4.1.1](https://tools.ietf.org/html/rfc1035)
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Flags {
    flags: u16,
}

impl Flags {
    /// Creates new Flags out of on-wire representation.
    pub fn new(flags: u16) -> Flags {
        Flags { flags }
    }

    /// Converts to underlying primitive type.
    pub fn as_u16(&self) -> u16 {
        self.flags
    }

    /// Returns the **QR** flag.
    ///
    /// The flag indicates if the message is a **QUERY** (`false`) or a **RESPONSE** (`true`).
    pub fn qr(&self) -> bool {
        get_bit!(self.flags, 15)
    }

    /// Sets the **QR** flag.
    ///
    /// **QUERY** = `false`
    /// **RESPONSE** = `true`
    pub fn set_qr(&mut self, value: bool) {
        set_bit!(self.flags, 15, value);
    }

    /// Returns the message **OPCODE**.
    pub fn opcode(&self) -> Result<OpCode> {
        OpCode::try_from(((self.flags & 0b0111_1000_0000_0000) >> 11) as u8)
    }

    /// Sets the **OPCODE**.
    pub fn set_opcode(&mut self, opcode: OpCode) {
        let mask = 0b0111_1000_0000_0000;
        self.flags = (self.flags & !mask) | (opcode as u16) << 11;
    }

    /// Returns the **AA** flag.
    ///
    /// AA - authoritative answer.
    /// This bit is valid in responses, and specifies that
    /// the responding name server is an authority for the domain name in question section.
    pub fn aa(&self) -> bool {
        get_bit!(self.flags, 10)
    }

    /// Sets the **AA** flag.
    pub fn set_aa(&mut self, value: bool) {
        set_bit!(self.flags, 10, value);
    }

    /// Returns the **TC** flag.
    ///
    /// TC specifies that the message was truncated due to length greater than that permitted on the
    /// transmission channel.
    pub fn tc(&self) -> bool {
        get_bit!(self.flags, 9)
    }

    /// Sets the TC flag.
    pub fn set_tc(&mut self, value: bool) {
        set_bit!(self.flags, 9, value);
    }

    /// Returns the RD flag.
    ///
    /// RD - recursion desired.
    /// This flag may be set in a query and is copied into the response. If RD is set, it directs
    /// the name server to pursue the query recursively. Recursive query support is optional.
    pub fn rd(&self) -> bool {
        get_bit!(self.flags, 8)
    }

    /// Sets the RD flag.
    pub fn set_rd(&mut self, value: bool) {
        set_bit!(self.flags, 8, value);
    }

    /// Returns the RA flag.
    ///
    /// RA - recursion available.
    /// This flag is set or cleared in a response, and denotes whether recursive query support is
    /// available in the name server.
    pub fn ra(&self) -> bool {
        get_bit!(self.flags, 7)
    }

    /// Sets the RA flag.
    pub fn set_ra(&mut self, value: bool) {
        set_bit!(self.flags, 7, value);
    }

    /// Returns the Z field.
    ///
    /// Z - reserved for future use
    pub fn z(&self) -> u8 {
        (self.flags >> 4) as u8
    }

    /// Sets the Z field.
    pub fn set_z(&mut self, value: u8) {
        self.flags |= ((value & 0b0000_0111) << 4) as u16;
    }

    /// Returns the RCODE.
    pub fn rcode(&self) -> Result<RCode> {
        RCode::try_from((self.flags & 0b0000_0000_0000_1111) as u8)
    }

    /// Sets the RCODE.
    pub fn set_rcode(&mut self, rcode: RCode) {
        self.flags |= rcode as u16;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RsDnsError;
    use strum::IntoEnumIterator;

    type FlagGet = fn(&Flags) -> bool;
    type FlagSet = fn(&mut Flags, bool);

    fn test_bool_flag(get: FlagGet, set: FlagSet, mask: u16) {
        let mut f = Flags::default();
        assert_eq!(f.as_u16(), 0);
        assert_eq!(get(&f), false);

        set(&mut f, true);
        assert_eq!(get(&f), true);
        assert_eq!(f.as_u16(), mask);

        set(&mut f, false);
        assert_eq!(get(&f), false);
        assert_eq!(f.as_u16(), 0);
    }

    #[test]
    fn test_bool_flags() {
        test_bool_flag(Flags::qr, Flags::set_qr, 0b1000_0000_0000_0000);
        test_bool_flag(Flags::aa, Flags::set_aa, 0b0000_0100_0000_0000);
        test_bool_flag(Flags::tc, Flags::set_tc, 0b0000_0010_0000_0000);
        test_bool_flag(Flags::rd, Flags::set_rd, 0b0000_0001_0000_0000);
        test_bool_flag(Flags::ra, Flags::set_ra, 0b0000_0000_1000_0000);
    }

    #[test]
    fn test_opcode() {
        for opcode in OpCode::iter() {
            let f = Flags {
                flags: (opcode as u16) << 11,
            };
            assert_eq!(f.opcode().unwrap(), opcode);

            let mut f = Flags::default();
            assert_eq!(f.as_u16(), 0);

            f.set_opcode(opcode);
            assert_eq!(f.opcode().unwrap(), opcode);
            assert_eq!((f.as_u16() & 0b0111_1000_0000_0000) >> 11, opcode as u16);
        }

        for i in 0..16 {
            if OpCode::iter().find(|oc| *oc as u16 == i).is_none() {
                let f = Flags {
                    flags: (i << 11) as u16,
                };
                match f.opcode() {
                    Err(RsDnsError::ProtocolUnknownOpCode(v)) => assert_eq!(v, i as u8),
                    _ => panic!("unexpected success"),
                }
            }
        }
    }

    #[test]
    fn test_rcode() {
        for rcode in RCode::iter() {
            let f = Flags {
                flags: rcode as u16,
            };
            assert_eq!(f.rcode().unwrap(), rcode);

            let mut f = Flags::default();
            assert_eq!(f.as_u16(), 0);

            f.set_rcode(rcode);
            assert_eq!(f.rcode().unwrap(), rcode);
            assert_eq!(f.as_u16() & 0b0000_0000_0000_1111, rcode as u16);
        }

        for i in 0..16 {
            if RCode::iter().find(|rc| *rc as u16 == i).is_none() {
                let f = Flags { flags: i as u16 };
                match f.rcode() {
                    Err(RsDnsError::ProtocolUnknownRCode(v)) => assert_eq!(v, i as u8),
                    _ => panic!("unexpected success"),
                }
            }
        }
    }

    #[test]
    fn test_z() {
        for i in 0..8 {
            let f = Flags { flags: i << 4 };
            assert_eq!(f.z(), i as u8);

            let mut f = Flags::default();
            assert_eq!(f.z(), 0);

            f.set_z(i as u8);
            assert_eq!(f.z(), i as u8);
        }

        for i in 8..256 {
            let mut f = Flags::default();
            assert_eq!(f.z(), 0);

            f.set_z(i as u8);
            assert_eq!(f.z(), (i % 8) as u8);
            assert_eq!(f.as_u16(), ((i % 8) << 4) as u16);
        }
    }
}
