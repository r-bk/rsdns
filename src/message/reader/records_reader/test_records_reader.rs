use crate::{
    constants::{Class, RecordsSection, Type},
    message::reader::*,
    names::{InlineName, Name},
    records::data::*,
};
use std::{
    convert::TryInto,
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

// ; <<>> ch4 0.6.0 git:34a0bcd <<>> --read bbc.com.json
// ;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 2099
// ;; flags: qr rd ra; QUERY: 1, ANSWER: 4, AUTHORITY: 8, ADDITIONAL: 12
//
// ;; QUESTION SECTION:
// ;bbc.com.                      IN     A
//
// ;; ANSWER SECTION:
// bbc.com.                300    IN     A      151.101.128.81
// bbc.com.                300    IN     A      151.101.192.81
// bbc.com.                300    IN     A      151.101.64.81
// bbc.com.                300    IN     A      151.101.0.81
//
// ;; AUTHORITY SECTION:
// bbc.com.                106241 IN     NS     ddns1.bbc.com.
// bbc.com.                106241 IN     NS     dns0.bbc.com.
// bbc.com.                106241 IN     NS     ddns0.bbc.com.
// bbc.com.                106241 IN     NS     ddns1.bbc.co.uk.
// bbc.com.                106241 IN     NS     dns1.bbc.co.uk.
// bbc.com.                106241 IN     NS     ddns0.bbc.co.uk.
// bbc.com.                106241 IN     NS     dns1.bbc.com.
// bbc.com.                106241 IN     NS     dns0.bbc.co.uk.
//
// ;; ADDITIONAL SECTION:
// dns0.bbc.co.uk.         106241 IN     A      198.51.44.9
// dns0.bbc.com.           106241 IN     A      198.51.44.73
// dns1.bbc.co.uk.         106241 IN     A      198.51.45.9
// dns1.bbc.com.           106241 IN     A      198.51.45.73
// ddns0.bbc.co.uk.        106241 IN     A      148.163.199.1
// ddns0.bbc.com.          106241 IN     A      148.163.199.129
// ddns1.bbc.co.uk.        106241 IN     A      148.163.199.65
// ddns1.bbc.com.          79756  IN     A      148.163.199.193
// dns0.bbc.co.uk.         169870 IN     AAAA   2620:4d:4000:6259:7:9:0:1
// dns1.bbc.co.uk.         169870 IN     AAAA   2a00:edc0:6259:7:9::2
// ddns0.bbc.co.uk.        169870 IN     AAAA   2607:f740:e04e::1
// ddns1.bbc.co.uk.        169870 IN     AAAA   2607:f740:e04e:4::1
#[rustfmt::skip]
const M0: [u8; 494] = [
    0x08, 0x33, 0x81, 0x80, 0x00, 0x01, 0x00, 0x04, 0x00, 0x08, 0x00, 0x0c, // |.3..........| 0
    0x03, 0x62, 0x62, 0x63, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, // |.bbc.com....| 12
    0x01, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x01, 0x2c, 0x00, // |..........,.| 24
    0x04, 0x97, 0x65, 0x80, 0x51, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, // |..e.Q.......| 36
    0x00, 0x01, 0x2c, 0x00, 0x04, 0x97, 0x65, 0xc0, 0x51, 0xc0, 0x0c, 0x00, // |..,...e.Q...| 48
    0x01, 0x00, 0x01, 0x00, 0x00, 0x01, 0x2c, 0x00, 0x04, 0x97, 0x65, 0x40, // |......,...e@| 60
    0x51, 0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x01, 0x2c, 0x00, // |Q.........,.| 72
    0x04, 0x97, 0x65, 0x00, 0x51, 0xc0, 0x0c, 0x00, 0x02, 0x00, 0x01, 0x00, // |..e.Q.......| 84
    0x01, 0x9f, 0x01, 0x00, 0x08, 0x05, 0x64, 0x64, 0x6e, 0x73, 0x31, 0xc0, // |......ddns1.| 96
    0x0c, 0xc0, 0x0c, 0x00, 0x02, 0x00, 0x01, 0x00, 0x01, 0x9f, 0x01, 0x00, // |............| 108
    0x07, 0x04, 0x64, 0x6e, 0x73, 0x30, 0xc0, 0x0c, 0xc0, 0x0c, 0x00, 0x02, // |..dns0......| 120
    0x00, 0x01, 0x00, 0x01, 0x9f, 0x01, 0x00, 0x08, 0x05, 0x64, 0x64, 0x6e, // |.........ddn| 132
    0x73, 0x30, 0xc0, 0x0c, 0xc0, 0x0c, 0x00, 0x02, 0x00, 0x01, 0x00, 0x01, // |s0..........| 144
    0x9f, 0x01, 0x00, 0x11, 0x05, 0x64, 0x64, 0x6e, 0x73, 0x31, 0x03, 0x62, // |.....ddns1.b| 156
    0x62, 0x63, 0x02, 0x63, 0x6f, 0x02, 0x75, 0x6b, 0x00, 0xc0, 0x0c, 0x00, // |bc.co.uk....| 168
    0x02, 0x00, 0x01, 0x00, 0x01, 0x9f, 0x01, 0x00, 0x07, 0x04, 0x64, 0x6e, // |..........dn| 180
    0x73, 0x31, 0xc0, 0xa6, 0xc0, 0x0c, 0x00, 0x02, 0x00, 0x01, 0x00, 0x01, // |s1..........| 192
    0x9f, 0x01, 0x00, 0x08, 0x05, 0x64, 0x64, 0x6e, 0x73, 0x30, 0xc0, 0xa6, // |.....ddns0..| 204
    0xc0, 0x0c, 0x00, 0x02, 0x00, 0x01, 0x00, 0x01, 0x9f, 0x01, 0x00, 0x07, // |............| 216
    0x04, 0x64, 0x6e, 0x73, 0x31, 0xc0, 0x0c, 0xc0, 0x0c, 0x00, 0x02, 0x00, // |.dns1.......| 228
    0x01, 0x00, 0x01, 0x9f, 0x01, 0x00, 0x07, 0x04, 0x64, 0x6e, 0x73, 0x30, // |........dns0| 240
    0xc0, 0xa6, 0xc0, 0xf7, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x9f, 0x01, // |............| 252
    0x00, 0x04, 0xc6, 0x33, 0x2c, 0x09, 0xc0, 0x79, 0x00, 0x01, 0x00, 0x01, // |...3,..y....| 264
    0x00, 0x01, 0x9f, 0x01, 0x00, 0x04, 0xc6, 0x33, 0x2c, 0x49, 0xc0, 0xbd, // |.......3,I..| 276
    0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x9f, 0x01, 0x00, 0x04, 0xc6, 0x33, // |...........3| 288
    0x2d, 0x09, 0xc0, 0xe4, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x9f, 0x01, // |-...........| 300
    0x00, 0x04, 0xc6, 0x33, 0x2d, 0x49, 0xc0, 0xd0, 0x00, 0x01, 0x00, 0x01, // |...3-I......| 312
    0x00, 0x01, 0x9f, 0x01, 0x00, 0x04, 0x94, 0xa3, 0xc7, 0x01, 0xc0, 0x8c, // |............| 324
    0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x9f, 0x01, 0x00, 0x04, 0x94, 0xa3, // |............| 336
    0xc7, 0x81, 0xc0, 0xa0, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x9f, 0x01, // |............| 348
    0x00, 0x04, 0x94, 0xa3, 0xc7, 0x41, 0xc0, 0x65, 0x00, 0x01, 0x00, 0x01, // |.....A.e....| 360
    0x00, 0x01, 0x37, 0x8c, 0x00, 0x04, 0x94, 0xa3, 0xc7, 0xc1, 0xc0, 0xf7, // |..7.........| 372
    0x00, 0x1c, 0x00, 0x01, 0x00, 0x02, 0x97, 0x8e, 0x00, 0x10, 0x26, 0x20, // |..........& | 384
    0x00, 0x4d, 0x40, 0x00, 0x62, 0x59, 0x00, 0x07, 0x00, 0x09, 0x00, 0x00, // |.M@.bY......| 396
    0x00, 0x01, 0xc0, 0xbd, 0x00, 0x1c, 0x00, 0x01, 0x00, 0x02, 0x97, 0x8e, // |............| 408
    0x00, 0x10, 0x2a, 0x00, 0xed, 0xc0, 0x62, 0x59, 0x00, 0x07, 0x00, 0x09, // |..*...bY....| 420
    0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xc0, 0xd0, 0x00, 0x1c, 0x00, 0x01, // |............| 432
    0x00, 0x02, 0x97, 0x8e, 0x00, 0x10, 0x26, 0x07, 0xf7, 0x40, 0xe0, 0x4e, // |......&..@.N| 444
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xc0, 0xa0, // |............| 456
    0x00, 0x1c, 0x00, 0x01, 0x00, 0x02, 0x97, 0x8e, 0x00, 0x10, 0x26, 0x07, // |..........&.| 468
    0xf7, 0x40, 0xe0, 0x4e, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // |.@.N........| 480
    0x00, 0x01, /*                                                       */ // |..| 492
];

#[test]
fn test_whole_message() {
    let mr = MessageReader::new(&M0[..]).expect("failed to create MessageReader");
    let question = mr.question_ref().expect("question_ref failed");

    let mut headers = Vec::new();
    let mut rr = mr.records_reader();
    assert_eq!(rr.count(), 24);
    while rr.has_records() {
        let header = rr.header_ref().expect("header_ref failed");
        rr.skip_data(header.marker()).expect("skip_data failed");

        match header.section() {
            RecordsSection::Answer => assert!(question.qname.eq(header.name()).expect("eq failed")),
            RecordsSection::Additional => {
                assert!(question.qname.ne(header.name()).expect("eq failed"))
            }
            _ => {}
        }

        headers.push(header);
    }

    for h in headers {
        if h.section() != RecordsSection::Authority {
            continue;
        }

        assert_eq!(h.rtype(), Type::Ns);
        assert_eq!(h.rclass(), Class::In);
        assert_eq!(h.ttl(), 106241);

        let name: Name = h.name().try_into().expect("name_ref::try_into failed");
        assert_eq!(name.as_str(), "bbc.com.");

        rr.data_at::<Ns>(h.marker()).expect("data_at failed");
    }
}

#[test]
fn test_answer_section() {
    let mr = MessageReader::new(&M0[..]).expect("failed to create MessageReader");
    let mut rr = mr.records_reader_for(RecordsSection::Answer);
    let mut records = Vec::new();
    let mut headers = Vec::new();
    let mut count = 4;

    loop {
        if !rr.has_records() {
            break;
        }
        assert_eq!(count, rr.count());

        let header = rr.header::<InlineName>().expect("failed to read header");

        assert_eq!(header.name().as_str(), "bbc.com.");
        assert_eq!(header.rtype(), Type::A);
        assert_eq!(header.rclass(), Class::In);
        assert_eq!(header.section(), RecordsSection::Answer);
        assert_eq!(header.ttl(), 300);
        assert_eq!(header.rdlen(), 4);

        records.push(rr.data::<A>(header.marker()).expect("failed to read data"));
        headers.push(header);

        count -= 1;
    }

    assert_eq!(headers.len(), 4);
    assert_eq!(records.len(), 4);
    assert_eq!(
        records,
        vec![
            A {
                address: Ipv4Addr::from_str("151.101.128.81").unwrap()
            },
            A {
                address: Ipv4Addr::from_str("151.101.192.81").unwrap()
            },
            A {
                address: Ipv4Addr::from_str("151.101.64.81").unwrap()
            },
            A {
                address: Ipv4Addr::from_str("151.101.0.81").unwrap()
            },
        ]
    );

    for (h, d) in headers.iter().zip(records.iter()) {
        assert_eq!(*d, rr.data_at::<A>(h.marker()).expect("data_at failed"));
    }
}

#[test]
fn test_data_bytes() {
    let mr = MessageReader::new(&M0[..]).expect("failed to create MessageReader");
    let mut rr = mr.records_reader_for(RecordsSection::Additional);

    let mut markers = Vec::new();
    let mut data = Vec::new();

    while rr.has_records() {
        let marker = rr.marker().expect("marker failed");
        if marker.rtype() == Type::Aaaa {
            data.push(rr.data_bytes(&marker).expect("data_bytes failed"));
            markers.push(marker);
        } else {
            rr.skip_data(&marker).expect("skip_data failed");
        }
    }

    let addresses = vec![
        Ipv6Addr::from_str("2620:4d:4000:6259:7:9:0:1").unwrap(),
        Ipv6Addr::from_str("2a00:edc0:6259:7:9::2").unwrap(),
        Ipv6Addr::from_str("2607:f740:e04e::1").unwrap(),
        Ipv6Addr::from_str("2607:f740:e04e:4::1").unwrap(),
    ];
    assert_eq!(data.len(), addresses.len());

    for (d, a) in data.iter().zip(addresses.iter()) {
        assert_eq!(d, &a.octets());
    }

    for (d, m) in data.iter().zip(markers.iter()) {
        assert_eq!(&rr.data_bytes_at(m).expect("data_bytes_at failed"), d);
    }
}
