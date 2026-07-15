// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use super::{In6Addr, InAddr};
use crate::{ByteRepr, Value};
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
