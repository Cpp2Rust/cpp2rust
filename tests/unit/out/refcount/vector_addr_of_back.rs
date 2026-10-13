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
    let outer: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
    let mut inner: Vec<i32> = Vec::new();
    {
        let a0_clone = inner.clone();
        (*outer.borrow_mut()).push(a0_clone)
    };
    let mut sink: Ptr<Vec<i32>> = ((outer.as_pointer() as Ptr<Vec<i32>>).to_last());
    assert!(((*sink.upgrade().deref()).len() == 0_usize));
    let mut p: Ptr<Vec<Vec<i32>>> = (outer.as_pointer());
    sink = ((Ptr::<Vec<Vec<i32>>>::decay(&(p)) as Ptr<Vec<i32>>).to_last());
    assert!(((*sink.upgrade().deref()).len() == 0_usize));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
