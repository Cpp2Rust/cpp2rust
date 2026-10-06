extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f_0(list: Vec<i32>) {
    let list: Value<Vec<i32>> = Rc::new(RefCell::new(list));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut i1: i32 = 3;
    let mut i2: i32 = 0_i32;
    let mut carr1: [i32; 2] = [1, 2];
    let mut carr2: [i32; 3] = [1, 0_i32, 0_i32];
    let arr: Value<Vec<i32>> = Rc::new(RefCell::new(vec![1, 2, 3]));
    let mut vec_: Vec<i32> = vec![1, 2, 3];
    ({ f_0(vec![1, 2, 3, 4]) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
