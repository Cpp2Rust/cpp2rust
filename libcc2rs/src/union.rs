// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::{cell::RefCell, rc::Rc};

use crate::Ptr;
use crate::reinterpret::{ByteRepr, OriginalAlloc, SliceOriginalAlloc};

pub struct UnionStorage {
    bytes: Rc<RefCell<Vec<u8>>>,
}

impl UnionStorage {
    pub fn new(size: usize) -> Self {
        UnionStorage {
            bytes: Rc::new(RefCell::new(vec![0u8; size])),
        }
    }

    pub fn reinterpret<T: ByteRepr>(&self, offset: usize) -> Ptr<T> {
        let alloc: Rc<dyn OriginalAlloc> = Rc::new(SliceOriginalAlloc {
            weak: Rc::downgrade(&self.bytes),
        });
        Ptr::reinterpreted(alloc, offset)
    }
}

impl Clone for UnionStorage {
    fn clone(&self) -> Self {
        UnionStorage {
            bytes: Rc::new(RefCell::new(self.bytes.borrow().clone())),
        }
    }
}
