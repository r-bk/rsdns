#[cfg(feature = "net-std")]
#[allow(unused_imports)]
use rsdns::clients::std::Client;

cfg_if::cfg_if! {
    if #[cfg(feature = "net-std")] {
        #[test]
        fn test_partial_eq_doesnt_cause_compilation_errors() {
            let b: Vec<u8> = Vec::new();
            assert_eq!(b, []);
        }
    }
}
