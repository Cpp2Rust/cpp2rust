extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct base {
    #[offset(0)]
    pub kind: i32,
}
impl ByteRepr for base {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.kind.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            kind: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct derived {
    #[offset(0)]
    pub head: base,
    #[offset(8)]
    pub value: usize,
}
impl ByteRepr for derived {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.head.to_bytes(&mut buf[0..4]);
        self.value.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            head: <base>::from_bytes(&buf[0..4]),
            value: <usize>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<Ptr<derived>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(16usize).reinterpret_cast::<derived>(),
    ));
    assert!((((!((*d.borrow()).is_null())) as i32) != 0));
    field!(field!((*d.borrow()), head), kind).write(3);
    field!((*d.borrow()), value).write(7_usize);
    let b: Value<Ptr<base>> = Rc::new(RefCell::new((field_ptr!((*d.borrow()), head))));
    let back: Value<Ptr<derived>> =
        Rc::new(RefCell::new((*b.borrow()).reinterpret_cast::<derived>()));
    assert!(
        ((({
            let _lhs = (*back.borrow()).clone();
            _lhs == (*d.borrow()).clone()
        }) as i32)
            != 0)
    );
    assert!(((((*back.borrow()).with(|__s| __s.value) == 7_usize) as i32) != 0));
    assert!(((((*back.borrow()).with(|__s| __s.head.kind) == 3) as i32) != 0));
    field!((*back.borrow()), value).write(8_usize);
    assert!(((((*d.borrow()).with(|__s| __s.value) == 8_usize) as i32) != 0));
    field!((*b.borrow()), kind).write(4);
    assert!(((((*d.borrow()).with(|__s| __s.head.kind) == 4) as i32) != 0));
    libcc2rs::free_refcount((*back.borrow()).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
