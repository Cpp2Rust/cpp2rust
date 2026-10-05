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
pub fn double_it_0(mut x: i32) -> i32 {
    return (x * 2);
}
pub fn negate_1(mut x: i32) -> i32 {
    return -x;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S {}
impl S {
    pub fn pick_1(mut x: i32) -> i32 {
        return (x + 1);
    }
    pub fn pick_2(mut x: i64) -> i32 {
        return ((x as i32) + 2);
    }
    pub fn solo(mut x: i32) -> i32 {
        return (x + 3);
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut p1: FnPtr<fn(i32) -> i32> = (FnPtr::<fn(i32) -> i32>::new(S::pick_1));
    let mut p2: FnPtr<fn(i32) -> i32> = FnPtr::<fn(i32) -> i32>::new(S::solo);
    assert!((({ p1.call(5,) }) == 6));
    assert!((({ p2.call(5,) }) == 8));
    assert!((({ S::pick_2(5_i64,) }) == 7));
    let mut h3: Handler = Handler {
        tag: 3,
        cb: (FnPtr::<fn(i32) -> i32>::new(S::pick_1)),
    };
    assert!((({ h3.cb.call(1,) }) == 2));
    let mut h1: Handler = Handler {
        tag: 1,
        cb: FnPtr::<fn(i32) -> i32>::new(double_it_0),
    };
    let mut h2: Handler = Handler {
        tag: 2,
        cb: FnPtr::<fn(i32) -> i32>::new(negate_1),
    };
    assert!(!((h1.cb).is_null()));
    assert!((({ h1.cb.call(5,) }) == 10));
    assert!((({ h2.cb.call(7,) }) == -7_i32));
    h1.cb = FnPtr::<fn(i32) -> i32>::new(negate_1);
    assert!((({ h1.cb.call(3,) }) == -3_i32));
    assert!(({ (h1.cb).clone() } == { (h2.cb).clone() }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
