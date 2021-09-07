#[cfg(test)]
use crate::constants::{OpCode, RCode};
use crate::message::{MessageType, OpCodeValue, RCodeValue};

macro_rules! get_bit {
    ($e:expr, $l:literal) => {
        ($e & (1 << $l)) != 0
    };
}

cfg_any_client! {
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
}

/// Message flags.
///
/// [RFC 1035 section 4.1.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-4.1.1)
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Flags {
    bits: u16,
}

impl Flags {
    /// Creates new (default) [`Flags`].
    ///
    /// Default [`Flags`] have [`MessageType::Query`] and [`OpCode::Query`].
    ///
    /// [`OpCode::Query`]: crate::constants::OpCode::Query
    pub fn new() -> Flags {
        Flags { bits: 0 }
    }

    /// Returns the message type.
    pub fn message_type(self) -> MessageType {
        (get_bit!(self.bits, 15)).into()
    }

    /// Sets the message type.
    #[cfg(test)]
    pub(crate) fn set_message_type(&mut self, message_type: MessageType) -> &mut Self {
        let value: bool = message_type.into();
        set_bit!(self.bits, 15, value);
        self
    }

    /// Returns the message opcode.
    #[inline]
    pub fn opcode(self) -> OpCodeValue {
        let bits = ((self.bits & 0b0111_1000_0000_0000) >> 11) as u8;
        bits.into()
    }

    /// Sets the message opcode.
    #[cfg(test)]
    pub(crate) fn set_opcode(&mut self, opcode: OpCode) -> &mut Self {
        let mask = 0b0111_1000_0000_0000;
        self.bits = (self.bits & !mask) | (opcode as u16) << 11;
        self
    }

    /// Returns the authoritative answer flag.
    ///
    /// This flag is valid in responses, and specifies that
    /// the responding name server is an authority for the domain name in question section.
    pub fn authoritative_answer(self) -> bool {
        get_bit!(self.bits, 10)
    }

    /// Sets the authoritative answer flag.
    #[cfg(test)]
    pub(crate) fn set_authoritative_answer(&mut self, value: bool) -> &mut Self {
        set_bit!(self.bits, 10, value);
        self
    }

    /// Returns the truncated flag.
    ///
    /// This flag specifies that the message was truncated due to length greater than that
    /// permitted on the transmission channel.
    pub fn truncated(self) -> bool {
        get_bit!(self.bits, 9)
    }

    /// Sets the truncated flag.
    #[cfg(test)]
    pub(crate) fn set_truncated(&mut self, value: bool) -> &mut Self {
        set_bit!(self.bits, 9, value);
        self
    }

    /// Returns the recursion desired flag.
    ///
    /// This flag may be set in a query and is copied into the response. When set, it directs
    /// the name server to pursue the query recursively. Recursive query support is optional.
    pub fn recursion_desired(self) -> bool {
        get_bit!(self.bits, 8)
    }

    cfg_any_client! {
        /// Sets the recursion desired flag.
        pub(crate) fn set_recursion_desired(&mut self, value: bool) -> &mut Self {
            set_bit!(self.bits, 8, value);
            self
        }
    }

    /// Returns the recursion available flag.
    ///
    /// This flag is set or cleared in a response, and denotes whether recursive query support is
    /// available in the name server.
    pub fn recursion_available(self) -> bool {
        get_bit!(self.bits, 7)
    }

    /// Sets the recursion available flag.
    #[cfg(test)]
    pub(crate) fn set_recursion_available(&mut self, value: bool) -> &mut Self {
        set_bit!(self.bits, 7, value);
        self
    }

    /// Returns the Z field.
    ///
    /// Z - reserved for future use
    #[cfg(test)]
    pub(crate) fn z(self) -> u8 {
        (self.bits >> 4) as u8
    }

    /// Sets the Z field.
    #[cfg(test)]
    pub(crate) fn set_z(&mut self, value: u8) -> &mut Self {
        self.bits |= ((value & 0b0000_0111) << 4) as u16;
        self
    }

    /// Returns the response code.
    pub fn response_code(self) -> RCodeValue {
        let bits = self.bits & 0b0000_0000_0000_1111;
        bits.into()
    }

    /// Sets the response code.
    #[cfg(test)]
    pub(crate) fn set_response_code(&mut self, rcode: RCode) -> &mut Self {
        self.bits |= rcode as u16;
        self
    }
}

impl std::fmt::Debug for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#b}", self.bits)
    }
}

impl std::convert::From<u16> for Flags {
    #[inline]
    fn from(flags: u16) -> Flags {
        Flags { bits: flags }
    }
}

impl std::convert::From<Flags> for u16 {
    #[inline]
    fn from(f: Flags) -> u16 {
        f.bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use std::convert::TryFrom;

    type FlagGet = fn(Flags) -> bool;
    type FlagSet = fn(&mut Flags, bool) -> &mut Flags;

    fn test_bool_flag(get: FlagGet, set: FlagSet, mask: u16) {
        let mut f = Flags::default();
        assert_eq!(u16::from(f), 0);
        assert_eq!(get(f), false);

        set(&mut f, true);
        assert_eq!(get(f), true);
        assert_eq!(u16::from(f), mask);

        set(&mut f, false);
        assert_eq!(get(f), false);
        assert_eq!(u16::from(f), 0);
    }

    #[test]
    fn test_bool_flags() {
        test_bool_flag(
            Flags::authoritative_answer,
            Flags::set_authoritative_answer,
            0b0000_0100_0000_0000,
        );
        test_bool_flag(
            Flags::truncated,
            Flags::set_truncated,
            0b0000_0010_0000_0000,
        );
        test_bool_flag(
            Flags::recursion_desired,
            Flags::set_recursion_desired,
            0b0000_0001_0000_0000,
        );
        test_bool_flag(
            Flags::recursion_available,
            Flags::set_recursion_available,
            0b0000_0000_1000_0000,
        );
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
        for opcode in OpCode::VALUES {
            let f = Flags {
                bits: (opcode as u16) << 11,
            };
            assert_eq!(f.opcode(), opcode);

            let mut f = Flags::default();
            assert_eq!(u16::from(f), 0);

            f.set_opcode(opcode);
            assert_eq!(f.opcode(), opcode);
            assert_eq!((u16::from(f) & 0b0111_1000_0000_0000) >> 11, opcode as u16);
        }

        for i in 0..16 {
            if OpCode::VALUES.iter().find(|oc| **oc as u16 == i).is_none() {
                let f = Flags {
                    bits: (i << 11) as u16,
                };
                assert_eq!(f.opcode(), i as u8);
            }
        }
    }

    #[test]
    fn test_response_code() {
        for rcode in RCode::VALUES {
            let f = Flags { bits: rcode as u16 };
            assert_eq!(f.response_code(), rcode);

            let mut f = Flags::default();
            assert_eq!(u16::from(f), 0);

            f.set_response_code(rcode);
            assert_eq!(f.response_code(), rcode);
            assert_eq!(u16::from(f) & 0b0000_0000_0000_1111, rcode as u16);
        }

        for i in 0..16 {
            if RCode::VALUES.iter().find(|rc| **rc as u16 == i).is_none() {
                let f = Flags { bits: i as u16 };
                matches!(
                    RCode::try_from(f.response_code()),
                    Err(Error::UnknownRCode(v)) if v == i
                );
            }
        }
    }

    #[test]
    fn test_z() {
        for i in 0..8 {
            let f = Flags { bits: i << 4 };
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
            assert_eq!(u16::from(f), ((i % 8) << 4) as u16);
        }
    }
}
