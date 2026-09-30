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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let x: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc(5)));
    let out: Value<i32> = Rc::new(RefCell::new(((*x.borrow()).read())));
    (*x.borrow()).delete();
    assert!(((*out.borrow()) == 5));
    let y: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc(Default::default())));
    (*y.borrow()).write(9);
    assert!((((*y.borrow()).read()) == 9));
    (*y.borrow()).delete();
    let p: Value<Ptr<Pair>> = Rc::new(RefCell::new(Ptr::alloc(<Pair>::default())));
    field!((*p.borrow()), x).write(1);
    field!((*p.borrow()), y).write(2);
    assert!(
        ({
            let _lhs = (*p.borrow()).with(|__s| __s.x);
            _lhs + (*p.borrow()).with(|__s| __s.y)
        } == 3)
    );
    (*p.borrow()).delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
