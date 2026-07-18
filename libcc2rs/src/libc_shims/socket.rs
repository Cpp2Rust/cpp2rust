// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use super::{In6Addr, InAddr};
use crate::{ByteRepr, Ptr, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Sockaddr {
    pub sa_family: Value<u16>,
    pub sa_data: Value<Box<[u8]>>,
}

pub struct SockaddrIn {
    pub sin_family: Value<u16>,
    pub sin_port: Value<u16>,
    pub sin_addr: Value<InAddr>,
    pub sin_zero: Value<Box<[u8]>>,
}

#[derive(Default)]
pub struct SockaddrIn6 {
    pub sin6_family: Value<u16>,
    pub sin6_port: Value<u16>,
    pub sin6_flowinfo: Value<u32>,
    pub sin6_addr: Value<In6Addr>,
    pub sin6_scope_id: Value<u32>,
}

pub struct SockaddrUn {
    pub sun_family: Value<u16>,
    pub sun_path: Value<Box<[u8]>>,
}

pub struct SockaddrStorage {
    pub ss_family: Value<u16>,
    pub __pad: Value<Box<[u8]>>,
}

impl SockaddrIn {
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(l: &::libc::sockaddr_in) -> Self {
        Self {
            sin_family: Rc::new(RefCell::new(l.sin_family as u16)),
            sin_port: Rc::new(RefCell::new(l.sin_port)),
            sin_addr: Rc::new(RefCell::new(InAddr {
                s_addr: Rc::new(RefCell::new(l.sin_addr.s_addr)),
            })),
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
        let s = Self::default();
        *s.sin_family.borrow_mut() = ::libc::AF_INET as u16;
        *s.sin_port.borrow_mut() = port.to_be();
        *s.sin_addr.borrow().s_addr.borrow_mut() = u32::from(*addr).to_be();
        s
    }

    #[cfg(target_os = "linux")]
    pub fn to_libc(&self) -> ::libc::sockaddr_in {
        let mut sin_zero = [0u8; 8];
        sin_zero.copy_from_slice(&self.sin_zero.borrow());
        ::libc::sockaddr_in {
            sin_family: *self.sin_family.borrow(),
            sin_port: *self.sin_port.borrow(),
            sin_addr: ::libc::in_addr {
                s_addr: *self.sin_addr.borrow().s_addr.borrow(),
            },
            sin_zero,
        }
    }
}

impl SockaddrIn6 {
    #[allow(clippy::unnecessary_cast)]
    pub fn from_libc(l: &::libc::sockaddr_in6) -> Self {
        Self {
            sin6_family: Rc::new(RefCell::new(l.sin6_family as u16)),
            sin6_port: Rc::new(RefCell::new(l.sin6_port)),
            sin6_flowinfo: Rc::new(RefCell::new(l.sin6_flowinfo)),
            sin6_addr: Rc::new(RefCell::new(In6Addr {
                s6_addr: Rc::new(RefCell::new(
                    l.sin6_addr.s6_addr.to_vec().into_boxed_slice(),
                )),
            })),
            sin6_scope_id: Rc::new(RefCell::new(l.sin6_scope_id)),
        }
    }

    pub fn from_ipv6(addr: &::std::net::Ipv6Addr, port: u16) -> Self {
        let s = Self::default();
        *s.sin6_family.borrow_mut() = ::libc::AF_INET6 as u16;
        *s.sin6_port.borrow_mut() = port.to_be();
        s.sin6_addr
            .borrow()
            .s6_addr
            .borrow_mut()
            .copy_from_slice(&addr.octets());
        s
    }

    #[cfg(target_os = "linux")]
    pub fn to_libc(&self) -> ::libc::sockaddr_in6 {
        let mut s6_addr = [0u8; 16];
        s6_addr.copy_from_slice(&self.sin6_addr.borrow().s6_addr.borrow());
        ::libc::sockaddr_in6 {
            sin6_family: *self.sin6_family.borrow(),
            sin6_port: *self.sin6_port.borrow(),
            sin6_flowinfo: *self.sin6_flowinfo.borrow(),
            sin6_addr: ::libc::in6_addr { s6_addr },
            sin6_scope_id: *self.sin6_scope_id.borrow(),
        }
    }
}

impl Default for Sockaddr {
    fn default() -> Self {
        Self {
            sa_family: Rc::new(RefCell::new(0)),
            sa_data: Rc::new(RefCell::new(vec![0u8; 14].into_boxed_slice())),
        }
    }
}

impl Default for SockaddrIn {
    fn default() -> Self {
        Self {
            sin_family: Rc::new(RefCell::new(0)),
            sin_port: Rc::new(RefCell::new(0)),
            sin_addr: Rc::new(RefCell::new(InAddr::default())),
            sin_zero: Rc::new(RefCell::new(vec![0u8; 8].into_boxed_slice())),
        }
    }
}

impl Default for SockaddrUn {
    fn default() -> Self {
        Self {
            sun_family: Rc::new(RefCell::new(0)),
            sun_path: Rc::new(RefCell::new(vec![0u8; 108].into_boxed_slice())),
        }
    }
}

impl Default for SockaddrStorage {
    fn default() -> Self {
        Self {
            ss_family: Rc::new(RefCell::new(0)),
            __pad: Rc::new(RefCell::new(vec![0u8; 126].into_boxed_slice())),
        }
    }
}

impl Clone for Sockaddr {
    fn clone(&self) -> Self {
        Self {
            sa_family: Rc::new(RefCell::new(*self.sa_family.borrow())),
            sa_data: Rc::new(RefCell::new(self.sa_data.borrow().clone())),
        }
    }
}

impl Clone for SockaddrIn {
    fn clone(&self) -> Self {
        Self {
            sin_family: Rc::new(RefCell::new(*self.sin_family.borrow())),
            sin_port: Rc::new(RefCell::new(*self.sin_port.borrow())),
            sin_addr: Rc::new(RefCell::new(self.sin_addr.borrow().clone())),
            sin_zero: Rc::new(RefCell::new(self.sin_zero.borrow().clone())),
        }
    }
}

impl Clone for SockaddrIn6 {
    fn clone(&self) -> Self {
        Self {
            sin6_family: Rc::new(RefCell::new(*self.sin6_family.borrow())),
            sin6_port: Rc::new(RefCell::new(*self.sin6_port.borrow())),
            sin6_flowinfo: Rc::new(RefCell::new(*self.sin6_flowinfo.borrow())),
            sin6_addr: Rc::new(RefCell::new(self.sin6_addr.borrow().clone())),
            sin6_scope_id: Rc::new(RefCell::new(*self.sin6_scope_id.borrow())),
        }
    }
}

impl Clone for SockaddrUn {
    fn clone(&self) -> Self {
        Self {
            sun_family: Rc::new(RefCell::new(*self.sun_family.borrow())),
            sun_path: Rc::new(RefCell::new(self.sun_path.borrow().clone())),
        }
    }
}

impl Clone for SockaddrStorage {
    fn clone(&self) -> Self {
        Self {
            ss_family: Rc::new(RefCell::new(*self.ss_family.borrow())),
            __pad: Rc::new(RefCell::new(self.__pad.borrow().clone())),
        }
    }
}

impl ByteRepr for Sockaddr {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.sa_family.borrow()).to_bytes(&mut buf[0..2]);
        buf[2..16].copy_from_slice(&self.sa_data.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            sa_family: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            sa_data: Rc::new(RefCell::new(buf[2..16].to_vec().into_boxed_slice())),
        }
    }
}

