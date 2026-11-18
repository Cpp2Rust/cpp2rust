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
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
    let mut idx: usize = (*v.borrow()).len();
    let mut p: Ptr<i32> = ({
        if idx >= (*v.borrow()).len() {
            panic!("out of bounds access")
        }
        v.decay().offset((idx) as isize)
    });
    return (((p).is_null()) as i32);
}
pub fn __cpp2rust_init_globals() {}
