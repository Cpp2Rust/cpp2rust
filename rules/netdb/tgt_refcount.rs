// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn t1() -> libcc2rs::Addrinfo {
    Default::default()
}

fn f1(a0: Ptr<i8>, a1: Ptr<i8>, a2: Ptr<Addrinfo>, a3: Ptr<Ptr<Addrinfo>>) -> i32 {
    let __node = a0;
    let __service = a1;
    let __hints = a2;
    let __out = a3;
    let __family = if __hints.is_null() {
        ::libc::AF_UNSPEC
    } else {
        __hints.with(|__h| __h.ai_family)
    };
    let __socktype = if __hints.is_null() {
        0
    } else {
        __hints.with(|__h| __h.ai_socktype)
    };
    let __protocol = if __hints.is_null() {
        0
    } else {
        __hints.with(|__h| __h.ai_protocol)
    };
    let __port: u16 = if __service.is_null() {
        0
    } else {
        __service.to_rust_string().parse().unwrap_or(0)
    };
    let mut __addrs: Vec<::std::net::IpAddr> = Vec::new();
    if __node.is_null() {
        if __family == ::libc::AF_INET6 {
            __addrs.push(::std::net::IpAddr::V6(::std::net::Ipv6Addr::UNSPECIFIED));
        } else {
            __addrs.push(::std::net::IpAddr::V4(::std::net::Ipv4Addr::UNSPECIFIED));
        }
    } else {
        let __host = __node.to_rust_string();
        match __host.parse::<::std::net::IpAddr>() {
            Ok(__ip) => __addrs.push(__ip),
            Err(_) => {
                use ::std::net::ToSocketAddrs;
                match (__host.as_str(), __port).to_socket_addrs() {
                    Ok(__it) => {
                        for __sa in __it {
                            __addrs.push(__sa.ip());
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }
    __addrs.retain(|__ip| match __family {
        ::libc::AF_INET => __ip.is_ipv4(),
        ::libc::AF_INET6 => __ip.is_ipv6(),
        _ => true,
    });
    if __addrs.is_empty() {
        ::libc::EAI_NONAME
    } else {
        let mut __next = Ptr::<Addrinfo>::null();
        for __ip in __addrs.iter().rev() {
            let mut __ai = Addrinfo::default();
            __ai.ai_socktype = __socktype;
            __ai.ai_protocol = __protocol;
            let __storage = Ptr::alloc(SockaddrStorage::default());
            match __ip {
                ::std::net::IpAddr::V4(__v4) => {
                    __ai.ai_family = ::libc::AF_INET;
                    __ai.ai_addrlen = ::std::mem::size_of::<::libc::sockaddr_in>() as u32;
                    __storage
                        .reinterpret_cast::<SockaddrIn>()
                        .write(SockaddrIn::from_ipv4(__v4, __port));
                }
                ::std::net::IpAddr::V6(__v6) => {
                    __ai.ai_family = ::libc::AF_INET6;
                    __ai.ai_addrlen = ::std::mem::size_of::<::libc::sockaddr_in6>() as u32;
                    __storage
                        .reinterpret_cast::<SockaddrIn6>()
                        .write(SockaddrIn6::from_ipv6(__v6, __port));
                }
            }
            __ai.ai_addr = __storage.reinterpret_cast::<Sockaddr>();
            __ai.ai_next = __next.clone();
            __next = Ptr::alloc(__ai);
        }
        __out.write(__next);
        0
    }
}

fn f2(a0: Ptr<Addrinfo>) {
    let mut __cur = a0;
    while !__cur.is_null() {
        let __next = __cur.with(|__ai| {
            if !__ai.ai_addr.is_null() {
                __ai.ai_addr.delete();
            }
            __ai.ai_next.clone()
        });
        __cur.delete();
        __cur = __next;
    }
}
