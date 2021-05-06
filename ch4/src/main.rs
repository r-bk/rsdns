use anyhow::Result;

#[cfg(windows)]
pub(crate) mod win;

#[cfg(any(
    all(
        feature = "net-tokio",
        any(feature = "net-async-std", feature = "net-smol", feature = "net-std")
    ),
    all(
        feature = "net-async-std",
        any(feature = "net-smol", feature = "net-std")
    ),
    all(feature = "net-smol", feature = "net-std"),
))]
compile_error!("Exactly one of the net features may be selected...");

#[cfg(any(feature = "net-tokio", feature = "net-async-std", feature = "net-smol"))]
mod async_main {
    include!(concat!(env!("OUT_DIR"), "/async_main.rs"));
}

#[cfg(feature = "net-std")]
mod std_main {
    include!(concat!(env!("OUT_DIR"), "/std_main.rs"));
}

pub(crate) mod args;
pub(crate) mod output;

cfg_if::cfg_if! {
    if #[cfg(feature = "net-tokio")] {
        #[tokio::main(flavor = "current_thread")]
        async fn main() -> Result<()> {
            async_main::main().await
        }
    } else if #[cfg(feature = "net-async-std")] {
        #[async_std::main]
        async fn main() -> Result<()> {
            async_main::main().await
        }
    } else if #[cfg(feature = "net-smol")] {
        fn main() -> Result<()> {
            smol::block_on(async {
                async_main::main().await
            })
        }
    } else if #[cfg(feature = "net-std")] {
        fn main() -> Result<()> {
            std_main::main()
        }
    } else {
        compile_error!("One of the net features must be enabled!!!");
    }
}
