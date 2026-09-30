extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Triple {
    #[offset(0)]
    pub a: i32,
    #[offset(4)]
    pub b: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub p: Pair,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p: Value<Ptr<Pair>> = Rc::new(RefCell::new(Ptr::alloc(Pair { x: 1, y: 2 })));
    let out: Value<i32> = Rc::new(RefCell::new({
        let _lhs = (*p.borrow()).with(|__s| __s.x);
        _lhs + (*p.borrow()).with(|__s| __s.y)
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
    assert!(((*q.borrow()).with(|__s| __s.a) == 2));
    assert!(((*q.borrow()).with(|__s| __s.b) == 3));
    assert!(((*q.borrow()).with(|__s| __s.p.x) == 0) && ((*q.borrow()).with(|__s| __s.p.y) == 0));
    (*q.borrow()).delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
