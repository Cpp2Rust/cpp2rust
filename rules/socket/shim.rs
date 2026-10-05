// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, CChar, DeepClone, In6Addr, InAddr, Ptr, Record, Value};
use std::cell::RefCell;
use std::rc::Rc;

// The fields are at the offsets of the byte representation below.
#[derive(DeepClone, Record, ByteRepr)]
#[byte_size(16)]
pub struct Sockaddr {
    #[offset(0)]
    pub sa_family: u16,
    #[offset(2)]
    #[byte_size(14)]
    pub sa_data: Value<Box<[i8]>>,
}

#[derive(DeepClone, Record, ByteRepr)]
#[byte_size(16)]
pub struct SockaddrIn {
    #[offset(0)]
    pub sin_family: u16,
    #[offset(2)]
    pub sin_port: u16,
    #[offset(4)]
    pub sin_addr: InAddr,
    #[offset(8)]
    #[byte_size(8)]
    pub sin_zero: Value<Box<[u8]>>,
}

#[derive(Clone, Default, Record, ByteRepr)]
#[byte_size(28)]
pub struct SockaddrIn6 {
    #[offset(0)]
    pub sin6_family: u16,
    #[offset(2)]
    pub sin6_port: u16,
    #[offset(4)]
    pub sin6_flowinfo: u32,
    #[offset(8)]
    pub sin6_addr: In6Addr,
    #[offset(24)]
    pub sin6_scope_id: u32,
}

#[derive(DeepClone, Record, ByteRepr)]
#[byte_size(110)]
pub struct SockaddrUn {
    #[offset(0)]
    pub sun_family: u16,
    #[offset(2)]
    #[byte_size(108)]
    pub sun_path: Value<Box<[i8]>>,
}

#[derive(DeepClone, Record, ByteRepr)]
#[byte_size(128)]
pub struct SockaddrStorage {
    #[offset(0)]
    pub ss_family: u16,
    #[offset(2)]
    #[byte_size(126)]
    pub __pad: Value<Box<[u8]>>,
}

impl SockaddrIn {
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(l: &::libc::sockaddr_in) -> Self {
        Self {
            sin_family: l.sin_family as u16,
            sin_port: l.sin_port,
            sin_addr: InAddr {
                s_addr: l.sin_addr.s_addr,
            },
            sin_zero: Rc::new(RefCell::new(
                l.sin_zero
                    .iter()
                    .map(|&b| b as u8)
                    .collect::<Vec<u8>>()
                    .into_boxed_slice(),
            )),
        }
    }

    pub fn from_ipv4(addr: &::std::net::Ipv4Addr, port: u16) -> Self {
        Self {
            sin_family: ::libc::AF_INET as u16,
            sin_port: port.to_be(),
            sin_addr: InAddr {
                s_addr: u32::from(*addr).to_be(),
            },
            ..Self::default()
        }
    }

    #[cfg(target_os = "linux")]
    pub fn to_libc(&self) -> ::libc::sockaddr_in {
        let mut sin_zero = [0u8; 8];
        sin_zero.copy_from_slice(&self.sin_zero.borrow());
        ::libc::sockaddr_in {
            sin_family: self.sin_family,
            sin_port: self.sin_port,
            sin_addr: ::libc::in_addr {
                s_addr: self.sin_addr.s_addr,
            },
            sin_zero,
        }
    }

    #[cfg(target_os = "macos")]
    pub fn to_libc(&self) -> ::libc::sockaddr_in {
        let mut sin_zero = [0i8; 8];
        for (dst, src) in sin_zero.iter_mut().zip(self.sin_zero.borrow().iter()) {
            *dst = *src as i8;
        }
        ::libc::sockaddr_in {
            sin_len: ::std::mem::size_of::<::libc::sockaddr_in>() as u8,
            sin_family: self.sin_family as u8,
            sin_port: self.sin_port,
            sin_addr: ::libc::in_addr {
                s_addr: self.sin_addr.s_addr,
            },
            sin_zero,
        }
    }
}

impl SockaddrIn6 {
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(l: &::libc::sockaddr_in6) -> Self {
        Self {
            sin6_family: l.sin6_family as u16,
            sin6_port: l.sin6_port,
            sin6_flowinfo: l.sin6_flowinfo,
            sin6_addr: In6Addr {
                s6_addr: Rc::new(RefCell::new(
                    l.sin6_addr.s6_addr.to_vec().into_boxed_slice(),
                )),
            },
            sin6_scope_id: l.sin6_scope_id,
        }
    }

