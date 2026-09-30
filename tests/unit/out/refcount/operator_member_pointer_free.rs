extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Inner {
    #[offset(0)]
    pub x: i32,
}
impl ByteRepr for Inner {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg)]
pub struct S {
    #[offset(0)]
    pub data: Box<[i32]>,
    #[offset(12)]
    pub inner: Inner,
}
impl Default for S {
    fn default() -> Self {
        S {
            data: (0..3).map(|_| 0_i32).collect::<Box<[i32]>>(),
            inner: <Inner>::default(),
        }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.data.to_bytes(&mut buf[0..12]);
        self.inner.to_bytes(&mut buf[12..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            data: <Box<[i32]>>::from_bytes(&buf[0..12]),
            inner: <Inner>::from_bytes(&buf[12..16]),
        }
    }
}
pub fn operator_deref_0(s: Ptr<S>) -> Ptr<Inner> {
    return field_ptr!(s, inner);
}
pub fn operator_addr_1(s: Ptr<S>) -> Ptr<i32> {
    return ((field_ptr!(s, data) as Ptr<i32>).offset(0));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S {
        data: Box::new([1, 2, 3]),
        inner: Inner { x: 9 },
    }));
    assert!(
        (({
            let _s: Ptr<S> = s.as_pointer();
            operator_deref_0(_s)
        })
        .with(|__s: &Inner| __s.x)
            == 9)
    );
    ({
        let _s: Ptr<S> = s.as_pointer();
        operator_deref_0(_s)
    })
    .with_mut(|__s: &mut Inner| __s.x = 10);
    assert!(({ (*s.borrow()).inner.x } == 10));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(
        ({
            let _s: Ptr<S> = s.as_pointer();
            operator_addr_1(_s)
        }),
    ));
    assert!((((*p.borrow()).read()) == 1));
    (*p.borrow()).write(5);
    assert!(((*s.borrow()).data[(0) as usize] == 5));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
