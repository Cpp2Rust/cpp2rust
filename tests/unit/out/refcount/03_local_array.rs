extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let arr1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2])));
    (*arr1.borrow_mut())[(0) as usize] = 3;
    (*arr1.borrow_mut())[(1) as usize] = 4;
    return ((*arr1.borrow())[(0) as usize] + (*arr1.borrow())[(1) as usize]);
}
