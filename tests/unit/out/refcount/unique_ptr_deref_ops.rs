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
    let mut p: Option<Value<i32>> = Some(Rc::new(RefCell::new(10)));
    (*p.as_ref().unwrap().borrow_mut()) += 5;
    (*p.as_ref().unwrap().borrow_mut()) -= 3;
    (*p.as_ref().unwrap().borrow_mut()) *= 2;
    let mut q: Option<Value<i32>> = Some(Rc::new(RefCell::new(1)));
    let mut sum: i32 = ((*p.as_ref().unwrap().borrow()) + (*q.as_ref().unwrap().borrow()));
    assert!((sum == 25));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
