use crate::{Error, Result};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

mod bindings {
    windows::include_bindings!();
}

use bindings::Windows::Win32::{
    NetworkManagement::IpHelper::*,
    Networking::WinSock::{SOCKADDR_IN, SOCKADDR_IN6},
    System::Diagnostics::Debug::*,
};

pub fn get_dns_servers() -> Result<Vec<IpAddr>> {
    let flags = GAA_FLAG_INCLUDE_TUNNEL_BINDINGORDER;

    let ans = unsafe {
        let mut buf_size = 0;
        let error = GetAdaptersAddresses(
            AF_UNSPEC,
            flags,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut buf_size,
        );

        match WIN32_ERROR::from(error) {
            ERROR_BUFFER_OVERFLOW => {}
            e => {
                return Err(Error::Win32Error(
                    "GetAdaptersAddress #1 unexpected error",
                    e.0,
                ));
            }
        }

        let block_size = std::mem::size_of::<IP_ADAPTER_ADDRESSES_LH>() as u32;
        let new_capacity = buf_size + block_size;
        let mut buf = Vec::<u8>::with_capacity(new_capacity as usize);

        let (prefix, body, _) = buf.align_to_mut::<IP_ADAPTER_ADDRESSES_LH>();

        let mut buf_size = new_capacity - prefix.len() as u32;
        let error = GetAdaptersAddresses(
            AF_UNSPEC,
            flags,
            std::ptr::null_mut(),
            body.as_mut_ptr(),
            &mut buf_size,
        );

        match WIN32_ERROR::from(error) {
            NO_ERROR => {}
            e => {
                return Err(Error::Win32Error("GetAdaptersAddress #2 failed", e.0));
            }
        }

        let mut ans = Vec::new();
        let mut p_adapter = body.as_mut_ptr();
        while !p_adapter.is_null() {
            let adapter_dns_servers = get_adapter_dns_servers(&*p_adapter);
            for sa in &adapter_dns_servers {
                if !ans.contains(sa) {
                    ans.push(*sa);
                }
            }
            p_adapter = (*p_adapter).Next;
        }
        ans
    };

    Ok(ans)
}

unsafe fn get_adapter_dns_servers(a: &IP_ADAPTER_ADDRESSES_LH) -> Vec<IpAddr> {
    let mut p_address = a.FirstDnsServerAddress;
    let mut ans = Vec::new();
    while !p_address.is_null() {
        let sock_addr = (*p_address).Address.lpSockaddr;
        if !sock_addr.is_null() {
            match ADDRESS_FAMILY::from((*sock_addr).sa_family as u32) {
                AF_INET => {
                    let p_sockaddr_in: *const SOCKADDR_IN = sock_addr.cast();
                    let ipv4 = Ipv4Addr::from(u32::from_be((*p_sockaddr_in).sin_addr.S_un.S_addr));
                    if !ipv4.is_unspecified() {
                        ans.push(ipv4.into());
                    }
                }
                AF_INET6 => {
                    let p_sockaddr_in6: *const SOCKADDR_IN6 = sock_addr.cast();
                    let ipv6 = Ipv6Addr::from((*p_sockaddr_in6).sin6_addr.u.Byte);
                    if !ipv6.is_unspecified() && !is_unicast_site_local(&ipv6) {
                        ans.push(ipv6.into());
                    }
                }
                _ => {}
            }
        }
        p_address = (*p_address).Next;
    }
    ans
}

#[inline]
fn is_unicast_site_local(ipv6: &Ipv6Addr) -> bool {
    // Ipv6Addr::is_unicast_site_local is available only in nightly build for now
    (ipv6.segments()[0] & 0xffc0) == 0xfec0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_dns_servers() {
        let nameservers = get_dns_servers().expect("failed to get dns servers");

        println!("found {} nameservers", nameservers.len());
        for ns in nameservers {
            println!("{}", ns);
        }
    }
}
