use anyhow::{bail, Result};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV6};

mod bindings {
    windows::include_bindings!();
}

use bindings::Windows::Win32::{
    Debug::WIN32_ERROR,
    IpHelper::*,
    WinSock::{SOCKADDR_IN, SOCKADDR_IN6},
};

pub fn get_dns_servers() -> Result<Vec<SocketAddr>> {
    let flags = GET_ADAPTERS_ADDRESSES_FLAGS::GAA_FLAG_INCLUDE_TUNNEL_BINDINGORDER;

    let ans = unsafe {
        let mut buf_size = 0;
        let error = GetAdaptersAddresses(
            ADDRESS_FAMILY::AF_UNSPEC,
            flags,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut buf_size,
        );

        match WIN32_ERROR::from(error) {
            WIN32_ERROR::ERROR_BUFFER_OVERFLOW => {}
            e => {
                bail!("GetAdaptersAddresses#1 failed: {:?}", e);
            }
        }

        let block_size = std::mem::size_of::<IP_ADAPTER_ADDRESSES_LH>() as u32;
        let new_capacity = buf_size + block_size;
        let mut buf = Vec::<u8>::with_capacity(new_capacity as usize);

        let (prefix, body, _) = buf.align_to_mut::<IP_ADAPTER_ADDRESSES_LH>();

        let mut buf_size = new_capacity - prefix.len() as u32;
        let error = GetAdaptersAddresses(
            ADDRESS_FAMILY::AF_UNSPEC,
            flags,
            std::ptr::null_mut(),
            body.as_mut_ptr(),
            &mut buf_size,
        );

        match WIN32_ERROR::from(error) {
            WIN32_ERROR::NO_ERROR => {}
            e => {
                bail!("GetAdaptersAddresses#2 failed: {:?}", e);
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

unsafe fn get_adapter_dns_servers(a: &IP_ADAPTER_ADDRESSES_LH) -> Vec<SocketAddr> {
    let mut p_address = a.FirstDnsServerAddress;
    let mut ans = Vec::new();
    while !p_address.is_null() {
        let sock_addr = (*p_address).Address.lpSockaddr;
        if !sock_addr.is_null() {
            match ADDRESS_FAMILY::from((*sock_addr).sa_family as u32) {
                ADDRESS_FAMILY::AF_INET => {
                    let p_sockaddr_in: *const SOCKADDR_IN = sock_addr.cast();
                    let ipv4 = Ipv4Addr::from(u32::from_be((*p_sockaddr_in).sin_addr.S_un.S_addr));
                    if !ipv4.is_unspecified() {
                        let mut port = u16::from_be((*p_sockaddr_in).sin_port);
                        if port == 0 {
                            port = 53;
                        }
                        ans.push(SocketAddr::from((ipv4, port)));
                    }
                }
                ADDRESS_FAMILY::AF_INET6 => {
                    let p_sockaddr_in6: *const SOCKADDR_IN6 = sock_addr.cast();
                    let ipv6 = Ipv6Addr::from((*p_sockaddr_in6).sin6_addr.u.Byte);
                    if !ipv6.is_unspecified() && !is_unicast_site_local(&ipv6) {
                        let mut port = u16::from_be((*p_sockaddr_in6).sin6_port);
                        if port == 0 {
                            port = 53;
                        }
                        let flowinfo = u32::from_be((*p_sockaddr_in6).sin6_flowinfo);
                        let scope_id = u32::from_be((*p_sockaddr_in6).Anonymous.sin6_scope_id);
                        ans.push(SocketAddr::from(SocketAddrV6::new(
                            ipv6, port, flowinfo, scope_id,
                        )));
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
