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
    let v: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::from([true])));
    assert!(v.decay().offset(0_usize as isize).read());
    let mut r: Ptr<bool> = v.decay().offset(0_usize as isize);
    let mut copy: Ptr<bool> = (r).clone();
    assert!(copy.read());
    return 0;
}
pub fn __cpp2rust_init_globals() {}
