use super::*;
use crate::{
    bytes::Reader,
    constants::DOMAIN_NAME_MAX_POINTERS,
    names::{InlineName, Name},
    Error,
};

#[test]
fn test_basic_flow() {
    let packet = b"\x03sub\x07example\x03com\x00";
    let dn: InlineName = read_domain_name(&mut Cursor::new(&packet[..])).unwrap();

    assert_eq!(dn.as_str(), "sub.example.com.");
}

#[test]
fn test_basic_flow_string() {
    let packet = b"\x03sub\x07example\x03com\x00";
    let dn: Name = read_domain_name(&mut Cursor::new(&packet[..])).unwrap();

    assert_eq!(dn.as_str(), "sub.example.com.");
}

#[test]
fn test_root_domain_name() {
    let packet = b"\x00";

    let dn: InlineName = read_domain_name(&mut Cursor::new(&packet[..])).unwrap();

    assert_eq!(dn.as_str(), ".");
}

#[test]
fn test_root_domain_name_string() {
    let packet = b"\x00";

    let dn: Name = read_domain_name(&mut Cursor::new(&packet[..])).unwrap();

    assert_eq!(dn.as_str(), ".");
}

#[test]
fn test_basic_flow_with_pointers() {
    let packet = b"\x03com\x00\x07example\xC0\x00\x03sub\xC0\x05";

    let dn: InlineName = read_domain_name(&mut Cursor::with_pos(&packet[..], 15)).unwrap();

    assert_eq!(dn.as_str(), "sub.example.com.");
}

#[test]
fn test_invalid_pointer() {
    let packet = b"\x03com\x00\x07example\xC0\x13\x03sub\xC0\x05";

    assert!(matches!(
        read_domain_name::<InlineName>(&mut Cursor::with_pos(&packet[..], 5)),
        Err(Error::DomainNameBadPointer { pointer: p, max_offset: o }) if p == 0x13 && o == 15
    ));
}

#[test]
fn test_pointer_loop() {
    let packet = b"\x03com\x00\x07example\xC0\x0F\x03sub\xC0\x05";

    assert!(matches!(
        read_domain_name::<InlineName>(&mut Cursor::with_pos(&packet[..], 15)),
        Err(Error::DomainNameTooMuchPointers)
    ));
}

#[test]
fn test_invalid_label_type() {
    let packet = b"\x03com\x00\x07example\xC0\x0F\x03sub\xA0\x05";

    assert!(matches!(
        read_domain_name::<InlineName>(&mut Cursor::with_pos(&packet[..], 15)),
        Err(Error::DomainNameBadLabelType(l)) if l == 0xA0
    ));
}

#[test]
fn test_too_much_pointers() {
    let mut packet: Vec<u8> = b"\x07example\x03com\x00".iter().cloned().collect();
    let start = packet.len();

    for i in 0..DOMAIN_NAME_MAX_POINTERS {
        let offset = if i == 0 { 0 } else { start + 2 * (i - 1) };

        packet.push(0xC0);
        packet.push(offset as u8);
    }

    {
        let dn: InlineName =
            read_domain_name(&mut Cursor::with_pos(packet.as_ref(), packet.len() - 2)).unwrap();
        assert_eq!(dn.as_str(), "example.com.");
    }

    {
        packet.push(0xC0);
        packet.push((start + 2 * (DOMAIN_NAME_MAX_POINTERS - 1)) as u8);

        assert!(matches!(
            read_domain_name::<InlineName>(&mut Cursor::with_pos(
                packet.as_ref(),
                packet.len() - 2
            )),
            Err(Error::DomainNameTooMuchPointers)
        ));
    }
}

#[test]
fn test_cursor_read() {
    let packet = b"\x03sub\x07example\x03com\x00";
    let mut cursor = Cursor::new(&packet[..]);
    let dn: InlineName = cursor.read().unwrap();

    assert_eq!(dn.as_str(), "sub.example.com.");
}
