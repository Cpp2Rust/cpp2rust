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
    let mut a: Ptr<i32> = libcc2rs::malloc_refcount(4usize).reinterpret_cast::<i32>();
    a.write(42);
    assert!(((a.read()) == 42));
    libcc2rs::free_refcount((a).to_any());
    let mut arr: Ptr<i32> = libcc2rs::malloc_refcount((4usize as usize).wrapping_mul(2_usize))
        .reinterpret_cast::<i32>();
    elem!(arr, 0).write(0);
    elem!(arr, 1).write(1);
    assert!((((elem!(arr, 0).read()) + (elem!(arr, 1).read())) == 1));
    libcc2rs::free_refcount((arr).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
