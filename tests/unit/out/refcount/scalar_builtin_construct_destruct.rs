extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Pod {
    #[offset(0)]
    pub v: i32,
}
pub fn zero_0() -> Ptr<i32> {
    return Ptr::<i32>::null();
}
pub fn zero_1() -> i64 {
    return 0_i64;
}
pub fn destroy_2(mut p: Ptr<i32>) {}
pub fn destroy_3(mut p: Ptr<Pod>) {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut i: i32 = 0_i32;
    let mut d: f64 = 0_f64;
    let mut p: Ptr<i32> = ({ zero_0() });
    assert!((i == 0));
    assert!((d == 0_f64));
    assert!((p).is_null());
    assert!((({ zero_1() }) == 0_i64));
    let x: Value<i32> = Rc::new(RefCell::new(5));
    ({ destroy_2((x.as_pointer())) });
    assert!(((*x.borrow()) == 5));
    let pod: Value<Pod> = Rc::new(RefCell::new(Pod { v: 7 }));
    ({ destroy_3((pod.as_pointer())) });
    assert!(({ (*pod.borrow()).v } == 7));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
