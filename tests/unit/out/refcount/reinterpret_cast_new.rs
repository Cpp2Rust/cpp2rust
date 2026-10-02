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
    let p: Value<Ptr<u32>> = Rc::new(RefCell::new(Ptr::alloc(67305985_u32)));
    let bytes: Value<Ptr<u8>> = Rc::new(RefCell::new((*p.borrow()).reinterpret_cast::<u8>()));
    assert!((((elem!((*bytes.borrow()), 0).read()) as i32) == 1));
    assert!((((elem!((*bytes.borrow()), 1).read()) as i32) == 2));
    assert!((((elem!((*bytes.borrow()), 2).read()) as i32) == 3));
    assert!((((elem!((*bytes.borrow()), 3).read()) as i32) == 4));
    elem!((*bytes.borrow()), 0).write(16_u8);
    assert!((((*p.borrow()).read()) == 67306000_u32));
    (*p.borrow()).delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
