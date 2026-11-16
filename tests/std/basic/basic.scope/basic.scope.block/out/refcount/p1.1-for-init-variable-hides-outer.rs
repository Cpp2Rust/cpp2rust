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
    let mut index: i32 = 42;
    let mut iterations: i32 = 0;
    let mut sum: i32 = 0;
    {
        let mut index: i32 = 0;
        'loop_: while (index < 3) {
            iterations.prefix_inc();
            sum += index;
            index.prefix_inc();
        }
    }
    assert!((iterations == 3));
    assert!((sum == 3));
    assert!((index == 42));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
