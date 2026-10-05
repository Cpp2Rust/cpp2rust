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
    let mut x1: i32 = 1;
    let mut x2: i16 = 2_i16;
    let mut x3: u32 = 4_u32;
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 1;
        (*v.borrow_mut()).push(__a1)
    };
    {
        let __a1 = 2;
        (*v.borrow_mut()).push(__a1)
    };
    let mut sum: i32 = 0;
    'loop_: for mut elem in v.as_pointer() as Ptr<i32> {
        let mut elem: i32 = elem.read();
        sum += elem;
    }
    assert!((sum == 3));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
