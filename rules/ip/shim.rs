// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{AsPointer, ByteRepr, Ptr, Record, Value};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default, Record)]
pub struct InAddr {
    #[offset(0)]
    pub s_addr: u32,
}

#[derive(Record)]
pub struct In6Addr {
    #[offset(0)]
    pub s6_addr: Value<Box<[u8]>>,
}

impl In6Addr {
    pub fn s6_addr(&self) -> Ptr<u8> {
        self.s6_addr.as_pointer()
    }
}

impl Default for In6Addr {
    fn default() -> Self {
        Self {
            s6_addr: Rc::new(RefCell::new(vec![0u8; 16].into_boxed_slice())),
        }
    }
}

impl Clone for In6Addr {
    fn clone(&self) -> Self {
        Self {
            s6_addr: Rc::new(RefCell::new(self.s6_addr.borrow().clone())),
        }
    }
}

impl ByteRepr for InAddr {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.s_addr.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            s_addr: <u32>::from_bytes(&buf[0..4]),
        }
    }
}

impl ByteRepr for In6Addr {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        buf[0..16].copy_from_slice(&self.s6_addr.borrow());
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            s6_addr: Rc::new(RefCell::new(buf[0..16].to_vec().into_boxed_slice())),
        }
    }
}

impl ByteRepr for ::libc::in_addr {}
impl ByteRepr for ::libc::in6_addr {}
