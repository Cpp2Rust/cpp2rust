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
    let arr: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
        (0..10_usize)
            .map(|_| <i32>::default())
            .collect::<Box<[i32]>>(),
    )));
    let ptr: Value<Ptr<i32>> = Rc::new(RefCell::new((*arr.borrow()).offset((1) as isize)));
    let out: Value<i32> = Rc::new(RefCell::new(((*ptr.borrow()).read())));
    (*ptr.borrow()).delete();
    return (*out.borrow());
}
pub fn __cpp2rust_init_globals() {}
