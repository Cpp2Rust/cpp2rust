extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg)]
pub struct S {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: bool,
}
impl S {
    pub fn new() -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self { a: 11, b: true }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new() }
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..4]);
        self.b.to_bytes(&mut buf[4..5]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i32>::from_bytes(&buf[0..4]),
            b: <bool>::from_bytes(&buf[4..5]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Declared {
    #[offset(0)]
    pub v: i32,
}
impl Declared {}
impl ByteRepr for Declared {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.v.to_bytes(&mut buf[0..4]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            v: <i32>::from_bytes(&buf[0..4]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<Ptr<Declared>> = Rc::new(RefCell::new(Ptr::<Declared>::null()));
    assert!((*d.borrow()).is_null());
    let s: Value<S> = Rc::new(RefCell::new(S::new()));
    assert!(({ (*s.borrow()).a } == 11));
    assert!((({ (*s.borrow()).b } as i32) == (true as i32)));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
