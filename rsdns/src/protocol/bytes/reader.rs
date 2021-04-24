use crate::{protocol::bytes::Cursor, Result};
use std::net::Ipv4Addr;

pub trait Reader<T> {
    fn read(&mut self) -> crate::Result<T>;
}

impl Reader<Ipv4Addr> for Cursor<'_> {
    fn read(&mut self) -> Result<Ipv4Addr> {
        let ip4 = self.u32_be()?;
        Ok(Ipv4Addr::from(ip4))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_read_ipv4() {
        let buf = [192, 168, 2, 1];
        let mut cursor = Cursor::new(&buf);
        let ipv4: Ipv4Addr = cursor.read().expect("failed to read ipv4");
        assert_eq!(
            ipv4,
            Ipv4Addr::from_str("192.168.2.1").expect("failed to parse ipv4")
        );
    }
}
