// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::CStringIterator;
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

macro_rules! impl_string_literal {
    ($t:ty, $cache:ident) => {
        thread_local! {
            static $cache: RefCell<HashMap<&'static [$t], Rc<RefCell<Box<[$t]>>>>> =
                RefCell::new(HashMap::new());
        }

        impl Ptr<$t> {
            #[inline]
            pub fn from_string_literal(s: &'static [$t]) -> Self {
                $cache.with(|literals| {
                    let mut literals = literals.borrow_mut();
                    let weak = Rc::downgrade(literals.entry(s).or_insert_with(|| {
                        Rc::new(RefCell::new({
                            let mut v = Vec::with_capacity(s.len() + 1);
                            v.extend_from_slice(s);
                            v.push(0);
                            v.into_boxed_slice()
                        }))
                    }));
                    Ptr {
                        offset: 0,
                        kind: PtrKind::StackArray(weak),
                    }
                })
            }
        }
    };
}

impl_string_literal!(u8, STRING_LITERALS_U8);
impl_string_literal!(u16, STRING_LITERALS_U16);
impl_string_literal!(u32, STRING_LITERALS_U32);
impl_string_literal!(i32, STRING_LITERALS_I32);

impl Ptr<u8> {
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

    pub fn to_c_string_iterator(&self) -> CStringIterator {
        CStringIterator { ptr: self.clone() }
    }

    // Calls `f` with the bytes of the C string, excluding the terminating NUL.
    // The bytes are borrowed from the allocation, so `f` must not write to it.
    pub fn with_c_str<R>(&self, f: impl FnOnce(&[u8]) -> R) -> R {
        fn until_nul(tail: &[u8]) -> &[u8] {
            match tail.iter().position(|&b| b == 0) {
                Some(len) => &tail[..len],
                None => panic!("ub: unterminated string"),
            }
        }
        match &self.kind {
            PtrKind::Null => panic!("ub: null pointer"),
            PtrKind::StackSingle(weak) | PtrKind::HeapSingle(weak) => {
                assert_eq!(self.offset, 0, "ub: invalid offset");
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let b = rc.borrow();
                f(until_nul(std::slice::from_ref(&*b)))
            }
            PtrKind::StackArray(weak) | PtrKind::HeapArray(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let b = rc.borrow();
                f(until_nul(&b[self.offset..]))
            }
            PtrKind::StackVec(weak) | PtrKind::HeapVec(weak) => {
                let rc = weak.upgrade().expect("ub: dangling pointer");
                let b = rc.borrow();
                f(until_nul(&b[self.offset..]))
            }
            PtrKind::Reinterpreted(_) => f(&self.to_c_string_iterator().collect::<Vec<u8>>()),
        }
    }

    // The length of the C string, like `strlen`.
    pub fn c_str_len(&self) -> usize {
        self.with_c_str(|s| s.len())
    }

    // Copies the C string, excluding the terminating NUL, using a single
    // allocation. The vector has room for one more byte, which is what callers
    // usually append (a NUL or a newline).
    pub fn to_c_bytes(&self) -> Vec<u8> {
        self.with_c_str(|s| {
            let mut bytes = Vec::with_capacity(s.len() + 1);
            bytes.extend_from_slice(s);
            bytes
        })
    }

    pub fn to_rust_string(&self) -> String {
        self.with_c_str(|s| String::from_utf8_lossy(s).into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsPointer;

    // "ab\0cd\0": C string functions must stop at the first NUL.
    const EMBEDDED_NUL: [u8; 6] = [b'a', b'b', 0, b'c', b'd', 0];

    fn check(p: &Ptr<u8>) {
        assert_eq!(p.c_str_len(), 2);
        assert_eq!(p.to_c_bytes(), b"ab");
        assert_eq!(p.to_rust_string(), "ab");
        assert_eq!(p.to_c_string_iterator().count(), 2);
        assert_eq!(p.to_c_string_iterator().collect::<Vec<u8>>(), b"ab");
        assert_eq!(p.with_c_str(|s| s.to_vec()), b"ab");
        assert_eq!(p.offset(3).to_c_bytes(), b"cd");
        assert_eq!(p.offset(2).c_str_len(), 0);
    }

    #[test]
    fn stops_at_first_nul_in_array() {
        let p = Ptr::alloc_array(EMBEDDED_NUL.to_vec().into_boxed_slice());
        check(&p);
        p.delete();
    }

    #[test]
    fn stops_at_first_nul_in_vec() {
        let v = std::rc::Rc::new(RefCell::new(EMBEDDED_NUL.to_vec()));
        check(&v.as_pointer());
    }

    #[test]
    fn stops_at_first_nul_in_reinterpreted_view() {
        let p: Ptr<u16> = Ptr::alloc_array(
            EMBEDDED_NUL
                .chunks(2)
                .map(|c| u16::from_ne_bytes([c[0], c[1]]))
                .collect::<Vec<u16>>()
                .into_boxed_slice(),
        );
        let bytes = p.reinterpret_cast::<u8>();
        check(&bytes);
        bytes.delete();
    }

    #[test]
    fn empty_string() {
        let p = Ptr::<u8>::from_string_literal(b"");
        assert_eq!(p.c_str_len(), 0);
        assert!(p.to_c_bytes().is_empty());
        assert_eq!(p.to_rust_string(), "");
    }

    #[test]
    #[should_panic(expected = "ub: unterminated string")]
    fn unterminated_string_panics() {
        let p = Ptr::alloc_array(vec![b'a', b'b'].into_boxed_slice());
        p.c_str_len();
    }
}
