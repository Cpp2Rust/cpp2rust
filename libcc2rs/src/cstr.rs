// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::CStringIterator;
use crate::rc::{Ptr, PtrKind};
use crate::reinterpret::{ByteRepr, with_scratch};

// The element types of C strings: `char` and `signed char` map to `i8`,
// `unsigned char` to `u8`.
pub trait CChar: Copy + Default + PartialEq + ByteRepr + 'static {
    fn to_byte(self) -> u8;
    fn from_byte(b: u8) -> Self;
    // Calls `f` with `buf` viewed as a slice of `Self`; changes are copied back.
    fn with_bytes_mut<R>(buf: &mut [u8], f: impl FnOnce(&mut [Self]) -> R) -> R;
    // Calls `f` with `s` viewed as bytes. Free for u8; i8 needs a copy, which
    // lives on the stack for short slices.
    fn with_u8_slice<R>(s: &[Self], f: impl FnOnce(&[u8]) -> R) -> R;
    // Conversions between vectors of Self and of bytes, without reallocating.
    fn into_byte_vec(v: Vec<Self>) -> Vec<u8>;
    fn from_byte_vec(v: Vec<u8>) -> Vec<Self>;

    // The array initialized by a narrow string literal, e.g., char s[] = "abc".
    fn array_from_literal<const N: usize>(s: &[u8; N]) -> Box<[Self]> {
        Box::new(s.map(Self::from_byte))
    }
}

impl CChar for u8 {
    #[inline(always)]
    fn to_byte(self) -> u8 {
        self
    }
    #[inline(always)]
    fn from_byte(b: u8) -> Self {
        b
    }
    #[inline(always)]
    fn with_bytes_mut<R>(buf: &mut [u8], f: impl FnOnce(&mut [Self]) -> R) -> R {
        f(buf)
    }
    #[inline(always)]
    fn with_u8_slice<R>(s: &[Self], f: impl FnOnce(&[u8]) -> R) -> R {
        f(s)
    }
    #[inline(always)]
    fn into_byte_vec(v: Vec<Self>) -> Vec<u8> {
        v
    }
    #[inline(always)]
    fn from_byte_vec(v: Vec<u8>) -> Vec<Self> {
        v
    }
}

impl CChar for i8 {
    #[inline(always)]
    fn to_byte(self) -> u8 {
        self as u8
    }
    #[inline(always)]
    fn from_byte(b: u8) -> Self {
        b as i8
    }
    fn with_bytes_mut<R>(buf: &mut [u8], f: impl FnOnce(&mut [Self]) -> R) -> R {
        with_scratch(buf.len(), |chars: &mut [i8]| {
            for (c, &b) in chars.iter_mut().zip(buf.iter()) {
                *c = b as i8;
            }
            let r = f(chars);
            for (b, &c) in buf.iter_mut().zip(chars.iter()) {
                *b = c as u8;
            }
            r
        })
    }
    fn with_u8_slice<R>(s: &[Self], f: impl FnOnce(&[u8]) -> R) -> R {
        with_scratch(s.len(), |buf: &mut [u8]| {
            for (b, &c) in buf.iter_mut().zip(s) {
                *b = c as u8;
            }
            f(buf)
        })
    }
    // Both collects reuse the allocation, as i8 and u8 have the same layout.
    fn into_byte_vec(v: Vec<Self>) -> Vec<u8> {
        v.into_iter().map(|c| c as u8).collect()
    }
    fn from_byte_vec(v: Vec<u8>) -> Vec<Self> {
        v.into_iter().map(|b| b as i8).collect()
    }
}

