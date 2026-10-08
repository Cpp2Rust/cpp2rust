extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn only_default_0(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    'switch: {
        match { x } {
            _ => {
                r = 42;
                break 'switch;
            }
        }
    };
    return r;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ only_default_0(1,) }) == 42));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
