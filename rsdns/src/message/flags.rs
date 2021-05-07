use crate::{
    constants::{OpCode, ResponseCode},
    message::MessageType,
    Result,
};
use std::convert::TryFrom;

macro_rules! get_bit {
    ($e:expr, $l:literal) => {
        ($e & (1 << $l)) != 0
    };
}

macro_rules! set_bit {
    ($e:expr, $l:literal, $v:ident) => {
        let mask = 1 << $l;
        if $v {
            $e |= mask;
        } else {
            $e &= !mask;
        }
    };
}

/// DNS message header flags.
///
/// [RFC 1035 ~4.1.1](https://tools.ietf.org/html/rfc1035)
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Flags {
    flags: u16,
}

impl Flags {
    /// Creates new (zero) Flags.
    pub fn new() -> Flags {
        Flags { flags: 0 }
    }

    /// Converts to the underlying primitive type.
    pub fn as_u16(self) -> u16 {
        self.flags
    }

    /// Returns the message type.
    pub fn message_type(self) -> MessageType {
        (get_bit!(self.flags, 15)).into()
    }

    /// Sets the message type.
    pub fn set_message_type(&mut self, message_type: MessageType) -> Self {
        let value: bool = message_type.into();
        set_bit!(self.flags, 15, value);
        *self
    }

    /// Checks if the message type is [Query](MessageType::Query).
    #[inline]
    pub fn is_query(&self) -> bool {
        self.message_type() == MessageType::Query
    }

    /// Checks if the message type is [Response](MessageType::Response).
    #[inline]
    pub fn is_response(&self) -> bool {
        self.message_type() == MessageType::Response
    }

    /// Returns the message **OPCODE**.
    pub fn opcode(self) -> Result<OpCode> {
        OpCode::try_from(((self.flags & 0b0111_1000_0000_0000) >> 11) as u8)
    }

    /// Sets the **OPCODE**.
    pub fn set_opcode(&mut self, opcode: OpCode) -> Self {
        let mask = 0b0111_1000_0000_0000;
        self.flags = (self.flags & !mask) | (opcode as u16) << 11;
        *self
    }

    /// Returns the **AA** flag.
    ///
    /// AA - authoritative answer.
    /// This bit is valid in responses, and specifies that
    /// the responding name server is an authority for the domain name in question section.
    pub fn aa(self) -> bool {
        get_bit!(self.flags, 10)
    }

    /// Sets the **AA** flag.
    pub fn set_aa(&mut self, value: bool) -> Self {
        set_bit!(self.flags, 10, value);
        *self
    }

    /// Returns the **TC** flag.
    ///
    /// TC specifies that the message was truncated due to length greater than that permitted on the
    /// transmission channel.
    pub fn tc(self) -> bool {
        get_bit!(self.flags, 9)
    }

    /// Sets the TC flag.
    pub fn set_tc(&mut self, value: bool) -> Self {
        set_bit!(self.flags, 9, value);
        *self
    }

    /// Returns the RD flag.
    ///
    /// RD - recursion desired.
    /// This flag may be set in a query and is copied into the response. If RD is set, it directs
    /// the name server to pursue the query recursively. Recursive query support is optional.
    pub fn rd(self) -> bool {
        get_bit!(self.flags, 8)
    }

    /// Sets the RD flag.
    pub fn set_rd(&mut self, value: bool) -> Self {
        set_bit!(self.flags, 8, value);
        *self
    }

    /// Returns the RA flag.
    ///
    /// RA - recursion available.
    /// This flag is set or cleared in a response, and denotes whether recursive query support is
    /// available in the name server.
    pub fn ra(self) -> bool {
        get_bit!(self.flags, 7)
    }

    /// Sets the RA flag.
    pub fn set_ra(&mut self, value: bool) -> Self {
        set_bit!(self.flags, 7, value);
        *self
    }

    /// Returns the Z field.
    ///
    /// Z - reserved for future use
    pub fn z(self) -> u8 {
        (self.flags >> 4) as u8
    }

    /// Sets the Z field.
    pub fn set_z(&mut self, value: u8) -> Self {
        self.flags |= ((value & 0b0000_0111) << 4) as u16;
        *self
    }

    /// Returns the response code.
    pub fn response_code(self) -> Result<ResponseCode> {
        ResponseCode::try_from((self.flags & 0b0000_0000_0000_1111) as u8)
    }

    /// Sets the response code.
    pub fn set_response_code(&mut self, rcode: ResponseCode) -> Self {
        self.flags |= rcode as u16;
        *self
    }
}

impl std::fmt::Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#b}", self.flags)
    }
}

impl std::convert::From<u16> for Flags {
    #[inline]
    fn from(flags: u16) -> Flags {
        Flags { flags }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use strum::IntoEnumIterator;

    type FlagGet = fn(Flags) -> bool;
    type FlagSet = fn(&mut Flags, bool) -> Flags;

    fn test_bool_flag(get: FlagGet, set: FlagSet, mask: u16) {
        let mut f = Flags::default();
        assert_eq!(f.as_u16(), 0);
        assert_eq!(get(f), false);

        set(&mut f, true);
        assert_eq!(get(f), true);
        assert_eq!(f.as_u16(), mask);

        set(&mut f, false);
        assert_eq!(get(f), false);
        assert_eq!(f.as_u16(), 0);
    }

    #[test]
    fn test_bool_flags() {
        test_bool_flag(Flags::aa, Flags::set_aa, 0b0000_0100_0000_0000);
        test_bool_flag(Flags::tc, Flags::set_tc, 0b0000_0010_0000_0000);
        test_bool_flag(Flags::rd, Flags::set_rd, 0b0000_0001_0000_0000);
        test_bool_flag(Flags::ra, Flags::set_ra, 0b0000_0000_1000_0000);
    }

    #[test]
    fn test_message_flags() {
        let mut f = Flags::default();
        assert_eq!(f.message_type(), MessageType::Query);

        f.set_message_type(MessageType::Response);
        assert_eq!(f.message_type(), MessageType::Response);

        f.set_message_type(MessageType::Query);
        assert_eq!(f.message_type(), MessageType::Query);
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
                    Err(Error::UnknownOpCode(v)) => assert_eq!(v, i as u8),
                    _ => panic!("unexpected success"),
                }
            }
        }
    }

    #[test]
    fn test_response_code() {
        for rcode in ResponseCode::iter() {
            let f = Flags {
                flags: rcode as u16,
            };
            assert_eq!(f.response_code().unwrap(), rcode);

            let mut f = Flags::default();
            assert_eq!(f.as_u16(), 0);

            f.set_response_code(rcode);
            assert_eq!(f.response_code().unwrap(), rcode);
            assert_eq!(f.as_u16() & 0b0000_0000_0000_1111, rcode as u16);
        }

        for i in 0..16 {
            if ResponseCode::iter().find(|rc| *rc as u16 == i).is_none() {
                let f = Flags { flags: i as u16 };
                match f.response_code() {
                    Err(Error::UnknownResponseCode(v)) => assert_eq!(v, i as u8),
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
