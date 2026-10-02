extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..2_usize).map(|_| 0_u32).collect::<Box<[u32]>>(),
    )));
    elem!((*arr.borrow()), 0).write(67305985_u32);
    elem!((*arr.borrow()), 1).write(134678021_u32);
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new((*arr.borrow()).reinterpret_cast::<u8>()));
    assert!((((elem!((*bytes.borrow()), 0).read()) as i32) == 1));
    assert!((((elem!((*bytes.borrow()), 4).read()) as i32) == 5));
    assert!((((elem!((*bytes.borrow()), 7).read()) as i32) == 8));
    elem!((*bytes.borrow()), 0).write(170_u8);
    assert!(((elem!((*arr.borrow()), 0).read()) == 67306154_u32));
    elem!((*bytes.borrow()), 5).write(187_u8);
    assert!(((elem!((*arr.borrow()), 1).read()) == 134724357_u32));
    (*arr.borrow()).delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
