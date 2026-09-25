// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{AsPointer, ByteRepr, Ptr, Value};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct InAddr {
    pub s_addr: Value<u32>,
}

pub struct In6Addr {
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

impl Clone for InAddr {
    fn clone(&self) -> Self {
        Self {
            s_addr: Rc::new(RefCell::new(*self.s_addr.borrow())),
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
        (*self.s_addr.borrow()).to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            s_addr: Rc::new(RefCell::new(<u32>::from_bytes(&buf[0..4]))),
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
