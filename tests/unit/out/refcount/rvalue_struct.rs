extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct S {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
}
impl S {
    pub fn new(a: i32, b: i32) -> Self {
        let a: Value<i32> = Rc::new(RefCell::new(a));
        let b: Value<i32> = Rc::new(RefCell::new(b));
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a: (*a.borrow()),
            b: (*b.borrow()),
        }));
        let this: Ptr<S> = __this.as_pointer();
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl ByteRepr for S {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..4]);
        self.b.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i32>::from_bytes(&buf[0..4]),
            b: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s1: Value<S> = Rc::new(RefCell::new(S::new({ 1 }, { 2 })));
    let s2: Ptr<S> = s1.as_pointer();
    assert!((s2.with(|__s: &S| __s.a) == 1));
    assert!((s2.with(|__s: &S| __s.b) == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
