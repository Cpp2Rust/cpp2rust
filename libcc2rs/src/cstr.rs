// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::rc::{Ptr, PtrKind};

impl fmt::Display for Ptr<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            PtrKind::Null => write!(f, "NULL"),
            _ => {
                for value in self {
                    let ch = value.read();
                    if ch == 0 {
                        break;
                    }
                    write!(f, "{}", char::from(ch))?;
                }
                Ok(())
            }
        }
    }
}

type StringLiteralMap = HashMap<&'static [u8], Rc<RefCell<Vec<u8>>>>;

thread_local! {
    static STRING_LITERALS: RefCell<StringLiteralMap> = RefCell::new(HashMap::new());
}

impl Ptr<u8> {
    pub fn with_slice_mut<R>(&self, len: usize, f: impl FnOnce(&mut [u8]) -> R) -> R {
        let off = self.offset;
        match &self.kind {
            PtrKind::Null => panic!("ub: null pointer"),
            PtrKind::StackSingle(weak) | PtrKind::HeapSingle(weak) => {
                assert!(off == 0 && len <= 1, "ub: with_slice_mut out of bounds");
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let mut b = rc.borrow_mut();
                f(&mut std::slice::from_mut(&mut *b)[..len])
            }
            PtrKind::StackArray(weak) | PtrKind::HeapArray(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let mut b = rc.borrow_mut();
                f(&mut b[off..off + len])
            }
            PtrKind::Vec(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let mut b = rc.borrow_mut();
                f(&mut b[off..off + len])
            }
            PtrKind::Reinterpreted(data) => {
                let mut buf = vec![0u8; len];
                data.alloc.read_bytes(off, &mut buf);
                let r = f(&mut buf);
                data.alloc.write_bytes(off, &buf);
                r
            }
        }
    }

    pub fn with_slice<R>(&self, len: usize, f: impl FnOnce(&[u8]) -> R) -> R {
        let off = self.offset;
        match &self.kind {
            PtrKind::Null => panic!("ub: null pointer"),
            PtrKind::StackSingle(weak) | PtrKind::HeapSingle(weak) => {
                assert!(off == 0 && len <= 1, "ub: with_slice out of bounds");
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let b = rc.borrow();
                f(&std::slice::from_ref(&*b)[..len])
            }
            PtrKind::StackArray(weak) | PtrKind::HeapArray(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let b = rc.borrow();
                f(&b[off..off + len])
            }
            PtrKind::Vec(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let b = rc.borrow();
                f(&b[off..off + len])
            }
            PtrKind::Reinterpreted(data) => {
                let mut buf = vec![0u8; len];
                data.alloc.read_bytes(off, &mut buf);
                f(&buf)
            }
        }
    }

    #[allow(clippy::explicit_counter_loop)]
    pub fn memcpy(&self, src: &Self, len: usize) {
        if *self > *src {
            let mut dst = self.offset(len);
            let mut s = src.offset(len);
            for _ in 0..len {
                dst -= 1;
                s -= 1;
                dst.write(s.read());
            }
            return;
        }
        let mut dst = self.clone();
        let mut i: usize = 0;
        for value in src {
            if i >= len {
                break;
            }
            dst.write(value.read());
            dst += 1;
            i += 1;
        }
        assert_eq!(i, len, "ub: memcpy");
    }

    #[allow(clippy::explicit_counter_loop)]
    pub fn memset(&self, value: u8, num: usize) {
        let mut dst = self.clone();
        for _ in 0..num {
            dst.write(value);
            dst += 1;
        }
    }

    #[allow(clippy::explicit_counter_loop)]
    pub fn memcmp(&self, other: &Self, len: usize) -> i32 {
        let mut a = self.clone();
        let mut b = other.clone();
        for _ in 0..len {
            let va = a.read();
            let vb = b.read();
            if va != vb {
                return (va as i32).wrapping_sub(vb as i32);
            }
            a += 1;
            b += 1;
        }
        0
    }

    pub fn slice_until(&self, end: &Self) -> Vec<u8> {
        assert!(self.kind == end.kind, "ub: invalid slice");
        let start: usize = self.offset;
        let end: usize = end.offset;
        assert!(start <= end);
        assert!(end <= self.len());
        match self.kind {
            PtrKind::Null => panic!("ub: dereference of null pointer"),
            PtrKind::StackSingle(_) | PtrKind::HeapSingle(_) => {
                if start < end {
                    vec![self.read()]
                } else {
                    Vec::new()
                }
            }
            PtrKind::Vec(ref weak) => {
                let strong = weak.upgrade().expect("ub: dangling pointer");
                let raw = strong.borrow();
                raw[start..end].to_vec()
            }
            PtrKind::StackArray(ref weak) | PtrKind::HeapArray(ref weak) => {
                let strong = weak.upgrade().expect("ub: dangling pointer");
                let raw = strong.borrow();
                raw[start..end].to_vec()
            }
            PtrKind::Reinterpreted(ref data) => {
                let mut buf = vec![0u8; end.wrapping_sub(start)];
                data.alloc.read_bytes(start, &mut buf);
                buf
            }
        }
    }

    #[inline]
    pub fn from_string_literal(s: &'static [u8]) -> Self {
        STRING_LITERALS.with(|literals| {
            let mut literals = literals.borrow_mut();
            let weak = Rc::downgrade(literals.entry(s).or_insert_with(|| {
                Rc::new(RefCell::new({
                    let mut v = s.to_vec();
                    v.push(0);
                    v
                }))
            }));
            Ptr {
                offset: 0,
                kind: PtrKind::Vec(weak),
            }
        })
    }

    pub fn to_c_string_iterator(&self) -> CStringIterator {
        CStringIterator { ptr: self.clone() }
    }

    pub fn to_rust_string(&self) -> String {
        let bytes: Vec<u8> = self.to_c_string_iterator().collect();
        String::from_utf8_lossy(&bytes).into_owned()
    }
}

pub struct CStringIterator {
    ptr: Ptr<u8>,
}

impl Iterator for CStringIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        // read until the null terminator
        match self.ptr.read() {
            0 => None,
            ch => {
                self.ptr += 1;
                Some(ch)
            }
        }
    }
}
