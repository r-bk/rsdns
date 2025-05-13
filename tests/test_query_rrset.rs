#[cfg(any(
    feature = "net-tokio",
    feature = "net-async-std",
    feature = "net-smol",
    feature = "net-std"
))]
use {
    rsdns::records::{Class, RecordSet, data::A},
    std::{
        net::{IpAddr, SocketAddr, ToSocketAddrs},
        str::FromStr,
    },
};

#[cfg(any(
    feature = "net-tokio",
    feature = "net-async-std",
    feature = "net-smol",
    feature = "net-std"
))]
const HOSTNAME: &str = "a.gtld-servers.net";

cfg_if::cfg_if! {
    if #[cfg(feature = "net-tokio")] {
        use rsdns::clients::{tokio::Client, ClientConfig};
    } else if #[cfg(feature = "net-async-std")] {
        use rsdns::clients::{async_std::Client, ClientConfig};
    } else if #[cfg(feature = "net-smol")] {
        use rsdns::clients::{smol::Client, ClientConfig};
    } else if #[cfg(feature = "net-std")] {
        use rsdns::clients::ClientConfig;
    }
}

#[cfg(any(
    feature = "net-tokio",
    feature = "net-async-std",
    feature = "net-smol",
    feature = "net-std"
))]
fn check_rrset(rrset: &RecordSet<A>) {
    let ip_addrs: Vec<IpAddr> = (HOSTNAME, 53)
        .to_socket_addrs()
        .unwrap()
        .map(|sa| sa.ip())
        .collect();

    assert!(ip_addrs.is_empty() || !rrset.rdata.is_empty());
    if !ip_addrs.is_empty() {
        for rdata in &rrset.rdata {
            let addr = rdata.address;
            assert!(ip_addrs.iter().any(|ipa| *ipa == addr));
        }
    }
}

#[cfg(any(feature = "net-tokio", feature = "net-async-std", feature = "net-smol"))]
async fn test_async_query_rrset() {
    let config = ClientConfig::with_nameserver(SocketAddr::from_str("8.8.8.8:53").unwrap());
    let mut client = Client::new(config).await.unwrap();
    let rrset = client.query_rrset::<A>(HOSTNAME, Class::IN).await.unwrap();
    check_rrset(&rrset);
}

#[cfg(all(
    feature = "net-std",
    not(any(feature = "net-tokio", feature = "net-async-std", feature = "net-smol"))
))]
fn test_sync_query_rrset() {
    let config = ClientConfig::with_nameserver(SocketAddr::from_str("8.8.8.8:53").unwrap());
    let mut client = rsdns::clients::std::Client::new(config).unwrap();
    let rrset = client.query_rrset::<A>(HOSTNAME, Class::IN).unwrap();
    check_rrset(&rrset);
}

cfg_if::cfg_if! {
    if #[cfg(feature = "net-tokio")] {
        #[tokio::test]
        #[cfg_attr(miri, ignore)]
        async fn test_query_rrset() {
            test_async_query_rrset().await
        }

        #[tokio::test]
        #[cfg_attr(miri, ignore)]
        async fn test_query_rrset_spawn() {
            let handle = tokio::spawn(async { test_async_query_rrset().await } );
            handle.await.unwrap()
        }
    } else if #[cfg(feature = "net-async-std")] {
        #[async_std::test]
        #[cfg_attr(miri, ignore)]
        async fn test_query_rrset() {
            test_async_query_rrset().await
        }

        #[async_std::test]
        #[cfg_attr(miri, ignore)]
        async fn test_query_rrset_spawn() {
            let handle = async_std::task::spawn(async { test_async_query_rrset().await } );
            handle.await
        }
    } else if #[cfg(feature = "net-smol")] {
        #[smol_potat::test]
        #[cfg_attr(miri, ignore)]
        async fn test_query_rrset() {
            test_async_query_rrset().await
        }

        #[smol_potat::test]
        #[cfg_attr(miri, ignore)]
        async fn test_query_rrset_spawn() {
            let handle = smol::spawn(async { test_async_query_rrset().await } );
            handle.await
        }
    } else if #[cfg(feature = "net-std")] {
        #[test]
        #[cfg_attr(miri, ignore)]
        fn test_query_rrset() {
            test_sync_query_rrset()
        }
    }
}