    pub fn from_ipv6(addr: &::std::net::Ipv6Addr, port: u16) -> Self {
        let s = Self {
            sin6_family: ::libc::AF_INET6 as u16,
            sin6_port: port.to_be(),
            ..Self::default()
        };
        s.sin6_addr
            .s6_addr
            .borrow_mut()
            .copy_from_slice(&addr.octets());
        s
    }

    #[cfg(target_os = "linux")]
    pub fn to_libc(&self) -> ::libc::sockaddr_in6 {
        let mut s6_addr = [0u8; 16];
        s6_addr.copy_from_slice(&self.sin6_addr.s6_addr.borrow());
        ::libc::sockaddr_in6 {
            sin6_family: self.sin6_family,
            sin6_port: self.sin6_port,
            sin6_flowinfo: self.sin6_flowinfo,
            sin6_addr: ::libc::in6_addr { s6_addr },
            sin6_scope_id: self.sin6_scope_id,
        }
    }

    #[cfg(target_os = "macos")]
    pub fn to_libc(&self) -> ::libc::sockaddr_in6 {
        let mut s6_addr = [0u8; 16];
        s6_addr.copy_from_slice(&self.sin6_addr.s6_addr.borrow());
        ::libc::sockaddr_in6 {
            sin6_len: ::std::mem::size_of::<::libc::sockaddr_in6>() as u8,
            sin6_family: self.sin6_family as u8,
            sin6_port: self.sin6_port,
            sin6_flowinfo: self.sin6_flowinfo,
            sin6_addr: ::libc::in6_addr { s6_addr },
            sin6_scope_id: self.sin6_scope_id,
        }
    }
}

impl Default for Sockaddr {
    fn default() -> Self {
        Self {
            sa_family: 0,
            sa_data: Rc::new(RefCell::new(vec![0i8; 14].into_boxed_slice())),
        }
    }
}

impl Default for SockaddrIn {
    fn default() -> Self {
        Self {
            sin_family: 0,
            sin_port: 0,
            sin_addr: InAddr::default(),
            sin_zero: Rc::new(RefCell::new(vec![0u8; 8].into_boxed_slice())),
        }
    }
}

impl Default for SockaddrUn {
    fn default() -> Self {
        Self {
            sun_family: 0,
            sun_path: Rc::new(RefCell::new(vec![0i8; 108].into_boxed_slice())),
        }
    }
}

impl Default for SockaddrStorage {
    fn default() -> Self {
        Self {
            ss_family: 0,
            __pad: Rc::new(RefCell::new(vec![0u8; 126].into_boxed_slice())),
        }
    }
}

impl ByteRepr for ::libc::sockaddr {}
impl ByteRepr for ::libc::sockaddr_in {}
impl ByteRepr for ::libc::sockaddr_in6 {}
impl ByteRepr for ::libc::sockaddr_un {}
impl ByteRepr for ::libc::sockaddr_storage {}

impl Sockaddr {
    pub fn decode(
        addr: &Ptr<Sockaddr>,
        _len: u32,
    ) -> Option<Box<dyn nix::sys::socket::SockaddrLike>> {
        let family = addr.reinterpret_cast::<u16>().read();
        if family == ::libc::AF_INET as u16 {
            let m = addr.reinterpret_cast::<SockaddrIn>().read();
            Some(Box::new(nix::sys::socket::SockaddrIn::from(m.to_libc())))
        } else if family == ::libc::AF_INET6 as u16 {
            let m = addr.reinterpret_cast::<SockaddrIn6>().read();
            Some(Box::new(nix::sys::socket::SockaddrIn6::from(m.to_libc())))
        } else if family == ::libc::AF_UNIX as u16 {
            let m = addr.reinterpret_cast::<SockaddrUn>().read();
            let path = m.sun_path.borrow();
            let end = path.iter().position(|&c| c == 0).unwrap_or(path.len());
            CChar::with_u8_slice(&path[..end], nix::sys::socket::UnixAddr::new)
                .ok()
                .map(|u| Box::new(u) as Box<dyn nix::sys::socket::SockaddrLike>)
        } else {
            None
        }
    }

