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
    let a: Value<Ptr<i32>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount(::std::mem::size_of::<i32>()).reinterpret_cast::<i32>(),
    ));
    (*a.borrow()).write(42);
    assert!((((*a.borrow()).read()) == 42));
    libcc2rs::free_refcount((*a.borrow()).to_any());
    let arr: Value<Ptr<i32>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount((::std::mem::size_of::<i32>() as usize).wrapping_mul(2_usize))
            .reinterpret_cast::<i32>(),
    ));
    elem!((*arr.borrow()), 0).write(0);
    elem!((*arr.borrow()), 1).write(1);
    assert!((((elem!((*arr.borrow()), 0).read()) + (elem!((*arr.borrow()), 1).read())) == 1));
    libcc2rs::free_refcount((*arr.borrow()).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
