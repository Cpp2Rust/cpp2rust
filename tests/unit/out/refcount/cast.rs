extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type E = u32;
pub const E_A: E = 0;
pub const E_B: E = 1;
pub const E_C: E = 2;
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let size: Value<usize> = Rc::new(RefCell::new(1_usize));
    assert!(((*size.borrow()) == 1_usize));
    let ul: Value<u64> = Rc::new(RefCell::new(5_u64));
    let s1: Value<usize> = Rc::new(RefCell::new(((*ul.borrow()) as usize)));
    assert!(((*s1.borrow()) == 5_usize));
    (*ul.borrow_mut()) = (7_usize as u64);
    assert!(((*ul.borrow()) == 7_u64));
    let i: Value<i32> = Rc::new(RefCell::new(2));
    let e: Value<E> = Rc::new(RefCell::new(((*i.borrow()) as E)));
    assert!((((*e.borrow()) as i32) == (E_C as i32)));
    assert!((((*e.borrow()) as i32) == 2));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
