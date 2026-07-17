extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_ipv4_literal_0() {
    let hints: Value<libcc2rs::Addrinfo> = Rc::new(RefCell::new(libcc2rs::Addrinfo {
        ai_flags: Rc::new(RefCell::new(0)),
        ai_family: Rc::new(RefCell::new(<i32>::default())),
        ai_socktype: Rc::new(RefCell::new(<i32>::default())),
        ai_protocol: Rc::new(RefCell::new(<i32>::default())),
        ai_addrlen: Rc::new(RefCell::new(<u32>::default())),
        ai_addr: Rc::new(RefCell::new(Ptr::<libcc2rs::Sockaddr>::null())),
        ai_canonname: Rc::new(RefCell::new(Ptr::<u8>::null())),
        ai_next: Rc::new(RefCell::new(Ptr::<libcc2rs::Addrinfo>::null())),
    }));
    (*(*hints.borrow()).ai_family.borrow_mut()) = 2;
    (*(*hints.borrow()).ai_socktype.borrow_mut()) = libc::SOCK_STREAM;
    let res: Value<Ptr<libcc2rs::Addrinfo>> =
        Rc::new(RefCell::new(Ptr::<libcc2rs::Addrinfo>::null()));
    assert!(
        ((({
            let __node = Ptr::from_string_literal(b"127.0.0.1").clone();
            let __service = Ptr::from_string_literal(b"8080").clone();
            let __hints = (hints.as_pointer()).clone();
            let __out = (res.as_pointer()).clone();
            let __family = if __hints.is_null() {
                ::libc::AF_UNSPEC
            } else {
                __hints.with(|__h| *__h.ai_family.borrow())
            };
            let __socktype = if __hints.is_null() {
                0
            } else {
                __hints.with(|__h| *__h.ai_socktype.borrow())
            };
            let __protocol = if __hints.is_null() {
                0
            } else {
                __hints.with(|__h| *__h.ai_protocol.borrow())
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
                    let __ai = Addrinfo::default();
                    *__ai.ai_socktype.borrow_mut() = __socktype;
                    *__ai.ai_protocol.borrow_mut() = __protocol;
                    let __storage = Ptr::alloc(SockaddrStorage::default());
                    match __ip {
                        ::std::net::IpAddr::V4(__v4) => {
                            *__ai.ai_family.borrow_mut() = ::libc::AF_INET;
                            *__ai.ai_addrlen.borrow_mut() =
                                ::std::mem::size_of::<::libc::sockaddr_in>() as u32;
                            __storage
                                .reinterpret_cast::<SockaddrIn>()
                                .write(SockaddrIn::from_ipv4(__v4, __port));
                        }
                        ::std::net::IpAddr::V6(__v6) => {
                            *__ai.ai_family.borrow_mut() = ::libc::AF_INET6;
                            *__ai.ai_addrlen.borrow_mut() =
                                ::std::mem::size_of::<::libc::sockaddr_in6>() as u32;
                            __storage
                                .reinterpret_cast::<SockaddrIn6>()
                                .write(SockaddrIn6::from_ipv6(__v6, __port));
                        }
                    }
                    *__ai.ai_addr.borrow_mut() = __storage.reinterpret_cast::<Sockaddr>();
                    *__ai.ai_next.borrow_mut() = __next.clone();
                    __next = Ptr::alloc(__ai);
                }
                __out.write(__next);
                0
            }
        } == 0) as i32)
            != 0)
    );
    assert!((((!((*res.borrow()).is_null())) as i32) != 0));
    assert!(((((*(*(*res.borrow()).upgrade().deref()).ai_family.borrow()) == 2) as i32) != 0));
    assert!(
        ((({
            let _lhs = (*(*(*res.borrow()).upgrade().deref()).ai_socktype.borrow());
            _lhs == libc::SOCK_STREAM
        }) as i32)
            != 0)
    );
    assert!(
        (((((*(*(*res.borrow()).upgrade().deref()).ai_addrlen.borrow()) as usize) == 16usize)
            as i32)
            != 0)
    );
    assert!(
        (((!((*(*(*res.borrow()).upgrade().deref()).ai_addr.borrow()).is_null())) as i32) != 0)
    );
    let sin: Value<Ptr<libcc2rs::SockaddrIn>> = Rc::new(RefCell::new(
        (*(*(*res.borrow()).upgrade().deref()).ai_addr.borrow())
            .reinterpret_cast::<libcc2rs::SockaddrIn>(),
    ));
    assert!(
        (((((*(*(*sin.borrow()).upgrade().deref()).sin_family.borrow()) as i32) == 2) as i32) != 0)
    );
    let port_be: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ((8080 / 256) as u8),
        ((8080 % 256) as u8),
    ])));
    assert!(
        ((((((*(*sin.borrow()).upgrade().deref()).sin_port.as_pointer()) as Ptr::<u16>)
            .to_any()
            .memcmp(
                &((port_be.as_pointer() as Ptr::<u8>) as Ptr::<u8>).to_any(),
                2_usize
            )
            == 0) as i32)
            != 0)
    );
    let addr_be: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([127_u8, 0_u8, 0_u8, 1_u8])));
    assert!(
        ((((((*(*sin.borrow()).upgrade().deref()).sin_addr.as_pointer()) as Ptr<libcc2rs::InAddr>)
            .to_any()
            .memcmp(
                &((addr_be.as_pointer() as Ptr::<u8>) as Ptr::<u8>).to_any(),
                4_usize
            )
            == 0) as i32)
            != 0)
    );
    {
        let mut __cur = (*res.borrow()).clone();
        while !__cur.is_null() {
            let __next = __cur.with(|__ai| {
                let __addr = __ai.ai_addr.borrow();
                if !__addr.is_null() {
                    __addr.delete();
                }
                (*__ai.ai_next.borrow()).clone()
            });
            __cur.delete();
            __cur = __next;
        }
    };
}
pub fn test_ipv6_literal_1() {
    let hints: Value<libcc2rs::Addrinfo> = Rc::new(RefCell::new(libcc2rs::Addrinfo {
        ai_flags: Rc::new(RefCell::new(0)),
        ai_family: Rc::new(RefCell::new(<i32>::default())),
        ai_socktype: Rc::new(RefCell::new(<i32>::default())),
        ai_protocol: Rc::new(RefCell::new(<i32>::default())),
        ai_addrlen: Rc::new(RefCell::new(<u32>::default())),
        ai_addr: Rc::new(RefCell::new(Ptr::<libcc2rs::Sockaddr>::null())),
        ai_canonname: Rc::new(RefCell::new(Ptr::<u8>::null())),
        ai_next: Rc::new(RefCell::new(Ptr::<libcc2rs::Addrinfo>::null())),
    }));
    (*(*hints.borrow()).ai_family.borrow_mut()) = 10;
    (*(*hints.borrow()).ai_socktype.borrow_mut()) = libc::SOCK_STREAM;
    let res: Value<Ptr<libcc2rs::Addrinfo>> =
        Rc::new(RefCell::new(Ptr::<libcc2rs::Addrinfo>::null()));
    assert!(
        ((({
            let __node = Ptr::from_string_literal(b"::1").clone();
            let __service = Ptr::from_string_literal(b"443").clone();
            let __hints = (hints.as_pointer()).clone();
            let __out = (res.as_pointer()).clone();
            let __family = if __hints.is_null() {
                ::libc::AF_UNSPEC
            } else {
                __hints.with(|__h| *__h.ai_family.borrow())
            };
            let __socktype = if __hints.is_null() {
                0
            } else {
                __hints.with(|__h| *__h.ai_socktype.borrow())
            };
            let __protocol = if __hints.is_null() {
                0
            } else {
                __hints.with(|__h| *__h.ai_protocol.borrow())
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
                    let __ai = Addrinfo::default();
                    *__ai.ai_socktype.borrow_mut() = __socktype;
                    *__ai.ai_protocol.borrow_mut() = __protocol;
                    let __storage = Ptr::alloc(SockaddrStorage::default());
                    match __ip {
                        ::std::net::IpAddr::V4(__v4) => {
                            *__ai.ai_family.borrow_mut() = ::libc::AF_INET;
                            *__ai.ai_addrlen.borrow_mut() =
                                ::std::mem::size_of::<::libc::sockaddr_in>() as u32;
                            __storage
                                .reinterpret_cast::<SockaddrIn>()
                                .write(SockaddrIn::from_ipv4(__v4, __port));
                        }
                        ::std::net::IpAddr::V6(__v6) => {
                            *__ai.ai_family.borrow_mut() = ::libc::AF_INET6;
                            *__ai.ai_addrlen.borrow_mut() =
                                ::std::mem::size_of::<::libc::sockaddr_in6>() as u32;
                            __storage
                                .reinterpret_cast::<SockaddrIn6>()
                                .write(SockaddrIn6::from_ipv6(__v6, __port));
                        }
                    }
                    *__ai.ai_addr.borrow_mut() = __storage.reinterpret_cast::<Sockaddr>();
                    *__ai.ai_next.borrow_mut() = __next.clone();
                    __next = Ptr::alloc(__ai);
                }
                __out.write(__next);
                0
            }
        } == 0) as i32)
            != 0)
    );
    assert!((((!((*res.borrow()).is_null())) as i32) != 0));
    assert!(((((*(*(*res.borrow()).upgrade().deref()).ai_family.borrow()) == 10) as i32) != 0));
    assert!(
        (((((*(*(*res.borrow()).upgrade().deref()).ai_addrlen.borrow()) as usize) == 28usize)
            as i32)
            != 0)
    );
    assert!(
        (((!((*(*(*res.borrow()).upgrade().deref()).ai_addr.borrow()).is_null())) as i32) != 0)
    );
    let sin6: Value<Ptr<libcc2rs::SockaddrIn6>> = Rc::new(RefCell::new(
        (*(*(*res.borrow()).upgrade().deref()).ai_addr.borrow())
            .reinterpret_cast::<libcc2rs::SockaddrIn6>(),
    ));
    assert!(
        (((((*(*(*sin6.borrow()).upgrade().deref()).sin6_family.borrow()) as i32) == 10) as i32)
            != 0)
    );
    let port_be: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        ((443 / 256) as u8),
        ((443 % 256) as u8),
    ])));
    assert!(
        ((((((*(*sin6.borrow()).upgrade().deref()).sin6_port.as_pointer()) as Ptr::<u16>)
            .to_any()
            .memcmp(
                &((port_be.as_pointer() as Ptr::<u8>) as Ptr::<u8>).to_any(),
                2_usize
            )
            == 0) as i32)
            != 0)
    );
    let addr_be: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8,
        1_u8,
    ])));
    assert!(
        ((((((*(*sin6.borrow()).upgrade().deref()).sin6_addr.as_pointer())
            as Ptr<libcc2rs::In6Addr>)
            .to_any()
            .memcmp(
                &((addr_be.as_pointer() as Ptr::<u8>) as Ptr::<u8>).to_any(),
                16_usize
            )
            == 0) as i32)
            != 0)
    );
    {
        let mut __cur = (*res.borrow()).clone();
        while !__cur.is_null() {
            let __next = __cur.with(|__ai| {
                let __addr = __ai.ai_addr.borrow();
                if !__addr.is_null() {
                    __addr.delete();
                }
                (*__ai.ai_next.borrow()).clone()
            });
            __cur.delete();
            __cur = __next;
        }
    };
}
pub fn test_family_mismatch_2() {
    let hints: Value<libcc2rs::Addrinfo> = Rc::new(RefCell::new(libcc2rs::Addrinfo {
        ai_flags: Rc::new(RefCell::new(0)),
        ai_family: Rc::new(RefCell::new(<i32>::default())),
        ai_socktype: Rc::new(RefCell::new(<i32>::default())),
        ai_protocol: Rc::new(RefCell::new(<i32>::default())),
        ai_addrlen: Rc::new(RefCell::new(<u32>::default())),
        ai_addr: Rc::new(RefCell::new(Ptr::<libcc2rs::Sockaddr>::null())),
        ai_canonname: Rc::new(RefCell::new(Ptr::<u8>::null())),
        ai_next: Rc::new(RefCell::new(Ptr::<libcc2rs::Addrinfo>::null())),
    }));
    (*(*hints.borrow()).ai_family.borrow_mut()) = 10;
    (*(*hints.borrow()).ai_socktype.borrow_mut()) = libc::SOCK_STREAM;
    let res: Value<Ptr<libcc2rs::Addrinfo>> =
        Rc::new(RefCell::new(Ptr::<libcc2rs::Addrinfo>::null()));
    assert!(
        ((({
            let __node = Ptr::from_string_literal(b"127.0.0.1").clone();
            let __service = Ptr::from_string_literal(b"80").clone();
            let __hints = (hints.as_pointer()).clone();
            let __out = (res.as_pointer()).clone();
            let __family = if __hints.is_null() {
                ::libc::AF_UNSPEC
            } else {
                __hints.with(|__h| *__h.ai_family.borrow())
            };
            let __socktype = if __hints.is_null() {
                0
            } else {
                __hints.with(|__h| *__h.ai_socktype.borrow())
            };
            let __protocol = if __hints.is_null() {
                0
            } else {
                __hints.with(|__h| *__h.ai_protocol.borrow())
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
                    let __ai = Addrinfo::default();
                    *__ai.ai_socktype.borrow_mut() = __socktype;
                    *__ai.ai_protocol.borrow_mut() = __protocol;
                    let __storage = Ptr::alloc(SockaddrStorage::default());
                    match __ip {
                        ::std::net::IpAddr::V4(__v4) => {
                            *__ai.ai_family.borrow_mut() = ::libc::AF_INET;
                            *__ai.ai_addrlen.borrow_mut() =
                                ::std::mem::size_of::<::libc::sockaddr_in>() as u32;
                            __storage
                                .reinterpret_cast::<SockaddrIn>()
                                .write(SockaddrIn::from_ipv4(__v4, __port));
                        }
                        ::std::net::IpAddr::V6(__v6) => {
                            *__ai.ai_family.borrow_mut() = ::libc::AF_INET6;
                            *__ai.ai_addrlen.borrow_mut() =
                                ::std::mem::size_of::<::libc::sockaddr_in6>() as u32;
                            __storage
                                .reinterpret_cast::<SockaddrIn6>()
                                .write(SockaddrIn6::from_ipv6(__v6, __port));
                        }
                    }
                    *__ai.ai_addr.borrow_mut() = __storage.reinterpret_cast::<Sockaddr>();
                    *__ai.ai_next.borrow_mut() = __next.clone();
                    __next = Ptr::alloc(__ai);
                }
                __out.write(__next);
                0
            }
        } != 0) as i32)
            != 0)
    );
}
pub fn test_null_hints_3() {
    let res: Value<Ptr<libcc2rs::Addrinfo>> =
        Rc::new(RefCell::new(Ptr::<libcc2rs::Addrinfo>::null()));
    assert!(
        ((({
            let __node = Ptr::from_string_literal(b"127.0.0.1").clone();
            let __service = Ptr::from_string_literal(b"80").clone();
            let __hints = Ptr::<libcc2rs::Addrinfo>::null().clone();
            let __out = (res.as_pointer()).clone();
            let __family = if __hints.is_null() {
                ::libc::AF_UNSPEC
            } else {
                __hints.with(|__h| *__h.ai_family.borrow())
            };
            let __socktype = if __hints.is_null() {
                0
            } else {
                __hints.with(|__h| *__h.ai_socktype.borrow())
            };
            let __protocol = if __hints.is_null() {
                0
            } else {
                __hints.with(|__h| *__h.ai_protocol.borrow())
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
                    let __ai = Addrinfo::default();
                    *__ai.ai_socktype.borrow_mut() = __socktype;
                    *__ai.ai_protocol.borrow_mut() = __protocol;
                    let __storage = Ptr::alloc(SockaddrStorage::default());
                    match __ip {
                        ::std::net::IpAddr::V4(__v4) => {
                            *__ai.ai_family.borrow_mut() = ::libc::AF_INET;
                            *__ai.ai_addrlen.borrow_mut() =
                                ::std::mem::size_of::<::libc::sockaddr_in>() as u32;
                            __storage
                                .reinterpret_cast::<SockaddrIn>()
                                .write(SockaddrIn::from_ipv4(__v4, __port));
                        }
                        ::std::net::IpAddr::V6(__v6) => {
                            *__ai.ai_family.borrow_mut() = ::libc::AF_INET6;
                            *__ai.ai_addrlen.borrow_mut() =
                                ::std::mem::size_of::<::libc::sockaddr_in6>() as u32;
                            __storage
                                .reinterpret_cast::<SockaddrIn6>()
                                .write(SockaddrIn6::from_ipv6(__v6, __port));
                        }
                    }
                    *__ai.ai_addr.borrow_mut() = __storage.reinterpret_cast::<Sockaddr>();
                    *__ai.ai_next.borrow_mut() = __next.clone();
                    __next = Ptr::alloc(__ai);
                }
                __out.write(__next);
                0
            }
        } == 0) as i32)
            != 0)
    );
    assert!((((!((*res.borrow()).is_null())) as i32) != 0));
    assert!(((((*(*(*res.borrow()).upgrade().deref()).ai_family.borrow()) == 2) as i32) != 0));
    let sin: Value<Ptr<libcc2rs::SockaddrIn>> = Rc::new(RefCell::new(
        (*(*(*res.borrow()).upgrade().deref()).ai_addr.borrow())
            .reinterpret_cast::<libcc2rs::SockaddrIn>(),
    ));
    let addr_be: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([127_u8, 0_u8, 0_u8, 1_u8])));
    assert!(
        ((((((*(*sin.borrow()).upgrade().deref()).sin_addr.as_pointer()) as Ptr<libcc2rs::InAddr>)
            .to_any()
            .memcmp(
                &((addr_be.as_pointer() as Ptr::<u8>) as Ptr::<u8>).to_any(),
                4_usize
            )
            == 0) as i32)
            != 0)
    );
    {
        let mut __cur = (*res.borrow()).clone();
        while !__cur.is_null() {
            let __next = __cur.with(|__ai| {
                let __addr = __ai.ai_addr.borrow();
                if !__addr.is_null() {
                    __addr.delete();
                }
                (*__ai.ai_next.borrow()).clone()
            });
            __cur.delete();
            __cur = __next;
        }
    };
}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test_ipv4_literal_0() });
    ({ test_ipv6_literal_1() });
    ({ test_family_mismatch_2() });
    ({ test_null_hints_3() });
    return 0;
}