    pub fn encode(ss: &nix::sys::socket::SockaddrStorage, out: &Ptr<Sockaddr>, out_len: &Ptr<u32>) {
        use nix::sys::socket::{AddressFamily, SockaddrLike};
        match ss.family() {
            Some(AddressFamily::Inet) => {
                let l = ::libc::sockaddr_in::from(*ss.as_sockaddr_in().unwrap());
                out.reinterpret_cast::<SockaddrIn>()
                    .write(SockaddrIn::from_libc(&l));
            }
            Some(AddressFamily::Inet6) => {
                let l = ::libc::sockaddr_in6::from(*ss.as_sockaddr_in6().unwrap());
                out.reinterpret_cast::<SockaddrIn6>()
                    .write(SockaddrIn6::from_libc(&l));
            }
            _ => {}
        }
        out_len.write(ss.len());
    }
}

pub fn setsockopt_refcount(fd: i32, level: i32, optname: i32, optval: crate::AnyPtr) -> i32 {
    let res = match (level, optname) {
        (::libc::IPPROTO_TCP, ::libc::TCP_NODELAY) => {
            let v = optval.reinterpret_cast::<i32>().read() != 0;
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::TcpNoDelay, &v)
            })
        }
        (::libc::SOL_SOCKET, ::libc::SO_KEEPALIVE) => {
            let v = optval.reinterpret_cast::<i32>().read() != 0;
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::KeepAlive, &v)
            })
        }
        (::libc::IPPROTO_TCP, ::libc::TCP_KEEPINTVL) => {
            let v = optval.reinterpret_cast::<u32>().read();
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(
                    &borrowed,
                    nix::sys::socket::sockopt::TcpKeepInterval,
                    &v,
                )
            })
        }
        (::libc::IPPROTO_TCP, ::libc::TCP_KEEPCNT) => {
            let v = optval.reinterpret_cast::<u32>().read();
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::TcpKeepCount, &v)
            })
        }
        #[cfg(target_os = "linux")]
        (::libc::IPPROTO_IP, ::libc::IP_TOS) => {
            let v = optval.reinterpret_cast::<i32>().read();
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::Ipv4Tos, &v)
            })
        }
        #[cfg(target_os = "linux")]
        (::libc::IPPROTO_IPV6, ::libc::IPV6_TCLASS) => {
            let v = optval.reinterpret_cast::<i32>().read();
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::Ipv6TClass, &v)
            })
        }
        #[cfg(target_os = "linux")]
        (::libc::IPPROTO_TCP, ::libc::TCP_KEEPIDLE) => {
            let v = optval.reinterpret_cast::<u32>().read();
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::TcpKeepIdle, &v)
            })
        }
        #[cfg(target_os = "linux")]
        (::libc::SOL_SOCKET, ::libc::SO_BINDTODEVICE) => {
            let v = ::std::ffi::OsString::from(optval.reinterpret_cast::<u8>().to_rust_string());
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::BindToDevice, &v)
            })
        }
        #[cfg(target_os = "linux")]
        (::libc::IPPROTO_IP, ::libc::IP_BIND_ADDRESS_NO_PORT) => {
            let v = optval.reinterpret_cast::<i32>().read() != 0;
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(
                    &borrowed,
                    nix::sys::socket::sockopt::IpBindAddressNoPort,
                    &v,
                )
            })
        }
        #[cfg(target_os = "linux")]
        (::libc::IPPROTO_TCP, ::libc::TCP_FASTOPEN_CONNECT) => {
            let v = optval.reinterpret_cast::<i32>().read() != 0;
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(
                    &borrowed,
                    nix::sys::socket::sockopt::TcpFastOpenConnect,
                    &v,
                )
            })
        }
        #[cfg(target_os = "linux")]
        (::libc::SOL_SOCKET, ::libc::SO_PRIORITY) => {
            let v = optval.reinterpret_cast::<i32>().read();
            crate::FdRegistry::with_fd(fd, |borrowed| {
                nix::sys::socket::setsockopt(&borrowed, nix::sys::socket::sockopt::Priority, &v)
            })
        }
        (l, o) => panic!(
            "setsockopt: unsupported option (level={}, optname={})",
            l, o
        ),
    };
    match res {
        Ok(()) => 0,
        Err(e) => {
            crate::cpp2rust_errno().write(e as i32);
            -1
        }
    }
}
