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
    let N: Value<i32> = Rc::new(RefCell::new(5));
    let mut arr1: [i32; 5] = [0, 0, 0, 0, 0];
    let mut arr2: [i32; 5] = [1, 1, 1, 1, 1];
    let mut i: i32 = 0;
    'loop_: while (i < (*N.borrow())) {
        arr1[(i) as usize] = { (i + arr2[(i) as usize]) };
        i.prefix_inc();
    }
    let mut fatorial: i32 = 1;
    let mut i: i32 = 0;
    'loop_: while (i < (*N.borrow())) {
        fatorial *= arr1[(i) as usize];
        i.prefix_inc();
    }
    assert!((fatorial == 120));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
