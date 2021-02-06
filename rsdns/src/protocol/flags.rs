use crate::{
    protocol::{Opcode, Rcode},
    Result,
};
use std::convert::TryFrom;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Flags {
    flags: u16,
}

impl Flags {
    pub fn new(flags: u16) -> Flags {
        Flags { flags }
    }

    pub fn as_u16(&self) -> u16 {
        self.flags
    }

    pub fn qr(&self) -> bool {
        get_bit!(self.flags, 15)
    }

    pub fn set_qr(&mut self, value: bool) {
        set_bit!(self.flags, 15, value);
    }

    pub fn opcode(&self) -> Result<Opcode> {
        Opcode::try_from(((self.flags & 0b0111_1000_0000_0000) >> 11) as u8)
    }

    pub fn set_opcode(&mut self, opcode: Opcode) {
        let mask = 0b0111_1000_0000_0000;
        self.flags = (self.flags & !mask) | (opcode as u16) << 11;
    }

    pub fn aa(&self) -> bool {
        get_bit!(self.flags, 10)
    }

    pub fn set_aa(&mut self, value: bool) {
        set_bit!(self.flags, 10, value);
    }

    pub fn tc(&self) -> bool {
        get_bit!(self.flags, 9)
    }

    pub fn set_tc(&mut self, value: bool) {
        set_bit!(self.flags, 9, value);
    }

    pub fn rd(&self) -> bool {
        get_bit!(self.flags, 8)
    }

    pub fn set_rd(&mut self, value: bool) {
        set_bit!(self.flags, 8, value);
    }

    pub fn ra(&self) -> bool {
        get_bit!(self.flags, 7)
    }

    pub fn set_ra(&mut self, value: bool) {
        set_bit!(self.flags, 7, value);
    }

    pub fn z(&self) -> u8 {
        (self.flags >> 4) as u8
    }

    pub fn set_z(&mut self, value: u8) {
        self.flags |= ((value & 0b0000_0111) << 4) as u16;
    }

    pub fn rcode(&self) -> Result<Rcode> {
        Rcode::try_from((self.flags & 0b0000_0000_0000_1111) as u8)
    }

    pub fn set_rcode(&mut self, rcode: Rcode) {
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
        for opcode in Opcode::iter() {
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
            if Opcode::iter().find(|oc| *oc as u16 == i).is_none() {
                let f = Flags {
                    flags: (i << 11) as u16,
                };
                match f.opcode() {
                    Err(RsDnsError::ProtocolUnknownOpcode(v)) => assert_eq!(v, i as u8),
                    _ => panic!("unexpected success"),
                }
            }
        }
    }

    #[test]
    fn test_rcode() {
        for rcode in Rcode::iter() {
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
            if Rcode::iter().find(|rc| *rc as u16 == i).is_none() {
                let f = Flags { flags: i as u16 };
                match f.rcode() {
                    Err(RsDnsError::ProtocolUnknownRcode(v)) => assert_eq!(v, i as u8),
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
