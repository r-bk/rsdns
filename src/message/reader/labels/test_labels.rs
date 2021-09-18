use super::*;
use crate::{
    bytes::Reader,
    constants::DOMAIN_NAME_MAX_POINTERS,
    names::{InlineName, Name},
    Error,
};

// ; <<>> ch4 0.6.0 <<>> --read ibm.ns
// ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 19921
// ;; flags: qr rd ra; QUERY: 1, ANSWER: 8, AUTHORITY: 0, ADDITIONAL: 0
//
// ;; QUESTION SECTION:
// ;ibm.com.                      IN     NS
//
// ;; ANSWER SECTION:
// ibm.com.                7154   IN     NS     usc2.akam.net.
// ibm.com.                7154   IN     NS     asia3.akam.net.
// ibm.com.                7154   IN     NS     usw2.akam.net.
// ibm.com.                7154   IN     NS     eur2.akam.net.
// ibm.com.                7154   IN     NS     eur5.akam.net.
// ibm.com.                7154   IN     NS     ns1-99.akam.net.
// ibm.com.                7154   IN     NS     ns1-206.akam.net.
// ibm.com.                7154   IN     NS     usc3.akam.net.
//
// ;; Query time: 605.526µs
// ;; SERVER: 127.0.0.53:53
// ;; WHEN: Sat, 18 Sep 2021 07:25:20 +0300
// ;; MSG SIZE rcvd: 191
const IBM_NS: [u8; 191] = [
    0x4d, 0xd1, 0x81, 0x80, 0x00, 0x01, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, // |M...........|
    0x03, 0x69, 0x62, 0x6d, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x02, 0x00, // |.ibm.com....|
    0x01, 0xc0, 0x0c, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x1b, 0xf2, 0x00, // |............|
    0x0f, 0x04, 0x75, 0x73, 0x63, 0x32, 0x04, 0x61, 0x6b, 0x61, 0x6d, 0x03, // |..usc2.akam.|
    0x6e, 0x65, 0x74, 0x00, 0xc0, 0x0c, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, // |net.........|
    0x1b, 0xf2, 0x00, 0x08, 0x05, 0x61, 0x73, 0x69, 0x61, 0x33, 0xc0, 0x2a, // |.....asia3.*|
    0xc0, 0x0c, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x1b, 0xf2, 0x00, 0x07, // |............|
    0x04, 0x75, 0x73, 0x77, 0x32, 0xc0, 0x2a, 0xc0, 0x0c, 0x00, 0x02, 0x00, // |.usw2.*.....|
    0x01, 0x00, 0x00, 0x1b, 0xf2, 0x00, 0x07, 0x04, 0x65, 0x75, 0x72, 0x32, // |........eur2|
    0xc0, 0x2a, 0xc0, 0x0c, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x1b, 0xf2, // |.*..........|
    0x00, 0x07, 0x04, 0x65, 0x75, 0x72, 0x35, 0xc0, 0x2a, 0xc0, 0x0c, 0x00, // |...eur5.*...|
    0x02, 0x00, 0x01, 0x00, 0x00, 0x1b, 0xf2, 0x00, 0x09, 0x06, 0x6e, 0x73, // |..........ns|
    0x31, 0x2d, 0x39, 0x39, 0xc0, 0x2a, 0xc0, 0x0c, 0x00, 0x02, 0x00, 0x01, // |1-99.*......|
    0x00, 0x00, 0x1b, 0xf2, 0x00, 0x0a, 0x07, 0x6e, 0x73, 0x31, 0x2d, 0x32, // |.......ns1-2|
    0x30, 0x36, 0xc0, 0x2a, 0xc0, 0x0c, 0x00, 0x02, 0x00, 0x01, 0x00, 0x00, // |06.*........|
    0x1b, 0xf2, 0x00, 0x07, 0x04, 0x75, 0x73, 0x63, 0x33, 0xc0, 0x2a, // |.....usc3.*|
];

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

#[test]
fn test_labels() {
    let expectations: &[(usize, &[&[u8]])] = &[
        (12, &[b"ibm", b"com"]),
        (37, &[b"usc2", b"akam", b"net"]),
        (64, &[b"asia3", b"akam", b"net"]),
        (84, &[b"usw2", b"akam", b"net"]),
        (103, &[b"eur2", b"akam", b"net"]),
        (122, &[b"eur5", b"akam", b"net"]),
        (141, &[b"ns1-99", b"akam", b"net"]),
        (162, &[b"ns1-206", b"akam", b"net"]),
        (184, &[b"usc3", b"akam", b"net"]),
    ];

    for t in expectations {
        let cursor = Cursor::new(&IBM_NS[..]);
        let mut labels = Labels::new(cursor.clone_with_pos(t.0));
        let ex = t.1;
        for exl in ex {
            let l = labels.next().unwrap().unwrap();
            assert_eq!(&exl[..], l.bytes());
        }
        assert!(labels.next().is_none());
    }
}

#[test]
fn test_read_compressed_name() {
    let expectations: &[(usize, &str, usize)] = &[
        (12, "ibm.com.", 21),
        (37, "usc2.akam.net.", 52),
        (64, "asia3.akam.net.", 72),
        (84, "usw2.akam.net.", 91),
        (103, "eur2.akam.net.", 110),
        (122, "eur5.akam.net.", 129),
        (141, "ns1-99.akam.net.", 150),
        (162, "ns1-206.akam.net.", 172),
        (184, "usc3.akam.net.", 191),
    ];

    for t in expectations {
        let mut cursor = Cursor::with_pos(&IBM_NS[..], t.0);
        let dn: Name = read_domain_name(&mut cursor).unwrap();
        assert_eq!(dn.as_str(), t.1);
        assert_eq!(cursor.pos(), t.2);
    }
}

#[test]
fn test_skip_compressed_name() {
    let expectations: &[(usize, usize)] = &[
        (12, 21),
        (37, 52),
        (64, 72),
        (84, 91),
        (103, 110),
        (122, 129),
        (141, 150),
        (162, 172),
        (184, 191),
    ];

    for t in expectations {
        let mut cursor = Cursor::with_pos(&IBM_NS[..], t.0);
        skip_domain_name(&mut cursor).expect("skip_dn failed");
        assert_eq!(cursor.pos(), t.1);
    }
}
