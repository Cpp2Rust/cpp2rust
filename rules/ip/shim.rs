// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{AsPointer, ByteRepr, DeepClone, Ptr, Record, Value};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default, Record, ByteRepr)]
#[byte_size(4)]
pub struct InAddr {
    #[offset(0)]
    pub s_addr: u32,
}

#[derive(DeepClone, Record, ByteRepr)]
#[byte_size(16)]
pub struct In6Addr {
    #[offset(0)]
    #[byte_size(16)]
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

impl ByteRepr for ::libc::in_addr {}
impl ByteRepr for ::libc::in6_addr {}
