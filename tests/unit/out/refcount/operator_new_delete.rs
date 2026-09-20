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
    libcc2rs::free_refcount(((*a.borrow()).clone() as Ptr<i32>).to_any());
    let arr: Value<Ptr<i32>> = Rc::new(RefCell::new(
        libcc2rs::malloc_refcount((::std::mem::size_of::<i32>() as usize).wrapping_mul(2_usize))
            .reinterpret_cast::<i32>(),
    ));
    (*arr.borrow()).offset((0) as isize).write(0);
    (*arr.borrow()).offset((1) as isize).write(1);
    assert!(
        ((((*arr.borrow()).offset((0) as isize).read())
            + ((*arr.borrow()).offset((1) as isize).read()))
            == 1)
    );
    libcc2rs::free_refcount(((*arr.borrow()).clone() as Ptr<i32>).to_any());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
