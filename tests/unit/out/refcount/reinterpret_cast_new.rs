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
    let mut p: Ptr<u32> = Ptr::alloc(67305985_u32);
    let mut bytes: Ptr<u8> = p.reinterpret_cast::<u8>();
    assert!((((elem!(bytes, 0).read()) as i32) == 1));
    assert!((((elem!(bytes, 1).read()) as i32) == 2));
    assert!((((elem!(bytes, 2).read()) as i32) == 3));
    assert!((((elem!(bytes, 3).read()) as i32) == 4));
    elem!(bytes, 0).write(16_u8);
    assert!(((p.read()) == 67306000_u32));
    p.delete();
    return 0;
}
pub fn __cpp2rust_init_globals() {}
