extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Handler {
    #[offset(0)]
    pub tag: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub cb: FnPtr<fn(i32) -> i32>,
}
pub fn double_it_0(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return ((*x.borrow()) * 2);
}
pub fn negate_1(x: i32) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    return -(*x.borrow());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S {}
impl S {
    pub fn pick_1(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + 1);
    }
    pub fn pick_2(x: i64) -> i32 {
        let x: Value<i64> = Rc::new(RefCell::new(x));
        return (((*x.borrow()) as i32) + 2);
    }
    pub fn solo(x: i32) -> i32 {
        let x: Value<i32> = Rc::new(RefCell::new(x));
        return ((*x.borrow()) + 3);
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let p1: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new((FnPtr::<fn(i32) -> i32>::new(S::pick_1))));
    let p2: Value<FnPtr<fn(i32) -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::new(S::solo)));
    assert!((({ (*p1.borrow()).call(5,) }) == 6));
    assert!((({ (*p2.borrow()).call(5,) }) == 8));
    assert!((({ S::pick_2(5_i64,) }) == 7));
    let h3: Value<Handler> = Rc::new(RefCell::new(Handler {
        tag: 3,
        cb: (FnPtr::<fn(i32) -> i32>::new(S::pick_1)),
    }));
    assert!((({ { (*h3.borrow()).cb.clone() }.call(1,) }) == 2));
    let h1: Value<Handler> = Rc::new(RefCell::new(Handler {
        tag: 1,
        cb: FnPtr::<fn(i32) -> i32>::new(double_it_0),
    }));
    let h2: Value<Handler> = Rc::new(RefCell::new(Handler {
        tag: 2,
        cb: FnPtr::<fn(i32) -> i32>::new(negate_1),
    }));
    assert!(!(({ (*h1.borrow()).cb.clone() }).is_null()));
    assert!((({ { (*h1.borrow()).cb.clone() }.call(5,) }) == 10));
    assert!((({ { (*h2.borrow()).cb.clone() }.call(7,) }) == -7_i32));
    (*h1.borrow_mut()).cb = FnPtr::<fn(i32) -> i32>::new(negate_1);
    assert!((({ { (*h1.borrow()).cb.clone() }.call(3,) }) == -3_i32));
    assert!(({ { (*h1.borrow()).cb.clone() } } == { { (*h2.borrow()).cb.clone() } }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
