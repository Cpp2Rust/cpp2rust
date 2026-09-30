extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Pair {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl ByteRepr for Pair {
    fn byte_size() -> usize {
        8
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.x.to_bytes(&mut buf[0..4]);
        self.y.to_bytes(&mut buf[4..8]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            x: <i32>::from_bytes(&buf[0..4]),
            y: <i32>::from_bytes(&buf[4..8]),
        }
    }
}
#[derive(Clone, Record, VaArg, FnPtrArg, Default)]
pub struct Triple {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
    #[offset(8)]
    pub p: Pair,
}
impl ByteRepr for Triple {
    fn byte_size() -> usize {
        16
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        self.a.to_bytes(&mut buf[0..4]);
        self.b.to_bytes(&mut buf[4..8]);
        self.p.to_bytes(&mut buf[8..16]);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        Self {
            a: <i32>::from_bytes(&buf[0..4]),
            b: <i32>::from_bytes(&buf[4..8]),
            p: <Pair>::from_bytes(&buf[8..16]),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Ptr<Pair>> = Rc::new(RefCell::new(Ptr::alloc(Pair { x: 1, y: 2 })));
    let out: Value<i32> = Rc::new(RefCell::new({
        let _lhs = (*p.borrow()).with(|__s: &Pair| __s.x);
        _lhs + (*p.borrow()).with(|__s: &Pair| __s.y)
    }));
    (*p.borrow()).delete();
    assert!(((*out.borrow()) == 3));
    let t: Value<Triple> = Rc::new(RefCell::new(Triple {
        a: 1,
        b: 0_i32,
        p: <Pair>::default(),
    }));
    assert!(({ (*t.borrow()).a } == 1));
    assert!(({ (*t.borrow()).b } == 0));
    assert!(({ (*t.borrow()).p.x } == 0) && ({ (*t.borrow()).p.y } == 0));
    let q: Value<Ptr<Triple>> = Rc::new(RefCell::new(Ptr::alloc(Triple {
        a: 2,
        b: 3,
        p: <Pair>::default(),
    })));
    assert!(((*q.borrow()).with(|__s: &Triple| __s.a) == 2));
    assert!(((*q.borrow()).with(|__s: &Triple| __s.b) == 3));
    assert!(
        ((*q.borrow()).with(|__s: &Triple| __s.p.x) == 0)
            && ((*q.borrow()).with(|__s: &Triple| __s.p.y) == 0)
    );
    (*q.borrow()).delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
