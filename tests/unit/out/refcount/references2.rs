extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn change_0(p: Ptr<Option<Value<i32>>>) {
    let q: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(7)))));
    p.write((*q.borrow_mut()).take());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Option<Value<i32>>> = Rc::new(RefCell::new(Some(Rc::new(RefCell::new(5)))));
    ({ change_0(a.as_pointer()) });
    assert!(({ (((*a.borrow()).as_pointer().read()) == 7) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
