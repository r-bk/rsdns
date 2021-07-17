macro_rules! cfg_any_resolver {
    ($($item:item)*) => {
        $(
            #[cfg(
                any(
                    feature = "net-async-std",
                    feature = "net-smol",
                    feature = "net-std",
                    feature = "net-tokio",
                    test
                )
            )]
            $item
        )*
    }
}
