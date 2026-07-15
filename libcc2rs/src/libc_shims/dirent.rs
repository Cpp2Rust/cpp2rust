// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::{ByteRepr, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Dirent {
    pub d_ino: Value<u64>,
    pub d_off: Value<i64>,
    pub d_reclen: Value<u16>,
    pub d_type: Value<u8>,
    pub d_name: Value<Box<[u8]>>,
}

impl Default for Dirent {
    fn default() -> Self {
        Self {
            d_ino: Rc::new(RefCell::new(0)),
            d_off: Rc::new(RefCell::new(0)),
            d_reclen: Rc::new(RefCell::new(0)),
            d_type: Rc::new(RefCell::new(0)),
            d_name: Rc::new(RefCell::new(vec![0u8; 256].into_boxed_slice())),
        }
    }
}

impl Clone for Dirent {
    fn clone(&self) -> Self {
        Self {
            d_ino: Rc::new(RefCell::new(*self.d_ino.borrow())),
            d_off: Rc::new(RefCell::new(*self.d_off.borrow())),
            d_reclen: Rc::new(RefCell::new(*self.d_reclen.borrow())),
            d_type: Rc::new(RefCell::new(*self.d_type.borrow())),
            d_name: Rc::new(RefCell::new(self.d_name.borrow().clone())),
        }
    }
}

impl ByteRepr for Dirent {}

pub struct CDir {
    pub entries: Vec<(u64, Vec<u8>, u8)>,
    pub pos: ::std::cell::Cell<usize>,
}

impl ByteRepr for CDir {}

impl ByteRepr for ::libc::dirent {}