impl<T: CChar> fmt::Display for Ptr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            PtrKind::Null => write!(f, "NULL"),
            _ => {
                for value in self {
                    let ch = value.read().to_byte();
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

type LiteralCache<K, T> = RefCell<HashMap<&'static [K], Rc<RefCell<Box<[T]>>>>>;

macro_rules! impl_string_literal {
    ($t:ty, $cache:ident) => {
        thread_local! {
            static $cache: LiteralCache<$t, $t> =
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

thread_local! {
    static STRING_LITERALS_I8: LiteralCache<u8, i8> =
        RefCell::new(HashMap::new());
}

impl Ptr<i8> {
    // Narrow string literals are emitted as byte strings (b"..."), as Rust has
    // no literal syntax for `[i8]`.
    #[inline]
    pub fn from_string_literal(s: &'static [u8]) -> Self {
        STRING_LITERALS_I8.with(|literals| {
            let mut literals = literals.borrow_mut();
            let weak = Rc::downgrade(literals.entry(s).or_insert_with(|| {
                Rc::new(RefCell::new(
                    s.iter()
                        .map(|&c| c as i8)
                        .chain(std::iter::once(0))
                        .collect(),
                ))
            }));
            Ptr {
                offset: 0,
                kind: PtrKind::StackArray(weak),
            }
        })
    }
}

impl<T: CChar> Ptr<T> {
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
    pub fn memset(&self, value: T, num: usize) {
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
            // Bytes compare as unsigned char.
            let va = a.read().to_byte();
            let vb = b.read().to_byte();
            if va != vb {
                return (va as i32).wrapping_sub(vb as i32);
            }
            a += 1;
            b += 1;
        }
        0
    }

    pub fn to_c_string_iterator(&self) -> CStringIterator<T> {
        CStringIterator { ptr: self.clone() }
    }

    // Calls `f` with the bytes of the C string, excluding the terminating NUL.
    // The bytes are borrowed from the allocation, so `f` must not write to it.
    pub fn with_c_str<R>(&self, f: impl FnOnce(&[T]) -> R) -> R {
        fn until_nul<T: CChar>(tail: &[T]) -> &[T] {
            match tail.iter().position(|&b| b == T::default()) {
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
            PtrKind::Field(root) => {
                let root = crate::field::upgrade(root);
                let b = Ptr::<T>::borrow_field(&*root, self.offset);
                f(until_nul(std::slice::from_ref(&*b)))
            }
            PtrKind::Reinterpreted(_) => f(&self.to_c_string_iterator().collect::<Vec<T>>()),
        }
    }

    // The length of the C string, like `strlen`.
    pub fn c_str_len(&self) -> usize {
        self.with_c_str(|s| s.len())
    }

    // Copies the C string, excluding the terminating NUL, using a single
    // allocation. The vector has room for one more byte, which is what callers
    // usually append (a NUL or a newline).
    pub fn to_c_bytes(&self) -> Vec<T> {
        self.with_c_str(|s| {
            let mut bytes = Vec::with_capacity(s.len() + 1);
            bytes.extend_from_slice(s);
            bytes
        })
    }

    // Copies `bytes` to the buffer pointed to, without adding a NUL.
    pub fn write_c_bytes(&self, bytes: &[u8]) {
        self.with_slice_mut(bytes.len(), |s| {
            for (d, &b) in s.iter_mut().zip(bytes) {
                *d = T::from_byte(b);
            }
        })
    }

    // Allocates a heap copy of `bytes` followed by a NUL, like `strdup`.
    pub fn alloc_c_str(bytes: &[u8]) -> Self {
        Ptr::alloc_array(
            bytes
                .iter()
                .map(|&b| T::from_byte(b))
                .chain(std::iter::once(T::default()))
                .collect(),
        )
    }

    // Like `with_c_str`, with the C string viewed as bytes.
    pub fn with_c_bytes<R>(&self, f: impl FnOnce(&[u8]) -> R) -> R {
        self.with_c_str(|s| T::with_u8_slice(s, f))
    }

    // Like `to_c_bytes`, as bytes.
    pub fn to_c_u8_bytes(&self) -> Vec<u8> {
        T::into_byte_vec(self.to_c_bytes())
    }

    pub fn to_rust_string(&self) -> String {
        self.with_c_bytes(|s| String::from_utf8_lossy(s).into_owned())
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
    fn signed_char_strings() {
        let p = Ptr::<i8>::from_string_literal(b"\xe9a");
        assert_eq!(p.read(), -23);
        assert_eq!(p.c_str_len(), 2);
        assert_eq!(p.to_c_bytes(), [-23, b'a' as i8]);
        assert_eq!(p.to_c_u8_bytes(), b"\xe9a");
        // memcmp compares bytes as unsigned char.
        let q = Ptr::<i8>::from_string_literal(b"a");
        assert!(p.memcmp(&q, 1) > 0);

        let buf = Ptr::<i8>::alloc_c_str(b"xyz");
        buf.write_c_bytes(b"\xff");
        assert_eq!(buf.read(), -1);
        assert_eq!(buf.to_c_u8_bytes(), b"\xffyz");
        buf.delete();
    }

    #[test]
    #[should_panic(expected = "ub: unterminated string")]
    fn unterminated_string_panics() {
        let p = Ptr::alloc_array(vec![b'a', b'b'].into_boxed_slice());
        p.c_str_len();
    }
}
