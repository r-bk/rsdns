#![no_main]
use libfuzzer_sys::fuzz_target;
use rsdns::records::{RecordSet, data};

fuzz_target!(|data: &[u8]| {
    RecordSet::<data::A>::from_msg(data).ok();
    RecordSet::<data::Aaaa>::from_msg(data).ok();
    RecordSet::<data::Cname>::from_msg(data).ok();
    RecordSet::<data::Ns>::from_msg(data).ok();
    RecordSet::<data::Txt>::from_msg(data).ok();
    RecordSet::<data::Ptr>::from_msg(data).ok();
    RecordSet::<data::Mx>::from_msg(data).ok();
    RecordSet::<data::Soa>::from_msg(data).ok();
});