impl ByteRepr for SockaddrIn {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.sin_family.borrow()).to_bytes(&mut buf[0..2]);
        (*self.sin_port.borrow()).to_bytes(&mut buf[2..4]);
        (*self.sin_addr.borrow()).to_bytes(&mut buf[4..8]);
        buf[8..16].copy_from_slice(&self.sin_zero.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            sin_family: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            sin_port: Rc::new(RefCell::new(<u16>::from_bytes(&buf[2..4]))),
            sin_addr: Rc::new(RefCell::new(<InAddr>::from_bytes(&buf[4..8]))),
            sin_zero: Rc::new(RefCell::new(buf[8..16].to_vec().into_boxed_slice())),
        }
    }
}

impl ByteRepr for SockaddrIn6 {
    fn byte_size() -> usize {
        28
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.sin6_family.borrow()).to_bytes(&mut buf[0..2]);
        (*self.sin6_port.borrow()).to_bytes(&mut buf[2..4]);
        (*self.sin6_flowinfo.borrow()).to_bytes(&mut buf[4..8]);
        (*self.sin6_addr.borrow()).to_bytes(&mut buf[8..24]);
        (*self.sin6_scope_id.borrow()).to_bytes(&mut buf[24..28]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            sin6_family: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            sin6_port: Rc::new(RefCell::new(<u16>::from_bytes(&buf[2..4]))),
            sin6_flowinfo: Rc::new(RefCell::new(<u32>::from_bytes(&buf[4..8]))),
            sin6_addr: Rc::new(RefCell::new(<In6Addr>::from_bytes(&buf[8..24]))),
            sin6_scope_id: Rc::new(RefCell::new(<u32>::from_bytes(&buf[24..28]))),
        }
    }
}

impl ByteRepr for SockaddrUn {
    fn byte_size() -> usize {
        110
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.sun_family.borrow()).to_bytes(&mut buf[0..2]);
        buf[2..110].copy_from_slice(&self.sun_path.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            sun_family: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            sun_path: Rc::new(RefCell::new(buf[2..110].to_vec().into_boxed_slice())),
        }
    }
}

impl ByteRepr for SockaddrStorage {
    fn byte_size() -> usize {
        128
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self.ss_family.borrow()).to_bytes(&mut buf[0..2]);
        buf[2..128].copy_from_slice(&self.__pad.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            ss_family: Rc::new(RefCell::new(<u16>::from_bytes(&buf[0..2]))),
            __pad: Rc::new(RefCell::new(buf[2..128].to_vec().into_boxed_slice())),
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
            nix::sys::socket::UnixAddr::new(&path[..end])
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
